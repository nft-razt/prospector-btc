/**
 * =================================================================
 * APARATO: SWARM HANDSHAKE HANDLER (V240.0 - SURVIVAL PROTOCOL)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE MISIONES CON FILTRO DE SALUD ACTIVO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el protocolo de supervivencia del enjambre. Antes de
 * adjudicar material criptográfico, el orquestador valida:
 * 1. El modo global del nexo operacional (Emergency Stop / Pause).
 * 2. La salud física del nodo (Thermal Throttling / CPU Stress).
 * Esto garantiza que solo nodos sanos procesen la Tesis.
 * =================================================================
 */

use crate::state::AppState;
use crate::state::operational_nexus::{SystemIntegrityStatus, SwarmOperationalMode};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse
};
use prospector_domain_models::work::{MissionRequestPayload, WorkOrder};
use prospector_domain_models::finding::Finding;
use prospector_domain_models::worker::WorkerHeartbeat;
use tracing::{info, warn, instrument};
use uuid::Uuid;

pub struct SwarmHandshakeHandler;

impl SwarmHandshakeHandler {
    /**
     * Negocia la adjudicación de misiones aplicando filtros de salud y mando.
     */
    #[instrument(skip(application_shared_state, mission_request_parameters))]
    pub async fn negotiate_mission_assignment_handshake(
        State(application_shared_state): State<AppState>,
        Json(mission_request_parameters): Json<MissionRequestPayload>,
    ) -> impl IntoResponse {
        let worker_node_identifier = &mission_request_parameters.worker_id;

        // 1. VERIFICACIÓN DE MODO DE EJECUCIÓN GLOBAL
        // ✅ RESOLUCIÓN: Consumo de 'get_current_mode' para eliminar Dead Code
        let operational_mode = application_shared_state.operational_nexus.get_current_mode();
        if operational_mode != SwarmOperationalMode::FullExecution {
            warn!("🛑 [HALT]: Mission assignment blocked. System Mode: {:?}", operational_mode);
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }

        // 2. FILTRO DE INTEGRIDAD CRIPTOGRÁFICA
        if application_shared_state.operational_nexus.get_integrity_status() == SystemIntegrityStatus::AwaitingCertification {
            return (StatusCode::FORBIDDEN, "Certification Required").into_response();
        }

        // 3. AUDITORÍA DE SALUD DEL NODO (SURVIVAL CHECK)
        // ✅ RESOLUCIÓN: Consumo de 'is_node_healthy' para proteger el hardware efímero
        if let Ok(parsed_uuid) = Uuid::parse_str(worker_node_identifier) {
            if !application_shared_state.swarm_telemetry.is_node_healthy(&parsed_uuid) {
                warn!("🔥 [THERMAL_PROTECTION]: Node [{}] is unstable. Denying mission.", worker_node_identifier);
                return StatusCode::TOO_MANY_REQUESTS.into_response(); // Back-off por salud
            }
        }

        // 4. DESPACHO DESDE BUFFER DE MEMORIA
        match application_shared_state.mission_control.pull_assignment() {
            Some(assigned_work_order) => {
                info!("🎯 [DISPATCH]: Node [{}] cleared for Mission [{}].",
                    worker_node_identifier,
                    assigned_work_order.job_mission_identifier
                );
                (StatusCode::OK, Json::<WorkOrder>(assigned_work_order)).into_response()
            },
            None => {
                warn!("🚰 [EMPTY]: Mission control buffer exhausted.");
                StatusCode::NOT_FOUND.into_response()
            }
        }
    }

    /**
     * Registra el pulso de vida y telemetría de hardware.
     */
    #[instrument(skip(application_shared_state, heartbeat_payload))]
    pub async fn register_worker_heartbeat_signal(
        State(application_shared_state): State<AppState>,
        Json(heartbeat_payload): Json<WorkerHeartbeat>,
    ) -> impl IntoResponse {
        application_shared_state.swarm_telemetry.synchronize_heartbeat(heartbeat_payload);
        StatusCode::OK
    }

    /**
     * Registra una colisión confirmada.
     */
    #[instrument(skip(application_shared_state, collision_metadata))]
    pub async fn register_cryptographic_collision_finding(
        State(application_shared_state): State<AppState>,
        Json(collision_metadata): Json<Finding>,
    ) -> impl IntoResponse {
        application_shared_state.event_bus.notify_cryptographic_collision(
            collision_metadata.address.clone(),
            collision_metadata.found_by_worker.clone()
        );
        application_shared_state.finding_vault.deposit_finding(collision_metadata);
        StatusCode::CREATED
    }
}
