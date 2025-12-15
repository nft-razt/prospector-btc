// libs/domain/models-rs/src/lib.rs
// =================================================================
// APARATO: DOMAIN EXPORTS
// ESTADO: ACTUALIZADO CON TELEMETRÍA
// =================================================================

pub mod finding;
pub mod identity;
pub mod telemetry;
pub mod work;
pub mod worker; // ✅ MÓDULO AÑADIDO

mod tests_serialization;

// --- RE-EXPORTS ---
pub use finding::Finding;
pub use identity::{CreateIdentityPayload, Identity, IdentityStatus, RevokeIdentityPayload};
pub use telemetry::{RealTimeEvent, SystemMetrics};
pub use work::{ForensicTarget, JobCompletion, SearchStrategy, WorkOrder};
pub use worker::{WorkerHeartbeat, WorkerSnapshot}; // ✅ EXPORTACIÓN
