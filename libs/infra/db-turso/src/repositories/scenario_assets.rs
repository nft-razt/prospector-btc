/**
 * =================================================================
 * APARATO: SCENARIO ASSET MANAGER (V125.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE ARCHIVOS BINARIOS DE SIMULACIÓN
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el acceso asíncrono a las plantillas de ADN del sistema.
 * Esta versión utiliza Tokio FS para garantizar que la lectura de
 * buffers de 250KB no bloquee el hilo de ejecución de la API.
 * =================================================================
 */

use crate::errors::DbError;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing::{info, error};

pub struct ScenarioAssetManager {
    /// Ruta base donde residen las plantillas .bin en el servidor.
    base_assets_directory: PathBuf,
}

impl ScenarioAssetManager {
    /**
     * Construye una nueva instancia del gestor de activos.
     *
     * @param assets_path Ruta al directorio de plantillas binarias.
     */
    pub fn new(assets_path: &str) -> Self {
        Self {
            base_assets_directory: PathBuf::from(assets_path),
        }
    }

    /**
     * Recupera el contenido binario de una plantilla de simulación.
     *
     * @param scenario_identifier Nombre técnico de la plantilla.
     * @returns Result con el buffer de bytes o error de mapeo.
     */
    pub async fn retrieve_performance_template_blob(
        &self,
        scenario_identifier: &str
    ) -> Result<Vec<u8>, DbError> {
        let file_target_path = self.base_assets_directory.join(format!("{}.bin", scenario_identifier));

        if !file_target_path.exists() {
            error!("❌ [ASSET_NOT_FOUND]: Template {} missing.", scenario_identifier);
            return Err(DbError::MappingError("Scenario binary file not found".into()));
        }

        // ✅ RESOLUCIÓN: Uso de Tokio Async I/O nivelado en Cargo.toml
        let mut file_handle = File::open(&file_target_path)
            .await
            .map_err(|io_error| DbError::ConnectionError(io_error.to_string()))?;

        let mut binary_buffer = Vec::new();
        file_handle.read_to_end(&mut binary_buffer)
            .await
            .map_err(|io_error| DbError::ConnectionError(io_error.to_string()))?;

        info!("📂 [ASSET_LOADED]: Performance template {} secured.", scenario_identifier);

        Ok(binary_buffer)
    }
}
