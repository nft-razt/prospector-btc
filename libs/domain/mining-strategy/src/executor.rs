/**
 * =================================================================
 * APARATO: STRATEGY EXECUTOR MASTER BRIDGE (V240.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN LOGIC (ESTRATO L2)
 * RESPONSABILIDAD: ORQUESTACIÓN SOBERANA Y ANÁLISIS DE EFICIENCIA
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la unidad central de procesamiento del Worker. Coordina
 * la ejecución de misiones y realiza la auditoría de rendimiento final,
 * calculando la eficiencia computacional (Hashes/ms) antes de sellar
 * el reporte de auditoría inmutable.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use chrono::Utc;
use tracing::{info, warn, error, instrument};

// --- SINAPSIS CON MOTORES DE ESTRATEGIA ATÓMICOS ---
use crate::engines::sequential_engine::ProjectiveSequentialEngine;
use crate::engines::satoshi_xp_engine::SatoshiWindowsXpForensicEngine;
use crate::engines::android_lcg_engine::AndroidLcgForensicEngine;
use crate::engines::dictionary_engine::EntropyDictionaryEngine;
use crate::kangaroo::KangarooRunner;

// --- SINAPSIS CON MODELOS Y NÚCLEO ---
use prospector_domain_models::work::{AuditReport, SearchStrategy, WorkOrder};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::private_key::SafePrivateKey;

/**
 * Interfaz soberana para la gestión de colisiones detectadas.
 */
pub trait FindingHandler: Send + Sync {
    /**
     * Acción ejecutada inmediatamente al confirmar una colisión en el censo.
     */
    fn on_finding(&self, address: String, private_key: SafePrivateKey, source: String);
}

pub struct StrategyExecutor;

impl StrategyExecutor {
    /**
     * Ejecuta una misión táctica completa y certifica los resultados.
     *
     * # Mathematical Proof:
     * Garantiza el aislamiento de estados entre misiones. Realiza el cálculo
     * de eficiencia al finalizar para permitir optimizaciones futuras del algoritmo.
     */
    #[instrument(skip(mission_order, target_census, termination_signal, effort_accumulator, finding_callback, performance_dna_template))]
    pub fn execute_mission_sequence<H: FindingHandler>(
        mission_order: &WorkOrder,
        target_census: &ShardedFilter,
        termination_signal: Arc<AtomicBool>,
        effort_accumulator: Arc<AtomicU64>,
        node_identifier: String,
        finding_callback: &H,
        performance_dna_template: Option<&[u8]>
    ) -> AuditReport {
        let start_execution_instant = std::time::Instant::now();
        let mut mission_audit_checkpoint = String::new();
        let mut final_mission_status = "completed".to_string();

        info!(
            "🚀 [EXECUTOR]: Dispatching mission [{}] to atomic engines.",
            mission_order.job_mission_identifier
        );

        match &mission_order.strategy {
            // ESTRATEGIA 01: BARRIDO SECUENCIAL JACOBIANO (U256)
            SearchStrategy::Sequential { start_index_hexadecimal, .. } => {
                mission_audit_checkpoint = ProjectiveSequentialEngine::execute_optimized_audit(
                    start_index_hexadecimal,
                    10_000_000, // Límite de ráfaga antes de rotar
                    target_census,
                    &termination_signal,
                    effort_accumulator.clone(),
                    finding_callback
                );
            },

            // ESTRATEGIA 02: ARQUEOLOGÍA FORENSE SATOSHI-XP (Windows 2009)
            SearchStrategy::SatoshiWindowsXpForensic {
                uptime_seconds_start,
                uptime_seconds_end,
                hardware_clock_frequency,
                ..
            } => {
                if let Some(dna_buffer) = performance_dna_template {
                    mission_audit_checkpoint = SatoshiWindowsXpForensicEngine::execute_forensic_audit(
                        dna_buffer,
                        *hardware_clock_frequency,
                        *uptime_seconds_start,
                        *uptime_seconds_end,
                        target_census,
                        &termination_signal,
                        effort_accumulator.clone(),
                        finding_callback
                    );
                } else {
                    error!("❌ [EXECUTOR_FAULT]: DNA template required for Satoshi-XP strategy.");
                    final_mission_status = "error_missing_dna_artifact".to_string();
                }
            },

            // ESTRATEGIA 03: ARQUEOLOGÍA ANDROID (LCG CVE-2013)
            SearchStrategy::AndroidLcgForensic { seed_range_start, seed_range_end } => {
                mission_audit_checkpoint = AndroidLcgForensicEngine::execute_seed_sweep(
                    *seed_range_start,
                    *seed_range_end,
                    target_census,
                    &termination_signal,
                    effort_accumulator.clone(),
                    finding_callback
                );
            },

            // ESTRATEGIA 04: RESOLUCIÓN POLLARD'S KANGAROO
            SearchStrategy::KangarooLambda { target_public_key_hexadecimal, range_width_max } => {
                KangarooRunner::run(
                    target_public_key_hexadecimal,
                    "0",
                    *range_width_max,
                    finding_callback
                );
                mission_audit_checkpoint = "kangaroo_lambda_completed".to_string();
            },

            // ESTRATEGIA 05: DICCIONARIOS DE ENTROPÍA (BRAINWALLETS)
            SearchStrategy::Dictionary { dataset_resource_locator, .. } => {
                mission_audit_checkpoint = EntropyDictionaryEngine::execute_dictionary_audit(
                    &[dataset_resource_locator.clone()],
                    target_census,
                    &termination_signal,
                    effort_accumulator.clone(),
                    finding_callback
                );
            }
        }

        // 5. ANÁLISIS DE EFICIENCIA Y CIERRE DEL REPORTE
        let total_execution_time_milliseconds = start_execution_instant.elapsed().as_millis() as u64;
        let total_wallets_consulted = effort_accumulator.load(Ordering::SeqCst);

        let average_computational_efficiency = if total_execution_time_milliseconds > 0 {
            (total_wallets_consulted as f64) / (total_execution_time_milliseconds as f64)
        } else {
            0.0
        };

        if termination_signal.load(Ordering::SeqCst) {
            final_mission_status = "interrupted_by_signal".to_string();
            warn!("⚠️ [EXECUTOR]: Mission interrupted. Securing partial checkpoint.");
        }

        info!(
            "🏁 [MISSION_COMPLETE]: Audited {} wallets. Efficiency: {:.2} H/ms",
            total_wallets_consulted,
            average_computational_efficiency
        );

        AuditReport {
            job_mission_identifier: mission_order.job_mission_identifier.clone(),
            worker_node_identifier: node_identifier,
            total_wallets_audited: total_wallets_consulted.to_string(),
            execution_duration_milliseconds: total_execution_time_milliseconds,
            final_mission_status,
            audit_footprint_checkpoint: mission_audit_checkpoint,
            completed_at_timestamp: Utc::now().to_rfc3339(),
            average_computational_efficiency,
        }
    }
}
