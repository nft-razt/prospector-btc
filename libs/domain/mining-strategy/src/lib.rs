#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

//! # Prospector Domain Strategy
//!
//! Este crate implementa la lógica de negocio central para la auditoría de entropía.
//! Orquesta los diferentes motores de búsqueda (Secuencial, Diccionario, Forense)
//! y gestiona el ciclo de vida de las misiones criptográficas.
//!
//! ## Estratos de Ejecución
//! 1. **Engines:** Motores especializados que iteran sobre espacios de claves.
//! 2. **Executor:** Puente entre la orden de trabajo (DTO) y el motor matemático.
//! 3. **Validation:** Suites de pruebas de integridad para asegurar la corrección matemática.

// --- MÓDULOS DE COMPONENTES ---

/// Generación de claves basada en frases humanas (SHA256).
pub mod brainwallet;

/// Iterador para fuerza bruta combinatoria (Legacy).
pub mod combinatoric;

/// Iterador optimizado para ataques de diccionario.
pub mod dictionary;

/// Hub de Motores de Búsqueda (SatoshiXP, Sequential, Forensic).
pub mod engines;

/// Orquestador central de misiones y gestión de señales.
pub mod executor;

/// Algoritmo Pollard's Kangaroo para resolución de ECDLP rango corto.
pub mod kangaroo;

// --- MÓDULOS DE CERTIFICACIÓN ---

#[cfg(test)]
mod tests {
    /// Suite de pruebas de integridad secuencial.
    pub mod sequential_integrity;
}

// --- EXPORTACIONES SOBERANAS ---

pub use executor::{StrategyExecutor, FindingHandler};
pub use kangaroo::KangarooRunner;
pub use brainwallet::phrase_to_private_key;

pub use engines::sequential_engine::ProjectiveSequentialEngine;
pub use engines::satoshi_xp_engine::SatoshiWindowsXpForensicEngine;
pub use engines::forensic_engine::ForensicArchaeologyEngine;
pub use engines::dictionary_engine::EntropyDictionaryEngine;
