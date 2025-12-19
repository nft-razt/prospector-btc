// libs/infra/worker-client/src/client.rs
/**
 * =================================================================
 * APARATO: WORKER UPLINK CLIENT (V13.9 - HYGIENIC EDITION)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (L3)
 * RESPONSABILIDAD: ORQUESTACIÓN DE RED Y SINAPSIS CON EL CEREBRO
 *
 * ESTADO: ZERO-WARNINGS // HYDRA-ZERO SANEADO
 * ESTRATEGIA:
 * - Eliminación de residuos de compilación (Unused imports).
 * - Mantenimiento de la capacidad de métricas doctorales (L4).
 * =================================================================
 */
use crate::errors::ClientError;
use futures::future::join_all;
use log::{error, info}; // ✅ RESOLUCIÓN: 'warn' eliminado por redundancia
use reqwest::{Client, StatusCode};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;

// --- MODELOS DE DOMINIO (SSoT) ---
use prospector_domain_models::{
    Finding,
    JobCompletion,
    // ✅ RESOLUCIÓN: 'WorkerHeartbeat' eliminado (se utiliza JSON crudo en el modo Lite)
    WorkOrder,
};

/// Cliente soberano para la comunicación del enjambre.
pub struct WorkerClient {
    http_client: Client,
    base_url: String,
    auth_token: String,
    download_concurrency_guard: Arc<Semaphore>,
}

impl WorkerClient {
    /// Inicializa una nueva instancia del cliente de enlace.
    pub fn new(target_url: String, secret_token: String) -> Self {
        Self {
            http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            base_url: target_url.trim_end_matches('/').to_string(),
            auth_token: secret_token,
            download_concurrency_guard: Arc::new(Semaphore::new(2)),
        }
    }

    /**
     * SECUENCIA DE HIDRATACIÓN (L1-L3)
     * Descarga de forma paralela y atómica los fragmentos del Filtro de Bloom.
     */
    pub async fn hydrate_shards(
        &self,
        target_directory: &Path,
        shard_count: usize,
    ) -> Result<(), ClientError> {
        if !target_directory.exists() {
            std::fs::create_dir_all(target_directory)?;
        }

        info!("🧊 HYDRATION: Synchronizing {} data shards...", shard_count);

        let mut async_tasks = Vec::new();

        for index in 0..shard_count {
            let instance = self.replicate();
            let shard_path = target_directory.join(format!("filter_shard_{}.bin", index));
            let url = format!(
                "{}/resources/filters/filter_shard_{}.bin",
                self.base_url, index
            );
            let semaphore = Arc::clone(&self.download_concurrency_guard);

            async_tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.map_err(|_| {
                    ClientError::ServerError("Concurrency semaphore poisoned".to_string())
                })?;

                instance.download_shard_atomic(&url, &shard_path).await
            }));
        }

        let task_results = join_all(async_tasks).await;
        for result in task_results {
            match result {
                Ok(inner_res) => inner_res?,
                Err(join_err) => return Err(ClientError::ServerError(join_err.to_string())),
            }
        }

        info!("✅ HYDRATION_COMPLETE: Tactical map secured.");
        Ok(())
    }

    async fn download_shard_atomic(&self, url: &str, path: &Path) -> Result<(), ClientError> {
        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            return Err(ClientError::ServerError(format!(
                "Uplink Error: {}",
                response.status()
            )));
        }

        let data_payload = response.bytes().await?;
        tokio::fs::write(path, data_payload).await?;
        Ok(())
    }

    /// Adquiere una nueva orden de trabajo U256.
    pub async fn acquire_job(&self) -> Result<WorkOrder, ClientError> {
        let url = format!("{}/api/v1/swarm/job/acquire", self.base_url);
        let response = self
            .http_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send()
            .await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            return Err(ClientError::Unauthorized);
        }

        Ok(response.json::<WorkOrder>().await?)
    }

    /// Sella un trabajo como completado integrando métricas de esfuerzo computacional.
    pub async fn complete_job_with_metrics(
        &self,
        job_id: &str,
        total_hashes: u64,
        duration: u64,
    ) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/job/complete", self.base_url);
        let payload = JobCompletion {
            id: job_id.to_string(),
            total_hashes,
            actual_duration_sec: duration,
        };

        self.http_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(&payload)
            .send()
            .await?;

        Ok(())
    }

    /// Notifica una colisión confirmada a la Bóveda.
    pub async fn report_finding(&self, finding: &Finding) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/finding", self.base_url);
        let response = self
            .http_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(finding)
            .send()
            .await?;

        if response.status() == StatusCode::CREATED {
            info!(
                "🎯 [VAULT_SYNC]: Collision secured for address {}",
                finding.address
            );
            Ok(())
        } else {
            error!("💀 [CRITICAL_UPLINK_FAILURE]: Vault rejected report");
            Err(ClientError::ServerError(
                "Vault rejected collision report".into(),
            ))
        }
    }

    /// Latido de telemetría ligero para minimizar el consumo de CPU del worker.
    pub async fn send_heartbeat_lite(&self, worker_id: &str) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/heartbeat", self.base_url);
        let payload = serde_json::json!({
            "worker_id": worker_id,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "status": "active"
        });

        self.http_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(&payload)
            .send()
            .await?;

        Ok(())
    }

    fn replicate(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
            base_url: self.base_url.clone(),
            auth_token: self.auth_token.clone(),
            download_concurrency_guard: Arc::clone(&self.download_concurrency_guard),
        }
    }
}
