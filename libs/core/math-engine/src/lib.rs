#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

//! # Prospector Math Engine
//!
//! Este componente implementa la aritmética de curva elíptica secp256k1
//! optimizada para hardware `x86_64`, permitiendo la auditoría masiva
//! de entropía en el ledger de Bitcoin.

pub mod arithmetic;
pub mod context;
pub mod errors;
pub mod hashing;
pub mod private_key;
pub mod public_key;
pub mod kangaroo;

/**
 * PRELUDIO DE ÉLITE (V16.0)
 *
 * Re-exporta las entidades críticas para que el motor de búsqueda
 * opere con la máxima ergonomía y rendimiento.
 */
pub mod prelude {
    pub use crate::arithmetic::{
        add_u256_be,
        add_u64_to_u256_be,
        compare_u256_be,
        fast_hex_encode,
        sub_u256_be,
        U256_BYTE_SIZE,
    };
    pub use crate::errors::MathError;
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;
    pub use crate::context::global_context;
    pub use crate::hashing::{batch_sha256, hash160};
}
