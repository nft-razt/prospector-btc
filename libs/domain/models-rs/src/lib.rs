// libs/domain/models-rs/src/lib.rs
// =================================================================
// APARATO: DOMAIN MODELS BARREL
// RESPONSABILIDAD: EXPOSICIÓN PÚBLICA UNIFICADA (SSoT)
// ESTADO: REPARADO (EXPORTACIÓN FALTANTE AGREGADA)
// =================================================================

pub mod finding;
pub mod work;
pub mod worker;
pub mod identity;

// Módulo de pruebas unitarias internas
mod tests_serialization;

// Re-exports "Flattened" para facilitar el consumo en Apps (Orchestrator/Miner)
// Esto permite usar prospector_domain_models::Struct en lugar de ::work::Struct

pub use work::{WorkOrder, SearchStrategy, ForensicTarget, JobCompletion};
pub use finding::Finding;
pub use worker::{WorkerHeartbeat, WorkerSnapshot};

// CORRECCIÓN: Agregamos RevokeIdentityPayload a la lista de exportaciones
pub use identity::{
    Identity,
    IdentityStatus,
    CreateIdentityPayload,
    RevokeIdentityPayload
};
