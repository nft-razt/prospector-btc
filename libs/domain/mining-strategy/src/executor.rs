/**
 * =================================================================
 * APARATO: STRATEGY EXECUTOR MASTER BRIDGE (V220.0 - GOLD MASTER)
 * CLASIFICACIÓN: DOMAIN LOGIC (ESTRATO L2)
 * RESPONSABILIDAD: ORQUESTACIÓN SOBERANA DE MOTORES DE AUDITORÍA
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el centro neurálgico de despacho en el Worker.
 * Coordina la ejecución de las tres estrategias maestras (Sequential,
 * Forensic XP, y Kangaroo Lambda) garantizando que cada ciclo de
 * cómputo genere un 'AuditReport' inmutable y certificado.
 *
 * # Mathematical Proof:
 * El executor garantiza la separación de contextos (Isolating Runtime).
 * Utiliza adición Jacobiana para secuenciales y Pollard's Lambda para
 * resolución de logaritmo discreto en rangos conocidos.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use chrono::Utc;
use tracing::{info, warn, error, instrument};

// --- SINAPSIS CON MOTORES DE ESTRATEGIA (L2) ---
use crate::engines::sequential_engine::ProjectiveSequentialEngine;
use crate::engines::satoshi_xp_engine::SatoshiWindowsXpForensicEngine;
use crate::kangaroo::KangarooRunner;

// --- SINAPSIS CON MODELOS Y NÚCLEO (L1/L2) ---
use prospector_domain_models::work::{AuditReport, SearchStrategy, WorkOrder};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::private_key::SafePrivateKey;

/**
 * Contrato para el manejo de hallazgos criptográficos.
 * Implementado por el sistema de reporte del worker para transmitir colisiones al Orquestador.
 */
pub trait FindingHandler: Send + Sync {
    /**
     * Invocado inmediatamente al detectar una coincidencia en el filtro de Bloom.
     *
     * # Arguments
     * * `address` - Dirección Bitcoin (P2PKH) recuperada.
     * * `private_key` - Escalar privado derivado del material de entropía.
     * * `source` - Metadatos del origen para la reconstrucción forense en la tesis.
     */
    fn on_finding(&self, address: String, private_key: SafePrivateKey, source: String);
}

pub struct StrategyExecutor;

impl StrategyExecutor {
    /**
     * Ejecuta una misión de auditoría completa basada en la estrategia inyectada.
     *
     * # Errors:
     * Retorna un reporte con estatus 'error_missing_dna_artifact' si la estrategia
     * Satoshi-XP es invocada sin un buffer de rendimiento válido.
     *
     * # Performance:
     * Satura los hilos de CPU disponibles mediante el uso de adición Jacobiana O(1).
     * El throughput es reportado periódicamente al 'effort_accumulator'.
     *
     * @param mission_order Orden de trabajo con parámetros tácticos y límites de rango.
     * @param target_census Filtro de Bloom particionado cargado en la memoria RAM del nodo.
     * @param termination_signal Monitor de señales del sistema para interrupción de emergencia.
     * @param effort_accumulator Contador atómico de alta frecuencia para métricas de hashrate.
     * @param node_identifier Identificador único de la unidad física ejecutora.
     * @param finding_callback Delegado encargado de la persistencia de hallazgos positivos.
     * @param performance_dna_template Buffer binario opcional conteniendo el ADN de Windows XP.
     */
    #[instrument(skip(
        mission_order,
        target_census,
        termination_signal,
        effort_accumulator,
        finding_callback,
        performance_dna_template
    ))]
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
            "🚀 [EXECUTOR]: Commencing mission segment [{}] for Strata [{:?}].",
            mission_order.job_mission_identifier,
            mission_order.required_strata
        );

        match &mission_order.strategy {
            // ESTRATEGIA 01: BARRIDO SECUENCIAL JACOBIANO (U256)
            SearchStrategy::Sequential { start_index_hexadecimal, .. } => {
                mission_audit_checkpoint = ProjectiveSequentialEngine::execute_optimized_audit(
                    start_index_hexadecimal,
                    10_000_000, // Segmento nominal de ráfaga antes de checkpoint
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
                    error!("❌ [STRATEGY_FAULT]: Satoshi-XP requires a valid DNA template buffer.");
                    final_mission_status = "error_missing_dna_artifact".to_string();
                }
            },

            // ESTRATEGIA 03: RESOLUCIÓN POLLARD'S KANGAROO (ECDLP)
            // Se utiliza el campo 'dataset_resource_locator' como proxy para la clave pública objetivo
            SearchStrategy::Dictionary { dataset_resource_locator, .. } => {
                KangarooRunner::run(
                    dataset_resource_locator,
                    "0", // Offset de escalar base
                    1_000_000, // Ancho de banda de búsqueda
                    finding_callback
                );
                mission_audit_checkpoint = "kangaroo_lambda_completed".to_string();
            }
        }

        // Validación de interrupción por preemption (Google Colab / Signal)
        if termination_signal.load(Ordering::SeqCst) {
            final_mission_status = "interrupted_by_signal".to_string();
            warn!("⚠️ [EXECUTOR]: Execution detoured by signal. Sealing checkpoint.");
        }

        AuditReport {
            job_mission_identifier: mission_order.job_mission_identifier.clone(),
            worker_node_identifier: node_identifier,
            computational_effort_volume: effort_accumulator.load(Ordering::SeqCst).to_string(),
            execution_duration_milliseconds: start_execution_instant.elapsed().as_millis() as u64,
            final_mission_status,
            audit_footprint_checkpoint: mission_audit_checkpoint,
            completed_at_timestamp: Utc::now().to_rfc3339(),
        }
    }
}
