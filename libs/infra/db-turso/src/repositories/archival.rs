/**
 * =================================================================
 * APARATO: ARCHIVAL LEDGER REPOSITORY (V25.0 - ATOMIC)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: DRENAJE DE MISIONES CERTIFICADAS PARA L4
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use serde_json::{json, Value};
use tracing::{info, instrument};

pub struct ArchivalRepository {
    database_client: TursoClient,
}

impl ArchivalRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Recupera un lote de misiones que han sido completadas pero no migradas.
     * Sincronizado con el esquema V3.4.0.
     */
    #[instrument(skip(self))]
    pub async fn fetch_pending_strategic_migration(&self, batch_size: i32) -> Result<Vec<Value>, DbError> {
        let connection = self.database_client.get_connection()?;

        let query = r#"
            SELECT
                id, worker_id, total_hashes_effort, execution_duration_ms,
                audit_footprint_checkpoint, started_at, completed_at, strategy_type
            FROM jobs
            WHERE status = 'completed' AND archived_at IS NULL
            LIMIT ?1
        "#;

        let mut rows = connection.query(query, params![batch_size]).await?;
        let mut migration_batch = Vec::new();

        while let Some(row) = rows.next().await? {
            let entry = json!({
                "original_job_id": row.get::<String>(0)?,
                "worker_node_id": row.get::<String>(1)?,
                "computational_effort": row.get::<String>(2)?,
                "duration_ms": row.get::<i64>(3)?,
                "forensic_checkpoint": row.get::<String>(4)?,
                "timestamp_start": row.get::<String>(5)?,
                "timestamp_end": row.get::<String>(6)?,
                "strategy_applied": row.get::<String>(7)?
            });
            migration_batch.push(entry);
        }

        Ok(migration_batch)
    }

    /**
     * Sella los registros locales tras una migración exitosa al Cuartel General.
     */
    pub async fn seal_archived_records(&self, identifiers: Vec<String>) -> Result<(), DbError> {
        let connection = self.database_client.get_connection()?;

        for id in identifiers {
            connection.execute(
                "UPDATE jobs SET archived_at = CURRENT_TIMESTAMP WHERE id = ?1",
                params![id]
            ).await?;
        }

        Ok(())
    }
}
