// apps/orchestrator/src/handlers/swarm.rs
// =================================================================
// APARATO: SWARM TRAFFIC CONTROLLER (V7.0 - LOOP CLOSURE EDITION)
// RESPONSABILIDAD: GESTIÓN DE ALTA FRECUENCIA DE NODOS MINEROS
// CARACTERÍSTICAS:
// - Atomicidad: Manejo robusto de transacciones.
// - Integridad: Tipado estricto en DTOs.
// - Cierre de Ciclo: Verificación automática de escenarios de laboratorio.
// - Observabilidad: Tracing instrumentado para cada operación crítica.
// =================================================================

use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::Value;
use tracing::{error, info, instrument, warn};

// --- DOMINIO (Tipos Estrictos) ---
use prospector_domain_models::{Finding, JobCompletion, WorkerHeartbeat};

// --- INFRAESTRUCTURA (Acceso a Datos) ---
use prospector_infra_db::repositories::{FindingRepository, JobRepository, ScenarioRepository};

/// Macro utilitaria para obtener una conexión DB del pool o fallar rápido.
/// Reduce el ruido visual en los handlers que requieren transacciones manuales.
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

/// Endpoint: POST /api/v1/swarm/heartbeat
///
/// **Frecuencia:** Alta (Cada 30s por nodo).
/// **Responsabilidad:** Actualizar estado en memoria RAM (Volátil) y Buffer de Escritura.
/// No golpea la base de datos directamente para evitar saturación de I/O (patrón Write-Behind).
#[instrument(skip(state), fields(worker = %heartbeat.worker_id))]
pub async fn receive_heartbeat(
    State(state): State<AppState>,
    Json(heartbeat): Json<WorkerHeartbeat>,
) -> impl IntoResponse {
    // Delegamos la lógica de actualización y detección de nuevos nodos al Estado Global.
    // Esto es una operación en memoria extremadamente rápida.
    state.update_worker(heartbeat);
    StatusCode::OK.into_response()
}

/// Endpoint: POST /api/v1/swarm/job/acquire
///
/// **Responsabilidad:** Asignación transaccional de rangos de búsqueda (ACID).
/// Gestiona la concurrencia para asegurar que dos workers nunca reciban el mismo rango.
#[instrument(skip(state))]
pub async fn assign_job(State(state): State<AppState>) -> Response {
    // Obtenemos conexión fresca del pool
    let conn = get_conn_or_500!(state);
    let repo = JobRepository::new(conn);

    // TODO: En V8.0, extraer ID real del token JWT en el middleware y pasarlo aquí.
    let worker_placeholder = "worker-generic-v2";

    match repo.assign_work(worker_placeholder).await {
        Ok(work_order) => {
            info!(
                "💼 JOB ASIGNADO: {} [Strategy: {:?}] -> {}",
                work_order.id, work_order.strategy, worker_placeholder
            );
            Json(work_order).into_response()
        }
        Err(e) => {
            error!("❌ JOB ASSIGNMENT FAILED: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: POST /api/v1/swarm/job/keepalive
///
/// **Responsabilidad:** Evitar que el Reaper marque el trabajo como zombie.
/// Extiende el `last_heartbeat_at` del trabajo en la base de datos.
#[instrument(skip(state))]
pub async fn job_keep_alive(
    State(state): State<AppState>,
    Json(payload): Json<JobCompletion>,
) -> Response {
    let conn = get_conn_or_500!(state);
    let repo = JobRepository::new(conn);

    match repo.heartbeat(&payload.id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            warn!("⚠️ Fallo en Job KeepAlive [{}]: {}", payload.id, e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

/// Endpoint: POST /api/v1/swarm/job/complete
///
/// **Responsabilidad:** Cierre de ciclo de trabajo exitoso (Rango agotado sin hallazgos).
#[instrument(skip(state))]
pub async fn complete_job(
    State(state): State<AppState>,
    Json(payload): Json<JobCompletion>,
) -> Response {
    let conn = get_conn_or_500!(state);
    let repo = JobRepository::new(conn);

    match repo.complete(&payload.id).await {
        Ok(_) => {
            info!("🏁 Job completado y archivado: {}", payload.id);
            StatusCode::OK.into_response()
        }
        Err(e) => {
            error!("❌ JOB COMPLETION ERROR [{}]: {}", payload.id, e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: POST /api/v1/swarm/finding
///
/// **Responsabilidad:** ALERTA CRÍTICA. Persistencia de colisión y Cierre de Experimentos.
/// Este es el punto más importante del sistema.
#[instrument(skip(state))]
pub async fn report_finding(
    State(state): State<AppState>,
    Json(finding): Json<Finding>,
) -> Response {
    // 1. Log Forense Inmediato (Alta Visibilidad)
    warn!(
        "🚨 >>> COLISIÓN CRIPTOGRÁFICA DETECTADA <<< Address: {} | Source: {}",
        finding.address, finding.source_entropy
    );

    // 2. Persistencia en Bóveda (Indestructible)
    let finding_repo = FindingRepository::new(state.db.clone());

    match finding_repo.save(&finding).await {
        Ok(_) => {
            info!("💾 Hallazgo asegurado exitosamente en base de datos.");

            // -----------------------------------------------------------------
            // 3. CIERRE DE CICLO: VERIFICACIÓN DE LABORATORIO (NUEVO)
            // Comprobamos si este hallazgo corresponde a un escenario de prueba.
            // Esto permite que el sistema se "autocertifique".
            // -----------------------------------------------------------------
            let scenario_repo = ScenarioRepository::new(state.db.clone());

            // Spawn de tarea asíncrona para no bloquear la respuesta HTTP al worker
            // aunque en Axum/Tokio esto es rápido, es buena práctica separar efectos secundarios.
            let address_clone = finding.address.clone();
            tokio::spawn(async move {
                match scenario_repo.mark_as_verified(&address_clone).await {
                    Ok(true) => {
                        info!("🧪 ¡EUREKA! El hallazgo verificó un ESCENARIO DE PRUEBA activo.");
                    },
                    Ok(false) => {
                        info!("🦖 WILD CATCH: El hallazgo no corresponde a ningún test conocido. Es un hallazgo real o una colisión aleatoria.");
                    },
                    Err(e) => {
                        error!("⚠️ Error crítico intentando verificar escenario de laboratorio: {}", e);
                    }
                }
            });

            // 4. Activación de Sinapsis (Notificación Tiempo Real vía SSE)
            // Notificamos a todos los clientes del Dashboard conectados.
            state.events.notify_collision(
                "swarm-unit-confirmed".to_string(), // ID genérico o extraído del JWT
                finding.address,
            );

            StatusCode::CREATED.into_response()
        }
        Err(e) => {
            error!("💀 FATAL: FALLO DE PERSISTENCIA DE HALLAZGO: {}. Los datos podrían perderse si el worker se apaga.", e);
            // Incluso si falla la DB, deberíamos intentar alertar por otro canal en el futuro.
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Endpoint: GET /api/v1/admin/status
///
/// **Responsabilidad:** Legacy Polling (Compatibilidad hacia atrás para clientes sin SSE).
/// Devuelve la lista de workers activos desde la memoria RAM.
pub async fn get_system_status(State(state): State<AppState>) -> Json<Vec<WorkerHeartbeat>> {
    Json(state.get_active_workers())
}

/// Endpoint: POST /api/v1/swarm/panic
///
/// **Responsabilidad:** Recepción de cajas negras (Crash Dumps) de los workers.
/// Permite depurar por qué un nodo murió (ej: OOM, Panic de Rust, etc).
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
        .unwrap_or("No message provided");

    error!("💀 PANIC REPORT [Worker: {}]: {}", worker_id, msg);

    // En el futuro, esto podría disparar una notificación a Discord/Slack.
    StatusCode::OK
}
