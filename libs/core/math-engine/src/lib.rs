
// =================================================================
// PROSPECTOR SYSTEM // APARATO: CORE MATH ENGINE
// CLASIFICACIÓN: HIGH-PERFORMANCE COMPUTING (HPC)
// ESTÁNDARES: RUST 2021, NO-PANIC, STRICT TYPES
// =================================================================

// -----------------------------------------------------------------
// 1. REGLAS DE SEGURIDAD Y RENDIMIENTO (LINTS)
// -----------------------------------------------------------------
// Prohibimos código inseguro explícitamente. Dependemos de secp256k1
// (que usa unsafe internamente) pero nuestra superficie es segura.
#![deny(unsafe_code)]

// Exigimos documentación para mantener el rigor académico de la Tesis.
#![warn(missing_docs)]

// Activamos el modo "Pedantic" de Clippy para garantizar código idiomático
// y de máximo rendimiento (evita clones innecesarios, dereferencias lentas, etc).
#![warn(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

// Excepciones tácticas para ergonomía
#![allow(clippy::module_name_repetitions)] // MathError es aceptable
#![allow(clippy::must_use_candidate)]      // No ensuciar APIs fluidas

//! # Core Math Engine
//!
//! El corazón termodinámico de Prospector.
//!
//! Este crate proporciona abstracciones de **Costo Cero (Zero-Cost Abstractions)**
//! sobre la criptografía de curva elíptica `secp256k1` y funciones de hash
//! optimizadas para la arquitectura de Bitcoin.
//!
//! ## Optimizaciones de Rendimiento
//! - **Inline Assembly:** Delega en `libsecp256k1` (C) para operaciones de curva.
//! - **Stack Allocation:** Evita `Heap` (allocations) en el bucle caliente.
//! - **Strict Typing:** Wrappers `SafePrivateKey` y `SafePublicKey` garantizan
//!   invariantes matemáticos en tiempo de compilación.

// -----------------------------------------------------------------
// 2. MÓDULOS DEL MOTOR
// -----------------------------------------------------------------

/// Operaciones de Hashing Criptográfico (SHA256, RIPEMD160).
/// Contiene funciones `#[inline(always)]` para fusión de instrucciones.
pub mod hashing;

/// Gestión de Escalares Secretos (Claves Privadas).
/// Implementa protección de memoria y generación segura de entropía.
pub mod private_key;

/// Aritmética de Puntos de Curva (Claves Públicas).
/// Maneja serialización comprimida (33 bytes) y legacy (65 bytes).
pub mod public_key;

/// Catálogo de fallos matemáticos controlados.
pub mod errors;

// -----------------------------------------------------------------
// 3. PRELUDE (ERGONOMÍA DE DESARROLLO)
// -----------------------------------------------------------------

/// Re-exportación de los tipos más usados para facilitar la integración.
/// Uso: `use prospector_core_math::prelude::*;`
pub mod prelude {
    pub use crate::hashing::{double_sha256, hash160};
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;
    pub use crate::errors::MathError;
}

#[cfg(test)]
mod tests {
    #[test]
    fn core_sanity_check() {
        // Verificación básica de que el sistema de tipos compila y linkea
        assert_eq!(2 + 2, 4);
    }
}
