/**
 * =================================================================
 * APARATO: CERTIFICATION AUTHORITY SERVICE (V60.5 - GOLD MASTER)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: VALIDACIÓN DE INTEGRIDAD Y SELLADO DE SISTEMA
 *
 * VISION HIPER-HOLÍSTICA:
 * Escucha el bus de eventos en busca de señales de colisión. Si el
 * sistema está en 'CertificationInProgress', valida si el hallazgo
 * corresponde al Golden Vector (Bloque 1). Ante un match, autoriza
 * la transición a estado operativo para misiones de alto valor.
 * =================================================================
 */

use crate::state::AppState;
use crate::state::operational_nexus::SystemIntegrityStatus;
use prospector_domain_models::telemetry::RealTimeEvent;
use std::sync::Arc;
use tracing::{info, warn, instrument};

/// Dirección Bitcoin canónica del Bloque 1 para validación de la simulación.
const GOLDEN_VECTOR_ADDRESS: &str = "12cbqSREwGrvtd3LsBhymWvCX9A9Snd9E7";

pub struct CertificationAuthorityService {
    /// Referencia al estado neural atomizado de la aplicación.
    application_state: AppState,
}

impl CertificationAuthorityService {
    /**
     * Construye una nueva instancia de la autoridad de certificación.
     * @param application_state Estado compartido inyectado.
     */
    pub fn new(application_state: AppState) -> Self {
        Self { application_state }
    }

    /**
     * Inicia el daemon de escucha sobre el bus de eventos neural.
     * Se ejecuta en un hilo asíncrono persistente.
     */
    pub async fn spawn_integrity_listener(self: Arc<Self>) {
        let mut event_subscriber = self.application_state.event_bus.subscribe();

        info!("⚖️  [AUTHORITY]: Integrity listener online. Monitoring for Golden Vector signals.");

        tokio::spawn(async move {
            while let Ok(neural_event) = event_subscriber.recv().await {
                if let RealTimeEvent::CryptographicCollisionAlert { target_address, discovery_node } = neural_event {
                    self.evaluate_discovery_integrity(target_address, discovery_node).await;
                }
            }
        });
    }

    /**
     * Evalúa si una colisión reportada es el vector de certificación esperado.
     */
    #[instrument(skip(self, target_address, discovery_node))]
    async fn evaluate_discovery_integrity(
        &self,
        target_address: String,
        discovery_node: String
    ) {
        let current_integrity_level = self.application_state.operational_nexus.get_integrity_status();

        // Solo actuar si el sistema está activamente buscando una certificación
        if current_integrity_level != SystemIntegrityStatus::CertificationInProgress {
            return;
        }

        info!("🧪 [CERT_CHECK]: Analyzing discovery from node [{}]...", discovery_node);

        if target_address == GOLDEN_VECTOR_ADDRESS {
            info!("✅ [CERTIFIED]: Golden Vector match confirmed. System integrity verified.");

            // Transición del estado global a Operativo Certificado
            self.application_state.operational_nexus.update_integrity(
                SystemIntegrityStatus::CertifiedOperational
            );

            // Notificación al Neural Link del cambio de fase
            self.application_state.event_bus.notify_system_certified();

            info!("🚀 [PROSPECTOR_READY]: Full search capacity authorized for the enjambre.");
        } else {
            warn!("⚠️  [MISMATCH]: Node [{}] reported a collision that does not match the Golden Vector.", discovery_node);
        }
    }
}
