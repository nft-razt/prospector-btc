// apps/orchestrator/src/handlers/swarm.rs
// =================================================================
// APARATO: SWARM HANDLERS (TRAFFIC CONTROL)
// RESPONSABILIDAD: GESTIÓN DE ALTA FRECUENCIA DE MINEROS
// ESTADO: OPTIMIZADO (CONNECTION AWARE)
// =================================================================

use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
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

/// Helper interno para obtener conexión o devolver error HTTP inmediato.
/// Esto mantiene los handlers limpios y DRY (Don't Repeat Yourself).
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
/// Alta frecuencia. Solo actualiza memoria RAM (Redis-like behavior).
/// No toca la base de datos para máxima velocidad.
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
    // En el futuro: Claims del JWT para worker_id real
) -> Response {
    // 1. Adquisición de Conexión (Fail Fast)
    let conn = get_conn_or_500!(state);

    // 2. Instanciación del Repositorio con conexión viva
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
/// Evita que el Reaper marque el trabajo como Zombie en la DB.
#[instrument(skip(state))]
pub async fn job_keep_alive(
    State(state): State<AppState>,
    Json(payload): Json<JobCompletion>,
) -> Response {
    // 1. Adquisición de Conexión
    let conn = get_conn_or_500!(state);
    let repo = JobRepository::new(conn);

    match repo.heartbeat(&payload.id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => {
            // Si falla el heartbeat, es probable que el job ya no exista o haya expirado.
            // Retornamos 404 para que el worker sepa que debe pedir un trabajo nuevo.
            StatusCode::NOT_FOUND.into_response()
        },
    }
}

/// Endpoint: POST /job/complete
/// Cierra el ciclo de vida y libera al worker.
#[instrument(skip(state))]
pub async fn complete_job(
    State(state): State<AppState>,
    Json(payload): Json<JobCompletion>,
) -> Response {
    // 1. Adquisición de Conexión
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
/// EL EVENTO MÁS CRÍTICO DEL SISTEMA.
/// Este handler usa FindingRepository, que todavía acepta TursoClient (wrapper),
/// por lo que no requiere cambios drásticos, pero mantenemos la coherencia de logging.
#[instrument(skip(state))]
pub async fn report_finding(
    State(state): State<AppState>,
    Json(finding): Json<Finding>,
) -> Response {
    warn!("🚨 >>> COLISIÓN DETECTADA <<< Dirección: {}", finding.address);

    // FindingRepository maneja su propia conexión internamente (Legacy Mode por ahora)
    // Esto es aceptable ya que es una inserción simple, no una transacción compleja.
    let repo = FindingRepository::new(state.db.clone());

    match repo.save(&finding).await {
        Ok(_) => {
            info!("💾 Hallazgo asegurado en DB.");
            StatusCode::CREATED.into_response()
        },
        Err(e) => {
            error!("💀 FATAL: FALLO DE PERSISTENCIA DE HALLAZGO: {}", e);
            // Aunque falle la DB, el log ya capturó el evento (warn! arriba).
            // Aún así, retornamos error para que el worker reintente.
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: GET /status
/// Telemetría para el Dashboard.
pub async fn get_system_status(State(state): State<AppState>) -> Json<Vec<WorkerHeartbeat>> {
    Json(state.get_active_workers())
}
