#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

//! # Prospector Domain Strategy // Doctoral Research Stratum
//!
//! =================================================================
//! APARATO: MINING STRATEGY MASTER BARREL (V111.6 - SOBERANO)
//! CLASIFICACIÓN: DOMAIN REGISTRY (ESTRATO L2)
//! RESPONSABILIDAD: EXPOSICIÓN DE MOTORES Y AUDITORES FORENSES
//! =================================================================
//!
//! Este crate constituye el núcleo de ejecución estratégica del sistema.
//! Orquesta la transición entre la búsqueda secuencial proyectiva,
//! la arqueología de entropía Satoshi-XP y la auditoría de red real.

/// Implementación de derivación de llaves basadas en frases humanas (Brainwallets).
pub mod brainwallet;
/// Generación de entropía basada en permutaciones y rangos indexados.
pub mod combinatoric;
/// Motores de búsqueda basados en listas de palabras y diccionarios.
pub mod dictionary;
/// Núcleo de los motores de ejecución (Satoshi-XP, Secuencial Jacobiano, Forense).
pub mod engines;
/// Orquestador central encargado de ejecutar misiones y reportar resultados.
pub mod executor;
/// Implementación del algoritmo Pollard's Kangaroo para resolución de ECDLP.
pub mod kangaroo;
/// Auditor de vectores en tiempo real contra la red viva de Bitcoin.
pub mod forensic_auditor;

// --- RE-EXPORTACIONES SOBERANAS (SSoT) ---

pub use executor::{StrategyExecutor, FindingHandler};
pub use kangaroo::KangarooRunner;
pub use brainwallet::phrase_to_private_key;

pub use engines::sequential_engine::ProjectiveSequentialEngine;
pub use engines::satoshi_xp_engine::SatoshiWindowsXpForensicEngine;
pub use engines::forensic_engine::ForensicArchaeologyEngine;
pub use engines::dictionary_engine::EntropyDictionaryEngine;

pub use forensic_auditor::{ForensicVectorAuditor, VerifiedVectorAuditReport};

/// Módulos de certificación y pruebas de integración.
#[cfg(test)]
mod tests {
    /// Suite de certificación de integridad secuencial.
    pub mod sequential_integrity;
}
