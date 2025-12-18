// libs/core/math-engine/src/lib.rs
// =================================================================
// APARATO: CORE MATH ENGINE BOOTSTRAP (V15.0)
// RESPONSABILIDAD: PUNTO DE ENTRADA Y EXPOSICIÓN DEL NÚCLEO MATEMÁTICO
// ESTADO: RESOLUCIÓN DE LINT rustc(missing_docs)
// =================================================================

// Prohibición estricta de código inseguro para garantizar estabilidad de memoria
#![deny(unsafe_code)]
// Exigencia de documentación para todo elemento público (Estándar de Tesis MIT)
#![warn(missing_docs)]
// Activación de lints pedantes para código idiomático
#![warn(clippy::all, clippy::pedantic)]

//! # Prospector Math Engine
//!
//! Este crate implementa las primitivas matemáticas y criptográficas fundamentales
//! necesarias para la auditoría de seguridad en la curva elíptica `secp256k1`.
//!
//! ## Estructura de Capas
//! El motor se divide en módulos especializados que manejan desde la aritmética
//! de bajo nivel hasta algoritmos complejos de resolución del Logaritmo Discreto.

/// Motor aritmético de bajo nivel para arrays de bytes (Big-Endian U256).
/// Implementa suma y resta con acarreo manual optimizado para el bucle caliente.
pub mod arithmetic;

/// Gestión del Contexto Global de `secp256k1`.
/// Utiliza el patrón Singleton con `once_cell` para evitar re-inicializaciones costosas.
pub mod context;

/// Catálogo de variantes de error para operaciones matemáticas y criptográficas.
/// Provee trazabilidad mediante la integración con `thiserror`.
pub mod errors;

/// Abstracción de funciones de Hashing (SHA256, RIPEMD160).
/// Incluye implementaciones optimizadas para la generación de direcciones Bitcoin.
pub mod hashing;

/// Gestión segura de Claves Privadas (Escalares $k$).
/// Garantiza la integridad del material secreto dentro de los límites del grupo.
pub mod private_key;

/// Aritmética de Puntos en la Curva (Claves Públicas $P$).
/// Soporta operaciones de multiplicación escalar y tweaking de puntos.
pub mod public_key;

/// Implementación del algoritmo "Canguro" de Pollard (Lambda).
/// Solucionador paralelo diseñado para la búsqueda de claves en intervalos acotados.
pub mod kangaroo;

/// Preludio para importaciones masivas de alta eficiencia.
///
/// Reagrupa los tipos más utilizados para simplificar el desarrollo de estrategias
/// de minería en capas superiores.
pub mod prelude {
    pub use crate::arithmetic::{add_u256_be, add_u64_to_u256_be, sub_u256_be};
    pub use crate::errors::MathError;
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;
    pub use crate::kangaroo::{KangarooConfig, KangarooSolver};
}
