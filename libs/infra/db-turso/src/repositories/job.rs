use libsql::{Connection, params};
use anyhow::{Result, anyhow};
use chrono::{Utc, Duration};
use uuid::Uuid;
use tracing::{info, instrument};
// Importamos la fuente de verdad única
use prospector_domain_models::{WorkOrder, SearchStrategy};

// Configuración de rangos (En Fase 3 esto vendrá de una tabla de configuración dinámica)
const RANGE_STEP_SIZE: u64 = 1_000_000_000;
const ZOMBIE_THRESHOLD_MINUTES: i64 = 5;

pub struct JobRepository {
    conn: Connection,
}

impl JobRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Asigna trabajo a un worker.
    /// Lógica:
    /// 1. Busca trabajos abandonados ("Zombies") para reasignar.
    /// 2. Si no hay, busca trabajos pendientes.
    /// 3. Si no hay, genera un nuevo rango virgen ("Lazy Generation").
    #[instrument(skip(self))]
    pub async fn assign_work(&self, worker_id: &str) -> Result<WorkOrder> {
        let zombie_threshold = Utc::now() - Duration::minutes(ZOMBIE_THRESHOLD_MINUTES);

        // Iniciamos una transacción explícita para evitar condiciones de carrera
        let tx = self.conn.transaction().await?;

        // 1. BÚSQUEDA DE CANDIDATOS (ZOMBIE O PENDING)
        // Usamos FOR UPDATE implícito al intentar actualizar después
        let candidate = tx.query_row(
            r#"
            SELECT id, range_start, range_end
            FROM jobs
            WHERE (status = 'processing' AND last_heartbeat_at < ?1)
               OR status = 'pending'
            ORDER BY range_start ASC
            LIMIT 1
            "#,
            params![zombie_threshold.to_rfc3339()]
        ).await;

        match candidate {
            Ok(row) => {
                // --- CAMINO A: RECUPERACIÓN ---
                let id: String = row.get(0)?;
                let start: String = row.get(1)?;
                let end: String = row.get(2)?;

                // Revivimos el trabajo y lo asignamos al nuevo worker
                tx.execute(
                    r#"
                    UPDATE jobs
                    SET worker_id = ?1, status = 'processing', last_heartbeat_at = CURRENT_TIMESTAMP, attempt_count = attempt_count + 1
                    WHERE id = ?2
                    "#,
                    params![worker_id, &id]
                ).await?;

                tx.commit().await?;

                info!("♻️ JOB RECUPERADO: {} (Asignado a {})", id, worker_id);

                Ok(WorkOrder {
                    id,
                    target_duration_sec: 600,
                    strategy: SearchStrategy::Combinatoric {
                        prefix: "".to_string(), // Configurable en futuro
                        suffix: "".to_string(),
                        start_index: start,
                        end_index: end,
                    }
                })
            }
            Err(_) => {
                // --- CAMINO B: GENERACIÓN DE TERRITORIO VIRGEN ---

                // Obtenemos el límite más alto explorado hasta ahora
                // Casteamos a INTEGER para la función MAX de SQL, pero manejamos como u64 en Rust
                let max_row = tx.query_row(
                    "SELECT MAX(CAST(range_end AS INTEGER)) FROM jobs",
                    ()
                ).await;

                let next_start: u64 = match max_row {
                    Ok(row) => {
                        let val: Option<i64> = row.get(0).unwrap_or(None);
                        match val {
                            Some(v) => (v as u64) + 1,
                            None => 0,
                        }
                    },
                    Err(_) => 0,
                };

                let next_end = next_start + RANGE_STEP_SIZE;
                let new_id = Uuid::new_v4().to_string();

                // Insertamos el nuevo trabajo. Guardamos los números como TEXT para seguridad futura.
                tx.execute(
                    r#"
                    INSERT INTO jobs (id, range_start, range_end, status, worker_id, started_at, last_heartbeat_at)
                    VALUES (?1, ?2, ?3, 'processing', ?4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                    "#,
                    params![
                        &new_id,
                        next_start.to_string(),
                        next_end.to_string(),
                        worker_id
                    ]
                ).await?;

                tx.commit().await?;

                info!("✨ NUEVO TERRITORIO GENERADO: {} [{} - {}]", new_id, next_start, next_end);

                Ok(WorkOrder {
                    id: new_id,
                    target_duration_sec: 600,
                    strategy: SearchStrategy::Combinatoric {
                        prefix: "".to_string(),
                        suffix: "".to_string(),
                        start_index: next_start.to_string(),
                        end_index: next_end.to_string(),
                    }
                })
            }
        }
    }

    /// Latido del trabajo (Keep-Alive).
    pub async fn heartbeat(&self, job_id: &str) -> Result<()> {
        let n = self.conn.execute(
            "UPDATE jobs SET last_heartbeat_at = CURRENT_TIMESTAMP WHERE id = ?1",
            params![job_id]
        ).await?;

        if n == 0 {
            return Err(anyhow!("Job {} no encontrado o ya completado", job_id));
        }
        Ok(())
    }

    /// Marcar trabajo como completado exitosamente (Commit).
    pub async fn complete(&self, job_id: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE jobs SET status = 'completed', completed_at = CURRENT_TIMESTAMP WHERE id = ?1",
            params![job_id]
        ).await?;
        Ok(())
    }
}
