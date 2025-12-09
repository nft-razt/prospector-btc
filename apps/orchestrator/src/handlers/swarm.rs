use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::{info, warn, error, instrument};
use crate::state::AppState;

// IMPORTACIÓN ATÓMICA DESDE LA VERDAD ÚNICA
use prospector_domain_models::{
    WorkerHeartbeat,
    Finding,
    JobCompletion
};

use prospector_infra_db::repositories::{
    JobRepository,
    FindingRepository,
};

/// Endpoint: POST /heartbeat
/// Alta frecuencia. Solo actualiza memoria RAM (Redis-like behavior).
#[instrument(skip(state), fields(worker_id = %heartbeat.worker_id))]
pub async fn receive_heartbeat(
    State(state): State<AppState>,
    Json(heartbeat): Json<WorkerHeartbeat>,
) -> impl IntoResponse {
    state.update_worker(heartbeat);
    StatusCode::OK
}

/// Endpoint: POST /job/acquire
/// Asigna trabajo usando ACID transactions.
#[instrument(skip(state))]
pub async fn assign_job(
    State(state): State<AppState>,
    // En el futuro, el worker_id vendrá del Token JWT o payload.
    // Por ahora, usamos un identificador genérico para la transacción.
) -> impl IntoResponse {
    let repo = JobRepository::new(state.db.clone());
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
/// Evita que el Reaper marque el trabajo como Zombie.
#[instrument(skip(state))]
pub async fn job_keep_alive(
    State(state): State<AppState>,
    Json(payload): Json<JobCompletion>, // Reusamos struct que tiene 'id'
) -> impl IntoResponse {
    let repo = JobRepository::new(state.db.clone());
    match repo.heartbeat(&payload.id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND, // El trabajo ya no existe o expiró
    }
}

/// Endpoint: POST /job/complete
/// Cierra el ciclo de vida y libera al worker.
#[instrument(skip(state))]
pub async fn complete_job(
    State(state): State<AppState>,
    Json(payload): Json<JobCompletion>,
) -> impl IntoResponse {
    let repo = JobRepository::new(state.db.clone());
    match repo.complete(&payload.id).await {
        Ok(_) => {
            info!("🏁 Job {} completado exitosamente.", payload.id);
            StatusCode::OK
        },
        Err(e) => {
            error!("❌ Error cerrando Job {}: {}", payload.id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Endpoint: POST /finding
/// EL EVENTO MÁS CRÍTICO DEL SISTEMA.
#[instrument(skip(state))]
pub async fn report_finding(
    State(state): State<AppState>,
    Json(finding): Json<Finding>,
) -> impl IntoResponse {
    warn!("🚨 >>> COLISIÓN DETECTADA <<< Dirección: {}", finding.address);

    let repo = FindingRepository::new(state.db.clone());
    match repo.save(&finding).await {
        Ok(_) => {
            info!("💾 Hallazgo asegurado en DB.");
            StatusCode::CREATED
        },
        Err(e) => {
            error!("💀 FATAL: FALLO DE PERSISTENCIA DE HALLAZGO: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Endpoint: GET /status
/// Telemetría para el Dashboard.
pub async fn get_system_status(State(state): State<AppState>) -> Json<Vec<WorkerHeartbeat>> {
    Json(state.get_active_workers())
}
