#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

//! # Prospector Math Engine
//!
//! Este componente es el corazón algorítmico del sistema Prospector. Implementa
//! la aritmética de curva elíptica secp256k1 optimizada para hardware x86_64,
//! permitiendo la auditoría masiva de entropía en el ledger de Bitcoin.

/*
 * =================================================================
 * APARATO: CORE MATH ENGINE (V15.6 - LINT RESOLVED)
 * CLASIFICACIÓN: ESTRATO 1 - NÚCLEO MATEMÁTICO (L1)
 * RESPONSABILIDAD: ORQUESTACIÓN DE PRIMITIVAS CRIPTOGRÁFICAS
 *
 * ESTRATEGIA DE ÉLITE:
 * - Prohibición global de código inseguro (con excepciones locales).
 * - Exposición de un Preludio unificado para ergonomía de desarrollo.
 * - Sincronización absoluta con el espacio de búsqueda U256 (256-bit).
 * =================================================================
 */

/// Motor aritmético de bajo nivel.
/// Maneja buffers de 32 bytes (U256) con optimizaciones en ensamblador inline.
pub mod arithmetic;

/// Gestión del Contexto Global.
/// Utiliza el patrón Singleton para evitar la re-computación de tablas secp256k1.
pub mod context;

/// Catálogo de Errores Matemáticos.
/// Provee trazabilidad semántica para fallos en la derivación o aritmética.
pub mod errors;

/// Abstracción de Hashing (SHA256 / RIPEMD160).
/// Incluye kernels de hashing por lotes (Batch Hashing) para el motor de búsqueda.
pub mod hashing;

/// Gestión de Claves Privadas (Escalares).
/// Garantiza que el material secreto se mantenga dentro del orden de la curva (n).
pub mod private_key;

/// Aritmética de Puntos en la Curva (Claves Públicas).
/// Soporta multiplicación escalar y "Tweak Addition" para ataques de colisión.
pub mod public_key;

/// Solucionador ECDLP (Pollard's Kangaroo).
/// Implementación paralela para la resolución de claves en intervalos acotados.
pub mod kangaroo;

/**
 * PRELUDIO DE ÉLITE (V15.6)
 *
 * Este módulo re-exporta las entidades y funciones críticas para que el
 * Miner Worker y las Estrategias de Dominio operen con la máxima ergonomía.
 */
pub mod prelude {
    // Aritmética U256 (Sincronizada con arithmetic.rs)
    pub use crate::arithmetic::{
        add_u256_be, add_u64_to_u256_be, compare_u256_be, fast_hex_encode, sub_u256_be,
        U256_BYTE_SIZE,
    };

    // Estructuras de Datos de Seguridad
    pub use crate::errors::MathError;
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;

    // Motores de Búsqueda Avanzada
    pub use crate::kangaroo::{KangarooConfig, KangarooSolver};

    // Funciones de Hashing críticas
    pub use crate::hashing::{batch_sha256, hash160};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_holistic_prelude_integrity() {
        // Validación de que el tamaño U256 es constante en el sistema
        assert_eq!(U256_BYTE_SIZE, 32);

        // Verificación de que SafePrivateKey es instanciable a través del preludio
        let random_key = SafePrivateKey::new_random();
        let bytes = random_key.to_bytes();
        assert_eq!(bytes.len(), 32);
    }
}
