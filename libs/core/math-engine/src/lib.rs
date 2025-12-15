// libs/core/math-engine/src/lib.rs
// =================================================================
// APARATO: CORE MATH ENGINE BARREL
// ESTADO: PURIFICADO (NO NETWORK DEPS)
// =================================================================

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! # Core Math Engine
//! Motor criptográfico optimizado para Prospector BTC.
//!
//! Este crate es de BAJO NIVEL y PURO. No realiza operaciones de red ni I/O.

// Módulos Internos
pub mod context;
pub mod errors;
pub mod hashing;
pub mod private_key;
pub mod public_key;
// pub mod kangaroo; // Descomentar cuando la implementación esté lista

// PRELUDE
pub mod prelude {
    pub use crate::errors::MathError;
    pub use crate::hashing::{double_sha256, hash160};
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;
    // No exportamos global_context() directamente para forzar el uso a través de los structs seguros
}

#[cfg(test)]
mod tests {
    #[test]
    fn core_sanity_check() {
        assert_eq!(2 + 2, 4);
    }
}
