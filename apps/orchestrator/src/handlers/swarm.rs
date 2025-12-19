/**
 * =================================================================
 * APARATO: SWARM NETWORK HANDLERS (V25.0 - MISSION CERTIFIED)
 * CLASIFICACIÓN: API LAYER (L3)
 * RESPONSABILIDAD: GESTIÓN DE HANDSHAKE CON EL ENJAMBRE HYDRA
 *
 * ESTRATEGIA DE ÉLITE:
 * - Deferred Persistence: Los heartbeats se procesan vía buffer.
 * - Reactive Broadcast: Cada misión completada se emite al HUD del Dashboard.
 * - Zero-Abbreviation: Cumplimiento estricto de la nomenclatura soberana.
 * =================================================================
 */

use crate::state::AppState;
use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse};
use prospector_domain_models::work::{AuditReport, WorkOrder, SearchStrategy};
use prospector_infra_db::repositories::mission_repository::MissionRepository;
use tracing::{error, info, instrument};

/**
 * Procesa la solicitud de un nodo para adquirir una nueva misión de auditoría.
 *
 * @param application_state Estado compartido con enlace a Turso (Engine A).
 * @param payload Identificador y metadatos del nodo solicitante.
 */
#[instrument(skip(application_state))]
pub async fn handle_mission_acquisition(
    State(application_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let worker_node_identifier = payload["worker_id"].as_str().unwrap_or("unknown_hydra_unit");

    let database_connection = match application_state.db.get_connection() {
        Ok(connection) => connection,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Tactical Link Severed").into_response(),
    };

    let mission_repository = MissionRepository::new(database_connection);

    // Por defecto, el enjambre opera en modo Sequential a menos que el Lab dicte lo contrario
    match mission_repository.acquire_next_mission_atomic(
        worker_node_identifier,
        SearchStrategy::Sequential {
            start_index_hex: "0".to_string(),
            end_index_hex: "0".to_string()
        }
    ).await {
        Ok(work_order) => {
            info!("🛰️ [MISSION_DISPATCHED]: Identifier {} -> Unit {}",
                work_order.job_mission_identifier,
                worker_node_identifier
            );
            (StatusCode::OK, Json(work_order)).into_response()
        },
        Err(error) => {
            error!("❌ [DISPATCH_FAILURE]: {}", error);
            (StatusCode::SERVICE_UNAVAILABLE, "Mission queue exhausted").into_response()
        }
    }
}

/**
 * Certifica el reporte de auditoría enviado por un nodo y notifica al Neural Link.
 *
 * @param report Certificado de esfuerzo computacional inmutable.
 */
pub async fn handle_mission_completion(
    State(application_state): State<AppState>,
    Json(report): Json<AuditReport>,
) -> impl IntoResponse {
    let database_connection = match application_state.db.get_connection() {
        Ok(connection) => connection,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let mission_repository = MissionRepository::new(database_connection);

    // 1. SELLADO DEL LEDGER TÁCTICO (Turso)
    match mission_repository.finalize_mission_certification(&report).await {
        Ok(_) => {
            // 2. DIFUSIÓN AL DASHBOARD (Neural Link)
            // Emitimos el reporte completo al Bus de Eventos para que el HUD se actualice
            application_state.events.notify_mission_audit_certified(report);
            StatusCode::OK
        },
        Err(error) => {
            error!("💀 [CERTIFICATION_REJECTED]: Mission integrity fault: {}", error);
            StatusCode::CONFLICT
        }
    }
}
