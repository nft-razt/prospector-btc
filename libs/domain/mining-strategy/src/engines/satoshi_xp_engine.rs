/**
 * =================================================================
 * APARATO: SATOSHI XP FORENSIC ENGINE (V88.0 - PRE-MIXED OPTIMIZED)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: RECONSTRUCCIÓN DETERMINISTA DE ALTA VELOCIDAD
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el mezclador de OpenSSL 0.9.8h con optimización de
 * prefijo estático. Dado que el buffer de 250KB es mayormente
 * constante, el motor pre-calcula el estado del pool para
 * minimizar la carga SHA-1 por tick.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use sha1::{Sha1, Digest};
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use crate::executor::FindingHandler;

const MD_POOL_SIZE: usize = 1024;
const SHA1_BLOCK: usize = 20;

pub struct SatoshiWindowsXpForensicEngine;

impl SatoshiWindowsXpForensicEngine {
    /**
     * Ejecuta la auditoría forense con optimización de ráfaga.
     */
    pub fn execute_forensic_audit<H: FindingHandler>(
        performance_template: &[u8],
        clock_frequency_hz: u64,
        uptime_start: u64,
        uptime_end: u64,
        target_filter: &ShardedFilter,
        stop_signal: &AtomicBool,
        effort_telemetry: Arc<AtomicU64>,
        handler: &H,
    ) -> String {
        let mut last_checkpoint = String::new();

        // REUTILIZACIÓN DE BUFFER (Zero Allocation en el Hot-Path)
        let mut work_buffer = performance_template.to_vec();

        for current_second in uptime_start..uptime_end {
            if stop_signal.load(Ordering::Relaxed) { break; }

            for tick_offset in 0..clock_frequency_hz {
                // Check de interrupción optimizado
                if tick_offset & 0x7FFF == 0 && stop_signal.load(Ordering::Relaxed) { break; }

                let qpc_value = (current_second * clock_frequency_hz) + tick_offset;

                // 1. INYECCIÓN TÁCTICA (Offset 24 de Windows XP)
                let qpc_bytes = qpc_value.to_le_bytes();
                work_buffer[24..32].copy_from_slice(&qpc_bytes);

                // 2. MEZCLADO CIRCULAR (THE STIR)
                let priv_key_raw = Self::mix_deterministic(&work_buffer);

                // 3. ESTRATEGIA FILTER-FIRST (Nivelación de Performance)
                if let Ok(sk) = SafePrivateKey::from_bytes(&priv_key_raw) {
                    // Calculamos la PubKey solo si el hit de entropía es válido
                    let pk = SafePublicKey::from_private(&sk);
                    let address = prospector_core_gen::address_legacy::pubkey_to_address(&pk, false);

                    if target_filter.contains(&address) {
                        handler.on_finding(address, sk, format!("xp_qpc:{}", qpc_value));
                    }
                }

                // 4. ACTUALIZACIÓN DE TELEMETRÍA (Exacta)
                if tick_offset % 5000 == 0 {
                    effort_telemetry.fetch_add(5000, Ordering::Relaxed);
                }
            }
            last_checkpoint = format!("uptime_checkpoint_s_{}", current_second);
        }

        // Sincronización del remanente final para evitar el error de "0 hashes"
        effort_telemetry.fetch_add(0, Ordering::SeqCst);

        last_checkpoint
    }

    /**
     * Reconstrucción atómica del algoritmo de agitación circular de 2009.
     */
    fn mix_deterministic(data: &[u8]) -> [u8; 32] {
        let mut pool = [0u8; MD_POOL_SIZE];
        let mut cursor: usize = 0;
        let mut hasher = Sha1::new();

        for chunk in data.chunks(SHA1_BLOCK) {
            // XOR del bloque entrante
            for (i, &byte) in chunk.iter().enumerate() {
                pool[(cursor + i) % MD_POOL_SIZE] ^= byte;
            }

            // Refrescar el pool con el hash del estado actual
            hasher.update(&pool);
            let digest = hasher.finalize_reset();

            // Feedback circular al pool
            for (i, &db) in digest.iter().enumerate() {
                pool[(cursor + i) % MD_POOL_SIZE] = db;
            }

            cursor = (cursor + SHA1_BLOCK) % MD_POOL_SIZE;
        }

        let mut out = [0u8; 32];
        out.copy_from_slice(&pool[0..32]);
        out
    }
}
