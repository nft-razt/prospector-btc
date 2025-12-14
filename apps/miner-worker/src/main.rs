// apps/miner-worker/src/main.rs
// =================================================================
// APARATO: MINER WORKER ENTRY POINT (v5.0 - ATOMIZED)
// ESTADO: CLEAN ARCHITECTURE (ORCHESTRATION ONLY)
// =================================================================

use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::panic;

// ASYNC & CHANNELS
use tokio::sync::mpsc;
use tokio::time::sleep;
use anyhow::{Context, Result};

// NÚCLEO & LÓGICA
use prospector_core_probabilistic::filter_wrapper::RichListFilter;
use prospector_core_gen::wif::private_to_wif;
use prospector_core_math::private_key::SafePrivateKey;
use prospector_domain_strategy::{StrategyExecutor, ExecutorContext, FindingHandler};
use prospector_domain_models::Finding;

// INFRAESTRUCTURA (ELITE CLIENT)
use prospector_infra_worker_client::WorkerClient;

#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(long, env = "ORCHESTRATOR_URL")]
    orchestrator_url: String,

    #[arg(long, env = "WORKER_AUTH_TOKEN")]
    auth_token: String,

    #[arg(long, default_value = "drone-unit-generic")]
    worker_id: String,
}

// --- HANDLER DE HALLAZGOS ---
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

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "info"); }
    }
    env_logger::init();

    let args = Args::parse();
    println!("🚀 WORKER {} INICIANDO SECUENCIA HYDRA v5.0 (Atomized)", args.worker_id);

    // 1. PANIC HOOK (TELEMETRÍA DE ÚLTIMO ALIENTO)
    // Delegamos la lógica "sucia" de red bloqueante a la librería de cliente.
    let panic_url = args.orchestrator_url.clone();
    let panic_token = args.auth_token.clone();
    let panic_id = args.worker_id.clone();

    panic::set_hook(Box::new(move |panic_info| {
        let msg = panic_info.to_string();
        eprintln!("💀 FATAL ERROR (PANIC): {}", msg);
        // Llamada estática al método bloqueante de la librería
        WorkerClient::send_panic_blocking(&panic_url, &panic_token, &panic_id, &msg);
    }));

    // 2. INICIALIZACIÓN DEL CLIENTE
    let client = Arc::new(WorkerClient::new(
        args.orchestrator_url.clone(),
        args.auth_token.clone()
    )?);

    // 3. HIDRATACIÓN DEL FILTRO
    let filter_path = PathBuf::from("utxo_filter.bin");
    client.hydrate_filter(&filter_path).await
        .context("Fallo fatal en la fase de hidratación")?;

    // 4. CARGA EN RAM (CPU INTENSIVE)
    println!("🧠 Cargando Filtro en Memoria...");
    let filter = Arc::new(tokio::task::spawn_blocking(move || {
        RichListFilter::load_from_file(&filter_path).expect("Filtro corrupto o ilegible")
    }).await?);
    println!("🧠 Filtro cargado. Listo para minar.");

    // 5. CANALES DE REPORTE
    let (tx, mut rx) = mpsc::unbounded_channel();
    let client_clone = client.clone();

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

    // 6. GESTIÓN DE SEÑALES (Graceful Shutdown)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\n🛑 Señal de terminación recibida.");
        r.store(false, Ordering::SeqCst);
    }).unwrap_or_default();

    // 7. BUCLE PRINCIPAL (ORCHESTRATION LOOP)
    while running.load(Ordering::Relaxed) {
        match client.acquire_job().await {
            Ok(job) => {
                let job_id = job.id.clone();
                println!("🔨 JOB ADQUIRIDO: {} [{:?}]", job_id, job.strategy);

                // A. KEEPALIVE HEARTBEAT (Background Task)
                let ka_client = client.clone();
                let ka_job_id = job_id.clone();
                let ka_running = running.clone(); // Para saber si el worker sigue vivo globalmente

                // Usamos un token de cancelación local para parar el heartbeat cuando el job termine
                let (ka_stop_tx, mut ka_stop_rx) = tokio::sync::oneshot::channel::<()>();

                let keep_alive_handle = tokio::spawn(async move {
                    loop {
                        tokio::select! {
                            _ = sleep(Duration::from_secs(30)) => {
                                if !ka_running.load(Ordering::Relaxed) { break; }
                                if let Err(e) = ka_client.send_keepalive(&ka_job_id).await {
                                    eprintln!("⚠️ KeepAlive falló: {}", e);
                                }
                            }
                            _ = &mut ka_stop_rx => {
                                break; // Señal de parada recibida
                            }
                        }
                    }
                });

                // B. EJECUCIÓN MATEMÁTICA (Blocking Thread / Rayon)
                let f_ref = filter.clone();
                let reporter = ChannelReporter { sender: tx.clone() };
                let context = ExecutorContext::default();

                // movemos job dentro
                let mining_result = tokio::task::spawn_blocking(move || {
                    StrategyExecutor::execute(&job, &f_ref, &context, &reporter);
                }).await;

                // C. FINALIZACIÓN
                // Detener heartbeat inmediatamente
                let _ = ka_stop_tx.send(());
                let _ = keep_alive_handle.await;

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
