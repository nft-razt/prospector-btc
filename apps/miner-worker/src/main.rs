// apps/miner-worker/src/main.rs
// =================================================================
// APARATO: MINER WORKER (SMART OPERATOR v4.3)
// CARACTERÍSTICAS: RESILIENCIA DE RED, TELEMETRÍA DE PÁNICO, SIMD
// =================================================================

use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::panic;

// PARALELISMO & ASYNC
use tokio::sync::mpsc;
use tokio::time::sleep;
use anyhow::{Context, Result, anyhow};
use reqwest::{Client, header};

// DOMINIO & LÓGICA
use prospector_core_probabilistic::filter_wrapper::RichListFilter;
use prospector_core_gen::wif::private_to_wif;
use prospector_core_math::private_key::SafePrivateKey;
use prospector_domain_strategy::{StrategyExecutor, ExecutorContext, FindingHandler};

// IMPORTACIÓN DEL DOMINIO UNIFICADO
use prospector_domain_models::{
    Finding,
    WorkOrder,
    JobCompletion
};

// --- CONFIGURACIÓN CLI ---
#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(long, env = "ORCHESTRATOR_URL")]
    orchestrator_url: String,

    #[arg(long, env = "WORKER_AUTH_TOKEN")]
    auth_token: String,

    #[arg(long, default_value = "drone-unit-generic")]
    worker_id: String,
}

// --- CLIENTE HTTP INTELIGENTE ---
struct WorkerClient {
    client: Client,
    base_url: String,
    token: String,
}

impl WorkerClient {
    fn new(base_url: String, token: String) -> Self {
        // Headers de Evasión (Chrome Spoofing)
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"));

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Fallo crítico en stack de red");

        Self { client, base_url, token }
    }

    /// Descarga el filtro UTXO con estrategia de "Exponential Backoff".
    async fn hydrate_filter(&self, path: &PathBuf) -> Result<()> {
        if path.exists() {
            println!("✅ [CACHE] Filtro local detectado: {:?}", path);
            return Ok(());
        }

        let max_retries = 8;
        let mut delay = Duration::from_secs(2);

        for attempt in 1..=max_retries {
            println!("⬇️ [NET] Descargando Filtro UTXO (Intento {}/{})", attempt, max_retries);

            let url = format!("{}/resources/utxo_filter.bin", self.base_url);
            match self.client.get(&url).send().await {
                Ok(res) => {
                    if res.status().is_success() {
                        let bytes = res.bytes().await?;
                        tokio::fs::write(path, bytes).await?;
                        println!("✅ [IO] Filtro guardado exitosamente.");
                        return Ok(());
                    } else {
                        eprintln!("⚠️ Fallo HTTP {}: Reintentando...", res.status());
                    }
                }
                Err(e) => {
                    eprintln!("⚠️ Error de Red: {}. Reintentando...", e);
                }
            }

            if attempt < max_retries {
                println!("💤 Esperando {:?} antes del siguiente intento...", delay);
                sleep(delay).await;
                // Multiplicamos por 2, con un techo de 60 segundos
                delay = std::cmp::min(delay * 2, Duration::from_secs(60));
            }
        }

        Err(anyhow!("Imposible hidratar el filtro tras {} intentos.", max_retries))
    }

    async fn acquire_job(&self) -> Result<WorkOrder> {
        let url = format!("{}/api/v1/swarm/job/acquire", self.base_url);
        let res = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send().await?;

        if !res.status().is_success() {
            return Err(anyhow!("Error adquiriendo trabajo: HTTP {}", res.status()));
        }

        res.json::<WorkOrder>().await.context("Error deserializando WorkOrder")
    }

    async fn send_keepalive(&self, job_id: &str) -> Result<()> {
        let url = format!("{}/api/v1/swarm/job/keepalive", self.base_url);
        let payload = JobCompletion { id: job_id.to_string() };

        self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&payload)
            .send().await?;
        Ok(())
    }

    async fn complete_job(&self, job_id: &str) -> Result<()> {
        let url = format!("{}/api/v1/swarm/job/complete", self.base_url);
        let payload = JobCompletion { id: job_id.to_string() };

        self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&payload)
            .send().await?;
        Ok(())
    }

    async fn report_finding(&self, finding: &Finding) -> Result<()> {
        let url = format!("{}/api/v1/swarm/finding", self.base_url);
        loop {
            match self.client.post(&url)
                .header("Authorization", format!("Bearer {}", self.token))
                .json(finding)
                .send().await
            {
                Ok(res) if res.status().is_success() => return Ok(()),
                Ok(res) => eprintln!("⚠️ Error reportando hallazgo: HTTP {}", res.status()),
                Err(e) => eprintln!("⚠️ Error de red reportando hallazgo: {}", e),
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}

// --- CANAL DE REPORTE ---
struct ChannelReporter {
    sender: mpsc::UnboundedSender<Finding>,
}

impl FindingHandler for ChannelReporter {
    fn on_finding(&self, address: String, pk: SafePrivateKey, source: String) {
        println!("🚨 ¡COLISIÓN CONFIRMADA! Address: {}", address);
        let finding = Finding {
            address,
            private_key_wif: private_to_wif(&pk, false),
            source_entropy: source,
            wallet_type: "p2pkh_legacy".to_string(),
        };
        let _ = self.sender.send(finding);
    }
}

// --- MAIN LOOP ---
#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "info"); }
    }
    env_logger::init();

    let args = Args::parse();
    println!("🚀 WORKER {} INICIANDO SECUENCIA HYDRA v4.3", args.worker_id);

    // --- PANIC HOOK (TELEMETRÍA DE ÚLTIMO ALIENTO) ---
    // Clonamos args para moverlos dentro del closure del pánico
    let panic_args = args.clone();

    panic::set_hook(Box::new(move |panic_info| {
        let msg = panic_info.to_string();
        eprintln!("💀 FATAL ERROR (PANIC): {}", msg);

        // Usamos el cliente síncrono (blocking) porque el runtime async está muriendo
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/api/v1/swarm/panic", panic_args.orchestrator_url);

        let payload = serde_json::json!({
            "worker_id": panic_args.worker_id,
            "message": msg,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        // Intentamos enviar con un timeout corto. Fire and forget.
        let _ = client.post(url)
            .header("Authorization", format!("Bearer {}", panic_args.auth_token))
            .json(&payload)
            .timeout(Duration::from_secs(5))
            .send();
    }));

    let client = Arc::new(WorkerClient::new(args.orchestrator_url.clone(), args.auth_token.clone()));
    let filter_path = PathBuf::from("utxo_filter.bin");

    // 3. Hidratación Resiliente
    client.hydrate_filter(&filter_path).await
        .context("Fallo fatal en la fase de hidratación")?;

    // 4. Carga en RAM
    println!("🧠 Cargando Filtro en Memoria...");
    let filter = Arc::new(tokio::task::spawn_blocking(move || {
        RichListFilter::load_from_file(&filter_path).expect("Filtro corrupto o ilegible")
    }).await?);
    println!("🧠 Filtro cargado. Listo para minar.");

    // 5. Canales
    let (tx, mut rx) = mpsc::unbounded_channel();
    let client_clone = client.clone();

    // 6. Sub-rutina de reporte
    tokio::spawn(async move {
        while let Some(finding) = rx.recv().await {
            println!("📤 Subiendo hallazgo a la Bóveda...");
            if let Err(e) = client_clone.report_finding(&finding).await {
                eprintln!("❌ Imposible reportar hallazgo: {}", e);
            } else {
                println!("✅ Hallazgo asegurado.");
            }
        }
    });

    // 7. Manejo de Señales
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\n🛑 Señal de terminación recibida.");
        r.store(false, Ordering::SeqCst);
    }).unwrap_or_default();

    // 8. BUCLE DE TRABAJO
    while running.load(Ordering::Relaxed) {
        match client.acquire_job().await {
            Ok(job) => {
                let job_id = job.id.clone();
                println!("🔨 JOB ADQUIRIDO: {} [{:?}]", job_id, job.strategy);

                let ka_client = client.clone();
                let ka_job_id = job_id.clone();
                let ka_running = running.clone();

                let keep_alive_handle = tokio::spawn(async move {
                    while ka_running.load(Ordering::Relaxed) {
                        sleep(Duration::from_secs(30)).await;
                        if let Err(e) = ka_client.send_keepalive(&ka_job_id).await {
                            eprintln!("⚠️ KeepAlive falló: {}", e);
                        }
                    }
                });

                let f_ref = filter.clone();
                let reporter = ChannelReporter { sender: tx.clone() };
                let context = ExecutorContext::default();

                let mining_result = tokio::task::spawn_blocking(move || {
                    StrategyExecutor::execute(&job, &f_ref, &context, &reporter);
                }).await;

                keep_alive_handle.abort();

                if let Err(e) = mining_result {
                    eprintln!("❌ Error crítico en motor de minería: {}", e);
                } else {
                    if let Err(e) = client.complete_job(&job_id).await {
                        eprintln!("❌ Error reportando completitud: {}", e);
                    } else {
                        println!("🏁 Job {} finalizado.", job_id);
                    }
                }
            },
            Err(e) => {
                println!("💤 Esperando asignación (Error/Idle: {})...", e);
                sleep(Duration::from_secs(10)).await;
            }
        }
    }

    println!("👋 Worker desconectado.");
    Ok(())
}
