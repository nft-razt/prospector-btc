// libs/domain/mining-strategy/src/executor.rs
// =================================================================
// APARATO: STRATEGY EXECUTOR (SHARDING COMPLIANT)
// RESPONSABILIDAD: ORQUESTACIÓN DE VECTORES DE ATAQUE
// MEJORA: SOPORTE NATIVO PARA FILTROS PARTICIONADOS
// =================================================================

use num_bigint::BigUint;
use num_traits::Zero;
use rayon::prelude::*;
use std::str::FromStr;

use prospector_domain_models::{ForensicTarget, SearchStrategy, WorkOrder};
// ✅ CAMBIO CRÍTICO: Importamos ShardedFilter
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_math::private_key::SafePrivateKey;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_probabilistic::sharded::ShardedFilter;

use crate::combinatoric::CombinatoricIterator;
use crate::dictionary::DictionaryIterator;
use prospector_domain_forensics::{AndroidLcgIterator, DebianIterator};

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
    /// Acepta `ShardedFilter` para consultas O(1) distribuidas.
    pub fn execute<H: FindingHandler>(
        job: &WorkOrder,
        filter: &ShardedFilter, // ✅ Tipo actualizado
        context_data: &ExecutorContext,
        handler: &H,
    ) {
        match &job.strategy {
            // --- ESTRATEGIA 1: COMBINATORIA (BIGINT SUPPORT) ---
            SearchStrategy::Combinatoric {
                prefix,
                suffix,
                start_index,
                end_index,
            } => {
                let start = BigUint::from_str(start_index).unwrap_or_else(|_| BigUint::zero());
                let end = BigUint::from_str(end_index).unwrap_or_else(|_| BigUint::zero());

                let iter = CombinatoricIterator::new(start, end, prefix.clone(), suffix.clone());
                iter.par_bridge().for_each(|(phrase, pk)| {
                    Self::check_candidate(filter, pk, format!("comb:{}", phrase), handler);
                });
            }

            // --- ESTRATEGIA 2: DICCIONARIO (BRAINWALLETS) ---
            SearchStrategy::Dictionary {
                dataset_url: _,
                limit,
            } => {
                if let Some(words) = &context_data.dictionary_cache {
                    let iter = DictionaryIterator::new(words, *limit);
                    iter.par_bridge().for_each(|(phrase, pk)| {
                        Self::check_candidate(filter, pk, format!("dict:{}", phrase), handler);
                    });
                }
            }

            // --- ESTRATEGIA 3: FORENSE (ARQUEOLOGÍA DIGITAL) ---
            SearchStrategy::ForensicScan {
                target,
                range_start,
                range_end,
            } => {
                let start = u64::from_str(range_start).unwrap_or(0);
                let end = u64::from_str(range_end).unwrap_or(0);

                match target {
                    ForensicTarget::DebianOpenSSL => {
                        let iter = DebianIterator::new(start, end);
                        iter.par_bridge().for_each(|(source, pk)| {
                            Self::check_candidate(filter, pk, source, handler);
                        });
                    }
                    ForensicTarget::AndroidSecureRandom => {
                        // Simula el generador LCG débil de Java (CVE-2013-7372)
                        let iter = AndroidLcgIterator::new(start, end);
                        iter.par_bridge().for_each(|(source, pk)| {
                            Self::check_candidate(filter, pk, source, handler);
                        });
                    }
                }
            }

            // --- ESTRATEGIA 4: ALEATORIA (MONTE CARLO) ---
            SearchStrategy::Random { .. } => {
                // Placeholder para futuro fuzzing aleatorio puro
            }
        }
    }

    /// Ciclo caliente (Hot Loop) de verificación.
    #[inline(always)]
    fn check_candidate<H: FindingHandler>(
        filter: &ShardedFilter, // ✅ Tipo actualizado
        pk: SafePrivateKey,
        source: String,
        handler: &H,
    ) {
        // 1. Derivación PubKey (Hot Path optimizado con Global Context)
        let pub_key = SafePublicKey::from_private(&pk);

        // 2. Generación Dirección Legacy
        let addr = pubkey_to_address(&pub_key, false);

        // 3. Consulta al Filtro Particionado
        // ShardedFilter se encarga internamente de hashear la dirección
        // y consultar solo el shard relevante.
        if filter.contains(&addr) {
            handler.on_finding(addr, pk, source);
        }
    }
}
