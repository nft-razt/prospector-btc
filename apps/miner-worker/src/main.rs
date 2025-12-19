/**
 * =================================================================
 * APARATO: HYDRA-ZERO WORKER KERNEL (V60.0 - ELITE HARDENED)
 * CLASIFICACIÓN: APPLICATION LAYER (L1)
 * RESPONSABILIDAD: ORQUESTACIÓN DE BÚSQUEDA Y GESTIÓN DE SEÑALES
 *
 * ESTRATEGIA DE ÉLITE:
 * - Signal Interception: Captura SIGTERM/SIGINT para sellado forense.
 * - Hardware Pinning: Afinidad de núcleos para evitar context-switching.
 * - Sharded Hydration: Descarga paralela de filtros Bloom O(1).
 * - Atomic Telemetry: Contador de hashes inyectado en el Kernel Assembler.
 * =================================================================
 */

mod cpu_manager;

use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info, warn};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::signal;

// --- SINAPSIS INTERNA (Nx Monorepo) ---
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::work::{AuditReport, WorkOrder};
use prospector_domain_models::Finding;
use prospector_domain_strategy::{StrategyExecutor, FindingHandler};
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

    #[arg(long, default_value = "hydra-secure-unit")]
    worker_identifier: String,
}

/**
 * Manejador de Hallazgos con Puente de Red.
 * Transmite colisiones al orquestador en tiempo real.
 */
struct SwarmReporter {
    transmission_channel: mpsc::UnboundedSender<Finding>,
    node_identifier: String,
    active_mission_id: Arc<tokio::sync::RwLock<Option<String>>>,
}

impl FindingHandler for SwarmReporter {
    fn on_finding(&self, address: String, private_key: SafePrivateKey, source: String) {
        use prospector_core_gen::wif::private_to_wif;

        let wallet_import_format = private_to_wif(&private_key, false);
        info!("🚨 [COLLISION_DETECTED]: Target found at [{}]", address);

        let current_mission = futures::executor::block_on(async {
            self.active_mission_id.read().await.clone()
        });

        let discovery = Finding {
            address,
            private_key_wif: wallet_import_format,
            source_entropy: source,
            wallet_type: "p2pkh_legacy_uncompressed".to_string(),
            found_by_worker: self.node_identifier.clone(),
            job_id: current_mission,
            detected_at: chrono::Utc::now().to_rfc3339(),
        };

        let _ = self.transmission_channel.send(discovery);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let configuration = WorkerArguments::parse();

    info!("🛡️  [HYDRA_KERNEL]: Initializing Secure Grid Unit [ID: {}]", configuration.worker_identifier);

    // 1. OPTIMIZACIÓN DE HARDWARE (Thread Affinity)
    cpu_manager::optimize_process_affinity().context("Failed to secure CPU cores")?;

    // 2. SINAPSIS DE RED Y HIDRATACIÓN DEL CENSO
    let uplink_client = Arc::new(WorkerClient::new(
        configuration.orchestrator_endpoint.clone(),
        configuration.authentication_token.clone(),
    ));

    let cache_directory = PathBuf::from("swarm_data_cache");
    uplink_client.hydrate_shards(&cache_directory, FILTRATION_SHARD_COUNT).await?;

    let target_filter = Arc::new(
        tokio::task::spawn_blocking(move || {
            ShardedFilter::load_from_dir(&cache_directory, FILTRATION_SHARD_COUNT)
        })
        .await?
        .context("Bloom Filter Hydration Failed")?,
    );

    // 3. CANAL DE REPORTE ASÍNCRONO PARA HALLAZGOS
    let (finding_tx, mut finding_rx) = mpsc::unbounded_channel::<Finding>();
    let reporter_client = Arc::clone(&uplink_client);

    tokio::spawn(async move {
        while let Some(collision) = finding_rx.recv().await {
            // Nota: report_finding usa el modelo Finding nivelado
            if let Err(error) = reporter_client.report_finding(&collision).await {
                error!("⚠️  [VAULT_SYNC_FAULT]: Could not secure finding: {}", error);
            }
        }
    });

    // 4. GESTIÓN SOBERANA DE SEÑALES (SIGTERM/SIGINT)
    let shutdown_signal = Arc::new(AtomicBool::new(false));
    let shutdown_clone = Arc::clone(&shutdown_signal);

    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        warn!("🛑 [SIGNAL_RECEIVED]: Initiating graceful mission seal...");
        shutdown_clone.store(true, Ordering::Relaxed);
    });

    // 5. BUCLE PRINCIPAL DE AUDITORÍA (THE MISSION LOOP)
    let current_mission_id = Arc::new(tokio::sync::RwLock::new(None));

    info!("🔥 [IGNITION]: Grid unit operational. Awaiting tactical assignments.");

    while !shutdown_signal.load(Ordering::Relaxed) {
        match uplink_client.request_mission_assignment(&configuration.worker_identifier).await {
            Ok(mission_order) => {
                let assignment_identifier = mission_order.job_mission_identifier.clone();
                *current_mission_id.write().await = Some(assignment_identifier.clone());

                // A. MONITOR DE SALUD (Heartbeat)
                let hb_signal = Arc::clone(&shutdown_signal);
                let hb_client = Arc::clone(&uplink_client);
                let hb_node_id = configuration.worker_identifier.clone();
                let (stop_hb_tx, mut stop_hb_rx) = tokio::sync::oneshot::channel();

                tokio::spawn(async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(HEARTBEAT_INTERVAL_SECONDS));
                    loop {
                        tokio::select! {
                            _ = interval.tick() => {
                                if hb_signal.load(Ordering::Relaxed) { break; }
                                let _ = hb_client.send_heartbeat_lite(&hb_node_id).await;
                            }
                            _ = &mut stop_hb_rx => break,
                        }
                    }
                });

                // B. EJECUCIÓN DEL KERNEL MATEMÁTICO (L1/L2 Handshake)
                let exec_filter = Arc::clone(&target_filter);
                let exec_signal = Arc::clone(&shutdown_signal);
                let reporter = SwarmReporter {
                    transmission_channel: finding_tx.clone(),
                    node_identifier: configuration.worker_identifier.clone(),
                    active_mission_id: Arc::clone(&current_mission_id),
                };

                let audit_report = tokio::task::spawn_blocking(move || {
                    StrategyExecutor::execute_mission_sequence(
                        &mission_order,
                        &exec_filter,
                        exec_signal,
                        &reporter
                    )
                }).await?;

                // C. CERTIFICACIÓN Y SELLADO (L3 Link)
                let _ = stop_hb_tx.send(()); // Detener latidos para esta misión

                if let Err(error) = uplink_client.submit_audit_certification(&audit_report).await {
                    error!("❌ [CERTIFICATION_FAILED]: Could not seal mission {}: {}",
                        assignment_identifier, error);
                } else {
                    info!("✅ [MISSION_SEALED]: Result archived in Strategic Ledger.");
                }

                *current_mission_id.write().await = None;

                // Si recibimos señal de apagado durante la ejecución, rompemos el bucle aquí
                if shutdown_signal.load(Ordering::Relaxed) { break; }
            },
            Err(error) => {
                warn!("💤 [IDLE]: Waiting for network mission ({}). Retrying in 10s.", error);
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    }

    info!("🏁 [KERNEL_SHUTDOWN]: Grid unit deactivated. All audit footprints secured.");
    Ok(())
}
