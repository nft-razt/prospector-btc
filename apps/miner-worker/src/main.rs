// apps/miner-worker/src/main.rs
// =================================================================
// APARATO: MINER WORKER KERNEL (v5.5 - SHARDED)
// RESPONSABILIDAD: ORQUESTACIÓN CON CARGA DE DATOS PARALELA
// =================================================================

mod cpu_manager;

use clap::Parser;
use std::panic;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use tokio::sync::mpsc;
use tokio::time::sleep;

// ✅ CAMBIO CRÍTICO: Usamos ShardedFilter
use prospector_core_gen::wif::private_to_wif;
use prospector_core_math::private_key::SafePrivateKey;
use prospector_core_probabilistic::sharded::ShardedFilter;

use prospector_domain_models::Finding;
use prospector_domain_strategy::{ExecutorContext, FindingHandler, StrategyExecutor};
use prospector_infra_worker_client::WorkerClient;

/// Configuración de Sharding por defecto.
/// Debe coincidir con lo generado por Census Taker.
const DEFAULT_SHARD_COUNT: usize = 4;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
struct Args {
    #[arg(long, env = "ORCHESTRATOR_URL")]
    orchestrator_url: String,

    #[arg(long, env = "WORKER_AUTH_TOKEN")]
    auth_token: String,

    #[arg(long, default_value = "drone-unit-generic")]
    worker_id: String,
}

struct ChannelReporter {
    sender: mpsc::UnboundedSender<Finding>,
}

impl FindingHandler for ChannelReporter {
    fn on_finding(&self, address: String, pk: SafePrivateKey, source: String) {
        println!("\n🚨 ¡COLISIÓN CONFIRMADA! Address: {}", address);
        let finding = Finding {
            address,
            private_key_wif: private_to_wif(&pk, false),
            source_entropy: source,
            wallet_type: "p2pkh_legacy".to_string(),
        };
        if let Err(e) = self.sender.send(finding) {
            eprintln!("❌ ERROR CRÍTICO REPORTE: {}", e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    let args = Args::parse();
    println!("🚀 WORKER {} ONLINE [HYDRA v5.5 SHARDED]", args.worker_id);

    // 1. Hardware Optimization (Elite CPU Affinity)
    if let Err(e) = cpu_manager::optimize_process_affinity() {
        eprintln!("⚠️ Affinity Warning: {}", e);
    }

    // 2. Panic Telemetry
    let panic_url = args.orchestrator_url.clone();
    let panic_token = args.auth_token.clone();
    let panic_id = args.worker_id.clone();
    panic::set_hook(Box::new(move |info| {
        let msg = info.to_string();
        eprintln!("💀 PANIC: {}", msg);
        WorkerClient::send_panic_blocking(&panic_url, &panic_token, &panic_id, &msg);
    }));

    // 3. Client Init
    let client = Arc::new(WorkerClient::new(
        args.orchestrator_url.clone(),
        args.auth_token.clone(),
    )?);

    // 4. DATA HYDRATION (PARALLEL SHARDS)
    // Descargamos 4 archivos simultáneamente en lugar de 1 gigante.
    let filter_dir = PathBuf::from("filters_data");

    println!(
        "⬇️ Iniciando descarga paralela de {} shards...",
        DEFAULT_SHARD_COUNT
    );
    client
        .hydrate_shards(&filter_dir, DEFAULT_SHARD_COUNT)
        .await
        .context("Fallo fatal en hidratación de shards")?;

    // 5. MEMORY LOADING
    println!("🧠 Cargando ShardedFilter en RAM...");
    let filter = Arc::new(
        tokio::task::spawn_blocking(move || {
            // Carga los 4 archivos usando mmap en paralelo (definido en core/sharded.rs)
            ShardedFilter::load_from_dir(&filter_dir, DEFAULT_SHARD_COUNT)
                .expect("Datos corruptos o ilegibles en disco")
        })
        .await?,
    );

    println!(
        "✅ Filtro Operativo. Elementos Indexados: {}",
        filter.total_count()
    );

    // 6. Report Channel
    let (tx, mut rx) = mpsc::unbounded_channel();
    let client_reporter = client.clone();

    tokio::spawn(async move {
        while let Some(finding) = rx.recv().await {
            println!("📤 Enviando hallazgo...");
            if let Err(e) = client_reporter.report_finding(&finding).await {
                eprintln!("❌ ERROR RED: {}", e);
            } else {
                println!("✅ Hallazgo asegurado en Bóveda.");
            }
        }
    });

    // 7. Main Loop
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\n🛑 Señal de parada. Apagando...");
        r.store(false, Ordering::SeqCst);
    })
    .unwrap_or_default();

    while running.load(Ordering::Relaxed) {
        match client.acquire_job().await {
            Ok(job) => {
                let job_id = job.id.clone();
                println!("🔨 JOB: {} [{:?}]", job_id, job.strategy);

                // A. Heartbeat
                let ka_client = client.clone();
                let ka_job_id = job_id.clone();
                let ka_running = running.clone();
                let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel();

                tokio::spawn(async move {
                    loop {
                        tokio::select! {
                            _ = sleep(Duration::from_secs(30)) => {
                                if !ka_running.load(Ordering::Relaxed) { break; }
                                let _ = ka_client.send_keepalive(&ka_job_id).await;
                            }
                            _ = &mut stop_rx => break,
                        }
                    }
                });

                // B. Ejecución
                let f_ref = filter.clone();
                let reporter = ChannelReporter { sender: tx.clone() };
                let ctx = ExecutorContext::default();

                let start = std::time::Instant::now();

                // ELITE EXECUTION BLOCK
                let res = tokio::task::spawn_blocking(move || {
                    StrategyExecutor::execute(&job, &f_ref, &ctx, &reporter);
                })
                .await;

                let _ = stop_tx.send(());

                if let Err(e) = res {
                    eprintln!("❌ Error ejecución (Panic): {}", e);
                } else {
                    let _ = client.complete_job(&job_id).await;
                    println!("🏁 Fin Job: {:.2?}", start.elapsed());
                }
            }
            Err(e) => {
                println!("💤 Esperando asignación ({})", e);
                sleep(Duration::from_secs(10)).await;
            }
        }
    }

    Ok(())
}
