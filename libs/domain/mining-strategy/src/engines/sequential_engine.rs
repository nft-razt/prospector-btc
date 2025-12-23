/**
 * =================================================================
 * APARATO: PROJECTIVE SEQUENTIAL ENGINE (V130.0 - BATCH PROJECTION)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: AUDITORÍA DE RANGO CONMontgomery Batch Inversion
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el protocolo de "Cargador Táctico". Acumula puntos en
 * el dominio Jacobiano y ejecuta la proyección afín en ráfagas de 256.
 * Esto erradica el coste del inverso modular individual, permitiendo
 * alcanzar velocidades de grado industrial en hardware efímero.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use crate::executor::FindingHandler;

/// Tamaño de la ráfaga de inversión por lotes.
const BATCH_MAGAZINE_SIZE: usize = 256;

pub struct ProjectiveSequentialEngine;

impl ProjectiveSequentialEngine {
    const GENERATOR_AFFINE_X_WORDS: [u64; 4] = [0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC];
    const GENERATOR_AFFINE_Y_WORDS: [u64; 4] = [0x9C47D08FFB10D4B8, 0xFD17B448A6855419, 0x5DA4FBFC0E1108A8, 0x483ADA7726A3C465];

    pub fn execute_optimized_audit<H: FindingHandler>(
        start_hexadecimal_index: &str,
        iteration_burst_limit: u64,
        target_census_filter: &ShardedFilter,
        termination_signal: &AtomicBool,
        effort_telemetry: Arc<AtomicU64>,
        finding_delegate: &H,
    ) -> String {
        // 1. INICIALIZACIÓN DEL ACUMULADOR SOBERANO
        let mut current_scalar_bytes = [0u8; 32];
        if hex::decode_to_slice(start_hexadecimal_index.trim(), &mut current_scalar_bytes).is_err() {
            return start_hexadecimal_index.to_string();
        }

        let private_key_instance = SafePrivateKey::from_bytes(&current_scalar_bytes).unwrap();
        let public_key_affine = SafePublicKey::from_private(&private_key_instance);
        let public_key_raw_bytes = public_key_affine.to_bytes(false);

        let mut current_jacobian_point = JacobianPoint::from_affine(
            bytes_to_words_u256(&public_key_raw_bytes[1..33].try_into().unwrap()),
            bytes_to_words_u256(&public_key_raw_bytes[33..65].try_into().unwrap())
        );

        let generator_affine_x = FieldElement { internal_words: Self::GENERATOR_AFFINE_X_WORDS };
        let generator_affine_y = FieldElement { internal_words: Self::GENERATOR_AFFINE_Y_WORDS };

        let mut processed_cycle_count: u64 = 0;

        // 2. MAGAZINE BUFFER: Buffer de ráfaga para Montgomery
        let mut points_magazine: Vec<JacobianPoint> = Vec::with_capacity(BATCH_MAGAZINE_SIZE);
        let mut scalars_magazine: Vec<[u8; 32]> = Vec::with_capacity(BATCH_MAGAZINE_SIZE);

        // 3. BUCLE CALIENTE (HOT PATH)
        while processed_cycle_count < iteration_burst_limit {
            if termination_signal.load(Ordering::Relaxed) { break; }

            // A. CARGA DEL MAGAZINE
            points_magazine.push(current_jacobian_point);
            scalars_magazine.push(current_scalar_bytes);

            // B. DISPARO DE RÁFAGA (Cuando el cargador está lleno)
            if points_magazine.len() == BATCH_MAGAZINE_SIZE {
                Self::process_magazine_burst(
                    &points_magazine,
                    &scalars_magazine,
                    target_census_filter,
                    finding_delegate
                );
                points_magazine.clear();
                scalars_magazine.clear();
                effort_telemetry.fetch_add(BATCH_MAGAZINE_SIZE as u64, Ordering::Relaxed);
            }

            // C. INCREMENTO JACOBIANO MIXTO (P = P + G)
            current_jacobian_point = UnifiedCurveEngine::add_mixed_deterministic(
                &current_jacobian_point,
                &generator_affine_x,
                &generator_affine_y
            );

            // D. INCREMENTO DEL ESCALAR TÁCTICO
            let _ = add_u64_to_u256_be(&mut current_scalar_bytes, 1);
            processed_cycle_count += 1;
        }

        // 4. LIMPIEZA DE REMANENTE (Flush del magazine incompleto)
        if !points_magazine.is_empty() {
            Self::process_magazine_burst(&points_magazine, &scalars_magazine, target_census_filter, finding_delegate);
            effort_telemetry.fetch_add(points_magazine.len() as u64, Ordering::Relaxed);
        }

        hex::encode(current_scalar_bytes)
    }

    /**
     * Procesa una ráfaga de puntos mediante Montgomery Batch Inversion.
     */
    fn process_magazine_burst<H: FindingHandler>(
        points: &[JacobianPoint],
        scalars: &[[u8; 32]],
        filter: &ShardedFilter,
        handler: &H
    ) {
        // 1. Extraer todas las coordenadas Z del lote
        let z_coords: Vec<FieldElement> = points.iter().map(|p| p.z).collect();

        // 2. INVERSIÓN MODULAR POR LOTE (Costo: 1 Inverso + 3N Multiplicaciones)
        let z_inverses = FieldElement::batch_invert_sovereign(&z_coords);

        // 3. PROYECCIÓN Y VERIFICACIÓN
        for i in 0..points.len() {
            let z_inv = &z_inverses[i];
            let z_inv_squared = z_inv.square_modular();

            // x = X * (Z^-2)
            let affine_x = points[i].x.multiply_modular(&z_inv_squared);
            let affine_x_bytes = words_to_bytes_u256(&affine_x.internal_words);

            // Derivación de dirección comprimida
            let bitcoin_address = prospector_core_gen::address_legacy::pubkey_from_x_to_address(&affine_x_bytes, true);

            if filter.contains(&bitcoin_address) {
                let safe_key = SafePrivateKey::from_bytes(&scalars[i]).unwrap();
                handler.on_finding(bitcoin_address, safe_key, "montgomery_batch_v130".into());
            }
        }
    }
}
