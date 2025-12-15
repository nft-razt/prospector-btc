use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Latido del corazón enviado por el minero (Telemetría Numérica).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerHeartbeat {
    pub worker_id: Uuid,
    pub hostname: String,
    pub hashrate: u64,
    pub current_job_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
}

/// Instantánea visual y de estado del worker (Telemetría Visual/Panóptico).
/// Utilizado por el Provisioner para reportar capturas de pantalla de Colab.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerSnapshot {
    /// ID del worker (String para flexibilidad con formatos de Playwright)
    pub worker_id: String,

    /// Estado reportado por el navegador (running, captcha, error)
    pub status: String,

    /// Imagen codificada en Base64 (data:image/jpeg;base64,...)
    pub snapshot_base64: String,

    /// Fecha de la captura (ISO 8601 String)
    pub timestamp: String,
}
