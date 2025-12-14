// apps/orchestrator/src/routes.rs
// =================================================================
// APARATO: ROUTER (TRAFFIC CONTROL CENTER)
// ESTADO: REFACTORIZADO (PANIC ROUTE ADDED)
// =================================================================

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use crate::state::AppState;
use crate::handlers::{admin, swarm};
use crate::middleware::auth_guard;

pub fn create_router(state: AppState) -> Router {

    // 1. ARTERIA SWARM (Tráfico de Minería - Alta Frecuencia)
    let swarm_routes = Router::new()
        .route("/heartbeat", post(swarm::receive_heartbeat))
        .route("/job/acquire", post(swarm::assign_job))
        .route("/job/keepalive", post(swarm::job_keep_alive))
        .route("/job/complete", post(swarm::complete_job))
        .route("/finding", post(swarm::report_finding))

        // 🔥 NUEVO: Telemetría de Pánico
        .route("/panic", post(swarm::receive_panic_alert))

        // Seguridad
        .route_layer(middleware::from_fn(auth_guard));

    // 2. ARTERIA ADMIN (Gestión y Vigilancia)
    let admin_routes = Router::new()
        .route("/status", get(swarm::get_system_status))

        // Identidades
        .route("/identities", post(admin::upload_identity))
        .route("/identities", get(admin::list_identities))
        .route("/identities/lease", get(admin::lease_identity))
        .route("/identities/revoke", post(admin::revoke_identity))

        // Panóptico
        .route("/worker-snapshot", post(admin::upload_snapshot))
        .route("/worker-snapshots", get(admin::list_snapshots))

        // Seguridad
        .route_layer(middleware::from_fn(auth_guard));

    // 3. ENSAMBLAJE FINAL
    Router::new()
        .nest("/api/v1/swarm", swarm_routes)
        .nest("/api/v1/admin", admin_routes)
        .with_state(state)
        .route("/health", get(|| async { "OK" }))
}
