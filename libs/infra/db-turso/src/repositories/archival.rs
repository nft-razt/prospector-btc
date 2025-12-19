// libs/infra/db-turso/src/repositories/archival.rs
/**
 * =================================================================
 * APARATO: ARCHIVAL REPOSITORY (V15.0 - ANALYTICS ENABLED)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (L3)
 * RESPONSABILIDAD: DRENAJE DE MÉTRICAS TÁCTICAS PARA L4
 * ESTADO: GOLD MASTER // NO ABBREVIATIONS
 * =================================================================
 */
use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use serde_json::{json, Value};
use tracing::{debug, error, instrument};

/// Repositorio especializado en la extracción masiva de datos para el Cuartel General (Supabase).
pub struct ArchivalRepository {
    client: TursoClient,
}

impl ArchivalRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { client }
    }

    /**
     * Recupera trabajos completados que aún no han sido migrados al archivo estratégico.
     *
     * # Parámetros
     * * `batch_size` - Cantidad de registros a procesar para optimizar el throughput de red.
     *
     * # Nivelación V15.0
     * Ahora extrae 'strategy_type' y 'total_hashes' generados por el Kernel Assembler.
     */
    #[instrument(skip(self))]
    pub async fn get_pending_migration(&self, batch_size: i32) -> Result<Vec<Value>, DbError> {
        let database_connection = self.client.get_connection()?;

        // Consulta alineada con SCHEMA_VERSION 3.3.0
        let query = r#"
            SELECT
                id, range_start, range_end, strategy_type,
                total_hashes, worker_id, started_at, completed_at
            FROM jobs
            WHERE status = 'completed' AND archived_at IS NULL
            LIMIT ?1
        "#;

        let mut database_rows = database_connection
            .query(query, params![batch_size])
            .await?;
        let mut migration_payload = Vec::new();

        while let Some(row) = database_rows.next().await? {
            // Construcción del DTO compatible con el esquema de Supabase (Postgres BIGINT)
            let entry = json!({
                "original_job_id": row.get::<String>(0)?,
                "range_start": row.get::<String>(1)?,
                "range_end": row.get::<String>(2)?,
                "strategy_type": row.get::<String>(3).unwrap_or_else(|_| "Combinatoric".into()),
                "total_hashes": row.get::<i64>(4).unwrap_or(0),
                "worker_id": row.get::<String>(5)?,
                "started_at": row.get::<String>(6)?,
                "completed_at": row.get::<String>(7)?
            });
            migration_payload.push(entry);
        }

        Ok(migration_payload)
    }

    /**
     * Sella los registros en Turso tras una migración exitosa a L4.
     * Previene la duplicidad de datos en el archivo estratégico.
     */
    pub async fn mark_as_archived(&self, job_identifiers: Vec<String>) -> Result<(), DbError> {
        let database_connection = self.client.get_connection()?;

        for identifier in job_identifiers {
            database_connection
                .execute(
                    "UPDATE jobs SET archived_at = CURRENT_TIMESTAMP WHERE id = ?1",
                    params![identifier],
                )
                .await?;
        }

        Ok(())
    }
}
