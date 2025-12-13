// libs/domain/models-rs/src/lib.rs
// =================================================================
// APARATO: DOMAIN MODELS BARREL
// RESPONSABILIDAD: EXPOSICIÓN PÚBLICA UNIFICADA (SSoT)
// =================================================================

pub mod finding;
pub mod work;
pub mod worker;
pub mod identity;

// Módulo de pruebas unitarias internas de serialización
mod tests_serialization;

// --- RE-EXPORTS FLATTENED (La API Pública) ---

// 1. Trabajo y Estrategia
pub use work::{
    WorkOrder,
    SearchStrategy,
    ForensicTarget, // ✅ Ahora existe en work.rs
    JobCompletion   // ✅ Ahora existe en work.rs
};

// 2. Hallazgos (El Tesoro)
pub use finding::Finding;

// 3. Telemetría de Workers
pub use worker::{
    WorkerHeartbeat,
    WorkerSnapshot
};

// 4. Identidad y Acceso
pub use identity::{
    Identity,
    IdentityStatus,
    CreateIdentityPayload,
    RevokeIdentityPayload
};
