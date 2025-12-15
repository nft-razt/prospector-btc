// libs/core/math-engine/src/lib.rs
// =================================================================
// APARATO: CORE MATH ENGINE BARREL
// ESTADO: PURIFICADO & DOCUMENTADO (LINT FREE)
// =================================================================

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! # Core Math Engine
//!
//! Motor criptográfico de bajo nivel optimizado para el sistema Prospector BTC.
//!
//! Este crate provee las primitivas matemáticas fundamentales (Curva Elíptica `secp256k1`,
//! Hashing `SHA256`/`RIPEMD160`) con un enfoque en **Rendimiento**, **Seguridad de Memoria**
//! y **Abstracciones de Costo Cero**.
//!
//! Es un componente **PURO**: No realiza operaciones de red, no accede a bases de datos
//! y minimiza las asignaciones en el Heap.

/// Funciones de Hashing Criptográfico (`SHA256`, `RIPEMD160`, `DoubleSHA256`).
/// Optimizadas para inlining.
pub mod hashing;

/// Gestión segura de Escalares Secretos (Claves Privadas).
/// Encapsula la lógica de generación de entropía y serialización.
pub mod private_key;

/// Aritmética de Puntos de Curva (Claves Públicas).
/// Maneja la derivación $P = k * G$ y la serialización comprimida/legacy.
pub mod public_key;

/// Catálogo de errores matemáticos y de formato.
pub mod errors;

/// Contexto Global Estático para `libsecp256k1`.
/// Implementa el patrón Singleton para evitar la re-inicialización costosa de tablas.
pub mod context;

// pub mod kangaroo; // Descomentar cuando la implementación esté lista

/// Preludio del motor matemático.
///
/// Re-exporta los tipos y funciones más utilizados para facilitar la integración
/// en otros crates del sistema (como `miner-worker` o `generators`).
pub mod prelude {
    pub use crate::hashing::{double_sha256, hash160};
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;
    pub use crate::errors::MathError;
    // No exportamos global_context() directamente para forzar el uso a través de los structs seguros
}

#[cfg(test)]
mod tests {
    #[test]
    fn core_sanity_check() {
        assert_eq!(2 + 2, 4);
    }
}
