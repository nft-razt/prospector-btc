
// =================================================================
// APARATO: CORE PROBABILISTIC
// ESTÁNDARES: RUST 2021, SERIALIZACIÓN BINARIA
// =================================================================

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

//! # Core Probabilistic
//!
//! Provee el mecanismo de filtrado O(1) para el sistema Prospector.
//!
//! ## Estrategia de Tesis:
//! En lugar de consultar la DB por cada dirección generada, consultamos
//! este filtro en memoria RAM. Solo si el filtro da positivo, incurrimos
//! en el costo de I/O de red.

/// Wrapper seguro y serializable del Filtro de Bloom.
pub mod filter_wrapper;

/// Errores de serialización o lógica.
pub mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        assert_eq!(true, true);
    }
}
