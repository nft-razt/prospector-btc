// libs/core/math-engine/src/lib.rs
// =================================================================
// APARATO: CORE MATH ENGINE BARREL
// ESTADO: ACTUALIZADO V7.1 (ARITHMETIC + KANGAROO EXPOSED)
// =================================================================

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! # Core Math Engine
//!
//! Motor criptográfico de bajo nivel optimizado para el sistema Prospector BTC.

/// Funciones de Hashing Criptográfico (`SHA256`, `RIPEMD160`).
pub mod hashing;

/// Gestión segura de Escalares Secretos (Claves Privadas).
pub mod private_key;

/// Aritmética de Puntos de Curva (Claves Públicas).
pub mod public_key;

/// Catálogo de errores matemáticos y de formato.
pub mod errors;

/// Contexto Global Estático para `libsecp256k1`.
pub mod context;

/// Motor aritmético de bytes (BigEndian).
pub mod arithmetic; // ✅ MÓDULO NUEVO

/// Algoritmo Pollard's Lambda para búsquedas de rango corto.
pub mod kangaroo; // ✅ MÓDULO HABILITADO

/// Preludio del motor matemático.
pub mod prelude {
    pub use crate::hashing::{double_sha256, hash160};
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;
    pub use crate::errors::MathError;
    pub use crate::kangaroo::{KangarooSolver, KangarooConfig};
    pub use crate::arithmetic::add_u128_to_u256_be; // ✅ EXPORTACIÓN
}

#[cfg(test)]
mod tests {
    #[test]
    fn core_sanity_check() {
        assert_eq!(2 + 2, 4);
    }
}
