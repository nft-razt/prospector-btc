// libs/infra/worker-client/src/client.rs
use std::time::Duration;
use std::path::Path;
use reqwest::{Client, header};
use tokio::time::sleep;
use log::{info, warn, error};
use prospector_domain_models::{WorkOrder, JobCompletion, Finding};
use crate::errors::ClientError;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

#[derive(Clone)]
pub struct WorkerClient {
    inner: Client,
    base_url: String,
    auth_token: String,
}

impl WorkerClient {
    /// Crea un nuevo cliente con headers optimizados para evasión.
    pub fn new(base_url: String, auth_token: String) -> Result<Self, ClientError> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static(USER_AGENT));

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            inner: client,
            base_url: base_url.trim_end_matches('/').to_string(),
            auth_token,
        })
    }

    /// Descarga el filtro UTXO con estrategia de "Exponential Backoff".
    pub async fn hydrate_filter(&self, path: &Path) -> Result<(), ClientError> {
        if path.exists() {
            info!("✅ [CACHE] Filtro local detectado: {:?}", path);
            return Ok(());
        }

        let max_retries = 8;
        let mut delay = Duration::from_secs(2);

        for attempt in 1..=max_retries {
            info!("⬇️ [NET] Descargando Filtro UTXO (Intento {}/{})", attempt, max_retries);
            let url = format!("{}/resources/utxo_filter.bin", self.base_url);

            match self.inner.get(&url).send().await {
                Ok(res) if res.status().is_success() => {
                    let bytes = res.bytes().await?;
                    tokio::fs::write(path, bytes).await?;
                    info!("✅ [IO] Filtro guardado exitosamente.");
                    return Ok(());
                },
                Ok(res) => warn!("⚠️ Fallo HTTP {}: Reintentando...", res.status()),
                Err(e) => warn!("⚠️ Error de Red: {}. Reintentando...", e),
            }

            if attempt < max_retries {
                info!("💤 Esperando {:?} antes del siguiente intento...", delay);
                sleep(delay).await;
                delay = std::cmp::min(delay * 2, Duration::from_secs(60));
            }
        }

        Err(ClientError::HydrationFailed)
    }

    /// Adquiere trabajo del Orquestador.
    pub async fn acquire_job(&self) -> Result<WorkOrder, ClientError> {
        let url = format!("{}/api/v1/swarm/job/acquire", self.base_url);
        let res = self.authenticated_request(self.inner.post(&url)).await?;

        if !res.status().is_success() {
            return Err(ClientError::ServerError(format!("HTTP {}", res.status())));
        }

        Ok(res.json::<WorkOrder>().await?)
    }

    /// Envía un latido de "Sigo vivo".
    pub async fn send_keepalive(&self, job_id: &str) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/job/keepalive", self.base_url);
        let payload = JobCompletion { id: job_id.to_string() };

        let res = self.authenticated_request(self.inner.post(&url).json(&payload)).await?;

        if !res.status().is_success() {
            return Err(ClientError::ServerError(format!("Keepalive rechazado: {}", res.status())));
        }
        Ok(())
    }

    /// Reporta trabajo completado.
    pub async fn complete_job(&self, job_id: &str) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/job/complete", self.base_url);
        let payload = JobCompletion { id: job_id.to_string() };

        let res = self.authenticated_request(self.inner.post(&url).json(&payload)).await?;

        if !res.status().is_success() {
            return Err(ClientError::ServerError(format!("Completion fallida: {}", res.status())));
        }
        Ok(())
    }

    /// Reporta un hallazgo (Colisión). Reintentos infinitos hasta éxito.
    pub async fn report_finding(&self, finding: &Finding) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/finding", self.base_url);

        loop {
            match self.authenticated_request(self.inner.post(&url).json(finding)).await {
                Ok(res) if res.status().is_success() => return Ok(()),
                Ok(res) => error!("⚠️ Error reportando hallazgo: HTTP {}", res.status()),
                Err(e) => error!("⚠️ Error de red reportando hallazgo: {}", e),
            }
            sleep(Duration::from_secs(5)).await;
        }
    }

    /// Método estático síncrono (bloqueante) para usar en Panic Hooks.
    /// Crea su propio cliente efímero ya que el runtime async puede estar muerto.
    pub fn send_panic_blocking(base_url: &str, token: &str, worker_id: &str, msg: &str) {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/api/v1/swarm/panic", base_url.trim_end_matches('/'));

        let payload = serde_json::json!({
            "worker_id": worker_id,
            "message": msg,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let _ = client.post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&payload)
            .timeout(Duration::from_secs(5))
            .send();
    }

    // Helper interno para inyectar token
    async fn authenticated_request(&self, req: reqwest::RequestBuilder) -> Result<reqwest::Response, ClientError> {
        req.header("Authorization", format!("Bearer {}", self.auth_token))
           .send()
           .await
           .map_err(ClientError::NetworkError)
    }
}
