pub mod finding;
pub mod work;
pub mod worker;
pub mod identity;

// Re-exports "Flattened" para facilitar el consumo en Apps
pub use work::{WorkOrder, SearchStrategy, ForensicTarget, JobCompletion};
pub use finding::Finding;
pub use worker::WorkerHeartbeat;
pub use identity::{Identity, CreateIdentityPayload, IdentityStatus};
