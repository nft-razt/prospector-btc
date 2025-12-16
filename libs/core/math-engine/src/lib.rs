// libs/core/math-engine/src/lib.rs
// =================================================================
// APARATO: CORE MATH ENGINE BARREL (V3.0 - ACADEMIC DOCS)
// RESPONSABILIDAD: PUNTO DE ENTRADA AL NÚCLEO MATEMÁTICO
// ESTÁNDAR: RUSTDOC COMPLIANT & STRICT LINTING
// =================================================================

// Prohibición estricta de código inseguro para garantizar estabilidad de memoria.
#![deny(unsafe_code)]
// Exigencia de documentación para todo elemento público (Estándar de Tesis).
#![warn(missing_docs)]
// Activación de lints pedantes para código idiomático.
#![warn(clippy::all, clippy::pedantic)]
// Excepciones tácticas para ergonomía
#![allow(clippy::module_name_repetitions)]

//! # Prospector Core Math Engine
//!
//! Este crate implementa las primitivas matemáticas y criptográficas fundamentales
//! necesarias para la auditoría de seguridad en la curva elíptica `secp256k1`.
//!
//! ## Fundamentos Teóricos
//!
//! ### 1. Criptografía de Curva Elíptica (ECC)
//! El sistema se basa en la ecuación de Weierstrass sobre un campo finito $\mathbb{F}_p$:
//! $$ y^2 = x^3 + 7 \pmod{p} $$
//!
//! Donde $p = 2^{256} - 2^{32} - 977$.
//!
//! ### 2. Problema del Logaritmo Discreto (ECDLP)
//! La seguridad de Bitcoin reside en la intratabilidad de encontrar $k$ dado $P$, donde:
//! $$ P = k \cdot G $$
//!
//! Este motor provee herramientas para atacar versiones debilitadas de este problema
//! (ej: entropía reducida) utilizando algoritmos como **Pollard's Kangaroo**.
//!
//! ### 3. Aritmética de Alta Precisión
//! Se implementan operaciones sobre enteros de 256 bits (`U256`) sin signo,
//! necesarios para manipular escalares y coordenadas que exceden la capacidad
//! de los registros de CPU estándar (64 bits).

/// Operaciones de hashing criptográfico (SHA-256, RIPEMD-160).
/// Usado para la generación de direcciones y fingerprints.
pub mod hashing;

/// Gestión segura de claves privadas (Escalares $k$).
/// Garantiza $0 < k < n$ (Orden de la curva).
pub mod private_key;

/// Aritmética de puntos en la curva (Claves Públicas $P$).
/// Soporta operaciones de grupo como la adición y multiplicación escalar.
pub mod public_key;

/// Catálogo de errores matemáticos y criptográficos.
pub mod errors;

/// Contexto global (Singleton) para tablas pre-computadas de `secp256k1`.
/// Optimiza el rendimiento evitando la re-inicialización de memoria.
pub mod context;

/// Motor aritmético de bajo nivel para arrays de bytes (Big-Endian U256).
/// Implementa suma y resta con acarreo (carry/borrow) manual.
pub mod arithmetic;

/// Implementación del algoritmo "Canguro" de Pollard (Lambda).
/// Permite la resolución paralela del ECDLP en intervalos acotados.
pub mod kangaroo;

/// Preludio para importación ergonómica de los tipos más comunes.
pub mod prelude {
    pub use crate::arithmetic::{add_u256_be, compare_u256_be, sub_u256_be};
    pub use crate::errors::MathError;
    pub use crate::hashing::{double_sha256, hash160};
    pub use crate::kangaroo::{KangarooConfig, KangarooSolver};
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;
}
