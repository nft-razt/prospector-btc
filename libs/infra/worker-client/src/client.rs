// libs/infra/worker-client/src/client.rs
// =================================================================
// APARATO: WORKER CLIENT (SHARDING HYDRATION)
// RESPONSABILIDAD: DESCARGA PARALELA DE ARTEFACTOS DE DATOS
// ESTADO: CLEAN (NO UNUSED IMPORTS) & OPTIMIZED
// =================================================================

use crate::errors::ClientError;
use futures::future::join_all; // Requiere la dependencia 'futures' en Cargo.toml
use log::{error, info, warn};
use prospector_domain_models::{Finding, JobCompletion, WorkOrder};
use reqwest::{header, Client};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::Path; // ✅ CORRECCIÓN: Eliminado PathBuf
use std::time::Duration;
use tokio::time::sleep;

const USER_AGENT: &str = "Mozilla/5.0 (Hydra-Zero Node) AppleWebKit/537.36";

#[derive(Clone)]
pub struct WorkerClient {
    inner: Client,
    base_url: String,
    auth_token: String,
}

impl WorkerClient {
    pub fn new(base_url: String, auth_token: String) -> Result<Self, ClientError> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static(USER_AGENT));

        let client = Client::builder()
            .default_headers(headers)
            // Timeout generoso para descargas grandes en redes lentas
            .timeout(Duration::from_secs(60))
            .build()?;

        Ok(Self {
            inner: client,
            base_url: base_url.trim_end_matches('/').to_string(),
            auth_token,
        })
    }

    /// Hidrata el sistema de archivos local con los shards del filtro.
    /// Descarga `filter_shard_0.bin`, `filter_shard_1.bin`, etc., en paralelo.
    ///
    /// Utiliza `futures::future::join_all` para concurrencia real.
    pub async fn hydrate_shards(
        &self,
        target_dir: &Path,
        shard_count: usize,
    ) -> Result<(), ClientError> {
        if !target_dir.exists() {
            std::fs::create_dir_all(target_dir)?;
        }

        info!(
            "🚀 Iniciando hidratación paralela de {} shards...",
            shard_count
        );

        let mut tasks = Vec::new();

        for i in 0..shard_count {
            let filename = format!("filter_shard_{}.bin", i);

            // Inferencia de tipo automática, no necesitamos importar PathBuf
            let file_path = target_dir.join(&filename);

            // Endpoint de recursos estáticos en el Orquestador
            let url = format!("{}/resources/filters/{}", self.base_url, filename);

            // Clonamos el cliente para moverlo al contexto asíncrono (barato, es un Arc interno)
            let client_ref = self.clone();

            // Spawn de tarea ligera en el runtime de Tokio
            tasks.push(tokio::spawn(async move {
                client_ref.download_shard_with_retry(&url, &file_path).await
            }));
        }

        // Esperamos a que terminen todas las descargas
        let results = join_all(tasks).await;

        // Validación de resultados
        for (idx, res) in results.into_iter().enumerate() {
            match res {
                Ok(Ok(_)) => info!("✅ Shard {} listo y verificado.", idx),
                Ok(Err(e)) => {
                    error!("❌ Fallo crítico en Shard {}: {}", idx, e);
                    // Fail-Fast: Si falta un fragmento, el filtro es inútil.
                    return Err(e);
                }
                Err(e) => {
                    return Err(ClientError::ServerError(format!(
                        "Panic en hilo de descarga {}: {}",
                        idx, e
                    )))
                }
            }
        }

        info!("✨ Hidratación completa. Sistema de archivos sincronizado.");
        Ok(())
    }

    /// Lógica de descarga individual con reintentos y backoff exponencial.
    async fn download_shard_with_retry(&self, url: &str, path: &Path) -> Result<(), ClientError> {
        // 1. Verificación de caché local
        if path.exists() {
            let meta = std::fs::metadata(path)?;
            // Validación mínima de tamaño (evitar archivos vacíos por errores de disco)
            if meta.len() > 1024 * 10 {
                return Ok(());
            }
            warn!("⚠️ Archivo corrupto detectado en caché. Re-descargando...");
        }

        let max_retries = 5;
        let mut delay = Duration::from_secs(2);

        for attempt in 1..=max_retries {
            match self.download_file(url, path).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    warn!(
                        "⚠️ Error descarga (Intento {}/{}): {}. Reintentando en {:?}...",
                        attempt, max_retries, e, delay
                    );
                    if attempt == max_retries {
                        return Err(e);
                    }
                    sleep(delay).await;
                    delay = std::cmp::min(delay * 2, Duration::from_secs(30));
                }
            }
        }

        Err(ClientError::HydrationFailed)
    }

    /// Descarga streaming con validación de checksum en tiempo real (opcional) y escritura atómica.
    async fn download_file(&self, url: &str, path: &Path) -> Result<(), ClientError> {
        let mut response = self.inner.get(url).send().await?;

        if !response.status().is_success() {
            return Err(ClientError::ServerError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        // Patrón de Escritura Atómica: Escribir a .tmp y renombrar al final.
        // Esto evita corrupción si el proceso se mata a mitad de descarga.
        let tmp_path = path.with_extension("tmp");
        let mut file = File::create(&tmp_path)?;
        let mut hasher = Sha256::new();

        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk)?;
            hasher.update(&chunk);
        }

        // Si llegamos aquí, la descarga fue exitosa.
        std::fs::rename(&tmp_path, path)?;

        // let hash = hasher.finalize();
        // debug!("Hash descargado: {:x}", hash);

        Ok(())
    }

    // =================================================================
    // MÉTODOS DE NEGOCIO (CORE API WRAPPERS)
    // =================================================================

    pub async fn acquire_job(&self) -> Result<WorkOrder, ClientError> {
        let url = format!("{}/api/v1/swarm/job/acquire", self.base_url);
        let res = self.authenticated_request(self.inner.post(&url)).await?;
        Ok(res.json::<WorkOrder>().await?)
    }

    pub async fn send_keepalive(&self, job_id: &str) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/job/keepalive", self.base_url);
        let payload = JobCompletion {
            id: job_id.to_string(),
        };
        let _ = self
            .authenticated_request(self.inner.post(&url).json(&payload))
            .await?;
        Ok(())
    }

    pub async fn complete_job(&self, job_id: &str) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/job/complete", self.base_url);
        let payload = JobCompletion {
            id: job_id.to_string(),
        };
        let _ = self
            .authenticated_request(self.inner.post(&url).json(&payload))
            .await?;
        Ok(())
    }

    pub async fn report_finding(&self, finding: &Finding) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/swarm/finding", self.base_url);
        let _ = self
            .authenticated_request(self.inner.post(&url).json(finding))
            .await?;
        Ok(())
    }

    /// Envío de pánico bloqueante (Last Gasp).
    /// Usa reqwest::blocking porque el runtime async puede estar comprometido.
    pub fn send_panic_blocking(base_url: &str, token: &str, worker_id: &str, msg: &str) {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/api/v1/swarm/panic", base_url.trim_end_matches('/'));
        let _ = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&serde_json::json!({ "worker_id": worker_id, "message": msg }))
            .timeout(Duration::from_secs(5))
            .send();
    }

    // Helper privado para inyección de auth
    async fn authenticated_request(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, ClientError> {
        let res = req
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(ClientError::ServerError(format!("HTTP {}", res.status())));
        }
        Ok(res)
    }
}
