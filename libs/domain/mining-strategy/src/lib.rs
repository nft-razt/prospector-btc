#![deny(unsafe_code)]
#![warn(missing_docs)]

//! # Prospector Mining Strategies
//!
//! Provee una arquitectura polimórfica para la ejecución de auditorías
//! criptográficas. Coordina motores especializados en diccionarios,
//! rangos secuenciales y patrones forenses.

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

/// MOTORES ATÓMICOS DE ÉLITE (Nuevos módulos integrados V8.6)
pub mod engines {
    pub mod sequential_engine;
}

// --- RE-EXPORTS SOBERANOS ---

pub use brainwallet::{BrainwalletIterator, phrase_to_private_key};
pub use combinatoric::CombinatoricIterator;
pub use dictionary::DictionaryIterator;
pub use kangaroo::KangarooRunner;
pub use engines::sequential_engine::ProjectiveSequentialEngine;

pub use executor::{
    StrategyExecutor,
    FindingHandler,
    ExecutorContext
};

#[cfg(test)]
mod tests_execution;
