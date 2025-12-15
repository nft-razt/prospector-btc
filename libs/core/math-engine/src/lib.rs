// libs/core/math-engine/src/lib.rs
// =================================================================
// APARATO: CORE MATH ENGINE BARREL
// ESTADO: V7.1 (ARITHMETIC + KANGAROO)
// =================================================================

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! # Core Math Engine
//! Motor criptográfico optimizado para Prospector BTC.

pub mod hashing;
pub mod private_key;
pub mod public_key;
pub mod errors;
pub mod context;
pub mod arithmetic; // ✅ MÓDULO NUEVO
pub mod kangaroo;

/// Preludio del motor matemático.
pub mod prelude {
    pub use crate::hashing::{double_sha256, hash160};
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;
    pub use crate::errors::MathError;
    pub use crate::kangaroo::{KangarooSolver, KangarooConfig};
    pub use crate::arithmetic::add_u128_to_u256_be;
}

#[cfg(test)]
mod tests {
    #[test]
    fn core_sanity_check() {
        assert_eq!(1 + 1, 2);
    }
}
