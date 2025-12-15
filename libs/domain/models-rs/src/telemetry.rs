// libs/domain/models-rs/src/telemetry.rs
// =================================================================
// APARATO: TELEMETRY DATA MODELS (v6.3 - SYNC FIX)
// RESPONSABILIDAD: ESTRUCTURAS DE DATOS EN TIEMPO REAL (SSE PAYLOAD)
// ESTADO: SINCRONIZADO CON CONTRATO TYPESCRIPT (SnapshotReceived ADDED)
// =================================================================

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

// ✅ IMPORTACIÓN CRÍTICA: Necesitamos el modelo visual definido en worker.rs
use crate::worker::WorkerSnapshot;

/// Métrica agregada del sistema completo.
/// Enviada periódicamente vía Server-Sent Events (SSE).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemMetrics {
    /// Total de nodos reportándose en la ventana de tiempo activa (último minuto).
    pub active_nodes: usize,

    /// Suma total de hashrate (Hashes por segundo).
    pub global_hashrate: u64,

    /// Cantidad de trabajos actualmente asignados y no completados.
    pub jobs_in_flight: usize,

    /// Timestamp ISO 8601 de la agregación.
    pub timestamp: String,
}

/// Eventos discretos que el Dashboard debe conocer inmediatamente.
/// Utiliza un enum "Tagged" para fácil discriminación en TypeScript.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum RealTimeEvent {
    /// Actualización periódica de métricas del enjambre.
    Metrics(SystemMetrics),

    /// Alerta de alta prioridad: Un worker ha encontrado una colisión.
    ColissionAlert { address: String, worker_id: String },

    /// Notificación de nuevo nodo (Opcional, para logs de eventos).
    NodeJoined { worker_id: String, hostname: String },

    /// ✅ NUEVO: Transmisión de Vigilancia Visual (Panóptico).
    /// Permite que la imagen viaje por el bus de eventos hasta el Frontend.
    SnapshotReceived(WorkerSnapshot),
}
