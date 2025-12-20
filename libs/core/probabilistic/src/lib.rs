#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! # Core Probabilistic Stratum
//!
//! Implementación de estructuras de datos para la verificación de pertenencia
//! con complejidad temporal O(1). Utiliza Filtros de Bloom particionados
//! para optimización de memoria en nodos efímeros.

/// Envoltorio atómico para la estructura de Bloom individual.
pub mod filter_wrapper;
/// Orquestador para el particionamiento y enrutamiento de fragmentos.
pub mod sharded;
/// Definiciones de errores específicos del estrato probabilístico.
pub mod errors;

// --- RE-EXPORTS SOBERANOS ---
pub use errors::FilterError;
pub use filter_wrapper::RichListFilter;
pub use sharded::ShardedFilter;
