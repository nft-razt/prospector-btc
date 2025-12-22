/**
 * =================================================================
 * APARATO: HYDRA WORKER KERNEL (V90.0 - SOBERANO CLEAN)
 * CLASIFICACIÓN: APPLICATION LAYER (ENTRY POINT)
 * RESPONSABILIDAD: ORQUESTACIÓN DE MISIÓN Y PROTOCOLO DE SELLADO
 * =================================================================
 */

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{info, warn};

use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::work::{WorkOrder, MissionRequestPayload, NodeHardwareCapacity};
use prospector_domain_models::finding::Finding;
use prospector_domain_strategy::executor::{StrategyExecutor, FindingHandler};
use prospector_infra_worker_client::WorkerClient;
use prospector_core_math::private_key::SafePrivateKey;

/// Configuración de resiliencia del nodo.
const FILTRATION_PARTITION_COUNT: usize = 4;

#[derive(Parser, Debug)]
#[command(author, version, about = "Hydra-Zero Sovereign Audit Node")]
struct WorkerArguments {
    #[arg(long, env = "ORCHESTRATOR_URL")]
    orchestrator_endpoint: String,
    #[arg(long, env = "WORKER_AUTH_TOKEN")]
    authentication_token: String,
    #[arg(long, default_value = "hydra-node-alpha")]
    worker_node_identifier: String,
}

struct SwarmFindingReporter {
    tx: mpsc::UnboundedSender<Finding>,
    node_id: String,
}

impl FindingHandler for SwarmFindingReporter {
    fn on_finding(&self, address: String, private_key: SafePrivateKey, source: String) {
        let discovery = Finding {
            address,
            private_key_wif: prospector_core_gen::wif::private_to_wif(&private_key, false),
            source_entropy: source,
            wallet_type: "p2pkh_legacy".to_string(),
            found_by_worker: self.node_id.clone(),
            job_id: None,
            detected_at: chrono::Utc::now().to_rfc3339(),
        };
        let _ = self.tx.send(discovery);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let configuration = WorkerArguments::parse();

    info!("🛡️ [KERNEL]: Ignition sequence started for unit [{}]", configuration.worker_node_identifier);

    let orchestrator_uplink = Arc::new(WorkerClient::new(
        configuration.orchestrator_endpoint.clone(),
        configuration.authentication_token.clone(),
    ));

    let (findings_tx, mut findings_rx) = mpsc::unbounded_channel::<Finding>();
    let reporter_uplink = Arc::clone(&orchestrator_uplink);

    tokio::spawn(async move {
        while let Some(collision) = findings_rx.recv().await {
            let _ = reporter_uplink.transmit_found_collision(&collision).await;
        }
    });

    let global_shutdown_signal = Arc::new(AtomicBool::new(false));
    let shutdown_flag = Arc::clone(&global_shutdown_signal);

    tokio::spawn(async move {
        if let Ok(_) = tokio::signal::ctrl_c().await {
            warn!("⚠️ [SIGNAL]: Termination requested. Sealing audit trail...");
            shutdown_flag.store(true, Ordering::SeqCst);
        }
    });

    let cache_dir = PathBuf::from("census_cache");

    while !global_shutdown_signal.load(Ordering::SeqCst) {
        let handshake_payload = MissionRequestPayload {
            worker_id: configuration.worker_node_identifier.clone(),
            hardware_capacity: NodeHardwareCapacity {
                ram_available_mb: 8192,
                cpu_cores: num_cpus::get() as u32,
                supports_avx2: is_x86_feature_detected!("avx2"),
            },
        };

        match orchestrator_uplink.negotiate_mission_assignment(&handshake_payload).await {
            Ok(mission_order) => {
                if let Err(e) = execute_mission_lifecycle(
                    mission_order,
                    &orchestrator_uplink,
                    findings_tx.clone(),
                    Arc::clone(&global_shutdown_signal),
                    configuration.worker_node_identifier.clone(),
                    &cache_dir
                ).await {
                    warn!("⚠️ [MISSION_ABORTED]: {}", e);
                }
            }
            Err(_) => tokio::time::sleep(Duration::from_secs(15)).await,
        }
    }

    Ok(())
}

async fn execute_mission_lifecycle(
    mission_order: WorkOrder,
    uplink: &Arc<WorkerClient>,
    findings_sender: mpsc::UnboundedSender<Finding>,
    shutdown_signal: Arc<AtomicBool>,
    node_id: String,
    cache_dir: &PathBuf
) -> Result<()> {
    uplink.hydrate_mission_census_strata(&mission_order, cache_dir).await?;

    let strata_label = match mission_order.required_strata {
        prospector_domain_models::work::TargetStrata::SatoshiEra => "satoshi_era",
        _ => "standard_legacy",
    };
    let filter_path = cache_dir.join(strata_label);

    let filter = Arc::new(tokio::task::spawn_blocking(move || {
        ShardedFilter::load_from_directory(&filter_path, FILTRATION_PARTITION_COUNT)
    }).await??);

    let effort_accumulator = Arc::new(AtomicU64::new(0));
    let reporter = SwarmFindingReporter { tx: findings_sender, node_id: node_id.clone() };

    let audit_report = tokio::task::spawn_blocking(move || {
        StrategyExecutor::execute_mission_sequence(
            &mission_order,
            &filter,
            shutdown_signal,
            effort_accumulator,
            node_id,
            &reporter
        )
    }).await?;

    uplink.submit_audit_certification(&audit_report).await?;
    Ok(())
}
