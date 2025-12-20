/**
 * =================================================================
 * APARATO: ENVIRONMENTAL TEMPLATE HYDRATOR (V120.0 - SOBERANO)
 * CLASIFICACIÓN: WORKER INFRASTRUCTURE (ESTRATO L3)
 * RESPONSABILIDAD: ADQUISICIÓN Y CACHÉ DE ESCENARIOS FORENSES
 * =================================================================
 */

use std::path::PathBuf;
use tokio::fs;
use sha2::{Sha256, Digest};
use crate::client::WorkerClient;
use crate::errors::ClientError;

pub struct EnvironmentalTemplateHydrator;

impl EnvironmentalTemplateHydrator {
    /**
     * Asegura que el trabajador posea la plantilla binaria necesaria para la simulación.
     * Si el archivo ya existe y el checksum coincide, evita la descarga.
     */
    pub async fn hydrate_scenario_template(
        client: &WorkerClient,
        scenario_identifier: &str,
        expected_checksum: &str,
        cache_directory: &PathBuf
    ) -> Result<Vec<u8>, ClientError> {
        let local_file_path = cache_directory.join(format!("{}.bin", scenario_identifier));

        // 1. VERIFICACIÓN DE CACHÉ LOCAL
        if local_file_path.exists() {
            let existing_data = fs::read(&local_file_path).await?;
            if Self::verify_integrity(&existing_data, expected_checksum) {
                info!("✅ [HYDRATION]: Cache hit. Template {} verified.", scenario_identifier);
                return Ok(existing_data);
            }
        }

        // 2. DESCARGA SOBERANA DESDE EL ORQUESTADOR
        info!("⬇️ [HYDRATION]: Downloading scenario {} DNA...", scenario_identifier);
        let downloaded_data = client.download_template_blob(scenario_identifier).await?;

        // 3. VALIDACIÓN DE INTEGRIDAD POST-TRANSFIRIENCIA
        if !Self::verify_integrity(&downloaded_data, expected_checksum) {
            return Err(ClientError::HydrationFailed);
        }

        // 4. PERSISTENCIA EN DISCO EFÍMERO
        fs::create_dir_all(cache_directory).await?;
        fs::write(&local_file_path, &downloaded_data).await?;

        Ok(downloaded_data)
    }

    fn verify_integrity(data: &[u8], expected_hex_checksum: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result_checksum = format!("{:x}", hasher.finalize());
        result_checksum == expected_hex_checksum
    }
}
