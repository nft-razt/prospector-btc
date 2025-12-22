/**
 * =================================================================
 * APARATO: SATOSHI XP FORENSIC ENGINE (V91.5 - HYPER-OPTIMIZED)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: RECONSTRUCCIÓN DETERMINISTA DE ALTA VELOCIDAD
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la simulación de baja entropía de 2009. Esta versión
 * utiliza inyección in-place basada en offsets técnicos constantes,
 * garantizando cero advertencias de compilación.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use sha1::{Sha1, Digest};
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use crate::executor::FindingHandler;

const MESSAGE_DIGEST_POOL_CAPACITY_BYTES: usize = 1024;
const SHA1_OUTPUT_LENGTH_BYTES: usize = 20;

/// Desplazamiento técnico del contador de rendimiento en el buffer de Windows XP.
const QUERY_PERFORMANCE_COUNTER_OFFSET: usize = 24;

/// Motor de arqueología digital para la simulación de estados de sistema Windows XP.
pub struct SatoshiWindowsXpForensicEngine;

impl SatoshiWindowsXpForensicEngine {
    /**
     * Ejecuta una auditoría forense utilizando optimización de prefijo estático.
     */
    pub fn execute_forensic_audit<H: FindingHandler>(
        performance_snapshot_template: &[u8],
        clock_frequency_hz: u64,
        uptime_start_seconds: u64,
        uptime_end_seconds: u64,
        target_census_filter: &ShardedFilter,
        termination_signal: &AtomicBool,
        effort_telemetry_accumulator: Arc<AtomicU64>,
        collision_handler: &H,
    ) -> String {
        let mut last_processed_checkpoint_identifier = String::new();

        let (base_pool, base_cursor) = Self::precompute_static_entropy_prefix(
            &performance_snapshot_template[0..SHA1_OUTPUT_LENGTH_BYTES]
        );

        let dynamic_buffer_segment = &performance_snapshot_template[SHA1_OUTPUT_LENGTH_BYTES..];

        for current_uptime_second in uptime_start_seconds..uptime_end_seconds {
            if termination_signal.load(Ordering::Relaxed) { break; }

            for tick_offset in 0..clock_frequency_hz {
                if tick_offset % 100_000 == 0 && termination_signal.load(Ordering::Relaxed) { break; }

                let query_performance_counter_value = (current_uptime_second * clock_frequency_hz) + tick_offset;

                let private_key_material = Self::mix_dynamic_segment_with_qpc(
                    base_pool,
                    base_cursor,
                    dynamic_buffer_segment,
                    query_performance_counter_value
                );

                if let Ok(safe_private_key) = SafePrivateKey::from_bytes(&private_key_material) {
                    let public_key_instance = SafePublicKey::from_private(&safe_private_key);
                    let derived_bitcoin_address = prospector_core_gen::address_legacy::pubkey_to_address(&public_key_instance, false);

                    if target_census_filter.contains(&derived_bitcoin_address) {
                        collision_handler.on_finding(
                            derived_bitcoin_address,
                            safe_private_key,
                            format!("forensic_xp_qpc:{}", query_performance_counter_value)
                        );
                    }
                }

                if tick_offset % 10_000 == 0 {
                    effort_telemetry_accumulator.fetch_add(10_000, Ordering::Relaxed);
                }
            }
            last_processed_checkpoint_identifier = format!("uptime_checkpoint_s_{}", current_uptime_second);
        }

        last_processed_checkpoint_identifier
    }

    fn precompute_static_entropy_prefix(prefix_data: &[u8]) -> ([u8; 1024], usize) {
        let mut pool = [0u8; 1024];
        let mut hasher = Sha1::new();
        for (i, &byte) in prefix_data.iter().enumerate() { pool[i] = byte; }
        hasher.update(&pool);
        let digest = hasher.finalize();
        pool[0..SHA1_OUTPUT_LENGTH_BYTES].copy_from_slice(&digest);
        (pool, SHA1_OUTPUT_LENGTH_BYTES)
    }

    fn mix_dynamic_segment_with_qpc(
        mut pool: [u8; 1024],
        mut cursor: usize,
        dynamic_data: &[u8],
        qpc_value: u64
    ) -> [u8; 32] {
        let mut sha1_engine = Sha1::new();
        let qpc_bytes = qpc_value.to_le_bytes();
        let mut local_dynamic_buffer = dynamic_data.to_vec();

        // ✅ RESOLUCIÓN WARNING: Uso dinámico de la constante para inyección in-place
        let relative_qpc_offset = QUERY_PERFORMANCE_COUNTER_OFFSET - SHA1_OUTPUT_LENGTH_BYTES;
        local_dynamic_buffer[relative_qpc_offset..relative_qpc_offset + 8].copy_from_slice(&qpc_bytes);

        for data_chunk in local_dynamic_buffer.chunks(SHA1_OUTPUT_LENGTH_BYTES) {
            for (i, byte) in data_chunk.iter().enumerate() {
                pool[(cursor + i) % MESSAGE_DIGEST_POOL_CAPACITY_BYTES] ^= *byte;
            }
            sha1_engine.update(&pool);
            let digest_result = sha1_engine.finalize_reset();
            for (i, &digest_byte) in digest_result.iter().enumerate() {
                let wrapped_index = (cursor + i) % MESSAGE_DIGEST_POOL_CAPACITY_BYTES;
                pool[wrapped_index] = digest_byte;
            }
            cursor = (cursor + SHA1_OUTPUT_LENGTH_BYTES) % MESSAGE_DIGEST_POOL_CAPACITY_BYTES;
        }

        let mut derived_key = [0u8; 32];
        derived_key.copy_from_slice(&pool[0..32]);
        derived_key
    }
}
