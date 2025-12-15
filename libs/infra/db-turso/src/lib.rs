// =================================================================
// APARATO: INFRA DB (TURSO ADAPTER)
// =================================================================

pub mod client;
pub mod errors;
pub mod repositories;
pub mod schema; // Script de inicialización

// Re-exports para facilitar el uso
pub use client::TursoClient;
