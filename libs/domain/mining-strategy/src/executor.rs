// =================================================================
// APARATO: STRATEGY EXECUTOR (ADAPTADO A STRINGS)
// RESPONSABILIDAD: EJECUCIÓN PARALELA Y CONVERSIÓN DE TIPOS
// =================================================================

use std::str::FromStr;
use rayon::prelude::*;

use prospector_domain_models::{WorkOrder, SearchStrategy, ForensicTarget};
use prospector_core_probabilistic::filter_wrapper::RichListFilter;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_math::private_key::SafePrivateKey;
use prospector_core_gen::address_legacy::pubkey_to_address;

use crate::combinatoric::CombinatoricIterator;
use crate::dictionary::DictionaryIterator;
use prospector_domain_forensics::DebianIterator;

pub trait FindingHandler: Sync + Send {
    fn on_finding(&self, address: String, pk: SafePrivateKey, source: String);
}

pub struct StrategyExecutor;

impl StrategyExecutor {
    /// Ejecuta la estrategia definida en el WorkOrder.
    /// Realiza parsing seguro de los rangos (String -> u64).
    pub fn execute<H: FindingHandler>(
        job: &WorkOrder,
        filter: &RichListFilter,
        context_data: &ExecutorContext,
        handler: &H
    ) {
        match &job.strategy {
            // ESTRATEGIA: COMBINATORIA
            // El rango viene como String desde la DB. Lo parseamos a u64 para el iterador.
            // En el futuro, CombinatoricIterator debería aceptar BigInt.
            SearchStrategy::Combinatoric { prefix, suffix, start_index, end_index } => {
                let start = u64::from_str(start_index).unwrap_or(0);
                let end = u64::from_str(end_index).unwrap_or(0);

                let iter = CombinatoricIterator::new(start, end, prefix.clone(), suffix.clone());

                // Paralelismo SIMD/Rayon
                iter.par_bridge().for_each(|(phrase, pk)| {
                    Self::check_candidate(filter, pk, format!("comb:{}", phrase), handler);
                });
            },

            // ESTRATEGIA: DICCIONARIO
            SearchStrategy::Dictionary { dataset_url: _, limit } => {
                if let Some(words) = &context_data.dictionary_cache {
                    let iter = DictionaryIterator::new(words, *limit);
                    iter.par_bridge().for_each(|(phrase, pk)| {
                        Self::check_candidate(filter, pk, format!("dict:{}", phrase), handler);
                    });
                }
            },

            // ESTRATEGIA: FORENSE
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
                        // Implementación pendiente para Android PRNG
                    }
                }
            },

            SearchStrategy::Random { .. } => {
                // Implementación pendiente
            }
        }
    }

    /// HOT LOOP: Chequeo de colisión.
    #[inline(always)]
    fn check_candidate<H: FindingHandler>(
        filter: &RichListFilter,
        pk: SafePrivateKey,
        source: String,
        handler: &H
    ) {
        let pub_key = SafePublicKey::from_private(&pk);
        // Usamos formato no comprimido (Legacy) porque ahí están las monedas viejas perdidas.
        let addr = pubkey_to_address(&pub_key, false);

        if filter.contains(&addr) {
            handler.on_finding(addr, pk, source);
        }
    }
}

#[derive(Default)]
pub struct ExecutorContext {
    pub dictionary_cache: Option<Vec<String>>,
}
