/**
 * =================================================================
 * APARATO: STRATEGY EXECUTOR BRIDGE (V217.0 - DOCUMENTED)
 * CLASIFICACIÓN: DOMAIN LOGIC (ESTRATO L2)
 * RESPONSABILIDAD: DESPACHO DE ALGORITMOS Y GENERACIÓN DE REPORTES
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use chrono::Utc;
use tracing::{info, error, instrument};

use crate::engines::sequential_engine::ProjectiveSequentialEngine;
use crate::engines::satoshi_xp_engine::SatoshiWindowsXpForensicEngine;
use prospector_domain_models::work::{AuditReport, SearchStrategy, WorkOrder};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::private_key::SafePrivateKey;

/// Contrato para el manejo de hallazgos criptográficos.
/// Permite desacoplar la lógica de minería del sistema de reporte.
pub trait FindingHandler: Send + Sync {
    /// Invocado cuando se detecta una colisión confirmada.
    ///
    /// # Argumentos
    /// * `address` - Dirección pública encontrada.
    /// * `private_key` - Clave privada recuperada.
    /// * `source` - Metadatos del origen (ej: "sequential_scan").
    fn on_finding(&self, address: String, private_key: SafePrivateKey, source: String);
}

/// Orquestador de estrategias de búsqueda.
/// Actúa como fachada para los diferentes motores de auditoría.
pub struct StrategyExecutor;

impl StrategyExecutor {
    /// Ejecuta una misión de auditoría completa y devuelve el certificado de esfuerzo.
    #[instrument(skip(mission_order, target_census, termination_signal, effort_accumulator, finding_callback))]
    pub fn execute_mission_sequence<H: FindingHandler>(
        mission_order: &WorkOrder,
        target_census: &ShardedFilter,
        termination_signal: Arc<AtomicBool>,
        effort_accumulator: Arc<AtomicU64>,
        node_identifier: String,
        finding_callback: &H,
    ) -> AuditReport {
        let start_execution_time = std::time::Instant::now();
        let mut mission_audit_footprint = String::new();
        let mut final_status_label = "completed".to_string();

        info!("🚀 [EXECUTOR]: Commencing mission segment {}.", mission_order.job_mission_identifier);

        match &mission_order.strategy {
            SearchStrategy::Sequential { start_index_hexadecimal, .. } => {
                mission_audit_footprint = ProjectiveSequentialEngine::execute_optimized_audit(
                    start_index_hexadecimal,
                    10_000_000,
                    target_census,
                    &termination_signal,
                    effort_accumulator.clone(),
                    finding_callback
                );
            },

            SearchStrategy::SatoshiWindowsXpForensic {
                scenario_template_identifier: _,
                uptime_seconds_start,
                uptime_seconds_end,
                hardware_clock_frequency
            } => {
                let mock_template = vec![0u8; 250000];

                mission_audit_footprint = SatoshiWindowsXpForensicEngine::execute_forensic_audit(
                    &mock_template,
                    *hardware_clock_frequency,
                    *uptime_seconds_start,
                    *uptime_seconds_end,
                    target_census,
                    &termination_signal,
                    effort_accumulator.clone(),
                    finding_callback
                );
            },

            _ => {
                error!("❌ [STRATEGY_FAULT]: Strategy not implemented.");
                final_status_label = "error_unsupported_strategy".to_string();
            }
        }

        if termination_signal.load(Ordering::SeqCst) {
            final_status_label = "interrupted_by_signal".to_string();
        }

        AuditReport {
            job_mission_identifier: mission_order.job_mission_identifier.clone(),
            worker_node_identifier: node_identifier,
            computational_effort_volume: effort_accumulator.load(Ordering::SeqCst).to_string(),
            execution_duration_milliseconds: start_execution_time.elapsed().as_millis() as u64,
            final_mission_status: final_status_label,
            audit_footprint_checkpoint: mission_audit_footprint,
            completed_at_timestamp: Utc::now().to_rfc3339(),
        }
    }
}
