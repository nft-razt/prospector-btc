/**
 * =================================================================
 * APARATO: ROUTING NETWORK MATRIX (V24.0 - DEPLOYMENT HARDENED)
 * CLASIFICACIÓN: API LAYER (L3)
 * RESPONSABILIDAD: GESTIÓN DE ENDPOINTS Y PERÍMETROS DE SEGURIDAD
 *
 * ESTRATEGIA DE ÉLITE:
 * - Liveness Bypass: El endpoint /health/liveness es público para el orquestador de Render.
 * - Semantic Alignment: Vinculación con los handlers de misión nivelados (V8.7).
 * - Zero-Regression: Mapeo estricto de estratos (Swarm, Admin, Lab, Stream).
 * =================================================================
 */

use crate::handlers::{admin, lab, stream, swarm, health}; // 'health' es el aparato V12.0 entregado antes
use crate::middleware::{auth_guard, health_guard};
use crate::state::AppState;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

/**
 * Construye la matriz de rutas definitiva para el ecosistema Prospector.
 *
 * @param application_state: Estado neural compartido.
 * @returns Router configurado para producción.
 */
pub fn create_router(application_state: AppState) -> Router {

    // --- ESTRATO 0: DIAGNÓSTICO PÚBLICO (Sin seguridad para permitir el Bootstrapping) ---
    let public_diagnostics = Router::new()
        .route("/liveness", get(health::perform_liveness_probe));

    // --- ESTRATO 1: SWARM (Nodos de Minería) ---
    let swarm_stratum = Router::new()
        .route("/mission/acquire", post(swarm::handle_mission_acquisition))
        .route("/mission/complete", post(swarm::handle_mission_completion))
        .route("/heartbeat", post(swarm::receive_heartbeat)) // Mantenido para telemetría de hardware
        .layer(middleware::from_fn_with_state(
            application_state.clone(),
            health_guard,
        ))
        .layer(middleware::from_fn(auth_guard));

    // --- ESTRATO 2: LAB (Forensics & QA) ---
    let laboratory_stratum = Router::new()
        .route("/scenarios", post(lab::crystallize_new_scenario))
        .route("/verify", post(lab::verify_entropy_vector))
        .layer(middleware::from_fn(auth_guard));

    // --- ESTRATO 3: ADMIN & COMMAND (Dashboard Control) ---
    let command_stratum = Router::new()
        .route("/identities", get(admin::list_identities))
        .route("/identities/inject", post(admin::upload_identity))
        .route("/status/nodes", get(swarm::get_node_cluster_status))
        .layer(middleware::from_fn(auth_guard));

    // --- ESTRATO 4: STREAM (Neural Link SSE) ---
    let stream_stratum = Router::new()
        .route("/metrics", get(stream::stream_metrics))
        .layer(middleware::from_fn(auth_guard));

    // --- ENSAMBLAJE FINAL DE LA ARQUITECTURA ---
    Router::new()
        .nest("/api/v1/health", public_diagnostics)
        .nest("/api/v1/swarm", swarm_stratum)
        .nest("/api/v1/lab", laboratory_stratum)
        .nest("/api/v1/admin", command_stratum)
        .nest("/api/v1/stream", stream_stratum)
        .with_state(application_state)
        // Fallback para monitoreo simple de uptime
        .route("/health", get(|| async { "SYSTEM_ALIVE" }))
}
