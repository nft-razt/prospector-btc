// libs/infra/db-turso/src/repositories/audit_repository.rs
/**
 * =================================================================
 * APARATO: STRATEGIC AUDIT REPOSITORY (V50.1 - CLEAN)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: REGISTRO ACÍDICO Y CADENA DE CONTINUIDAD
 *
 * MEJORAS:
 * - Eliminación de imports no usados (Connection, error).
 * - Optimización de hashing con sha2::Sha256.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_models::work::AuditReport;
use sha2::{Sha256, Digest};
use tracing::{info, instrument};

pub struct AuditRepository {
    /// Cliente de conexión táctica inyectado.
    database_client: TursoClient,
}

impl AuditRepository {
    /**
     * Construye una nueva instancia del repositorio de auditoría.
     */
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Sella un reporte de auditoría inyectando metadatos forenses y el Hash de Continuidad.
     *
     * @param report Estructura validada del esfuerzo computacional reportado por el worker.
     * @returns El hash de integridad generado para esta misión.
     */
    #[instrument(skip(self, report))]
    pub async fn seal_mission_audit_trail(&self, report: &AuditReport) -> Result<String, DbError> {
        let connection = self.database_client.get_connection()?;

        // 1. ADQUISICIÓN DEL ESLABÓN ANTERIOR (Continuity Trace)
        let last_hash_query = "
            SELECT integrity_hash FROM jobs
            WHERE status = 'completed'
            ORDER BY completed_at DESC LIMIT 1
        ";

        let mut rows = connection.query(last_hash_query, ()).await?;
        let previous_integrity_hash: String = if let Some(row) = rows.next().await? {
            row.get(0)?
        } else {
            "PROSPECTOR_GENESIS_V10.8".to_string()
        };

        // 2. GENERACIÓN DEL NUEVO SELLO (Criptographic Linking)
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(previous_integrity_hash.as_bytes());
        sha256_hasher.update(report.job_mission_identifier.as_bytes());
        sha256_hasher.update(report.computational_effort_volume.as_bytes());
        sha256_hasher.update(report.audit_footprint_checkpoint.as_bytes());
        let current_integrity_hash = format!("{:x}", sha256_hasher.finalize());

        // 3. PERSISTENCIA ATÓMICA (Motor A - Turso)
        let update_sql = "
            UPDATE jobs
            SET status = 'completed',
                total_hashes_effort = ?2,
                execution_duration_ms = ?3,
                audit_footprint_checkpoint = ?4,
                integrity_hash = ?5,
                completed_at = CURRENT_TIMESTAMP
            WHERE id = ?1
        ";

        connection.execute(update_sql, params![
            report.job_mission_identifier.clone(),
            report.computational_effort_volume.clone(),
            report.execution_duration_milliseconds as i64,
            report.audit_footprint_checkpoint.clone(),
            current_integrity_hash.clone()
        ]).await?;

        info!("🛡️ [AUDIT_SEALED]: Mission {} linked with hash {}",
            report.job_mission_identifier, &current_integrity_hash[0..8]);

        Ok(current_integrity_hash)
    }

    /**
     * Recupera el volumen total de misiones certificadas en el Ledger Táctico.
     */
    pub async fn get_certified_missions_count(&self) -> Result<u64, DbError> {
        let connection = self.database_client.get_connection()?;
        let mut rows = connection.query("SELECT COUNT(*) FROM jobs WHERE status = 'completed'", ()).await?;

        if let Some(row) = rows.next().await? {
            let count: i64 = row.get(0)?;
            Ok(count as u64)
        } else {
            Ok(0)
        }
    }

    /**
     * Identifica el desfase de misiones completadas pendientes de migración estratégica.
     */
    pub async fn get_pending_archival_volume(&self) -> Result<u64, DbError> {
        let connection = self.database_client.get_connection()?;
        let mut rows = connection.query(
            "SELECT COUNT(*) FROM jobs WHERE status = 'completed' AND archived_at IS NULL",
            ()
        ).await?;

        if let Some(row) = rows.next().await? {
            let count: i64 = row.get(0)?;
            Ok(count as u64)
        } else {
            Ok(0)
        }
    }
}
