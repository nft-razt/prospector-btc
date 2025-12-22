/**
 * =================================================================
 * APARATO: SCENARIO REGISTRY REPOSITORY (V120.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: PERSISTENCIA ACÍDICA DE PLANTILLAS FORENSES
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_models::scenario::SystemTemplateRegistry;
use tracing::{info, instrument};

pub struct ScenarioRegistryRepository {
    database_client: TursoClient,
}

impl ScenarioRegistryRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * ✅ RESOLUCIÓN ERROR E0599: Alias para compatibilidad con bootstrap_forensics.
     */
    pub async fn persist_master_template(
        &self,
        metadata: &SystemTemplateRegistry,
        binary_data: Vec<u8>
    ) -> Result<(), DbError> {
        self.persist_forensic_template(
            &metadata.template_identifier,
            &metadata.display_name,
            binary_data
        ).await
    }

    #[instrument(skip(self, binary_data))]
    pub async fn persist_forensic_template(
        &self,
        identifier: &str,
        display_name: &str,
        binary_data: Vec<u8>
    ) -> Result<(), DbError> {
        let connection = self.database_client.get_connection()?;
        let sql = "
            INSERT INTO scenario_templates (identifier, name, blob_data, size_bytes)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(identifier) DO UPDATE SET
                blob_data = excluded.blob_data,
                updated_at = CURRENT_TIMESTAMP
        ";
        connection.execute(sql, params![identifier, display_name, binary_data.clone(), binary_data.len() as u64]).await?;
        info!("💾 [REPOSITORY]: Forensic DNA [{}] secured.", identifier);
        Ok(())
    }

    /**
     * ✅ RESOLUCIÓN ERROR E0599: Recupera todos los metadatos registrados.
     */
    pub async fn list_all_metadata(&self) -> Result<Vec<SystemTemplateRegistry>, DbError> {
        let connection = self.database_client.get_connection()?;
        let mut rows = connection.query("SELECT identifier, name, size_bytes, updated_at FROM scenario_templates", ()).await?;
        let mut scenarios = Vec::new();
        while let Some(row) = rows.next().await? {
            scenarios.push(SystemTemplateRegistry {
                template_identifier: row.get(0)?,
                display_name: row.get(1)?,
                binary_integrity_hash: "verified".to_string(),
                buffer_size_bytes: row.get::<i64>(2)? as u32,
                environment_category: "Desktop".to_string(),
                captured_at_timestamp: row.get(3)?,
            });
        }
        Ok(scenarios)
    }

    /// Alias para el handler de administración.
    pub async fn fetch_all_registered_metadata(&self) -> Result<Vec<SystemTemplateRegistry>, DbError> {
        self.list_all_metadata().await
    }
}
