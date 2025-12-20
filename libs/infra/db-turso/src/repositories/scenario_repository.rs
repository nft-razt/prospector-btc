/**
 * =================================================================
 * APARATO: SCENARIO REGISTRY REPOSITORY (V110.0 - PERSISTENCIA TOTAL)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: PERSISTENCIA ACÍDICA DEL CATÁLOGO DE ESCENARIOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Este componente garantiza que las plantillas de entropía (ADN de sistema)
 * posean persistencia física en Turso. Implementa un sistema de
 * almacenamiento híbrido donde los metadatos son indexables y el
 * material binario se guarda como un objeto BLOB atómico.
 *
 * ESTRATEGIA DE ÉLITE:
 * - SQL Schema Evolution: Incluye la definición de la tabla soberana.
 * - Binary Large Object (BLOB) Support: Almacena los 250KB directamente en DB.
 * - Idempotent Registration: Previene duplicados mediante identificadores únicos.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::{params, Connection, Row};
use prospector_domain_models::scenario::SystemTemplateRegistry; // Referencia al modelo L2
use tracing::{info, error, instrument};

pub struct ScenarioRegistryRepository {
    database_connection: Connection,
}

impl ScenarioRegistryRepository {
    /**
     * Inicializa el repositorio inyectando una conexión activa del pool.
     */
    pub fn new(connection: Connection) -> Self {
        Self { database_connection: connection }
    }

    /**
     * Ejecuta la inicialización estructural de la tabla en Turso.
     * Este comando es persistente y solo se ejecuta una vez.
     */
    pub async fn initialize_schema(&self) -> Result<(), DbError> {
        let sql_definition = r#"
            CREATE TABLE IF NOT EXISTS scenario_registry (
                template_identifier TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                binary_template_blob BLOB NOT NULL,
                binary_integrity_hash TEXT NOT NULL,
                buffer_size_bytes INTEGER NOT NULL,
                environment_category TEXT NOT NULL,
                captured_at_timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            );
        "#;

        self.database_connection.execute(sql_definition, ()).await?;
        Ok(())
    }

    /**
     * Registra y guarda físicamente una plantilla binaria en Turso.
     *
     * # Parámetros
     * * `template_metadata` - Estructura de datos descriptiva.
     * * `binary_data` - Los 250,000 bytes reales de Windows XP.
     */
    #[instrument(skip(self, binary_data))]
    pub async fn persist_master_template(
        &self,
        template_metadata: &SystemTemplateRegistry,
        binary_data: Vec<u8>
    ) -> Result<(), DbError> {
        let sql_command = r#"
            INSERT INTO scenario_registry (
                template_identifier, display_name, binary_template_blob,
                binary_integrity_hash, buffer_size_bytes, environment_category
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT(template_identifier) DO UPDATE SET
                binary_template_blob = excluded.binary_template_blob,
                binary_integrity_hash = excluded.binary_integrity_hash,
                display_name = excluded.display_name
        "#;

        self.database_connection.execute(sql_command, params![
            template_metadata.template_identifier.clone(),
            template_metadata.display_name.clone(),
            binary_data, // Inserción directa del BLOB
            template_metadata.binary_integrity_hash.clone(),
            template_metadata.buffer_size_bytes,
            template_metadata.environment_category.clone()
        ]).await?;

        info!("💾 [TURSO_PERSISTENCE]: Scenario {} DNA secured in database records.",
            template_metadata.template_identifier);
        Ok(())
    }

    /**
     * Recupera el material binario original para su distribución al enjambre.
     */
    pub async fn fetch_binary_blob(&self, identifier: &str) -> Result<Vec<u8>, DbError> {
        let mut rows = self.database_connection.query(
            "SELECT binary_template_blob FROM scenario_registry WHERE template_identifier = ?1",
            params![identifier]
        ).await?;

        if let Some(row) = rows.next().await? {
            let blob_data: Vec<u8> = row.get(0)?;
            Ok(blob_data)
        } else {
            Err(DbError::MappingError(format!("Scenario {} not found in records", identifier)))
        }
    }

    /**
     * Lista todos los escenarios registrados para el Dashboard.
     */
    pub async fn list_all_metadata(&self) -> Result<Vec<SystemTemplateRegistry>, DbError> {
        let mut rows = self.database_connection.query(
            "SELECT template_identifier, display_name, binary_integrity_hash, buffer_size_bytes, environment_category, captured_at_timestamp FROM scenario_registry",
            ()
        ).await?;

        let mut results = Vec::new();
        while let Some(row) = rows.next().await? {
            results.push(self.map_row_to_metadata(row)?);
        }
        Ok(results)
    }

    fn map_row_to_metadata(&self, row: Row) -> Result<SystemTemplateRegistry, DbError> {
        Ok(SystemTemplateRegistry {
            template_identifier: row.get(0)?,
            display_name: row.get(1)?,
            binary_integrity_hash: row.get(2)?,
            buffer_size_bytes: row.get(3)?,
            environment_category: row.get(4)?,
            captured_at_timestamp: row.get(5)?,
        })
    }
}
