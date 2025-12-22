/**
 * =================================================================
 * APARATO: MINING STRATEGY MASTER BARREL (V111.5 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN REGISTRY (ESTRATO L2)
 * RESPONSABILIDAD: EXPOSICIÓN DE MOTORES Y AUDITORES FORENSES
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la interfaz de exportación del cerebro algorítmico.
 * Garantiza que el Orquestador (L3) pueda acceder a los motores de
 * arqueología y a los validadores de red de forma unívoca.
 * =================================================================
 */

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

/// Implementación de derivación de llaves basadas en frases humanas.
pub mod brainwallet;
/// Generación de entropía basada en permutaciones y rangos.
pub mod combinatoric;
/// Motores de búsqueda basados en listas de palabras.
pub mod dictionary;
/// Núcleo de los motores de ejecución (Satoshi-XP, Secuencial, Forense).
pub mod engines;
/// Orquestador central encargado de ejecutar misiones.
pub mod executor;
/// Algoritmo Pollard's Kangaroo para resolución de ECDLP.
pub mod kangaroo;

// ✅ RESOLUCIÓN E0432: Declaración y exposición del auditor forense
/// Auditor forense de vectores en tiempo real contra la red Bitcoin.
pub mod forensic_auditor;

// --- RE-EXPORTACIONES SOBERANAS ---

pub use executor::{StrategyExecutor, FindingHandler};
pub use kangaroo::KangarooRunner;
pub use brainwallet::phrase_to_private_key;

pub use engines::sequential_engine::ProjectiveSequentialEngine;
pub use engines::satoshi_xp_engine::SatoshiWindowsXpForensicEngine;
pub use engines::forensic_engine::ForensicArchaeologyEngine;
pub use engines::dictionary_engine::EntropyDictionaryEngine;

// ✅ RESOLUCIÓN: Exposición nominal para el Orquestador
pub use forensic_auditor::{ForensicVectorAuditor, VerifiedVectorAuditReport};

/// Módulos de pruebas de integración para la certificación del sistema.
#[cfg(test)]
mod tests {
    pub mod sequential_integrity;
}
