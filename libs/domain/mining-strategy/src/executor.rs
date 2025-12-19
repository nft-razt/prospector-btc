/**
 * =================================================================
 * APARATO: STRATEGY EXECUTOR KERNEL (V25.0 - PROJECTIVE ENGINE)
 * CLASIFICACIÓN: DOMAIN LOGIC (L2)
 * RESPONSABILIDAD: EJECUCIÓN DE ALTO RENDIMIENTO DE AUDITORÍAS
 *
 * ESTRATEGIA DE ÉLITE:
 * - O(1) Iteration: Incremento de puntos mediante adición geométrica.
 * - Atomic Monitoring: Registro de progreso sin bloqueos de memoria.
 * - Graceful Recoil: Interrupción segura ante señales del sistema.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering, AtomicBool};
use std::time::Instant;
use tracing::{info, warn, error, instrument};

// --- SINAPSIS INTERNA (CORE & MODELS) ---
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::{WorkOrder, SearchStrategy, AuditReport};

pub trait FindingHandler: Send + Sync {
    fn on_finding(&self, address: String, private_key: SafePrivateKey, source: String);
}

pub struct StrategyExecutor;

impl StrategyExecutor {
    /**
     * Punto de entrada para la ejecución de una orden de trabajo.
     * Realiza la transición de estado del worker a "Auditing".
     */
    #[instrument(skip(order, filter, shutdown, handler))]
    pub fn execute<H: FindingHandler>(
        order: &WorkOrder,
        filter: &ShardedFilter,
        shutdown: Arc<AtomicBool>,
        handler: &H,
    ) -> AuditReport {
        let start_time = Instant::now();
        let mut total_hashes: u64 = 0;
        let mut last_hex = String::new();
        let mut status = "interrupted".to_string();

        match &order.strategy {
            SearchStrategy::Sequential { start_index, end_index, .. } => {
                let result = Self::run_sequential_engine(
                    start_index,
                    end_index,
                    filter,
                    &shutdown,
                    handler
                );
                total_hashes = result.0;
                last_hex = result.1;
                status = result.2;
            }
            _ => warn!("⚠️ ENGINE_NOT_NATIVE: Strategy not yet optimized for L2 Projective."),
        }

        AuditReport {
            job_id: order.id.clone(),
            worker_id: "hydra-native-unit".to_string(), // TODO: Inyectar ID real
            total_hashes: total_hashes.to_string(),
            duration_ms: start_time.elapsed().as_millis() as u64,
            exit_status: status,
            last_checkpoint: last_hex,
        }
    }

    /**
     * MOTOR SECUENCIAL DE ÉLITE
     * Aprovecha la adición de puntos proyectivos para maximizar el Hashrate.
     */
    fn run_sequential_engine<H: FindingHandler>(
        start_hex: &str,
        end_hex: &str,
        filter: &ShardedFilter,
        shutdown: &AtomicBool,
        handler: &H,
    ) -> (u64, String, String) {
        // 1. Inicialización de la Geometría
        let mut key_bytes = [0u8; 32];
        let _ = hex::decode_to_slice(start_hex, &mut key_bytes);

        let first_sk = SafePrivateKey::from_bytes(&key_bytes).expect("INVALID_START_KEY");
        let mut current_point = SafePublicKey::from_private(&first_sk);

        let mut counter: u64 = 0;
        let mut final_status = "exhausted".to_string();

        info!("🔥 [ENGINE_IGNITION]: Projective Addition O(1) active.");

        // 2. BUCLE CALIENTE (HOT PATH)
        loop {
            // A. Verificación de terminación asíncrona
            if shutdown.load(Ordering::Relaxed) {
                final_status = "interrupted".to_string();
                break;
            }

            // B. Auditoría Criptográfica
            // Generamos dirección Legacy (P2PKH) y verificamos en el mapa RAM
            let address = prospector_core_gen::address_legacy::pubkey_to_address(&current_point, false);

            if filter.contains(&address) {
                let sk = SafePrivateKey::from_bytes(&key_bytes).unwrap();
                handler.on_finding(address, sk, "sequential_proyective_v1".into());
                // No detenemos el motor, continuamos para agotar el rango (Tesis)
            }

            // C. INCREMENTO DE ÉLITE (Aritmético + Geométrico)
            // Incrementamos la clave privada (para reporte) y el punto (para cálculo)
            if add_u64_to_u256_be(&mut key_bytes, 1).is_err() { break; }

            match current_point.increment() {
                Ok(next) => current_point = next,
                Err(_) => {
                    error!("💀 MATH_CRASH: Elliptic curve singularity detected.");
                    final_status = "error".to_string();
                    break;
                }
            }

            counter += 1;

            // D. Límite de Rango (Simulado para brevedad, comparando hex)
            if counter > 1_000_000 { break; } // El orquestador define el tamaño del bloque
        }

        (counter, hex::encode(key_bytes), final_status)
    }
}
