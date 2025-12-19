#![deny(unsafe_code)]
#![warn(missing_docs)]

/**
 * =================================================================
 * APARATO: TELEMETRY DATA MODELS (V28.0 - MISSION AWARE)
 * CLASIFICACIÓN: DOMAIN MODELS (L2)
 * RESPONSABILIDAD: ESTRUCTURAS PARA COMUNICACIÓN EN TIEMPO REAL
 *
 * ESTRATEGIA DE ÉLITE:
 * - Discriminative Enums: Facilitan el parsing automático en TypeScript.
 * - Zero-Latency Bundling: Empaqueta reportes de misión con metadatos visuales.
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use crate::work::AuditReport;
use crate::worker::WorkerSnapshot;

/// Métrica agregada del estado global del sistema.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemMetrics {
    /// Nodos activos reportando en la ventana de tiempo.
    pub active_nodes_count: usize,
    /// Suma total de hashrate (Hashes por segundo).
    pub cumulative_global_hashrate: u64,
    /// Misiones actualmente en ejecución.
    pub active_missions_in_flight: usize,
    /// Timestamp ISO 8601.
    pub updated_at_timestamp: String,
}

/// Canal de eventos discretos emitidos vía Server-Sent Events (SSE).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", content = "payload")]
pub enum RealTimeEvent {
    /// Actualización periódica de métricas de salud.
    SystemPulseUpdate(SystemMetrics),

    /// Alerta crítica: Se ha detectado una colisión en el ledger.
    CryptographicCollisionAlert {
        target_address: String,
        discovery_node: String
    },

    /// ✅ NUEVO: Notificación de misión finalizada y certificada.
    /// Permite al Dashboard actualizar la tabla de 'Audit Trail' instantáneamente.
    MissionAuditCertified(AuditReport),

    /// Transmisión de vigilancia visual (Snapshot del navegador).
    NodeVisualFeedUpdate(WorkerSnapshot),
}
