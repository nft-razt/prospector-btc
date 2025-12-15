// libs/infra/db-turso/src/repositories/job/mod.rs
// =================================================================
// APARATO: JOB REPOSITORY (ORCHESTRATOR)
// ESTADO: OPTIMIZADO (PADDED RANGES)
// =================================================================

pub mod math;
pub mod queries;

use anyhow::{anyhow, Context, Result};
use chrono::{Duration, Utc};
use libsql::{params, Connection};
use prospector_domain_models::{SearchStrategy, WorkOrder};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

// Usamos el módulo local con soporte de Padding
use self::math::calculate_next_range;
use self::queries as sql;

const ZOMBIE_THRESHOLD_MINUTES: i64 = 5;

pub struct JobRepository {
    conn: Connection,
}

impl JobRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Asigna un trabajo a un worker.
    /// Prioriza recuperar trabajos abandonados (Zombies) antes de generar nuevos.
    #[instrument(skip(self))]
    pub async fn assign_work(&self, worker_id: &str) -> Result<WorkOrder> {
        let zombie_threshold = Utc::now() - Duration::minutes(ZOMBIE_THRESHOLD_MINUTES);

        // Iniciamos transacción ACID
        let tx = self
            .conn
            .transaction()
            .await
            .context("Fallo al iniciar transacción en JobRepository")?;

        // 1. ESTRATEGIA DE RECUPERACIÓN (ZOMBIES)
        let mut rows = tx
            .query(
                sql::ACQUIRE_ZOMBIE_OR_PENDING,
                params![zombie_threshold.to_rfc3339()],
            )
            .await
            .context("Fallo query zombie")?;

        if let Some(row) = rows.next().await? {
            let id: String = row.get(0)?;
            let start: String = row.get(1)?;
            let end: String = row.get(2)?;

            tx.execute(sql::REVIVE_JOB, params![worker_id, id.clone()])
                .await
                .context("Fallo update zombie")?;

            tx.commit().await?;

            info!(
                "♻️ JOB RECUPERADO: {} -> {} [Range: {}...]",
                id,
                worker_id,
                &start[0..10]
            );

            return Ok(self.build_work_order(id, start, end));
        }

        // 2. ESTRATEGIA DE EXPANSIÓN (NUEVO RANGO)
        // Obtenemos el último rango registrado. Gracias al Padding en `math.rs`,
        // el ordenamiento lexicográfico de SQL ahora es matemáticamente correcto.
        let mut max_rows = tx.query(sql::GET_MAX_RANGE, ()).await?;
        let max_row = max_rows.next().await?;

        let last_end = if let Some(row) = max_row {
            Some(row.get::<String>(0)?)
        } else {
            None
        };

        // Delegamos el cálculo y padding al motor matemático
        let (start_str, end_str) =
            calculate_next_range(last_end).context("Error crítico en aritmética de rangos")?;

        let new_id = Uuid::new_v4().to_string();

        tx.execute(
            sql::INSERT_NEW_JOB,
            params![
                new_id.clone(),
                start_str.clone(),
                end_str.clone(),
                worker_id
            ],
        )
        .await
        .context("Fallo insert nuevo job")?;

        tx.commit().await?;

        info!(
            "✨ NUEVO TERRITORIO: {} [Start: {}...]",
            new_id,
            &start_str[0..10]
        );

        Ok(self.build_work_order(new_id, start_str, end_str))
    }

    /// Construye el DTO para el worker.
    fn build_work_order(&self, id: String, start: String, end: String) -> WorkOrder {
        WorkOrder {
            id,
            target_duration_sec: 600,
            strategy: SearchStrategy::Combinatoric {
                prefix: "".to_string(), // TODO: Configurable por campaña
                suffix: "".to_string(),
                start_index: start,
                end_index: end,
            },
        }
    }

    #[instrument(skip(self))]
    pub async fn heartbeat(&self, job_id: &str) -> Result<()> {
        let n = self.conn.execute(sql::HEARTBEAT, params![job_id]).await?;
        if n == 0 {
            warn!("⚠️ Heartbeat ignorado para job fantasma: {}", job_id);
            return Err(anyhow!("Job no encontrado o inactivo"));
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn complete(&self, job_id: &str) -> Result<()> {
        let n = self.conn.execute(sql::COMPLETE, params![job_id]).await?;
        if n == 0 {
            error!("❌ Intento de completar job inexistente: {}", job_id);
            return Err(anyhow!("Job no encontrado"));
        }
        Ok(())
    }
}
