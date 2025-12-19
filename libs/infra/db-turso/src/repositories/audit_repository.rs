/**
 * =================================================================
 * APARATO: STRATEGIC AUDIT REPOSITORY (V19.0)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (L3)
 * RESPONSABILIDAD: REGISTRO ACÍDICO DE LA HUELLA DE AUDITORÍA
 *
 * ESTRATEGIA DE ÉLITE:
 * - Atomic Commits: Garantiza que el reporte se guarde íntegramente.
 * - BigInt Normalization: Maneja volúmenes de esfuerzo como cadenas deterministas.
 * - Zero-Regression: Alineado con el esquema V3.3.0 Archival Ready.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_models::work::AuditReport;
use tracing::{info, instrument};

pub struct AuditRepository {
    database_client: TursoClient,
}

impl AuditRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Sella un reporte de auditoría inyectando metadatos forenses.
     *
     * @param report: Estructura validada del esfuerzo computacional.
     */
    #[instrument(skip(self, report))]
    pub async fn persist_mission_completion(&self, report: &AuditReport) -> Result<(), DbError> {
        let connection = self.database_client.get_connection()?;

        let query_sequence = r#"
            UPDATE jobs
            SET
                status = 'completed',
                total_hashes = ?2,
                execution_duration_ms = ?3,
                final_status = ?4,
                audit_footprint_checkpoint = ?5,
                completed_at = ?6
            WHERE id = ?1
        "#;

        connection.execute(
            query_sequence,
            params![
                report.job_mission_identifier.clone(),
                report.computational_effort_volume.clone(),
                report.execution_duration_ms,
                report.final_mission_status.clone(),
                report.audit_footprint_checkpoint.clone(),
                report.completed_at_timestamp.clone()
            ],
        ).await?;

        info!(
            "🏁 [AUDIT_SEALED]: Mission {} completed by node {}. Footprint: {}",
            report.job_mission_identifier,
            report.worker_node_identifier,
            report.audit_footprint_checkpoint
        );

        Ok(())
    }
}
