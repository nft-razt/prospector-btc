// libs/infra/db-turso/src/repositories/finding.rs
/**
 * =================================================================
 * APARATO: FINDING REPOSITORY (V51.0 - PERSISTENCE ENGINE)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: PERSISTENCIA Y RASTREO DE HALLAZGOS CRIPTOGRÁFICOS
 *
 * MEJORAS:
 * - Implementación de `batch_persist_findings` con transacciones ACID.
 * - Limpieza de imports no utilizados.
 * - Instrumentación selectiva para observabilidad.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_models::finding::Finding;
use serde_json::{json, Value};
use tracing::{info, instrument};

pub struct FindingRepository {
    database_client: TursoClient,
}

impl FindingRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Persiste un lote de hallazgos en una única transacción atómica.
     * Requerido por el `FindingFlusherService` para vaciar el buffer de RAM.
     *
     * @param findings Vector de hallazgos provenientes del Vault en memoria.
     * @returns Número de registros insertados.
     */
    #[instrument(skip(self, findings))]
    pub async fn batch_persist_findings(&self, findings: Vec<Finding>) -> Result<usize, DbError> {
        if findings.is_empty() {
            return Ok(0);
        }

        let connection = self.database_client.get_connection()?;
        let transaction = connection.transaction().await?;

        let sql = r#"
            INSERT INTO findings (
                id, address, private_key_wif, source_entropy,
                wallet_type, found_by_worker, job_id, detected_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(id) DO NOTHING
        "#;

        for finding in &findings {
            // Generamos un ID si no viene (aunque el modelo Finding suele traerlo o ser implícito).
            // Asumimos que si no hay ID en el struct, generamos uno nuevo.
            let id = uuid::Uuid::new_v4().to_string();

            transaction.execute(sql, params![
                id,
                finding.address.clone(),
                finding.private_key_wif.clone(),
                finding.source_entropy.clone(),
                finding.wallet_type.clone(),
                finding.found_by_worker.clone(),
                finding.job_id.clone(),
                finding.detected_at.clone()
            ]).await?;
        }

        transaction.commit().await?;

        info!("💾 [FINDING_REPO]: Batch persisted {} discoveries.", findings.len());
        Ok(findings.len())
    }

    /**
     * Recupera un lote de hallazgos que aún no han sido replicados en Supabase.
     * Utilizado por el OutboxRelay para garantizar la redundancia geográfica.
     *
     * @param batch_limit Cantidad de hallazgos a procesar por ráfaga.
     */
    pub async fn fetch_pending_strategic_archival(&self, batch_limit: usize) -> Result<Vec<Value>, DbError> {
        let connection = self.database_client.get_connection()?;

        let query = "
            SELECT id, address, private_key_wif, source_entropy, wallet_type, found_by_worker, detected_at
            FROM findings
            WHERE archived_at IS NULL
            LIMIT ?1
        ";

        let mut rows = connection.query(query, params![batch_limit as i64]).await?;
        let mut results = Vec::new();

        while let Some(row) = rows.next().await? {
            results.push(json!({
                "original_id": row.get::<String>(0)?,
                "address": row.get::<String>(1)?,
                "private_key_wif": row.get::<String>(2)?,
                "source_entropy": row.get::<String>(3)?,
                "wallet_type": row.get::<String>(4)?,
                "found_by_worker": row.get::<String>(5)?,
                "detected_at": row.get::<String>(6)?
            }));
        }

        Ok(results)
    }

    /**
     * Sella los hallazgos como archivados exitosamente.
     */
    pub async fn mark_as_archived(&self, identifiers: Vec<String>) -> Result<(), DbError> {
        if identifiers.is_empty() { return Ok(()); }

        let connection = self.database_client.get_connection()?;
        // Nota: Turso/libSQL HTTP no soporta transacciones complejas en todos los modos,
        // pero un bucle de updates simples es seguro y efectivo aquí.
        for id in identifiers {
            connection.execute(
                "UPDATE findings SET archived_at = CURRENT_TIMESTAMP WHERE id = ?1",
                params![id]
            ).await?;
        }

        Ok(())
    }
}
