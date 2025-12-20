/**
 * =================================================================
 * APARATO: SWARM NETWORK HANDLERS (V125.0 - SOBERANO)
 * CLASIFICACIÓN: API LAYER (ESTRATO L3)
 * RESPONSABILIDAD: ORQUESTACIÓN DE HANDSHAKE Y CERTIFICACIÓN
 * =================================================================
 */

use crate::state::AppState;
use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse};
use tracing::{info, warn, error, instrument};
use prospector_domain_models::work::{AuditReport, JobCompletion, WorkOrder};
use prospector_infra_db::repositories::mission_repository::MissionRepository;

/**
 * Endpoint: POST /api/v1/swarm/mission/acquire
 *
 * Entrega misiones de auditoría respetando el estado de pausa operativa.
 */
#[instrument(skip(application_state))]
pub async fn handle_mission_acquisition(
    State(application_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let worker_node_identifier = payload["worker_id"].as_str().unwrap_or("unknown_unit");

    // 1. PROTOCOLO DE PAUSA GRADUAL (The Hibernation Gate)
    if !application_state.is_mission_acquisition_authorized() {
        warn!("⏸️ [OPERATIONAL_STANDBY]: Node {} placed in standby.", worker_node_identifier);
        return (
            StatusCode::NO_CONTENT,
            [("X-Swarm-Status", "Standby_Active")]
        ).into_response();
    }

    // 2. CONEXIÓN TÁCTICA
    let database_connection = match application_state.database_client.get_connection() {
        Ok(connection) => connection,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // 3. SECUENCIACIÓN DINÁMICA
    match MissionRepository::acquire_dynamic_mission_atomic(
        &database_connection,
        worker_node_identifier,
        "WIN_XP_SP3_GOLD",
        3579545
    ).await {
        Ok(work_order) => {
            info!("🛰️ [DISPATCH]: Mission {} sent to unit {}.",
                work_order.job_mission_identifier, worker_node_identifier);
            (StatusCode::OK, Json(work_order)).into_response()
        },
        Err(error) => {
            error!("❌ [QUEUE_FAULT]: Mission allocation failed: {}", error);
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}

/**
 * Endpoint: POST /api/v1/swarm/mission/complete
 *
 * Certifica el reporte de auditoría inmutable y notifica al Neural Link.
 */
pub async fn handle_mission_certification(
    State(application_state): State<AppState>,
    Json(unvalidated_report): Json<serde_json::Value>,
) -> impl IntoResponse {
    // SOPORTE MULTI-GENERACIONAL (Cero Regresiones)
    if let Ok(certified_report) = serde_json::from_value::<AuditReport>(unvalidated_report.clone()) {
        info!("✅ [AUDIT_SEALED]: Certifying forensic report for mission {}.", certified_report.job_mission_identifier);
        application_state.event_bus.notify_mission_audit_certified(certified_report);
        return StatusCode::OK.into_response();
    }

    // Fallback Legacy
    if let Ok(legacy_report) = serde_json::from_value::<JobCompletion>(unvalidated_report) {
        warn!("♻️ [LEGACY_HANDSHAKE]: Processing legacy completion.");
        return StatusCode::OK.into_response();
    }

    StatusCode::BAD_REQUEST.into_response()
}
