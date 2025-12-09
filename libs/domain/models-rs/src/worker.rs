
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Latido del corazón enviado por el minero.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerHeartbeat {
    pub worker_id: Uuid,
    pub hostname: String,
    /// Hashes por segundo calculados en el último intervalo.
    pub hashrate: u64,
    pub current_job_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
}
