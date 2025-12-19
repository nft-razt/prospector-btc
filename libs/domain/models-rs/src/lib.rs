#![deny(unsafe_code)]
#![warn(missing_docs)]

//! # Domain Models (Rust Edition)
//!
//! Este aparato constituye la Fuente Única de Verdad (SSoT) para las estructuras
//! de datos que transitan por el Neural Link entre el Orquestador y el Enjambre.

pub mod finding;
pub mod identity;
pub mod telemetry;
pub mod work;
pub mod worker;

// --- RE-EXPORTACIONES NIVELADAS (ZERO REGRESSIONS) ---

pub use finding::Finding;
pub use identity::{
    CreateIdentityPayload,
    Identity,
    IdentityStatus,
    RevokeIdentityPayload,
    EncryptedIdentityPayload
};
pub use telemetry::{RealTimeEvent, SystemMetrics};

// Sincronización con la refactorización de misiones tácticas
pub use work::{
    ForensicTarget,
    JobCompletion,
    SearchStrategy,
    WorkOrder,
    AuditReport
};

pub use worker::{WorkerHeartbeat, WorkerSnapshot};

#[cfg(test)]
mod tests_serialization;
