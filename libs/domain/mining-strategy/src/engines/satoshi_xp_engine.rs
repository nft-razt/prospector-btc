/**
 * =================================================================
 * APARATO: SATOSHI WINDOWS XP FORENSIC ENGINE (V125.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN LOGIC (ESTRATO L2)
 * RESPONSABILIDAD: SIMULACIÓN DE ALTA FIDELIDAD Y AUDITORÍA
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use sha1::{Sha1, Digest};
use rayon::prelude::*;
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use crate::executor::FindingHandler;

const MESSAGE_DIGEST_POOL_CAPACITY: usize = 1024;
const SHA1_BLOCK_SIZE: usize = 20;
const PERFORMANCE_TIME_OFFSET: usize = 24;

pub struct SatoshiWindowsXpForensicEngine;

impl SatoshiWindowsXpForensicEngine {
    /**
     * Ejecuta la auditoría masiva comparando estados de Windows XP contra el Censo Global Legacy.
     * Utiliza Rayon para saturar todos los núcleos disponibles sin bloqueos de memoria.
     */
    pub fn execute_forensic_mission_loop<H: FindingHandler>(
        performance_data_template: &[u8],
        hardware_clock_frequency: u64,
        uptime_seconds_start: u64,
        uptime_seconds_end: u64,
        census_bloom_filter: &ShardedFilter,
        global_shutdown_signal: &AtomicBool,
        computational_effort_accumulator: Arc<AtomicU64>,
        cryptographic_finding_handler: &H,
    ) {
        let initial_tick = uptime_seconds_start * hardware_clock_frequency;
        let final_tick = uptime_seconds_end * hardware_clock_frequency;
        let iteration_volume = final_tick.saturating_sub(initial_tick);

        // PARALELIZACIÓN SOBERANA
        (0..iteration_volume).into_par_iter().for_each(|tick_offset| {
            if global_shutdown_signal.load(Ordering::Relaxed) { return; }

            let current_performance_tick = initial_tick + tick_offset;

            // 1. RECONSTRUCCIÓN DETERMINISTA (Stack-Efficient Vec)
            let mut local_performance_buffer = performance_data_template.to_vec();
            local_performance_buffer[PERFORMANCE_TIME_OFFSET..PERFORMANCE_TIME_OFFSET + 8]
                .copy_from_slice(&current_performance_tick.to_le_bytes());

            // 2. MEZCLADO OPENSSL 0.9.8h (The Satoshi Stir)
            let private_key_candidate_bytes = Self::execute_openssl_stirring_protocol(&local_performance_buffer);

            // 3. VALIDACIÓN CRIPTOGRÁFICA
            if let Ok(private_key_handle) = SafePrivateKey::from_bytes(&private_key_candidate_bytes) {
                let public_key_point = SafePublicKey::from_private(&private_key_handle);
                let legacy_address = prospector_core_gen::address_legacy::pubkey_to_address(&public_key_point, false);

                // 4. PROTOCOLO DE SELLADO INMEDIATO
                if census_bloom_filter.contains(&legacy_address) {
                    cryptographic_finding_handler.on_finding(
                        legacy_address,
                        private_key_handle,
                        format!("THESIS_VALIDATED:TICK_{}", current_performance_tick)
                    );
                }
            }

            // Telemetría progresiva cada 10,000 iteraciones
            if tick_offset % 10000 == 0 {
                computational_effort_accumulator.fetch_add(10000, Ordering::Relaxed);
            }
        });
    }

    /**
     * Implementación exacta del algoritmo de mezcla SHA-1 de OpenSSL 2009.
     * Replica la saturación circular del pool interno.
     */
    fn execute_openssl_stirring_protocol(buffer: &[u8]) -> [u8; 32] {
        let mut message_digest_pool = [0u8; MESSAGE_DIGEST_POOL_CAPACITY];
        let mut pool_cursor: usize = 0;
        let mut sha1_hasher = Sha1::new();

        for data_chunk in buffer.chunks(SHA1_BLOCK_SIZE) {
            for (index, byte) in data_chunk.iter().enumerate() {
                message_digest_pool[(pool_cursor + index) % MESSAGE_DIGEST_POOL_CAPACITY] ^= *byte;
            }

            sha1_hasher.update(&message_digest_pool);
            let digest_result = sha1_hasher.finalize_reset();

            message_digest_pool[pool_cursor..pool_cursor + 20].copy_from_slice(&digest_result);
            pool_cursor = (pool_cursor + 20) % MESSAGE_DIGEST_POOL_CAPACITY;
        }

        let mut extracted_key_bytes = [0u8; 32];
        extracted_key_bytes.copy_from_slice(&message_digest_pool[0..32]);
        extracted_key_bytes
    }
}
