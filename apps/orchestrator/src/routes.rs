// apps/orchestrator/src/routes.rs
// =================================================================
// APARATO: ROUTING MATRIX (V14.0)
// RESPONSABILIDAD: ORQUESTACIÓN DE ENDPOINTS Y SEGURIDAD
// ESTADO: FULL SYNC // NO ABBREVIATIONS
// =================================================================

use crate::handlers::{admin, lab, stream, swarm};
use crate::middleware::{auth_guard, health_guard};
use crate::state::AppState;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

/// Construye el router principal con la topología de red Hydra-Zero.
pub fn create_router(application_state: AppState) -> Router {
    // --- 1. SWARM ESTRATO (Minería) ---
    let swarm_routes = Router::new()
        .route("/heartbeat", post(swarm::receive_heartbeat))
        .route("/job/acquire", post(swarm::assign_search_range))
        .route("/job/keepalive", post(swarm::extend_search_lease))
        .route("/job/complete", post(swarm::finalize_search_range))
        .route("/finding", post(swarm::register_cryptographic_finding))
        .route("/panic", post(swarm::receive_worker_panic))
        .layer(middleware::from_fn_with_state(
            application_state.clone(),
            health_guard,
        ))
        .layer(middleware::from_fn(auth_guard));

    // --- 2. LAB ESTRATO (QA & Forensics) ---
    let lab_routes = Router::new()
        .route("/scenarios", post(lab::crystallize_new_scenario))
        .route("/scenarios", get(lab::list_active_scenarios))
        .route("/verify", post(lab::verify_entropy_vector))
        .layer(middleware::from_fn(auth_guard));

    // --- 3. ADMIN ESTRATO (Management) ---
    let admin_routes = Router::new()
        .route("/status", get(swarm::get_node_cluster_status))
        .route("/identities", post(admin::upload_identity))
        .route("/identities", get(admin::list_identities))
        .route("/identities/lease", get(admin::lease_identity))
        .route("/identities/revoke", post(admin::revoke_identity))
        .route("/worker-snapshots", get(admin::list_snapshots))
        .layer(middleware::from_fn(auth_guard));

    // --- 4. STREAM ESTRATO (Real-time) ---
    let stream_routes = Router::new()
        .route("/metrics", get(stream::stream_metrics))
        .layer(middleware::from_fn(auth_guard));

    // ENSAMBLAJE DE LA RED
    Router::new()
        .nest("/api/v1/swarm", swarm_routes)
        .nest("/api/v1/admin", admin_routes)
        .nest("/api/v1/stream", stream_routes)
        .nest("/api/v1/lab", lab_routes)
        .with_state(application_state)
        .route("/health", get(|| async { "OK" }))
}
