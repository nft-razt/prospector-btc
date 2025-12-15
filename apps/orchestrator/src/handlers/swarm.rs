// apps/orchestrator/src/handlers/swarm.rs
// =================================================================
// APARATO: SWARM TRAFFIC CONTROLLER (v6.1 - NEURAL ENABLED)
// RESPONSABILIDAD: GESTIÓN DE ALTA FRECUENCIA DE NODOS MINEROS
// PATRÓN: HTTP ADAPTER -> DOMAIN LOGIC -> EVENT BUS
// =================================================================

use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::Value;
use tracing::{error, info, instrument, warn};

// DOMINIO (Tipos estrictos)
use prospector_domain_models::{Finding, JobCompletion, WorkerHeartbeat};

// INFRAESTRUCTURA (Acceso a Datos)
use prospector_infra_db::repositories::{FindingRepository, JobRepository};

/// Macro utilitaria para obtener conexión DB o fallar rápido.
/// Reduce el ruido visual en los handlers.
macro_rules! get_conn_or_500 {
    ($state:expr) => {
        match $state.db.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                error!("❌ DB CONNECTION ERROR: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    };
}

/// Endpoint: POST /heartbeat
/// Frecuencia: Alta (Cada 30s por nodo).
/// Responsabilidad: Actualizar estado en memoria RAM (Volátil).
#[instrument(skip(state), fields(worker = %heartbeat.worker_id))]
pub async fn receive_heartbeat(
    State(state): State<AppState>,
    Json(heartbeat): Json<WorkerHeartbeat>,
) -> impl IntoResponse {
    // Delegamos la lógica de actualización y detección de nuevos nodos al Estado.
    state.update_worker(heartbeat);
    StatusCode::OK.into_response()
}

/// Endpoint: POST /job/acquire
/// Responsabilidad: Asignación transaccional de rangos de búsqueda (ACID).
#[instrument(skip(state))]
pub async fn assign_job(State(state): State<AppState>) -> Response {
    let conn = get_conn_or_500!(state);
    let repo = JobRepository::new(conn);

    // TODO: Extraer ID real del token JWT en el middleware y pasarlo aquí.
    let worker_placeholder = "worker-generic-v2";

    match repo.assign_work(worker_placeholder).await {
        Ok(work_order) => {
            info!(
                "💼 Job asignado: {} [Strategy: {:?}]",
                work_order.id, work_order.strategy
            );
            Json(work_order).into_response()
        }
        Err(e) => {
            error!("❌ JOB ASSIGNMENT FAILED: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: POST /job/keepalive
/// Responsabilidad: Evitar que el Reaper marque el trabajo como zombie.
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
/// Responsabilidad: Cierre de ciclo de trabajo exitoso.
#[instrument(skip(state))]
pub async fn complete_job(
    State(state): State<AppState>,
    Json(payload): Json<JobCompletion>,
) -> Response {
    let conn = get_conn_or_500!(state);
    let repo = JobRepository::new(conn);

    match repo.complete(&payload.id).await {
        Ok(_) => {
            info!("🏁 Job completado: {}", payload.id);
            StatusCode::OK.into_response()
        }
        Err(e) => {
            error!("❌ JOB COMPLETION ERROR [{}]: {}", payload.id, e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: POST /finding
/// Responsabilidad: ALERTA CRÍTICA. Persistencia de colisión y notificación Push.
#[instrument(skip(state))]
pub async fn report_finding(
    State(state): State<AppState>,
    Json(finding): Json<Finding>,
) -> Response {
    // 1. Log Forense Inmediato
    warn!(
        "🚨 >>> COLISIÓN CRIPTOGRÁFICA DETECTADA <<< Address: {}",
        finding.address
    );

    // 2. Persistencia en Bóveda (Indestructible)
    let repo = FindingRepository::new(state.db.clone());

    match repo.save(&finding).await {
        Ok(_) => {
            info!("💾 Hallazgo asegurado en base de datos.");

            // 3. Activación de Sinapsis (Notificación Tiempo Real)
            // Notificamos a todos los clientes SSE conectados.
            state.events.notify_collision(
                "swarm-unit".to_string(), // ID genérico ya que Finding no trae worker_id actualmente
                finding.address,
            );

            StatusCode::CREATED.into_response()
        }
        Err(e) => {
            error!("💀 FATAL: FALLO DE PERSISTENCIA DE HALLAZGO: {}", e);
            // Incluso si falla la DB, deberíamos intentar alertar o volcar a log
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: GET /status
/// Responsabilidad: Legacy Polling (Compatibilidad hacia atrás).
pub async fn get_system_status(State(state): State<AppState>) -> Json<Vec<WorkerHeartbeat>> {
    Json(state.get_active_workers())
}

/// Endpoint: POST /panic
/// Responsabilidad: Recepción de cajas negras (Crash Dumps) de los workers.
#[instrument(skip(_state))]
pub async fn receive_panic_alert(
    State(_state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let worker_id = payload
        .get("worker_id")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let msg = payload
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("No message");

    error!("💀 PANIC REPORT [Worker: {}]: {}", worker_id, msg);

    // Aquí podríamos disparar un evento de sistema si tuviéramos un canal de "Alertas de Salud"

    StatusCode::OK
}
