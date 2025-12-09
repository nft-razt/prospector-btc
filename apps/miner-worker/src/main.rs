// =================================================================
// APARATO: MINER WORKER (SMART OPERATOR v2.1.2)
// ESTADO: FIXED (SYNTAX & MATCH ARMS)
// =================================================================

use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

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
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, env = "ORCHESTRATOR_URL")]
    orchestrator_url: String,

    #[arg(long, env = "WORKER_AUTH_TOKEN")]
    auth_token: String,

    #[arg(long, default_value = "drone-unit-v2")]
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

    /// Descarga el filtro UTXO
    async fn hydrate_filter(&self, path: &PathBuf) -> Result<()> {
        if path.exists() {
            println!("✅ [CACHE] Filtro local detectado.");
            return Ok(());
        }
        println!("⬇️ [NET] Descargando Filtro UTXO...");

        let url = format!("{}/resources/utxo_filter.bin", self.base_url);
        let res = self.client.get(&url).send().await?;

        if !res.status().is_success() {
            return Err(anyhow!("Fallo descarga filtro: HTTP {}", res.status()));
        }

        let bytes = res.bytes().await?;
        tokio::fs::write(path, bytes).await?;
        println!("✅ [IO] Filtro guardado.");
        Ok(())
    }

    /// Solicita trabajo (Lease)
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

    /// Envía señal de vida
    async fn send_keepalive(&self, job_id: &str) -> Result<()> {
        let url = format!("{}/api/v1/swarm/job/keepalive", self.base_url);
        let payload = JobCompletion { id: job_id.to_string() };

        self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&payload)
            .send().await?;
        Ok(())
    }

    /// Marca el trabajo como completado
    async fn complete_job(&self, job_id: &str) -> Result<()> {
        let url = format!("{}/api/v1/swarm/job/complete", self.base_url);
        let payload = JobCompletion { id: job_id.to_string() };

        self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&payload)
            .send().await?;
        Ok(())
    }

    /// Reporta un hallazgo
    async fn report_finding(&self, finding: &Finding) -> Result<()> {
        let url = format!("{}/api/v1/swarm/finding", self.base_url);
        self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(finding)
            .send().await?;
        Ok(())
    }
}

// --- CANAL DE REPORTE ---
struct ChannelReporter {
    sender: mpsc::UnboundedSender<Finding>,
}

impl FindingHandler for ChannelReporter {
    fn on_finding(&self, address: String, pk: SafePrivateKey, source: String) {
        println!("🚨 ¡COLISIÓN! Address: {}", address);
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
    // Configuración de logs con bloque unsafe para evitar E0133
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    let args = Args::parse();
    println!("🚀 WORKER {} INICIANDO SECUENCIA HYDRA", args.worker_id);

    let client = Arc::new(WorkerClient::new(args.orchestrator_url.clone(), args.auth_token.clone()));
    let filter_path = PathBuf::from("utxo_filter.bin");

    // 1. Hidratación
    loop {
        match client.hydrate_filter(&filter_path).await {
            Ok(_) => break,
            Err(e) => {
                eprintln!("⚠️ Fallo de hidratación: {}. Reintentando en 5s...", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }

    // 2. Carga en RAM
    println!("🧠 Cargando Filtro en Memoria...");
    let filter = Arc::new(tokio::task::spawn_blocking(move || {
        RichListFilter::load_from_file(&filter_path).expect("Filtro corrupto")
    }).await?);

    // 3. Preparación de Canales
    let (tx, mut rx) = mpsc::unbounded_channel();
    let client_clone = client.clone();

    // 4. Sub-rutina de reporte de hallazgos
    tokio::spawn(async move {
        while let Some(finding) = rx.recv().await {
            println!("📤 Subiendo hallazgo...");
            loop {
                match client_clone.report_finding(&finding).await {
                    Ok(_) => { println!("✅ Hallazgo confirmado."); break; },
                    Err(_) => sleep(Duration::from_secs(2)).await,
                }
            }
        }
    });

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\n🛑 Deteniendo worker...");
        r.store(false, Ordering::SeqCst);
    }).unwrap_or_default();

    // 5. BUCLE DE TRABAJO (CORREGIDO)
    while running.load(Ordering::Relaxed) {
        // MATCH: Maneja Ok(job) y Err(e) explícitamente
        match client.acquire_job().await {
            Ok(job) => {
                let job_id = job.id.clone();
                println!("🔨 TRABAJANDO: Job {} [{:?}]", job_id, job.strategy);

                // A. KEEPALIVE (Background Task)
                let ka_client = client.clone();
                let ka_job_id = job_id.clone();
                let keep_alive_task = tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(30)).await;
                        if let Err(e) = ka_client.send_keepalive(&ka_job_id).await {
                            eprintln!("⚠️ KeepAlive falló: {}", e);
                        }
                    }
                });

                // B. MINERÍA (Blocking Task)
                let f_ref = filter.clone();
                let reporter = ChannelReporter { sender: tx.clone() };
                let context = ExecutorContext::default();

                let mining_result = tokio::task::spawn_blocking(move || {
                    StrategyExecutor::execute(&job, &f_ref, &context, &reporter);
                }).await;

                // C. LIMPIEZA
                keep_alive_task.abort();

                if let Err(e) = mining_result {
                    eprintln!("❌ Error en minería: {}", e);
                } else {
                    // D. COMMIT FINAL
                    if let Err(e) = client.complete_job(&job_id).await {
                        eprintln!("❌ Error completando job: {}", e);
                    } else {
                        println!("🏁 Job completado.");
                    }
                }
            }, // FIN DEL BLOQUE OK
            Err(e) => {
                println!("💤 Sin trabajo o error de red: {}. Esperando 10s...", e);
                sleep(Duration::from_secs(10)).await;
            } // FIN DEL BLOQUE ERR
        } // FIN DEL MATCH
    } // FIN DEL WHILE

    println!("👋 Worker desconectado.");
    Ok(())
}
