/**
 * =================================================================
 * APARATO: SCENARIO ASSET MANAGER (V120.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE ARCHIVOS BINARIOS DE SIMULACIÓN
 *
 * ESTRATEGIA DE ÉLITE:
 * - Direct I/O: Lectura eficiente de plantillas desde el almacenamiento.
 * - Integrity Shield: Validación de archivos contra la base de datos táctica.
 * - No-Abbreviations: Nomenclatura descriptiva absoluta.
 * =================================================================
 */

use crate::errors::DbError;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing::{info, error};

pub struct ScenarioAssetManager {
    /// Ruta base donde residen las plantillas .bin en el servidor.
    base_assets_directory: PathBuf,
}

impl ScenarioAssetManager {
    pub fn new(assets_path: &str) -> Self {
        Self {
            base_assets_directory: PathBuf::from(assets_path),
        }
    }

    /**
     * Recupera el contenido binario de una plantilla de Windows XP.
     *
     * # Parámetros
     * * `scenario_identifier` - Nombre del archivo/escenario a recuperar.
     *
     * # Errors
     * Retorna `DbError::ConnectionError` si el recurso no existe en el sistema de archivos.
     */
    pub async fn retrieve_performance_template_blob(
        &self,
        scenario_identifier: &str
    ) -> Result<Vec<u8>, DbError> {
        let file_target_path = self.base_assets_directory.join(format!("{}.bin", scenario_identifier));

        if !file_target_path.exists() {
            error!("❌ [ASSET_NOT_FOUND]: Template {} missing at {:?}", scenario_identifier, file_target_path);
            return Err(DbError::MappingError("Scenario binary file not found".into()));
        }

        let mut file_handle = File::open(&file_target_path)
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;

        let mut binary_buffer = Vec::new();
        file_handle.read_to_end(&mut binary_buffer)
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;

        info!("📂 [ASSET_LOADED]: Performance template {} ({} bytes)",
            scenario_identifier, binary_buffer.len());

        Ok(binary_buffer)
    }
}
