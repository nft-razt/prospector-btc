// libs/domain/models-rs/src/worker.rs
// =================================================================
// APARATO: WORKER DOMAIN MODELS (V11.0)
// RESPONSABILIDAD: DEFINICIÓN DE TELEMETRÍA DE HARDWARE Y RED
// ESTADO: EXTENDED FOR THERMAL MONITORING
// =================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Latido del corazón enviado por el minero con métricas de salud de hardware.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerHeartbeat {
    pub worker_id: Uuid,
    pub hostname: String,
    pub hashrate: u64,
    pub current_job_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,

    // --- NUEVAS MÉTRICAS DE HARDWARE ---

    /// Frecuencia actual del procesador en MHz.
    /// Permite detectar 'throttling' por parte del proveedor cloud.
    pub cpu_frequency_mhz: u32,

    /// Carga total del sistema (0-100).
    pub cpu_load_percent: f32,

    /// Cantidad de núcleos físicos/lógicos en uso.
    pub core_count: u32,
}

/// Instantánea visual (Panóptico) - Se mantiene sin cambios para evitar regresiones.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerSnapshot {
    pub worker_id: String,
    pub status: String,
    pub snapshot_base64: String,
    pub timestamp: String,
}
