// libs/core/probabilistic/src/lib.rs
// =================================================================
// APARATO: CORE PROBABILISTIC (SHARDING ENABLED)
// CLASIFICACIÓN: DATA STRUCTURES
// ESTÁNDARES: RUST 2021, STRICT LINTING, ZERO-COST ABSTRACTION
// =================================================================

// Mantenemos la prohibición general de código inseguro.
// Las excepciones (como mmap) deben estar encapsuladas y justificadas
// dentro de sus respectivos módulos con #[allow(unsafe_code)].
#![deny(unsafe_code)]
// Exigimos documentación para mantener el rigor académico de la Tesis.
#![warn(missing_docs)]
// Activamos el modo "Pedantic" de Clippy para garantizar código idiomático.
#![warn(clippy::all, clippy::pedantic)]
// Excepciones tácticas para ergonomía
#![allow(clippy::module_name_repetitions)] // FilterError es aceptable

//! # Core Probabilistic
//!
//! Este crate provee las estructuras de datos probabilísticas necesarias para
//! la verificación de pertenencia en conjuntos masivos (Set Membership) con
//! complejidad temporal constante $O(1)$ y uso de memoria optimizado.
//!
//! ## Estrategia de Tesis (Hydra-Zero)
//! En lugar de consultar la base de datos por cada dirección generada (lo cual
//! crearía un cuello de botella de I/O insostenible), los workers consultan
//! este filtro en memoria RAM.
//!
//! ## Arquitectura de Datos
//! * **RichListFilter:** Wrapper sobre un Filtro de Bloom estándar.
//! * **ShardedFilter:** Orquestador que maneja múltiples filtros particionados
//!   para permitir descargas paralelas y gestión eficiente de memoria (mmap).

/// Wrapper seguro y serializable del Filtro de Bloom individual.
pub mod filter_wrapper;

/// Orquestador de filtros particionados (Sharding Strategy).
/// Permite manejar el dataset de UTXO completo fragmentado en múltiples archivos.
pub mod sharded;

/// Catálogo de errores de serialización, I/O y lógica probabilística.
pub mod errors;

// --- RE-EXPORTS (ERGONOMÍA DE API) ---
// Facilitamos el consumo desde las aplicaciones (Miner, Census)
pub use errors::FilterError;
pub use filter_wrapper::RichListFilter;
pub use sharded::ShardedFilter;

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        // Verificación básica de que el sistema de tipos compila y linkea
        assert_eq!(true, true);
    }
}
