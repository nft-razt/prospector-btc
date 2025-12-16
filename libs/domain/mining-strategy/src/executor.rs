// libs/domain/mining-strategy/src/executor.rs
// =================================================================
// APARATO: STRATEGY EXECUTOR (V8.0 - ELITE GOLD MASTER)
// RESPONSABILIDAD: ORQUESTACIÓN PARALELA DE VECTORES DE ATAQUE
// ESTADO: SANEADO, OPTIMIZADO Y DOCUMENTADO
// =================================================================

use num_bigint::BigUint;
use num_traits::Zero;
use rayon::prelude::*;
use std::str::FromStr;
use tracing::{debug, warn};

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
use crate::kangaroo::KangarooRunner;
use prospector_domain_forensics::{AndroidLcgIterator, DebianIterator};

/// Interfaz abstracta para el reporte de hallazgos (Finding).
/// Permite desacoplar la lógica de cálculo del mecanismo de transporte (HTTP/Console/Socket).
pub trait FindingHandler: Sync + Send {
    /// Callback invocado cuando se detecta una colisión confirmada.
    ///
    /// # Argumentos
    /// * `address` - La dirección pública Bitcoin (P2PKH).
    /// * `pk` - La clave privada recuperada (objeto seguro).
    /// * `source` - Metadatos sobre cómo se encontró (ej: "brainwallet:satoshi").
    fn on_finding(&self, address: String, pk: SafePrivateKey, source: String);
}

/// Contexto de ejecución de solo lectura compartido entre hilos de minería.
///
/// Optimiza el uso de memoria evitando clonaciones masivas de datasets estáticos
/// (como diccionarios de 100MB) en cada hilo de ejecución.
#[derive(Default)]
pub struct ExecutorContext {
    /// Caché de palabras en RAM (Heap Global) para ataques de diccionario.
    pub dictionary_cache: Option<Vec<String>>,
}

/// Motor de ejecución estática y balanceador de carga de estrategias.
/// Actúa como el "Cerebro" local del Worker.
pub struct StrategyExecutor;

impl StrategyExecutor {
    /// Ejecuta la orden de trabajo asignada utilizando paralelismo de datos (SIMD/Rayon).
    ///
    /// # Flujo de Datos
    /// 1. Decodifica la `strategy` del `WorkOrder`.
    /// 2. Instancia el iterador correspondiente (Generador de Entropía).
    /// 3. Convierte el iterador secuencial en un puente paralelo (`par_bridge`).
    /// 4. Distribuye la carga en todos los núcleos de la CPU disponibles.
    /// 5. Ejecuta `check_candidate` en el bucle caliente.
    pub fn execute<H: FindingHandler>(
        job: &WorkOrder,
        filter: &ShardedFilter,
        context_data: &ExecutorContext,
        handler: &H,
    ) {
        match &job.strategy {
            // =================================================================
            // ESTRATEGIA 1: COMBINATORIA (FUERZA BRUTA INTELIGENTE - U256)
            // =================================================================
            SearchStrategy::Combinatoric {
                prefix,
                suffix,
                start_index,
                end_index,
            } => {
                // Parseo seguro de BigInts. Si falla, asume 0 (Fail-Safe).
                let start = BigUint::from_str(start_index).unwrap_or_else(|_| BigUint::zero());
                let end = BigUint::from_str(end_index).unwrap_or_else(|_| BigUint::zero());

                debug!(
                    "🔨 Estrategia Combinatoria: {}...{}",
                    start_index.chars().take(10).collect::<String>(),
                    end_index.chars().take(10).collect::<String>()
                );

                let iter = CombinatoricIterator::new(start, end, prefix.clone(), suffix.clone());

                // Paralelismo: Rayon roba trabajo (Work-Stealing) automáticamente.
                iter.par_bridge().for_each(|(phrase, pk)| {
                    Self::check_candidate(filter, pk, format!("comb:{}", phrase), handler);
                });
            }

            // =================================================================
            // ESTRATEGIA 2: DICCIONARIO (BRAINWALLETS)
            // =================================================================
            SearchStrategy::Dictionary {
                dataset_url: _,
                limit,
            } => {
                if let Some(words) = &context_data.dictionary_cache {
                    debug!("📚 Estrategia Diccionario: Procesando {} palabras", words.len());
                    let iter = DictionaryIterator::new(words, *limit);

                    iter.par_bridge().for_each(|(phrase, pk)| {
                        Self::check_candidate(filter, pk, format!("dict:{}", phrase), handler);
                    });
                } else {
                    warn!("⚠️ Estrategia Diccionario solicitada pero caché vacía. Saltando.");
                }
            }

            // =================================================================
            // ESTRATEGIA 3: FORENSE (ARQUEOLOGÍA DE BUGS)
            // =================================================================
            SearchStrategy::ForensicScan {
                target,
                range_start,
                range_end,
            } => {
                let start = u64::from_str(range_start).unwrap_or(0);
                let end = u64::from_str(range_end).unwrap_or(0);

                debug!("🔍 Estrategia Forense: {:?} [{} - {}]", target, start, end);

                match target {
                    ForensicTarget::DebianOpenSSL => {
                        // CVE-2008-0166 (OpenSSL PRNG seed constraint)
                        let iter = DebianIterator::new(start, end);
                        iter.par_bridge().for_each(|(source, pk)| {
                            Self::check_candidate(filter, pk, source, handler);
                        });
                    }
                    ForensicTarget::AndroidSecureRandom => {
                        // CVE-2013-7372 (Java SecureRandom collision)
                        let iter = AndroidLcgIterator::new(start, end);
                        iter.par_bridge().for_each(|(source, pk)| {
                            Self::check_candidate(filter, pk, source, handler);
                        });
                    }
                }
            }

            // =================================================================
            // ESTRATEGIA 4: CANGURO (POLLARD'S LAMBDA / DISCRETE LOG)
            // =================================================================
            SearchStrategy::Kangaroo {
                target_pubkey,
                start_scalar,
                width,
            } => {
                debug!("🦘 Estrategia Canguro: Target {}", target_pubkey);
                // Delegación completa al adaptador especializado
                KangarooRunner::run(target_pubkey, start_scalar, *width, handler);
            }

            // =================================================================
            // ESTRATEGIA 5: ALEATORIA (MONTE CARLO)
            // =================================================================
            SearchStrategy::Random { .. } => {
                // Placeholder para futuro fuzzing de alta entropía.
                // Actualmente inactivo para priorizar vectores deterministas.
            }
        }
    }

    /// Ciclo Caliente (Hot Loop) de verificación.
    ///
    /// Esta función es crítica para el rendimiento. Se ejecuta millones de veces por segundo.
    ///
    /// # Optimizaciones
    /// * `#[inline(always)]`: Obliga al compilador a inyectar el código en el punto de llamada,
    ///   eliminando el overhead del stack frame.
    /// * `Global Context`: `SafePublicKey::from_private` usa tablas estáticas pre-calculadas.
    /// * `Sharded Check`: La consulta al filtro es O(1) con acceso directo a memoria mapeada.
    #[inline(always)]
    fn check_candidate<H: FindingHandler>(
        filter: &ShardedFilter,
        pk: SafePrivateKey,
        source: String,
        handler: &H,
    ) {
        // 1. Derivación de Clave Pública (ECC Multiplication)
        let pub_key = SafePublicKey::from_private(&pk);

        // 2. Generación de Dirección (Hashing RIPEMD160(SHA256))
        // Usamos formato no comprimido (false) por defecto para arqueología pre-2012.
        // TODO: En v9.0, hacer configurable compressed/uncompressed desde WorkOrder.
        let addr = pubkey_to_address(&pub_key, false);

        // 3. Verificación Probabilística (Bloom Filter Check)
        if filter.contains(&addr) {
            // ¡COLISIÓN! Reportamos inmediatamente al handler (Worker Client).
            handler.on_finding(addr, pk, source);
        }
    }
}
