/**
 * =================================================================
 * APARATO: NEURAL EVENT BUS SERVICE (V45.0 - FULL SPECTRUM)
 * CLASIFICACIÓN: APPLICATION SERVICES (ESTRATO L4)
 * RESPONSABILIDAD: DISPATCHER ASÍNCRONO DE SEÑALES ESTRATÉGICAS
 * =================================================================
 */

use tokio::sync::broadcast;
use tracing::{info, warn};
use prospector_domain_models::telemetry::{RealTimeEvent, SystemMetrics};
use prospector_domain_models::work::AuditReport;

const BROADCAST_BUFFER_CAPACITY: usize = 2048;

#[derive(Debug, Clone)]
pub struct EventBus {
    internal_transmission_sender: broadcast::Sender<RealTimeEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(BROADCAST_BUFFER_CAPACITY);
        Self { internal_transmission_sender: sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RealTimeEvent> {
        self.internal_transmission_sender.subscribe()
    }

    pub fn notify_system_pulse_update(&self, global_metrics: SystemMetrics) {
        let _ = self.internal_transmission_sender.send(
            RealTimeEvent::SystemPulseUpdate(global_metrics)
        );
    }

    pub fn notify_mission_audit_certified(&self, report: AuditReport) {
        info!("📢 [NEURAL_LINK]: Mission {} certified.", report.job_mission_identifier);
        let _ = self.internal_transmission_sender.send(
            RealTimeEvent::MissionAuditCertified(report)
        );
    }

    /**
     * ✅ RESOLUCIÓN ERROR E0599: Notificación de colisión criptográfica.
     */
    pub fn notify_cryptographic_collision(&self, address: String, node_id: String) {
        info!("🎯 [NEURAL_LINK]: Strategic collision detected by unit {}.", node_id);
        let _ = self.internal_transmission_sender.send(
            RealTimeEvent::CryptographicCollisionAlert {
                target_address: address,
                discovery_node: node_id,
            }
        );
    }

    pub fn notify_system_certified(&self) {
        let _ = self.internal_transmission_sender.send(
            RealTimeEvent::CryptographicCollisionAlert {
                target_address: "SYSTEM_CERTIFIED".to_string(),
                discovery_node: "AUTHORITY".to_string(),
            }
        );
    }

    pub fn notify_archival_drift(&self, drift_gap: u64, _total_tactical: u64) {
        warn!("🚨 [SYNC_DRIFT]: Archival gap detected: {} missions.", drift_gap);
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
