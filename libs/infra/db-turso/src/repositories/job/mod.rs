// libs/infra/db-turso/src/repositories/job/mod.rs
// =================================================================
// APARATO: JOB REPOSITORY (ORCHESTRATOR)
// =================================================================

pub mod math;    // <--- Apunta a math.rs en la misma carpeta
pub mod queries; // <--- Apunta a queries.rs en la misma carpeta

use libsql::{Connection, params};
use anyhow::{Result, anyhow, Context};
use chrono::{Utc, Duration};
use uuid::Uuid;
use tracing::{info, instrument, warn, error};
use prospector_domain_models::{WorkOrder, SearchStrategy};

// Usamos el módulo local que acabamos de definir
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

    #[instrument(skip(self))]
    pub async fn assign_work(&self, worker_id: &str) -> Result<WorkOrder> {
        let zombie_threshold = Utc::now() - Duration::minutes(ZOMBIE_THRESHOLD_MINUTES);

        let tx = self.conn.transaction().await
            .context("Fallo al iniciar transacción en JobRepository")?;

        // 1. INTENTO DE RECUPERACIÓN (ZOMBIES)
        let mut rows = tx.query(
            sql::ACQUIRE_ZOMBIE_OR_PENDING,
            params![zombie_threshold.to_rfc3339()]
        ).await.context("Fallo query zombie")?;

        if let Some(row) = rows.next().await? {
            let id: String = row.get(0)?;
            let start: String = row.get(1)?;
            let end: String = row.get(2)?;

            tx.execute(
                sql::REVIVE_JOB,
                params![worker_id, id.clone()]
            ).await.context("Fallo update zombie")?;

            tx.commit().await?;

            info!("♻️ JOB RECUPERADO: {} -> {}", id, worker_id);

            return Ok(self.build_work_order(id, start, end));
        }

        // 2. GENERACIÓN DE TERRITORIO VIRGEN
        let mut max_rows = tx.query(sql::GET_MAX_RANGE, ()).await?;
        let max_row = max_rows.next().await?;

        let last_end = if let Some(row) = max_row {
            Some(row.get::<String>(0)?)
        } else {
            None
        };

        // Delegamos la matemática compleja al módulo 'math'
        let (start_str, end_str) = calculate_next_range(last_end)
            .context("Error crítico en aritmética de rangos")?;

        let new_id = Uuid::new_v4().to_string();

        tx.execute(
            sql::INSERT_NEW_JOB,
            params![
                new_id.clone(),
                start_str.clone(),
                end_str.clone(),
                worker_id
            ]
        ).await.context("Fallo insert nuevo job")?;

        tx.commit().await?;

        info!("✨ NUEVO TERRITORIO: {} [Start: {}]", new_id, start_str);

        Ok(self.build_work_order(new_id, start_str, end_str))
    }

    fn build_work_order(&self, id: String, start: String, end: String) -> WorkOrder {
        WorkOrder {
            id,
            target_duration_sec: 600,
            strategy: SearchStrategy::Combinatoric {
                prefix: "".to_string(),
                suffix: "".to_string(),
                start_index: start,
                end_index: end,
            }
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
