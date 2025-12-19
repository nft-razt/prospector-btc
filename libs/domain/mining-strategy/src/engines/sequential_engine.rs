/**
 * =================================================================
 * APARATO: PROJECTIVE SEQUENTIAL KERNEL (V45.0 - ZERO-LATENCY)
 * CLASIFICACIÓN: DOMAIN STRATEGY (L2)
 * RESPONSABILIDAD: AUDITORÍA SECUENCIAL U256 MEDIANTE GEOMETRÍA JACOBIANA
 *
 * ESTRATEGIA DE ÉLITE:
 * - Geometric Accumulation: En lugar de P = k * G, realizamos P = P + G.
 * - Field Synergy: Handshake directo con FieldElement para reducir registros.
 * - Atomic Telemetry: Sincronización de 'computational_effort_volume' sin bloqueos.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use prospector_core_math::prelude::*;
use prospector_core_math::point::{JacobianPoint, AffinePoint};
use prospector_core_probabilistic::sharded::ShardedFilter;
use crate::executor::FindingHandler;

pub struct ProjectiveSequentialEngine;

impl ProjectiveSequentialEngine {
    /**
     * Ejecuta una secuencia de auditoría ininterrumpida sobre un rango de la curva.
     *
     * @param start_index_hex Punto de partida en formato hexadecimal.
     * @param iteration_limit Volumen de claves a auditar en esta misión.
     * @param target_filter Mapa UTXO fragmentado en RAM.
     * @param effort_counter Referencia atómica para telemetría global.
     */
    pub fn execute_atomic_scan<H: FindingHandler>(
        start_index_hex: &str,
        iteration_limit: u64,
        target_filter: &ShardedFilter,
        stop_signal: &AtomicBool,
        effort_counter: Arc<AtomicU64>,
        collision_handler: &H,
    ) -> String {
        // 1. HIDRATACIÓN DEL ESCALAR INICIAL (k)
        let mut current_private_key_bytes = [0u8; 32];
        let _ = hex::decode_to_slice(start_index_hex, &mut current_private_key_bytes);

        // 2. ASCENSIÓN A LA GEOMETRÍA JACOBIANA (P = k * G)
        let private_scalar = SafePrivateKey::from_bytes(&current_private_key_bytes)
            .expect("FATAL: Invalid start index for secp256k1");

        let initial_public_key = SafePublicKey::from_private(&private_scalar);
        let mut current_jacobian_point = JacobianPoint::from_affine(&initial_public_key.to_affine());

        // El generador G pre-computado en L1
        let generator_g_jacobian = JacobianPoint::generator_g();

        let mut internal_iteration_accumulator: u64 = 0;

        // 3. THE HOT LOOP (Zero-Inversion Path)
        while internal_iteration_accumulator < iteration_limit {
            if stop_signal.load(Ordering::Relaxed) { break; }

            // A. VERIFICACIÓN PROBABILÍSTICA
            // Convertimos a Afín solo para la validación (requiere 1 inverso modular)
            // NOTA: Se puede optimizar aún más comparando directamente en el espacio proyectivo
            let affine_candidate = current_jacobian_point.to_affine().unwrap();
            let bitcoin_address = prospector_core_gen::address_legacy::pubkey_to_address_from_affine(&affine_candidate, false);

            if target_filter.contains(&bitcoin_address) {
                let found_sk = SafePrivateKey::from_bytes(&current_private_key_bytes).unwrap();
                collision_handler.on_finding(bitcoin_address, found_sk, "strat_projective_v15".into());
            }

            // B. ADICIÓN GEOMÉTRICA (P = P + G)
            // Esta es la operación de Élite: solo multiplicaciones y sumas de campo mod p.
            current_jacobian_point = current_jacobian_point.add(&generator_g_jacobian);

            // C. INCREMENTO DEL ESCALAR (Audit Trail Sync)
            let _ = add_u64_to_u256_be(&mut current_private_key_bytes, 1);

            internal_iteration_accumulator += 1;

            // D. REPORTE DE TELEMETRÍA (Batching 1k para optimizar bus de memoria)
            if internal_iteration_accumulator % 1000 == 0 {
                effort_counter.fetch_add(1000, Ordering::Relaxed);
            }
        }

        // Retornamos el último punto auditado como Huella Forense
        hex::encode(current_private_key_bytes)
    }
}
