pub mod finding;
pub mod work;
pub mod worker;
pub mod identity;

// Módulo de pruebas unitarias
mod tests_serialization;

// Re-exports "Flattened" para facilitar el consumo en Apps
pub use work::{WorkOrder, SearchStrategy, ForensicTarget, JobCompletion};
pub use finding::Finding;
pub use worker::{WorkerHeartbeat, WorkerSnapshot}; // <--- AGREGADO WorkerSnapshot
pub use identity::{Identity, CreateIdentityPayload, IdentityStatus};
