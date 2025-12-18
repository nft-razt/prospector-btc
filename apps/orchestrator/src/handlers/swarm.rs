// apps/orchestrator/src/handlers/swarm.rs
// =================================================================
// APARATO: SWARM TRAFFIC HANDLERS (V18.0)
// CLASIFICACIÓN: API LAYER // TACTICAL COMMUNICATION
// RESPONSABILIDAD: ORQUESTACIÓN DE NODOS Y TELEMETRÍA
// ESTADO: GOLD MASTER // NO ABBREVIATIONS
// =================================================================

use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::{error, info, instrument, warn};
use serde_json::Value;

// --- MODELOS DE DOMINIO ---
use prospector_domain_models::{Finding, JobCompletion, WorkerHeartbeat};

// --- INFRAESTRUCTURA ---
use prospector_infra_db::repositories::{
    FindingRepository,
    JobRepository,
    ScenarioRepository
};

/// Endpoint: POST /api/v1/swarm/heartbeat
///
/// Registra el pulso de actividad y métricas de hardware de un nodo minero.
#[instrument(skip(application_state, heartbeat_payload), fields(worker = %heartbeat_payload.worker_id))]
pub async fn receive_heartbeat(
    State(application_state): State<AppState>,
    Json(heartbeat_payload): Json<WorkerHeartbeat>,
) -> impl IntoResponse {
    application_state.update_worker(heartbeat_payload);
    StatusCode::OK
}

/// Endpoint: POST /api/v1/swarm/job/acquire
///
/// Asigna un nuevo segmento de búsqueda U256 a una unidad de cómputo.
#[instrument(skip(application_state))]
pub async fn assign_search_range(
    State(application_state): State<AppState>
) -> Response {
    let database_connection = match application_state.db.get_connection() {
        Ok(connection) => connection,
        Err(error) => {
            error!("❌ DATABASE_ERROR: Connection acquisition failed: {}", error);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let job_repository = JobRepository::new(database_connection);
    let worker_identifier = "hydra-node-gateway";

    match job_repository.assign_to_worker(worker_identifier).await {
        Ok(work_order) => Json(work_order).into_response(),
        Err(error) => {
            error!("❌ ASSIGNMENT_FAULT: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: POST /api/v1/swarm/job/keepalive
///
/// Extiende el tiempo de vida (lease) de un trabajo activo.
#[instrument(skip(application_state))]
pub async fn extend_search_lease(
    State(application_state): State<AppState>,
    Json(completion_payload): Json<JobCompletion>,
) -> Response {
    let job_repository = JobRepository::new(application_state.db.get_connection().unwrap());

    match job_repository.report_progress_heartbeat(&completion_payload.id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response()
    }
}

/// Endpoint: POST /api/v1/swarm/job/complete
///
/// Sella un rango como totalmente auditado.
#[instrument(skip(application_state))]
pub async fn finalize_search_range(
    State(application_state): State<AppState>,
    Json(completion_payload): Json<JobCompletion>,
) -> Response {
    let job_repository = JobRepository::new(application_state.db.get_connection().unwrap());

    match job_repository.finalize_job_success(&completion_payload.id).await {
        Ok(_) => {
            info!("🏁 AUDIT_COMPLETE: Range [{}] verified.", completion_payload.id);
            StatusCode::OK.into_response()
        },
        Err(error) => {
            error!("❌ FINALIZATION_FAULT: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: POST /api/v1/swarm/finding
///
/// REPORTE DE COLISIÓN: Persistencia y alerta de hallazgo de clave privada.
#[instrument(skip(application_state, finding_payload))]
pub async fn register_cryptographic_finding(
    State(application_state): State<AppState>,
    Json(finding_payload): Json<Finding>,
) -> Response {
    let finding_repo = FindingRepository::new(application_state.db.clone());
    let scenario_repo = ScenarioRepository::new(application_state.db.clone());

    match finding_repo.persist_collision(&finding_payload).await {
        Ok(_) => {
            let target_address = finding_payload.address.clone();
            tokio::spawn(async move {
                let _ = scenario_repo.mark_as_verified(&target_address).await;
            });

            application_state.events.notify_collision(
                finding_payload.found_by_worker.clone(),
                finding_payload.address.clone()
            );

            StatusCode::CREATED.into_response()
        },
        Err(error) => {
            error!("💀 PERSISTENCE_CRASH: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: GET /api/v1/admin/status
///
/// Provee el estado actual del cluster de nodos desde memoria volátil.
pub async fn get_node_cluster_status(
    State(application_state): State<AppState>
) -> Json<Vec<WorkerHeartbeat>> {
    Json(application_state.get_active_workers())
}

/// Endpoint: POST /api/v1/swarm/panic
///
/// Recolecta reportes de fallos críticos (Blackbox) de los workers.
pub async fn receive_worker_panic(
    State(_application_state): State<AppState>,
    Json(panic_payload): Json<Value>,
) -> impl IntoResponse {
    let node_id = panic_payload.get("worker_id").and_then(Value::as_str).unwrap_or("UNKNOWN");
    let message = panic_payload.get("message").and_then(Value::as_str).unwrap_or("No details");

    error!("💀 NODE_PANIC: Unit [{}] report: {}", node_id, message);
    StatusCode::OK
}
