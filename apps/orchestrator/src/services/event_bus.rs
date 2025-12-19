/**
 * =================================================================
 * APARATO: NEURAL EVENT BUS SERVICE (V35.0 - MISSION CERTIFIED)
 * CLASIFICACIÓN: ESTRATO DE SERVICIOS (L1-APP)
 * RESPONSABILIDAD: DISPATCHER ASÍNCRONO DE TELEMETRÍA ESTRATÉGICA
 *
 * ESTRATEGIA DE ÉLITE:
 * - High-Throughput: Canal de difusión masiva (Broadcast) para 2048 eventos.
 * - SSoT Enforcement: Solo emite eventos definidos en el contrato RealTimeEvent.
 * - Zero-Abbreviation: Métodos semánticos para cada tipo de reporte forense.
 * =================================================================
 */

use tokio::sync::broadcast;
use tracing::{info, warn};
use prospector_domain_models::telemetry::{RealTimeEvent, SystemMetrics};
use prospector_domain_models::work::AuditReport;

/// Capacidad del buffer para absorber ráfagas de misiones completadas.
const BROADCAST_BUFFER_CAPACITY: usize = 2048;

#[derive(Debug, Clone)]
pub struct EventBus {
    internal_transmission_sender: broadcast::Sender<RealTimeEvent>,
}

impl EventBus {
    /**
     * Inicializa el motor de difusión asíncrona.
     */
    pub fn new() -> Self {
        let (internal_transmission_sender, _) = broadcast::channel(BROADCAST_BUFFER_CAPACITY);
        Self { internal_transmission_sender }
    }

    /**
     * Crea un nuevo enlace de suscripción para el Neural Link (SSE).
     */
    pub fn subscribe(&self) -> broadcast::Receiver<RealTimeEvent> {
        self.internal_transmission_sender.subscribe()
    }

    /**
     * Notifica el pulso vital del sistema al Dashboard.
     *
     * @param global_metrics Métricas agregadas de salud y hashrate.
     */
    pub fn notify_system_pulse_update(&self, global_metrics: SystemMetrics) {
        let _ = self.internal_transmission_sender.send(
            RealTimeEvent::SystemPulseUpdate(global_metrics)
        );
    }

    /**
     * Certifica y difunde una misión finalizada hacia la interfaz del operador.
     * Este es el núcleo de la trazabilidad forense de la tesis.
     *
     * @param mission_completion_report Reporte inmutable del esfuerzo realizado.
     */
    pub fn notify_mission_audit_certified(&self, mission_completion_report: AuditReport) {
        info!(
            "📢 [NEURAL_LINK]: Mission {} certified. Emitting to Strategic HUD.",
            mission_completion_report.job_mission_identifier
        );

        if let Err(error) = self.internal_transmission_sender.send(
            RealTimeEvent::MissionAuditCertified(mission_completion_report)
        ) {
            warn!("⚠️ [EVENT_BUS_LAG]: Broadcast channel saturated: {}", error);
        }
    }

    /**
     * Alerta sobre un hallazgo positivo en el espacio de búsqueda.
     */
    pub fn notify_cryptographic_collision(&self, address: String, node_id: String) {
        let _ = self.internal_transmission_sender.send(
            RealTimeEvent::CryptographicCollisionAlert {
                target_address: address,
                discovery_node: node_id,
            }
        );
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
