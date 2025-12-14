// apps/orchestrator/src/handlers/swarm.rs
// =================================================================
// APARATO: SWARM HANDLERS (TRAFFIC CONTROL)
// ESTADO: GOLD MASTER (ZERO WARNINGS)
// =================================================================

use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::{info, warn, error, instrument};
use serde_json::Value;
use crate::state::AppState;

// IMPORTACIÓN ATÓMICA DESDE LA VERDAD ÚNICA
use prospector_domain_models::{
    WorkerHeartbeat,
    Finding,
    JobCompletion
    // WorkOrder eliminado: El compilador infiere el tipo automáticamente.
};

use prospector_infra_db::repositories::{
    JobRepository,
    FindingRepository,
};

/// Helper interno para obtener conexión o devolver error HTTP inmediato.
macro_rules! get_conn_or_500 {
    ($state:expr) => {
        match $state.db.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                error!("❌ FATAL: No se pudo adquirir conexión DB: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    };
}

/// Endpoint: POST /heartbeat
/// Alta frecuencia. Solo actualiza memoria RAM.
#[instrument(skip(state), fields(worker_id = %heartbeat.worker_id))]
pub async fn receive_heartbeat(
    State(state): State<AppState>,
    Json(heartbeat): Json<WorkerHeartbeat>,
) -> impl IntoResponse {
    state.update_worker(heartbeat);
    StatusCode::OK.into_response()
}

/// Endpoint: POST /job/acquire
/// Asigna trabajo usando transacciones ACID estrictas.
#[instrument(skip(state))]
pub async fn assign_job(
    State(state): State<AppState>,
) -> Response {
    let conn = get_conn_or_500!(state);
    let repo = JobRepository::new(conn);
    let worker_placeholder = "worker-generic-v2";

    match repo.assign_work(worker_placeholder).await {
        Ok(work_order) => {
            info!("💼 Job {} asignado (Estrategia: {:?})", work_order.id, work_order.strategy);
            Json(work_order).into_response()
        },
        Err(e) => {
            error!("❌ CRITICAL: Fallo asignando trabajo: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: POST /job/keepalive
#[instrument(skip(state))]
pub async fn job_keep_alive(
    State(state): State<AppState>,
    Json(payload): Json<JobCompletion>,
) -> Response {
    let conn = get_conn_or_500!(state);
    let repo = JobRepository::new(conn);

    match repo.heartbeat(&payload.id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

/// Endpoint: POST /job/complete
#[instrument(skip(state))]
pub async fn complete_job(
    State(state): State<AppState>,
    Json(payload): Json<JobCompletion>,
) -> Response {
    let conn = get_conn_or_500!(state);
    let repo = JobRepository::new(conn);

    match repo.complete(&payload.id).await {
        Ok(_) => {
            info!("🏁 Job {} completado exitosamente.", payload.id);
            StatusCode::OK.into_response()
        },
        Err(e) => {
            error!("❌ Error cerrando Job {}: {}", payload.id, e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: POST /finding
#[instrument(skip(state))]
pub async fn report_finding(
    State(state): State<AppState>,
    Json(finding): Json<Finding>,
) -> Response {
    warn!("🚨 >>> COLISIÓN DETECTADA <<< Dirección: {}", finding.address);

    let repo = FindingRepository::new(state.db.clone());

    match repo.save(&finding).await {
        Ok(_) => {
            info!("💾 Hallazgo asegurado en DB.");
            StatusCode::CREATED.into_response()
        },
        Err(e) => {
            error!("💀 FATAL: FALLO DE PERSISTENCIA DE HALLAZGO: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: GET /status
pub async fn get_system_status(State(state): State<AppState>) -> Json<Vec<WorkerHeartbeat>> {
    Json(state.get_active_workers())
}

/// Endpoint: POST /panic
/// Recibe alertas de último aliento de workers moribundos.
#[instrument(skip(_state), fields(worker_id))]
pub async fn receive_panic_alert(
    State(_state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let worker_id = payload.get("worker_id").and_then(Value::as_str).unwrap_or("unknown");
    tracing::Span::current().record("worker_id", &worker_id);

    error!(
        "💀 PANIC ALERT RECEIVED FROM WORKER: {}. Payload: {}",
        worker_id,
        payload
    );

    StatusCode::OK
}
