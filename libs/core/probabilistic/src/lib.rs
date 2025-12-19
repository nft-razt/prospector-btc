#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! # Core Probabilistic
//!
//! Implementación de estructuras de datos para la verificación de pertenencia
//! con complejidad temporal $O(1)$.
//!
//! ## Arquitectura de Datos
//! * **`RichListFilter`**: Wrapper sobre un Filtro de Bloom estándar.
//! * **`ShardedFilter`**: Orquestador que maneja múltiples filtros particionados
//!   para permitir descargas paralelas y gestión de memoria eficiente.

pub mod filter_wrapper;
pub mod sharded;
pub mod errors;

// --- RE-EXPORTS (ERGONOMÍA DE API) ---

pub use errors::FilterError;
pub use filter_wrapper::RichListFilter;
pub use sharded::ShardedFilter;
