// libs/domain/mining-strategy/src/engines/dictionary_engine.rs
/**
 * =================================================================
 * APARATO: ENTROPY DICTIONARY ENGINE (V27.0 - DOCUMENTED)
 * CLASIFICACIÓN: DOMAIN STRATEGY (L2)
 * RESPONSABILIDAD: AUDITORÍA DE FRASES SEMILLA (BRAINWALLETS)
 * ESTADO: PRODUCTION READY
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use crate::executor::FindingHandler;

/// Motor especializado en la transformación de frases humanas a claves privadas.
/// Utiliza SHA-256 para derivar la entropía de cadenas de texto.
pub struct EntropyDictionaryEngine;

impl EntropyDictionaryEngine {
    /// Ejecuta una auditoría sobre una lista de frases candidatas.
    ///
    /// # Argumentos
    /// * `dictionary_dataset` - Vector de palabras a procesar.
    /// * `target_filter` - Estructura de Bloom para verificación O(1).
    /// * `stop_signal` - Receptor de interrupción externa.
    /// * `effort_counter` - Telemetría de hashrate global.
    /// * `collision_handler` - Delegado para reportar hallazgos.
    pub fn execute_dictionary_audit<H: FindingHandler>(
        dictionary_dataset: &[String],
        target_filter: &ShardedFilter,
        stop_signal: &AtomicBool,
        effort_counter: Arc<AtomicU64>,
        collision_handler: &H,
    ) -> String {
        let mut last_audited_index: usize = 0;

        for (current_index, secret_candidate) in dictionary_dataset.iter().enumerate() {
            if stop_signal.load(Ordering::Relaxed) { break; }

            // 1. DERIVACIÓN CRIPTOGRÁFICA (SHA256 Nativo L1)
            let private_key = crate::brainwallet::phrase_to_private_key(secret_candidate);
            let public_key = SafePublicKey::from_private(&private_key);
            let bitcoin_address = prospector_core_gen::address_legacy::pubkey_to_address(&public_key, false);

            // 2. VERIFICACIÓN CONTRA EL CENSO UTXO
            if target_filter.contains(&bitcoin_address) {
                collision_handler.on_finding(
                    bitcoin_address,
                    private_key,
                    format!("brainwallet_vector:{}", secret_candidate)
                );
            }

            // 3. ACTUALIZACIÓN DE HUELLA Y TELEMETRÍA
            last_audited_index = current_index;

            // Batching atómico cada 100 elementos para reducir congestión de CPU
            if current_index % 100 == 0 {
                effort_counter.fetch_add(100, Ordering::Relaxed);
            }
        }

        // Retorno de la huella forense: índice procesado
        format!("dictionary_checkpoint_index_{}", last_audited_index)
    }
}
