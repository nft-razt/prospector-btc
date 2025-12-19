/**
 * =================================================================
 * APARATO: MINER WORKER KERNEL (V35.0 - ELITE HARDENED)
 * CLASIFICACIÓN: APPLICATION LAYER (L1)
 * RESPONSABILIDAD: ORQUESTACIÓN DE BÚSQUEDA Y GESTIÓN ZK
 *
 * ESTRATEGIA DE ÉLITE:
 * - Secure Ignition: Descifrado In-Memory de identidades.
 * - Hardware Pinning: Afinidad de núcleos para evitar context-switching.
 * - Sharded Hydration: Descarga paralela de filtros Bloom.
 * - Atomic Telemetry: Contador de hashes inyectado en el Kernel Assembler.
 * =================================================================
 */
mod cpu_manager;

use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info, warn};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

// --- SINAPSIS INTERNA (Nx Monorepo) ---
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::{Finding, WorkerHeartbeat, WorkerSnapshot};
use prospector_domain_strategy::{ExecutorContext, FindingHandler, StrategyExecutor};
use prospector_infra_worker_client::WorkerClient;

/// Configuración Operativa del Enjambre
const FILTRATION_SHARD_COUNT: usize = 4;
const HEARTBEAT_INTERVAL_SECONDS: u64 = 30;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "Hydra-Zero Secure Node Kernel")]
struct WorkerArguments {
    #[arg(long, env = "ORCHESTRATOR_URL")]
    orchestrator_endpoint: String,

    #[arg(long, env = "WORKER_AUTH_TOKEN")]
    authentication_token: String,

    #[arg(long, env = "MASTER_VAULT_KEY")]
    master_vault_key: String,

    #[arg(long, default_value = "hydra-secure-unit")]
    worker_identifier: String,
}

/**
 * Manejador de Hallazgos con Puente de Red.
 */
struct SwarmReporter {
    transmission_channel: mpsc::UnboundedSender<Finding>,
    node_id: String,
    active_job_id: Arc<tokio::sync::RwLock<Option<String>>>,
}

impl FindingHandler for SwarmReporter {
    fn on_finding(&self, address: String, private_key: SafePrivateKey, source: String) {
        use prospector_core_gen::wif::private_to_wif;

        let wif = private_to_wif(&private_key, false);
        info!("🚨 COLLISION_DETECTED: Target found at [{}]", address);

        let current_job =
            futures::executor::block_on(async { self.active_job_id.read().await.clone() });

        let discovery = Finding {
            address,
            private_key_wif: wif,
            source_entropy: source,
            wallet_type: "p2pkh_legacy_uncompressed".to_string(),
            found_by_worker: self.node_id.clone(),
            job_id: current_job,
            detected_at: chrono::Utc::now().to_rfc3339(),
        };

        let _ = self.transmission_channel.send(discovery);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let configuration = WorkerArguments::parse();

    info!(
        "🛡️  HYDRA_NODE: Initializing Secure Unit [ID: {}]",
        configuration.worker_identifier
    );

    // 1. OPTIMIZACIÓN DE HARDWARE (Thread Affinity)
    cpu_manager::optimize_process_affinity().context("Failed to secure CPU cores")?;

    // 2. SINAPSIS DE RED Y ADQUISICIÓN DE FILTROS
    let uplink_client = Arc::new(WorkerClient::new(
        configuration.orchestrator_endpoint.clone(),
        configuration.authentication_token.clone(),
    ));

    let cache_directory = PathBuf::from("swarm_data_cache");
    uplink_client
        .hydrate_shards(&cache_directory, FILTRATION_SHARD_COUNT)
        .await?;

    let target_filter = Arc::new(
        tokio::task::spawn_blocking(move || {
            ShardedFilter::load_from_dir(&cache_directory, FILTRATION_SHARD_COUNT)
        })
        .await?
        .context("Bloom Filter Hydration Failed")?,
    );

    // 3. CANAL DE REPORTE ASÍNCRONO
    let (finding_tx, mut finding_rx) = mpsc::unbounded_channel::<Finding>();
    let reporter_client = Arc::clone(&uplink_client);

    tokio::spawn(async move {
        while let Some(collision) = finding_rx.recv().await {
            if let Err(error) = reporter_client.report_finding(&collision).await {
                error!("⚠️  UPLINK_SYNC_FAULT: Could not secure finding: {}", error);
            }
        }
    });

    // 4. BUCLE DE TRABAJO ESTRATÉGICO
    let is_running = Arc::new(AtomicBool::new(true));
    let hash_counter = Arc::new(AtomicU64::new(0));
    let current_job_id = Arc::new(tokio::sync::RwLock::new(None));

    info!("🔥 [IGNITION]: Grid unit is now operational and awaiting tasks.");

    while is_running.load(Ordering::Relaxed) {
        match uplink_client.acquire_job().await {
            Ok(work_order) => {
                let assignment_id = work_order.id.clone();
                *current_job_id.write().await = Some(assignment_id.clone());

                let iteration_start = std::time::Instant::now();
                let iteration_counter = Arc::new(AtomicU64::new(0));

                // A. TAREA DE TELEMETRÍA (Heartbeat)
                let ka_running = Arc::clone(&is_running);
                let ka_client = Arc::clone(&uplink_client);
                let (stop_hb_tx, mut stop_hb_rx) = tokio::sync::oneshot::channel();
                let ka_node_id = configuration.worker_identifier.clone();

                tokio::spawn(async move {
                    loop {
                        tokio::select! {
                            _ = sleep(Duration::from_secs(HEARTBEAT_INTERVAL_SECONDS)) => {
                                if !ka_running.load(Ordering::Relaxed) { break; }
                                let _ = ka_client.send_heartbeat_lite(&ka_node_id).await;
                            }
                            _ = &mut stop_hb_rx => break,
                        }
                    }
                });

                // B. EJECUCIÓN DEL KERNEL ASSEMBLER (L1)
                let exec_filter = Arc::clone(&target_filter);
                let exec_counter = Arc::clone(&iteration_counter);
                let reporter = SwarmReporter {
                    transmission_channel: finding_tx.clone(),
                    node_id: configuration.worker_identifier.clone(),
                    active_job_id: Arc::clone(&current_job_id),
                };

                tokio::task::spawn_blocking(move || {
                    StrategyExecutor::execute(&work_order, &exec_filter, exec_counter, &reporter);
                })
                .await?;

                // C. REPORTE DE MÉTRICAS DOCTORALES (L4)
                let _ = stop_hb_tx.send(());
                let total_hashes = iteration_counter.load(Ordering::SeqCst);
                let duration = iteration_start.elapsed().as_secs();

                let _ = uplink_client
                    .complete_job_with_metrics(&assignment_id, total_hashes, duration)
                    .await;

                *current_job_id.write().await = None;
                hash_counter.fetch_add(total_hashes, Ordering::Relaxed);
            }
            Err(error) => {
                warn!("💤 [IDLE]: Waiting for network assignment ({}).", error);
                sleep(Duration::from_secs(10)).await;
            }
        }
    }

    Ok(())
}
