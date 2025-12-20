/**
 * =================================================================
 * APARATO: WORKER UPLINK CLIENT (V16.0 - HYDRA PARALLEL)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: ORQUESTACIÓN DE RED Y ADQUISICIÓN DE CENSO
 *
 * ESTRATEGIA DE ÉLITE:
 * - Async Parallel Hydration: Descarga de shards mediante hilos concurrentes.
 * - Zero-Regression: Mantiene compatibilidad con el Orquestador en Render.
 * - Strict Naming: Nomenclatura descriptiva absoluta.
 * =================================================================
 */

use crate::errors::ClientError;
use reqwest::{Client, StatusCode};
use prospector_domain_models::work::{AuditReport, WorkOrder};
use prospector_domain_models::Finding;
use std::path::Path;
use tokio::fs;
use anyhow::{Result, Context};
use log::{info, error};

pub struct WorkerClient {
    internal_http_client: Client,
    orchestrator_base_url: String,
    authentication_token: String,
}

impl WorkerClient {
    pub fn new(target_url: String, secret_token: String) -> Self {
        Self {
            internal_http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
            orchestrator_base_url: target_url.trim_end_matches('/').to_string(),
            authentication_token: secret_token,
        }
    }

    /**
     * Realiza la hidratación paralela de los fragmentos del censo binario.
     *
     * @param cache_directory Ruta local para almacenar los shards (.bin).
     * @param partition_count Cantidad de fragmentos a descargar (V10.8 = 4).
     */
    pub async fn hydrate_shards_parallel(
        &self,
        cache_directory: &Path,
        partition_count: usize
    ) -> Result<()> {
        let base_download_url = std::env::var("FILTER_BASE_URL")
            .context("ENVIRONMENT_FAULT: FILTER_BASE_URL is not defined.")?;

        if !cache_directory.exists() {
            fs::create_dir_all(cache_directory).await?;
        }

        info!("⬇️ [HYDRATION]: Initiating parallel acquisition of {} shards...", partition_count);

        let mut asynchronous_tasks = Vec::new();

        for index in 0..partition_count {
            let file_name = format!("filter_shard_{}.bin", index);
            let download_url = format!("{}/{}", base_download_url, file_name);
            let destination_path = cache_directory.join(&file_name);

            let http_client = self.internal_http_client.clone();
            let token = self.authentication_token.clone();

            // Lanzamos la descarga en un hilo asíncrono dedicado (Hydra Acceleration)
            asynchronous_tasks.push(tokio::spawn(async move {
                let response = http_client.get(download_url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                if response.status() != StatusCode::OK {
                    return Err(anyhow::anyhow!("SHARD_DOWNLOAD_REJECTED: Status {}", response.status()));
                }

                let binary_payload = response.bytes().await?;
                fs::write(destination_path, binary_payload).await?;

                Ok::<(), anyhow::Error>(())
            }));
        }

        // Sincronización de tareas: El sistema espera a que el mapa esté completo
        for task in asynchronous_tasks {
            task.await.context("THREAD_PANIC: Hydration task collapsed.")??;
        }

        info!("✅ [HYDRATION]: All shards secured and verified in local cache.");
        Ok(())
    }

    pub async fn report_finding(&self, finding: &Finding) -> Result<()> {
        let endpoint_url = format!("{}/api/v1/swarm/finding", self.orchestrator_base_url);
        let response = self.internal_http_client
            .post(endpoint_url)
            .header("Authorization", format!("Bearer {}", self.authentication_token))
            .json(finding)
            .send()
            .await?;

        if response.status().is_success() { Ok(()) }
        else { Err(anyhow::anyhow!("VAULT_SYNC_ERROR: {}", response.status())) }
    }

    // ... (request_mission_assignment y submit_audit_certification sin cambios)
}
