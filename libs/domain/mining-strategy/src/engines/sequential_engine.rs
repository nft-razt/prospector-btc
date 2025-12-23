/**
 * =================================================================
 * APARATO: PROJECTIVE SEQUENTIAL ENGINE (V150.0 - BATCH PROJECTION)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: AUDITORÍA DE RANGO CON INVERSIÓN POR LOTE
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
use tracing::{info, instrument};

/// Tamaño de la ráfaga (Magazine) para la inversión modular por lotes.
/// Optimizado para el uso de líneas de caché L1/L2.
const BATCH_MAGAZINE_SIZE: usize = 256;

pub struct ProjectiveSequentialEngine;

impl ProjectiveSequentialEngine {
    /// Coordenadas afines del Punto Generador G de secp256k1 (U64 Limbs).
    const GENERATOR_AFFINE_X_WORDS: [u64; 4] = [0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC];
    const GENERATOR_AFFINE_Y_WORDS: [u64; 4] = [0x9C47D08FFB10D4B8, 0xFD17B448A6855419, 0x5DA4FBFC0E1108A8, 0x483ADA7726A3C465];

    /**
     * Ejecuta una auditoría optimizada sobre un segmento escalar U256.
     *
     * # Performance:
     * Amortiza el coste de la división modular (Inverso) mediante Montgomery Batching.
     * Realiza 1 inverso modular y ~768 multiplicaciones por cada 256 wallets.
     */
    #[instrument(skip(target_census_filter, termination_signal, effort_telemetry, finding_delegate))]
    pub fn execute_optimized_audit<H: FindingHandler>(
        start_hexadecimal_index: &str,
        iteration_burst_limit: u64,
        target_census_filter: &ShardedFilter,
        termination_signal: &AtomicBool,
        effort_telemetry: Arc<AtomicU64>,
        finding_delegate: &H,
    ) -> String {
        // 1. INICIALIZACIÓN DEL ESCALAR Y PUNTO JACOBIANO
        let mut current_scalar_bytes = [0u8; 32];
        if hex::decode_to_slice(start_hexadecimal_index.trim(), &mut current_scalar_bytes).is_err() {
            return start_hexadecimal_index.to_string();
        }

        let private_key_instance = SafePrivateKey::from_bytes(&current_scalar_bytes)
            .expect("MATH_FAULT: Invalid start scalar.");
        let public_key_affine = SafePublicKey::from_private(&private_key_instance);
        let public_key_raw_bytes = public_key_affine.to_bytes(false);

        // Ascensión al espacio proyectivo Jacobiano (Z=1)
        let mut current_jacobian_point = JacobianPoint::from_affine(
            bytes_to_words_u256(&public_key_raw_bytes[1..33].try_into().unwrap()),
            bytes_to_words_u256(&public_key_raw_bytes[33..65].try_into().unwrap())
        );

        let generator_affine_x = FieldElement { internal_words: Self::GENERATOR_AFFINE_X_WORDS };
        let generator_affine_y = FieldElement { internal_words: Self::GENERATOR_AFFINE_Y_WORDS };

        let mut processed_cycle_count: u64 = 0;

        // 2. MAGAZINE: Buffers para procesamiento por ráfaga
        let mut points_magazine: Vec<JacobianPoint> = Vec::with_capacity(BATCH_MAGAZINE_SIZE);
        let mut scalars_magazine: Vec<[u8; 32]> = Vec::with_capacity(BATCH_MAGAZINE_SIZE);

        // 3. BUCLE CALIENTE (HOT PATH)
        while processed_cycle_count < iteration_burst_limit {
            if termination_signal.load(Ordering::Relaxed) { break; }

            // A. CARGA DEL MAGAZINE
            points_magazine.push(current_jacobian_point);
            scalars_magazine.push(current_scalar_bytes);

            // B. PROCESAMIENTO DE RÁFAGA (Montgomery Inversion)
            if points_magazine.len() == BATCH_MAGAZINE_SIZE {
                Self::process_magazine_burst(
                    &points_magazine,
                    &scalars_magazine,
                    target_census_filter,
                    finding_delegate
                );

                // Actualización de telemetría de wallets consultadas
                effort_telemetry.fetch_add(BATCH_MAGAZINE_SIZE as u64, Ordering::Relaxed);

                points_magazine.clear();
                scalars_magazine.clear();
            }

            // C. INCREMENTO JACOBIANO MIXTO (P = P + G)
            // Operación O(1) libre de divisiones modulares.
            current_jacobian_point = UnifiedCurveEngine::add_mixed_deterministic(
                &current_jacobian_point,
                &generator_affine_x,
                &generator_affine_y
            );

            // D. INCREMENTO DEL ESCALAR PRIVADO (Aritmética de Bits L1)
            let _ = add_u64_to_u256_be(&mut current_scalar_bytes, 1);
            processed_cycle_count += 1;
        }

        // 4. LIMPIEZA DE REMANENTE (Flush del magazine incompleto)
        if !points_magazine.is_empty() {
            let remnant_count = points_magazine.len() as u64;
            Self::process_magazine_burst(&points_magazine, &scalars_magazine, target_census_filter, finding_delegate);
            effort_telemetry.fetch_add(remnant_count, Ordering::Relaxed);
        }

        hex::encode(current_scalar_bytes)
    }

    /**
     * Procesa un lote de puntos utilizando Montgomery Batch Inversion.
     * Transforma coordenadas Jacobianas a Afines y verifica contra el censo.
     */
    fn process_magazine_burst<H: FindingHandler>(
        points_collection: &[JacobianPoint],
        scalars_collection: &[[u8; 32]],
        target_filter: &ShardedFilter,
        collision_handler: &H
    ) {
        // 1. Extracción de coordenadas Z (Denominadores pendientes)
        let denominators_z: Vec<FieldElement> = points_collection.iter().map(|point| point.z).collect();

        // 2. INVERSIÓN POR LOTE SOBERANA (Montgomery's Trick)
        let inverses_z = FieldElement::batch_invert_sovereign(&denominadores_z);

        // 3. PROYECCIÓN Y VERIFICACIÓN
        for index in 0..points_collection.len() {
            let reciprocal_z = &inverses_z[index];
            let reciprocal_z_squared = reciprocal_z.square_modular();

            // x = X * (Z^-2) mod p
            let affine_x = points_collection[index].x.multiply_modular(&reciprocal_z_squared);
            let affine_x_bytes = words_to_bytes_u256(&affine_x.internal_words);

            // Derivación de dirección comprimida directamente desde la coordenada X
            let candidate_bitcoin_address = prospector_core_gen::address_legacy::pubkey_from_x_to_address(
                &affine_x_bytes,
                true
            );

            // Auditoría de coincidencia en el filtro de Bloom
            if target_filter.contains(&candidate_bitcoin_address) {
                let recovered_private_key = SafePrivateKey::from_bytes(&scalars_collection[index])
                    .expect("INTEGRITY_FAULT: Magazine scalar corrupted.");

                collision_handler.on_finding(
                    candidate_bitcoin_address,
                    recovered_private_key,
                    "sequential_batch_burst_v150".to_string()
                );
            }
        }
    }
}
