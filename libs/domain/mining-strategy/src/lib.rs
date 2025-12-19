// libs/domain/mining-strategy/src/lib.rs
/**
 * =================================================================
 * APARATO: MINING STRATEGY FACADE (V15.0)
 * CLASIFICACIÓN: DOMAIN LAYER (L2)
 * RESPONSABILIDAD: EXPOSICIÓN DE MOTORES DE BÚSQUEDA
 * ESTADO: ZERO-REGRESSIONS // FULLY EXPOSED
 * =================================================================
 */

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! # Prospector Mining Strategies
//!
//! Contiene las implementaciones de búsqueda estratégica para el enjambre Hydra.

/// Lógica de generación basada en frases humanas (Brainwallets).
pub mod brainwallet;

/// Búsqueda secuencial y permutaciones.
pub mod combinatoric;

/// Ataques basados en diccionarios masivos.
pub mod dictionary;

/// Algoritmo Pollard's Kangaroo para intervalos cortos.
pub mod kangaroo;

/// Orquestador central y contratos de ejecución.
pub mod executor;

// --- RE-EXPORTS SOBERANOS ---
// Estos export permiten al Miner Worker importar todo desde la raíz de la lib.

pub use brainwallet::BrainwalletIterator;
pub use combinatoric::CombinatoricIterator;
pub use dictionary::DictionaryIterator;
pub use kangaroo::KangarooRunner;

// ✅ RESOLUCIÓN: Exportación atómica de contratos de ejecución
pub use executor::{
    StrategyExecutor,
    FindingHandler,
    ExecutorContext
};

#[cfg(test)]
mod tests_execution;
