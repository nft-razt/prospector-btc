/**
 * =================================================================
 * APARATO: SOVEREIGN TELEMETRY CONTRACT (V40.1 - CLEANED)
 * CLASIFICACIÓN: DOMAIN MODELS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE SEÑALES DE ALTA DENSIDAD
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use crate::work::AuditReport;
// WorkerSnapshot eliminado por unused import warning

/// Métricas de hardware de alta frecuencia.
/// Optimizadas para empaquetamiento binario mediante MessagePack.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemMetrics {
    pub active_nodes_count: u32,
    pub cumulative_global_hashrate: u64,
    pub active_missions_in_flight: u32,
    /// Timestamp Unix en milisegundos para minimizar tamaño de string.
    pub timestamp_ms: u64,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "p")] // Alias cortos para el empaquetador binario
pub enum RealTimeEvent {
    #[serde(rename = "sp")]
    SystemPulseUpdate(SystemMetrics),

    #[serde(rename = "cc")]
    CryptographicCollisionAlert {
        target_address: String,
        discovery_node: String,
    },

    #[serde(rename = "ac")]
    MissionAuditCertified(AuditReport),

    #[serde(rename = "vr")]
    NodeVisualFrameReady {
        worker_identifier: String,
        operational_status: String,
        system_timestamp: u64,
    },
}
