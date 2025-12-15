// libs/domain/mining-strategy/src/executor.rs
// =================================================================
// APARATO: STRATEGY EXECUTOR (V7.1 - KANGAROO INTEGRATED)
// RESPONSABILIDAD: ORQUESTACIÓN DE TODOS LOS VECTORES DE ATAQUE
// =================================================================

use num_bigint::BigUint;
use num_traits::Zero;
use rayon::prelude::*;
use std::str::FromStr;

use prospector_domain_models::{ForensicTarget, SearchStrategy, WorkOrder};
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_math::private_key::SafePrivateKey;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_probabilistic::sharded::ShardedFilter;

use crate::combinatoric::CombinatoricIterator;
use crate::dictionary::DictionaryIterator;
use crate::kangaroo::KangarooRunner; // ✅ NUEVO IMPORT
use prospector_domain_forensics::{AndroidLcgIterator, DebianIterator};

/// Interfaz para reportar colisiones encontradas hacia el worker.
pub trait FindingHandler: Sync + Send {
    fn on_finding(&self, address: String, pk: SafePrivateKey, source: String);
}

#[derive(Default)]
pub struct ExecutorContext {
    pub dictionary_cache: Option<Vec<String>>,
}

pub struct StrategyExecutor;

impl StrategyExecutor {
    pub fn execute<H: FindingHandler>(
        job: &WorkOrder,
        filter: &ShardedFilter,
        context_data: &ExecutorContext,
        handler: &H,
    ) {
        match &job.strategy {
            // ... (Estrategias previas: Combinatoric, Dictionary, ForensicScan) ...

            SearchStrategy::Combinatoric { prefix, suffix, start_index, end_index } => {
                let start = BigUint::from_str(start_index).unwrap_or_else(|_| BigUint::zero());
                let end = BigUint::from_str(end_index).unwrap_or_else(|_| BigUint::zero());
                let iter = CombinatoricIterator::new(start, end, prefix.clone(), suffix.clone());
                iter.par_bridge().for_each(|(phrase, pk)| {
                    Self::check_candidate(filter, pk, format!("comb:{}", phrase), handler);
                });
            }

            SearchStrategy::Dictionary { dataset_url: _, limit } => {
                if let Some(words) = &context_data.dictionary_cache {
                    let iter = DictionaryIterator::new(words, *limit);
                    iter.par_bridge().for_each(|(phrase, pk)| {
                        Self::check_candidate(filter, pk, format!("dict:{}", phrase), handler);
                    });
                }
            }

            SearchStrategy::ForensicScan { target, range_start, range_end } => {
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
                        let iter = AndroidLcgIterator::new(start, end);
                        iter.par_bridge().for_each(|(source, pk)| {
                            Self::check_candidate(filter, pk, source, handler);
                        });
                    }
                }
            }

            // ✅ NUEVA INTEGRACIÓN: KANGAROO
            // Esta estrategia es especial: NO itera ciegamente contra el filtro Bloom.
            // Tiene su propio objetivo específico (target_pubkey) definido en el trabajo.
            SearchStrategy::Kangaroo { target_pubkey, start_scalar, width } => {
                // KangarooRunner maneja su propia lógica de éxito internamente
                KangarooRunner::run(
                    target_pubkey,
                    start_scalar,
                    *width,
                    handler
                );
            }

            SearchStrategy::Random { .. } => {}
        }
    }

    #[inline(always)]
    fn check_candidate<H: FindingHandler>(
        filter: &ShardedFilter,
        pk: SafePrivateKey,
        source: String,
        handler: &H,
    ) {
        let pub_key = SafePublicKey::from_private(&pk);
        let addr = pubkey_to_address(&pub_key, false);
        if filter.contains(&addr) {
            handler.on_finding(addr, pk, source);
        }
    }
}
// libs/domain/mining-strategy/src/executor.rs
// =================================================================
// APARATO: STRATEGY EXECUTOR (V7.1 - KANGAROO INTEGRATED)
// RESPONSABILIDAD: ORQUESTACIÓN DE VECTORES DE ATAQUE CRIPTOGRÁFICO
// CARACTERÍSTICAS:
// - Parallelism: Rayon Work-Stealing para saturación de CPU.
// - BigInt Support: Aritmética U256 para el espacio completo de claves.
// - Sharding: Ruteo eficiente O(1) contra filtros particionados.
// - Modularidad: Delegación de lógica compleja (Kangaroo) a adaptadores.
// =================================================================

use num_bigint::BigUint;
use num_traits::Zero;
use rayon::prelude::*;
use std::str::FromStr;

// --- DOMINIO & MODELOS ---
use prospector_domain_models::{ForensicTarget, SearchStrategy, WorkOrder};

// --- NÚCLEO MATEMÁTICO (CORE) ---
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_math::private_key::SafePrivateKey;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_probabilistic::sharded::ShardedFilter;

// --- ESTRATEGIAS (MÓDULOS LOCALES) ---
use crate::combinatoric::CombinatoricIterator;
use crate::dictionary::DictionaryIterator;
use crate::kangaroo::KangarooRunner; // ✅ ADAPTADOR CANGURO
use prospector_domain_forensics::{AndroidLcgIterator, DebianIterator};

/// Interfaz para el reporte de hallazgos (Finding).
/// Permite desacoplar la lógica de búsqueda del mecanismo de transporte (HTTP/Console).
pub trait FindingHandler: Sync + Send {
    /// Se invoca cuando una clave privada genera una dirección presente en el Filtro.
    fn on_finding(&self, address: String, pk: SafePrivateKey, source: String);
}

/// Contexto de ejecución de solo lectura compartido entre hilos.
/// Optimiza el uso de memoria evitando clonaciones de datasets grandes (Diccionarios).
#[derive(Default)]
pub struct ExecutorContext {
    /// Caché de palabras en RAM para evitar I/O repetitivo en ataques de diccionario.
    pub dictionary_cache: Option<Vec<String>>,
}

/// Motor de ejecución estática.
/// Actúa como un Router de Estrategias.
pub struct StrategyExecutor;

impl StrategyExecutor {
    /// Ejecuta la orden de trabajo asignada utilizando todos los recursos disponibles.
    ///
    /// # Argumentos
    /// * `job`: La definición del trabajo (Rango, Estrategia, Parámetros).
    /// * `filter`: El filtro probabilístico particionado (La "Base de Datos" en RAM).
    /// * `context_data`: Recursos compartidos (Diccionarios cacheados).
    /// * `handler`: Callback para reportar éxitos.
    pub fn execute<H: FindingHandler>(
        job: &WorkOrder,
        filter: &ShardedFilter,
        context_data: &ExecutorContext,
        handler: &H,
    ) {
        match &job.strategy {
            // =================================================================
            // ESTRATEGIA 1: COMBINATORIA (FUERZA BRUTA INTELIGENTE)
            // Recorre rangos numéricos secuenciales (U256) con prefijos/sufijos.
            // =================================================================
            SearchStrategy::Combinatoric {
                prefix,
                suffix,
                start_index,
                end_index,
            } => {
                // Parseo robusto de BigInts desde Strings para evitar desbordamiento de u64
                let start = BigUint::from_str(start_index).unwrap_or_else(|_| BigUint::zero());
                let end = BigUint::from_str(end_index).unwrap_or_else(|_| BigUint::zero());

                let iter = CombinatoricIterator::new(start, end, prefix.clone(), suffix.clone());

                // Paralelismo Automático: Rayon convierte el iterador secuencial en paralelo.
                iter.par_bridge().for_each(|(phrase, pk)| {
                    Self::check_candidate(filter, pk, format!("comb:{}", phrase), handler);
                });
            }

            // =================================================================
            // ESTRATEGIA 2: DICCIONARIO (BRAINWALLETS)
            // Prueba frases humanas comunes desde un dataset en memoria.
            // =================================================================
            SearchStrategy::Dictionary {
                dataset_url: _,
                limit,
            } => {
                if let Some(words) = &context_data.dictionary_cache {
                    let iter = DictionaryIterator::new(words, *limit);

                    iter.par_bridge().for_each(|(phrase, pk)| {
                        Self::check_candidate(filter, pk, format!("dict:{}", phrase), handler);
                    });
                } else {
                    // TODO: Emitir warning si el diccionario no está cargado
                }
            }

            // =================================================================
            // ESTRATEGIA 3: FORENSE (ARQUEOLOGÍA DE BUGS)
            // Reproduce generadores de números aleatorios defectuosos históricos.
            // =================================================================
            SearchStrategy::ForensicScan {
                target,
                range_start,
                range_end,
            } => {
                // Los rangos forenses (PIDs, Time seeds) suelen caber en u64
                let start = u64::from_str(range_start).unwrap_or(0);
                let end = u64::from_str(range_end).unwrap_or(0);

                match target {
                    ForensicTarget::DebianOpenSSL => {
                        // CVE-2008-0166: Iteración sobre Process IDs (PIDs)
                        let iter = DebianIterator::new(start, end);
                        iter.par_bridge().for_each(|(source, pk)| {
                            Self::check_candidate(filter, pk, source, handler);
                        });
                    }
                    ForensicTarget::AndroidSecureRandom => {
                        // CVE-2013-7372: Iteración sobre Time Seeds débiles en Java SecureRandom
                        let iter = AndroidLcgIterator::new(start, end);
                        iter.par_bridge().for_each(|(source, pk)| {
                            Self::check_candidate(filter, pk, source, handler);
                        });
                    }
                }
            }

            // =================================================================
            // ESTRATEGIA 4: CANGURO (POLLARD'S LAMBDA) ✅ INTEGRADO
            // Resolución de Logaritmo Discreto en rangos acotados O(sqrt(N)).
            // =================================================================
            SearchStrategy::Kangaroo {
                target_pubkey,
                start_scalar,
                width,
            } => {
                // Delegamos al Adaptador especializado.
                // KangarooRunner maneja su propia lógica de éxito interna y llama al handler si resuelve.
                KangarooRunner::run(
                    target_pubkey,
                    start_scalar,
                    *width,
                    handler
                );
            }

            // =================================================================
            // ESTRATEGIA 5: ALEATORIA (MONTE CARLO)
            // =================================================================
            SearchStrategy::Random { .. } => {
                // Placeholder para futuro fuzzing aleatorio puro de alta entropía
            }
        }
    }

    /// Ciclo Caliente (Hot Loop) de verificación.
    ///
    /// Esta función se ejecuta millones de veces por segundo.
    /// Debe ser `inline(always)` para evitar el overhead de llamada a función.
    #[inline(always)]
    fn check_candidate<H: FindingHandler>(
        filter: &ShardedFilter,
        pk: SafePrivateKey,
        source: String,
        handler: &H,
    ) {
        // 1. Derivación PubKey (Hot Path optimizado con Global Context)
        let pub_key = SafePublicKey::from_private(&pk);

        // 2. Generación Dirección Legacy (P2PKH Uncompressed)
        // La mayoría de los fondos perdidos antiguos usan direcciones no comprimidas.
        let addr = pubkey_to_address(&pub_key, false);

        // 3. Consulta al Filtro Particionado
        // ShardedFilter hashea la dirección y consulta solo el shard relevante en RAM.
        // Complejidad: O(1).
        if filter.contains(&addr) {
            handler.on_finding(addr, pk, source);
        }
    }
}
