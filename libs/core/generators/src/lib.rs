// =================================================================
// APARATO: CORE GENERATORS
// ESTÁNDARES: RUST 2021, STRICT LINTING
// =================================================================

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

//! # Core Generators
//!
//! Transforma primitivas criptográficas abstractas (Puntos en curva)
//! en formatos legibles por humanos y compatibles con la red Bitcoin.
//!
//! ## Foco en la Tesis
//! Implementa específicamente el esquema **P2PKH (Pay to Public Key Hash)**,
//! que es el formato dominante en la era temprana de Bitcoin (Satoshi Era).

/// Generación de Direcciones Legacy (empiezan con '1').
pub mod address_legacy;

/// Exportación de claves privadas (Wallet Import Format).
pub mod wif;

/// Errores de generación.
pub mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        assert_eq!(1, 1);
    }
}
