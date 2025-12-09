// libs/domain/mining-strategy/src/executor.rs
// =================================================================
// APARATO: STRATEGY EXECUTOR (BIGINT ADAPTER)
// RESPONSABILIDAD: ORQUESTACIÓN DE VECTORES DE ATAQUE
// ESTADO: OPTIMIZADO (U256 COMPATIBLE)
// =================================================================

use std::str::FromStr;
use rayon::prelude::*;
use num_bigint::BigUint;
use num_traits::Zero;

use prospector_domain_models::{WorkOrder, SearchStrategy, ForensicTarget};
use prospector_core_probabilistic::filter_wrapper::RichListFilter;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_math::private_key::SafePrivateKey;
use prospector_core_gen::address_legacy::pubkey_to_address;

use crate::combinatoric::CombinatoricIterator;
use crate::dictionary::DictionaryIterator;
use prospector_domain_forensics::DebianIterator;

/// Interfaz para reportar colisiones encontradas hacia el worker.
pub trait FindingHandler: Sync + Send {
    fn on_finding(&self, address: String, pk: SafePrivateKey, source: String);
}

/// Contexto de ejecución para datos compartidos (Read-Only).
#[derive(Default)]
pub struct ExecutorContext {
    pub dictionary_cache: Option<Vec<String>>,
}

/// Motor de ejecución estática.
pub struct StrategyExecutor;

impl StrategyExecutor {
    /// Ejecuta la estrategia definida en el WorkOrder.
    /// Utiliza paralelismo de datos (Rayon) para saturar los núcleos de la CPU.
    pub fn execute<H: FindingHandler>(
        job: &WorkOrder,
        filter: &RichListFilter,
        context_data: &ExecutorContext,
        handler: &H
    ) {
        match &job.strategy {
            // --- ESTRATEGIA 1: COMBINATORIA (BIGINT SUPPORT) ---
            SearchStrategy::Combinatoric { prefix, suffix, start_index, end_index } => {
                // Parsing robusto de Strings a BigUint
                // Si el parsing falla, asumimos 0 (Génesis) para no crashear el worker
                let start = BigUint::from_str(start_index).unwrap_or_else(|_| BigUint::zero());
                let end = BigUint::from_str(end_index).unwrap_or_else(|_| BigUint::zero());

                let iter = CombinatoricIterator::new(start, end, prefix.clone(), suffix.clone());

                // Bridge Paralelo: Rayon convierte el iterador secuencial en streams paralelos
                iter.par_bridge().for_each(|(phrase, pk)| {
                    Self::check_candidate(filter, pk, format!("comb:{}", phrase), handler);
                });
            },

            // --- ESTRATEGIA 2: DICCIONARIO (BRAINWALLETS) ---
            SearchStrategy::Dictionary { dataset_url: _, limit } => {
                if let Some(words) = &context_data.dictionary_cache {
                    let iter = DictionaryIterator::new(words, *limit);

                    iter.par_bridge().for_each(|(phrase, pk)| {
                        Self::check_candidate(filter, pk, format!("dict:{}", phrase), handler);
                    });
                }
            },

            // --- ESTRATEGIA 3: FORENSE (ARQUEOLOGÍA DIGITAL) ---
            // Nota: El iterador de Debian usa u64 (PIDs), así que parseamos a u64 aquí.
            SearchStrategy::ForensicScan { target, range_start, range_end } => {
                let start = u64::from_str(range_start).unwrap_or(0);
                let end = u64::from_str(range_end).unwrap_or(0);

                match target {
                    ForensicTarget::DebianOpenSSL => {
                        let iter = DebianIterator::new(start, end);

                        iter.par_bridge().for_each(|(source, pk)| {
                            Self::check_candidate(filter, pk, source, handler);
                        });
                    },
                    ForensicTarget::AndroidSecureRandom => {
                        // TODO: Implementar Android PRNG Bug
                    }
                }
            },

            // --- ESTRATEGIA 4: ALEATORIA (MONTE CARLO) ---
            SearchStrategy::Random { .. } => {
                // Implementación pendiente
            }
        }
    }

    /// Ciclo caliente (Hot Loop) de verificación.
    /// Debe ser lo más eficiente posible (Inline Always).
    #[inline(always)]
    fn check_candidate<H: FindingHandler>(
        filter: &RichListFilter,
        pk: SafePrivateKey,
        source: String,
        handler: &H
    ) {
        let pub_key = SafePublicKey::from_private(&pk);

        // Usamos formato NO comprimido (false) para maximizar compatibilidad con 2009-2010.
        let addr = pubkey_to_address(&pub_key, false);

        // Verificación O(1) en Filtro de Bloom
        if filter.contains(&addr) {
            handler.on_finding(addr, pk, source);
        }
    }
}
