/**
 * =================================================================
 * APARATO: DOMAIN MODELS MASTER HUB (V110.0)
 * =================================================================
 */

pub mod finding;
pub mod identity;
pub mod telemetry;
pub mod work;
pub mod worker;
pub mod scenario;
pub mod stratum; // ✅ NUEVO ESTRATO

// RE-EXPORTACIONES NIVELADAS
pub use stratum::StratumManifest;
pub use finding::Finding;
pub use identity::{Identity, IdentityStatus, EncryptedIdentityPayload};
pub use telemetry::{RealTimeEvent, SystemMetrics};
pub use work::{WorkOrder, SearchStrategy, TargetStrata, AuditReport};
pub use worker::{WorkerHeartbeat, WorkerSnapshot};
pub use scenario::SystemTemplateRegistry;
