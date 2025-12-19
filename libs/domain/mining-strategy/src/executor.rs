/**
 * =================================================================
 * APARATO: STRATEGY EXECUTOR KERNEL (V65.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN LOGIC (L2)
 * RESPONSABILIDAD: ORQUESTACIÓN POLIMÓRFICA DE AUDITORÍAS CRIPTOGRÁFICAS
 *
 * ESTRATEGIA DE ÉLITE:
 * - Dispatcher Polimórfico: Selecciona el motor atómico (Sequential, Dictionary, Forensic) en tiempo de ejecución.
 * - Zero-Inversion Hot Path: Utiliza coordenadas Jacobianas para avanzar en la curva sin divisiones modulares.
 * - Audit Trail Integrity: Genera un certificado inmutable (AuditReport) con la huella forense del trabajo realizado.
 * - Signal Awareness: Capacidad de interrupción atómica para despliegues efímeros (Google Colab).
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::Instant;
use chrono::Utc;
use tracing::{info, warn, error, instrument};

// --- SINAPSIS INTERNA: NÚCLEO MATEMÁTICO (L1) ---
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;

// --- SINAPSIS INTERNA: MODELOS DE DOMINIO (L2) ---
use prospector_domain_models::work::{AuditReport, SearchStrategy, WorkOrder};

// --- SINAPSIS INTERNA: MOTORES ATÓMICOS (L2-SUB) ---
use crate::engines::sequential_engine::ProjectiveSequentialEngine;
use crate::engines::dictionary_engine::EntropyDictionaryEngine;
use crate::engines::forensic_engine::ForensicArchaeologyEngine;

/**
 * Interfaz soberana para el reporte inmediato de hallazgos criptográficos.
 * Define el contrato de comunicación entre el motor matemático y el enlace de red.
 */
pub trait FindingHandler: Send + Sync {
    /**
     * Invocado al detectar una colisión positiva en el Filtro de Bloom.
     *
     * @param address Dirección Bitcoin (P2PKH) recuperada.
     * @param private_key Clave privada segura de 256 bits.
     * @param source Identificador de la fuente de entropía o motor generador.
     */
    fn on_finding(&self, address: String, private_key: SafePrivateKey, source: String);
}

/// Orquestador central de estrategias de búsqueda.
pub struct StrategyExecutor;

impl StrategyExecutor {
    /**
     * Ejecuta una secuencia de misión completa basada en la directiva de la Orden de Trabajo.
     *
     * @param mission_order Definición de la tarea y estrategia asignada por el Orquestador.
     * @param target_filter Estructura probabilística de UTXOs cargada en memoria.
     * @param shutdown_signal Receptor atómico de señales de terminación del sistema operativo.
     * @param collision_handler Delegado para la transmisión de resultados positivos.
     *
     * @returns AuditReport Certificado de integridad y esfuerzo computacional nivelado.
     */
    #[instrument(skip(mission_order, target_filter, shutdown_signal, collision_handler))]
    pub fn execute_mission_sequence<H: FindingHandler>(
        mission_order: &WorkOrder,
        target_filter: &ShardedFilter,
        shutdown_signal: Arc<AtomicBool>,
        collision_handler: &H,
    ) -> AuditReport {
        let execution_start_timer = Instant::now();
        let cumulative_effort_counter = Arc::new(AtomicU64::new(0));

        let mut final_mission_status = "completed".to_string();
        let mut audit_footprint_checkpoint = String::new();

        info!(
            "🚀 [EXECUTOR_IGNITION]: Starting mission {} using strategy {:?}",
            mission_order.job_mission_identifier,
            mission_order.strategy
        );

        // --- DESPACHO POLIMÓRFICO (ATOMIZACIÓN DE ESCENARIOS) ---
        match &mission_order.strategy {

            // ESCENARIO 1: Auditoría Secuencial Proyectiva (Espacio U256)
            SearchStrategy::Sequential { start_index_hex, .. } => {
                // Delegación al motor L2 optimizado con Coordenadas Jacobianas
                audit_footprint_checkpoint = ProjectiveSequentialEngine::execute_atomic_scan(
                    start_index_hex,
                    1_000_000, // Bloque de auditoría estándar de 1M hashes
                    target_filter,
                    &shutdown_signal,
                    cumulative_effort_counter.clone(),
                    collision_handler
                );
            },

            // ESCENARIO 2: Auditoría de Diccionarios de Entropía Humana (Brainwallets)
            SearchStrategy::Dictionary { dataset_url, batch_size } => {
                // TODO: Implementar el Streaming de datasets remotos vía Buffer O(1)
                // Por ahora se asume una hidratación local previa en el Estrato L4
                let mock_dataset = vec!["correct horse battery staple".to_string()];

                audit_footprint_checkpoint = EntropyDictionaryEngine::execute_dictionary_audit(
                    &mock_dataset,
                    target_filter,
                    &shutdown_signal,
                    cumulative_effort_counter.clone(),
                    collision_handler
                );
            },

            // ESCENARIO 3: Arqueología Forense (Simulación de PRNGs rotos)
            SearchStrategy::ForensicScan { vulnerability_target, .. } => {
                audit_footprint_checkpoint = ForensicArchaeologyEngine::execute_forensic_scan(
                    vulnerability_target,
                    target_filter,
                    &shutdown_signal,
                    cumulative_effort_counter.clone(),
                    collision_handler
                );
            },

            // ESCENARIO 4: Validación de Handshake Estático
            SearchStrategy::StaticHandshake { secret_source } => {
                Self::run_static_audit(
                    secret_source,
                    target_filter,
                    cumulative_effort_counter.clone(),
                    collision_handler
                );
                audit_footprint_checkpoint = secret_source.clone();
            }
        }

        // --- VALIDACIÓN DE ESTADO DE SALIDA ---
        if shutdown_signal.load(Ordering::Relaxed) {
            final_mission_status = "interrupted".to_string();
            warn!("🛑 [EXECUTOR_HALT]: Mission suspended by operator signal.");
        }

        // --- GENERACIÓN DEL CERTIFICADO DE AUDITORÍA (NIVELACIÓN V8.5) ---
        // Sincronizado con el Dashboard y el Strategic Ledger (Supabase)
        AuditReport {
            job_mission_identifier: mission_order.job_mission_identifier.clone(),
            worker_node_identifier: "hydra-secure-unit-v9".to_string(), // Inyectado por el Kernel de arranque
            computational_effort_volume: cumulative_effort_counter.load(Ordering::SeqCst).to_string(),
            execution_duration_ms: execution_start_timer.elapsed().as_millis() as u64,
            final_mission_status,
            audit_footprint_checkpoint,
            completed_at_timestamp: Utc::now().to_rfc3339(),
        }
    }

    /**
     * Motor interno para la validación inmediata de vectores de secreto únicos.
     *
     * @param secret Frase o semilla a auditar.
     */
    fn run_static_audit<H: FindingHandler>(
        secret: &str,
        filter: &ShardedFilter,
        counter: Arc<AtomicU64>,
        handler: &H
    ) {
        // Derivación directa SHA256 -> P2PKH
        let private_key = prospector_domain_strategy::brainwallet::phrase_to_private_key(secret);
        let public_key = SafePublicKey::from_private(&private_key);
        let address = prospector_core_gen::address_legacy::pubkey_to_address(&public_key, false);

        if filter.contains(&address) {
            handler.on_finding(address, private_key, "static_verification_audit".into());
        }
        counter.fetch_add(1, Ordering::Relaxed);
    }
}
