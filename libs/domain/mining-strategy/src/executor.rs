/**
 * =================================================================
 * APARATO: STRATEGY EXECUTOR KERNEL (V110.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN LOGIC (ESTRATO L2)
 * RESPONSABILIDAD: ORQUESTACIÓN DE MOTORES Y CANALIZACIÓN DE HALLAZGOS
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::Instant;
use tokio::sync::mpsc;
use chrono::Utc;
use tracing::{info, error};

use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::work::{AuditReport, SearchStrategy, WorkOrder};
use prospector_domain_models::Finding;

// Motores Atómicos
use crate::engines::sequential_engine::ProjectiveSequentialEngine;
use crate::engines::satoshi_xp_engine::SatoshiWindowsXpForensicEngine;

pub struct StrategyExecutor;

impl StrategyExecutor {
    /**
     * Ejecuta una misión de auditoría completa con reporte asíncrono.
     *
     * @param mission_order Instrucciones de rango y estrategia.
     * @param target_census_filter Filtro de Bloom particionado en RAM.
     * @param global_shutdown_signal Flag para terminación ordenada (Signal Handling).
     * @param findings_channel Canal de transmisión hacia el uplink de red.
     */
    pub fn execute_mission_sequence(
        mission_order: &WorkOrder,
        target_census_filter: &ShardedFilter,
        global_shutdown_signal: Arc<AtomicBool>,
        findings_channel: mpsc::UnboundedSender<Finding>,
        worker_id: String,
    ) -> AuditReport {
        let execution_start_timer = Instant::now();
        let cumulative_effort_counter = Arc::new(AtomicU64::new(0));
        let mut audit_footprint_checkpoint = String::new();
        let mut final_mission_status = "completed".to_string();

        info!("🚀 [EXECUTOR]: Ignition for mission {}", mission_order.job_mission_identifier);

        // Handler interno que empaqueta hallazgos y los envía al canal asíncrono
        let collision_handler = |address: String, private_key: SafePrivateKey, source: String| {
            let discovery = Finding {
                address,
                private_key_wif: prospector_core_gen::wif::private_to_wif(&private_key, false),
                source_entropy: source,
                wallet_type: "p2pkh_legacy_uncompressed".into(),
                found_by_worker: worker_id.clone(),
                job_id: Some(mission_order.job_mission_identifier.clone()),
                detected_at: Utc::now().to_rfc3339(),
            };
            let _ = findings_channel.send(discovery);
        };

        match &mission_order.strategy {
            // ESTRATEGIA A: Auditoría Secuencial Proyectiva (U256)
            SearchStrategy::Sequential { start_index_hexadecimal, end_index_hexadecimal: _ } => {
                audit_footprint_checkpoint = ProjectiveSequentialEngine::execute_atomic_scan(
                    start_index_hexadecimal,
                    10_000_000, // Volumen de ráfaga por misión
                    target_census_filter,
                    &global_shutdown_signal,
                    cumulative_effort_counter.clone(),
                    &collision_handler
                );
            },

            // ESTRATEGIA B: Arqueología Forense (Windows XP SP3)
            SearchStrategy::SatoshiWindowsXpForensic {
                scenario_template_identifier: _,
                uptime_seconds_start,
                uptime_seconds_end,
                hardware_clock_frequency
            } => {
                // Aquí se inyecta la simulación de saturación del MD_POOL de OpenSSL
                SatoshiWindowsXpForensicEngine::execute_high_speed_audit(
                    *hardware_clock_frequency,
                    *uptime_seconds_start,
                    *uptime_seconds_end,
                    target_census_filter,
                    &global_shutdown_signal,
                    cumulative_effort_counter.clone(),
                    &collision_handler
                );
                audit_footprint_checkpoint = format!("uptime_sec_{}", uptime_seconds_end);
            },

            _ => {
                error!("❌ [STRATEGY_FAULT]: Logic not implemented for target strata.");
                final_mission_status = "error_unsupported".into();
            }
        }

        if global_shutdown_signal.load(Ordering::SeqCst) {
            final_mission_status = "interrupted_by_signal".into();
        }

        AuditReport {
            job_mission_identifier: mission_order.job_mission_identifier.clone(),
            worker_node_identifier: worker_id,
            computational_effort_volume: cumulative_effort_counter.load(Ordering::SeqCst).to_string(),
            execution_duration_milliseconds: execution_start_timer.elapsed().as_millis() as u64,
            final_mission_status,
            audit_footprint_checkpoint,
            completed_at_timestamp: Utc::now().to_rfc3339(),
        }
    }
}
