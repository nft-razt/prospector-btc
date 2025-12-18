// libs/infra/worker-client/src/client.rs
// =================================================================
// APARATO: WORKER UPLINK CLIENT (V13.5 - ULTRA-LIGHT)
// RESPONSABILIDAD: COMUNICACIÓN DE RED PARA EL NODO MINERO
// ESTADO: CLEAN // HYGIENIC
// =================================================================

use crate::errors::ClientError;
use futures::future::join_all;
use log::info; // Eliminados 'error' y 'warn' no usados
use reqwest::{Client, StatusCode};
// Eliminados 'Digest' y 'Sha256' residuales del antiguo flujo de checksum manual
use std::path::Path;
use tokio::sync::Semaphore;
use std::sync::Arc;

use prospector_domain_models::{Finding, WorkerHeartbeat, WorkOrder, JobCompletion};

/// Cliente de alta disponibilidad para el enjambre de minería.
pub struct WorkerClient {
    http: Client,
    base_url: String,
    auth_token: String,
    download_concurrency_guard: Arc<Semaphore>,
}

impl WorkerClient {
    pub fn new(target_url: String, secret_token: String) -> Self {
        Self {
            http: Client::new(),
            base_url: target_url.trim_end_matches('/').to_string(),
            auth_token: secret_token,
            download_concurrency_guard: Arc::new(Semaphore::new(2)),
        }
    }

    /// Hidrata el sistema de archivos local con los fragmentos del filtro.
    pub async fn hydrate_shards(&self, target_directory: &Path, shard_count: usize) -> Result<(), ClientError> {
        if !target_directory.exists() { std::fs::create_dir_all(target_directory)?; }

        let mut async_tasks = Vec::new();

        for index in 0..shard_count {
            let instance = self.replicate();
            let shard_path = target_directory.join(format!("filter_shard_{}.bin", index));
            let url = format!("{}/resources/filters/filter_shard_{}.bin", self.base_url, index);
            let semaphore = Arc::clone(&self.download_concurrency_guard);

            async_tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                instance.download_shard_atomic(&url, &shard_path).await
            }));
        }

        let task_results = join_all(async_tasks).await;
        for result in task_results {
            result.map_err(|e| ClientError::ServerError(e.to_string()))??;
        }

        info!("✨ HYDRATION: Grid data synchronized.");
        Ok(())
    }

    async fn download_shard_atomic(&self, url: &str, path: &Path) -> Result<(), ClientError> {
        let response = self.http.get(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send().await?;

        if response.status() != StatusCode::OK {
            return Err(ClientError::ServerError(format!("Uplink Error: {}", response.status())));
        }

        let data_payload = response.bytes().await?;
        tokio::fs::write(path, data_payload).await?;
        Ok(())
    }

    /// Envía el latido del nodo con telemetría de hardware al orquestador.
    pub async fn send_heartbeat(&self, heartbeat: &WorkerHeartbeat) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/heartbeat", self.base_url);
        self.http.post(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(heartbeat)
            .send().await?;
        Ok(())
    }

    /// Adquiere una nueva orden de trabajo.
    pub async fn acquire_job(&self) -> Result<WorkOrder, ClientError> {
        let url = format!("{}/api/v1/swarm/job/acquire", self.base_url);
        let res = self.http.post(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send().await?;

        Ok(res.json::<WorkOrder>().await?)
    }

    /// Sella un trabajo como completado.
    pub async fn complete_job(&self, job_id: &str) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/job/complete", self.base_url);
        let payload = JobCompletion { id: job_id.to_string() };
        self.http.post(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(&payload)
            .send().await?;
        Ok(())
    }

    /// Notifica una colisión confirmada a la Bóveda.
    pub async fn report_finding(&self, finding: &Finding) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/finding", self.base_url);
        self.http.post(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(finding)
            .send().await?;
        Ok(())
    }

    fn replicate(&self) -> Self {
        Self {
            http: self.http.clone(),
            base_url: self.base_url.clone(),
            auth_token: self.auth_token.clone(),
            download_concurrency_guard: Arc::clone(&self.download_concurrency_guard),
        }
    }
}
