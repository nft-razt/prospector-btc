/**
 * =================================================================
 * APARATO: MINER EXECUTION ENGINE (V11.0 - FREE TIER HARDENED)
 * CLASIFICACIÓN: WORKER EXECUTION LAYER (ESTRATO L1-WORKER)
 * RESPONSABILIDAD: ORQUESTACIÓN DE MISIONES Y PROTECCIÓN DE RAM
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{info, warn, error, instrument};

// --- SINAPSIS INTERNA (V10.8 Core Sync) ---
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::work::{
    WorkOrder, AuditReport, MissionRequestPayload, NodeHardwareCapacity, TargetStrata
};
use prospector_domain_models::finding::Finding;
use prospector_domain_strategy::executor::{StrategyExecutor, FindingHandler};
use prospector_infra_worker_client::WorkerClient;
use crate::cpu_manager::HardwareMonitor;

struct SwarmFindingReporter {
    transmission_channel_sender: mpsc::UnboundedSender<Finding>,
    worker_node_identifier: String,
    active_mission_identifier: String,
}

impl FindingHandler for SwarmFindingReporter {
    fn on_finding(
        &self,
        address: String,
        private_key: prospector_core_math::private_key::SafePrivateKey,
        source: String
    ) {
        let discovery = Finding {
            address,
            private_key_wif: prospector_core_gen::wif::private_to_wif(&private_key, false),
            source_entropy: source,
            wallet_type: "p2pkh_legacy_uncompressed".to_string(),
            found_by_worker: self.worker_node_identifier.clone(),
            job_id: Some(self.active_mission_identifier.clone()),
            detected_at: chrono::Utc::now().to_rfc3339(),
        };
        let _ = self.transmission_channel_sender.send(discovery);
    }
}

pub struct MinerEngine {
    orchestrator_uplink: Arc<WorkerClient>,
    is_operational: Arc<AtomicBool>,
    worker_node_identifier: String,
    local_cache_directory: std::path::PathBuf,
}

impl MinerEngine {
    pub fn new(
        client: Arc<WorkerClient>,
        operational_signal: Arc<AtomicBool>,
        node_id: String,
        cache_path: std::path::PathBuf,
    ) -> Self {
        Self {
            orchestrator_uplink: client,
            is_operational: operational_signal,
            worker_node_identifier: node_id,
            local_cache_directory: cache_path,
        }
    }

    /**
     * Valida si la misión es apta para los 12GB de RAM del tier gratuito.
     * Previene el OOM (Out of Memory) y el baneo por inestabilidad de proceso.
     */
    fn is_resource_safe(&self, strata: &TargetStrata) -> bool {
        match strata {
            // El set táctico completo requiere hardware dedicado (>16GB RAM)
            TargetStrata::FullTacticalSet => {
                warn!("🛑 [MEMORY_SHIELD]: Mission rejected. Strata too large for Free Tier RAM.");
                false
            },
            _ => true,
        }
    }

    pub async fn ignite(&self) {
        info!("🚀 [ENGINE]: Sovereign node online. Initializing V11 mission loop.");

        let (findings_tx, mut findings_rx) = mpsc::unbounded_channel::<Finding>();
        let reporter_client = Arc::clone(&self.orchestrator_uplink);

        tokio::spawn(async move {
            while let Some(collision) = findings_rx.recv().await {
                if let Err(error) = reporter_client.transmit_found_collision(&collision).await {
                    error!("❌ [VAULT_SYNC_FAULT]: {}", error);
                }
            }
        });

        while self.is_operational.load(Ordering::Relaxed) {
            let hardware_metrics = HardwareMonitor::capture_instantaneous_metrics();
            let handshake_payload = MissionRequestPayload {
                worker_id: self.worker_node_identifier.clone(),
                hardware_capacity: NodeHardwareCapacity {
                    ram_available_mb: hardware_metrics.memory_utilization_bytes / (1024 * 1024),
                    cpu_cores: num_cpus::get() as u32,
                    supports_avx2: is_x86_feature_detected!("avx2"),
                },
            };

            match self.orchestrator_uplink.negotiate_mission_assignment(&handshake_payload).await {
                Ok(mission_order) => {
                    // --- NIVELACIÓN MVP: Memory Shield ---
                    if !self.is_resource_safe(&mission_order.required_strata) {
                        sleep(Duration::from_secs(60)).await;
                        continue;
                    }

                    if let Err(error) = self.process_mission_lifecycle(mission_order, findings_tx.clone()).await {
                        error!("⚠️ [MISSION_ABORTED]: {}", error);
                    }
                }
                Err(error) => {
                    warn!("💤 [IDLE]: Re-trying in 30s. Detail: {}", error);
                    sleep(Duration::from_secs(30)).await;
                }
            }
        }
    }

    async fn process_mission_lifecycle(
        &self,
        order: WorkOrder,
        findings_sender: mpsc::UnboundedSender<Finding>
    ) -> anyhow::Result<()> {
        let mission_id = order.job_mission_identifier.clone();

        // Hidratación y Carga (Preservando lógica V10.8)
        self.orchestrator_uplink.hydrate_mission_census_strata(&order, &self.local_cache_directory).await?;

        let strata_id = match order.required_strata {
            TargetStrata::SatoshiEra => "satoshi_era",
            TargetStrata::VulnerableLegacy => "vulnerable_legacy",
            TargetStrata::StandardLegacy => "standard_legacy",
            _ => "unknown",
        };

        let filter_path = self.local_cache_directory.join(strata_id);
        let target_filter = Arc::new(
            tokio::task::spawn_blocking(move || {
                ShardedFilter::load_from_directory(&filter_path, 4)
            }).await??
        );

        let effort_accumulator = Arc::new(AtomicU64::new(0));
        let reporter = SwarmFindingReporter {
            transmission_channel_sender: findings_sender,
            worker_node_identifier: self.worker_node_identifier.clone(),
            active_mission_identifier: mission_id.clone(),
        };

        let strategy_effort_ref = Arc::clone(&effort_accumulator);
        let stop_signal = Arc::clone(&self.is_operational);
        let node_id = self.worker_node_identifier.clone();

        let audit_report = tokio::task::spawn_blocking(move || {
            StrategyExecutor::execute_mission_sequence(
                &order,
                &target_filter,
                stop_signal,
                strategy_effort_ref,
                node_id,
                &reporter
            )
        }).await?;

        self.orchestrator_uplink.submit_audit_certification(&audit_report).await?;
        Ok(())
    }
}
