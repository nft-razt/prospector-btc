// libs/infra/db-turso/src/repositories/worker.rs
// =================================================================
// APARATO: WORKER SQL REPOSITORY
// RESPONSABILIDAD: PERSISTENCIA DE ESTADO DE NODOS
// ESTRATEGIA: TRANSACTIONAL BULK UPSERT
// =================================================================

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_models::WorkerHeartbeat;

pub struct WorkerRepository {
    client: TursoClient,
}

impl WorkerRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { client }
    }

    /// Ejecuta una actualización masiva de workers en una sola transacción ACID.
    /// Esto reduce drásticamente el Overhead de red (RTT) hacia Turso.
    pub async fn upsert_bulk(&self, workers: Vec<WorkerHeartbeat>) -> Result<usize, DbError> {
        if workers.is_empty() {
            return Ok(0);
        }

        let conn = self.client.get_connection()?;
        let tx = conn.transaction().await.map_err(DbError::QueryError)?;

        let query = r#"
            INSERT INTO workers (id, ip_address, version, status, last_seen_at, hashrate_avg, jobs_completed)
            VALUES (?1, 'unknown', 'v5.4', 'online', ?2, ?3, 0)
            ON CONFLICT(id) DO UPDATE SET
                last_seen_at = excluded.last_seen_at,
                hashrate_avg = excluded.hashrate_avg,
                status = 'online'
        "#;

        for w in &workers {
            // Convertimos UUID a string y DateTime a string ISO
            tx.execute(
                query,
                params![
                    w.worker_id.to_string(),
                    w.timestamp.to_rfc3339(),
                    w.hashrate as f64
                ],
            )
            .await
            .map_err(DbError::QueryError)?;
        }

        tx.commit().await.map_err(DbError::QueryError)?;

        Ok(workers.len())
    }
}
