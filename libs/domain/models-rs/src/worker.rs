/**
 * =================================================================
 * APARATO: WORKER DOMAIN MODELS (V12.0 - TELEMETRY UPGRADE)
 * CLASIFICACIÓN: DOMAIN ENTITIES (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE TELEMETRÍA DE ALTA FIDELIDAD
 * =================================================================
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use typeshare::typeshare;

/// Latido del corazón enriquecido con métricas de salud de hardware.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerHeartbeat {
    pub worker_id: Uuid,
    pub hostname: String,
    pub hashrate: u64,
    pub current_job_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,

    // --- ESTRATO DE SALUD DE HARDWARE (Nivelado) ---
    pub cpu_frequency_mhz: u32,
    pub cpu_load_percent: f32,
    pub thermal_celsius: f32,
    pub memory_used_bytes: u64,
    pub core_count: u32,
}

/// Instantánea visual de vigilancia.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerSnapshot {
    pub worker_id: String,
    pub status: String,
    pub snapshot_base64: String,
    pub timestamp: String,
}
