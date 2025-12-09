use axum::{
    routing::{get, post},
    Router,
};
use crate::state::AppState;
// Importamos desde el módulo unificado, ignorando handlers.rs si existiera
use crate::handlers::{admin, swarm};

pub fn create_router(state: AppState) -> Router {

    // TRÁFICO DE MINEROS (SWARM)
    let swarm_routes = Router::new()
        .route("/heartbeat", post(swarm::receive_heartbeat))
        .route("/job/acquire", post(swarm::assign_job))
        .route("/job/keepalive", post(swarm::job_keep_alive)) // Nuevo
        .route("/job/complete", post(swarm::complete_job))    // Nuevo
        .route("/finding", post(swarm::report_finding));

    // TRÁFICO DE ADMINISTRACIÓN
    let admin_routes = Router::new()
        .route("/status", get(swarm::get_system_status))
        .route("/identities", post(admin::upload_identity))
        .route("/identities", get(admin::list_identities))
        .route("/identities/lease", get(admin::lease_identity));

    Router::new()
        .nest("/api/v1/swarm", swarm_routes)
        .nest("/api/v1/admin", admin_routes)
        // Redirección de compatibilidad para workers v1 que no usan prefijo 'swarm'
        // (Opcional, pero recomendado si ya hay workers desplegados)
        .route("/api/v1/job", post(swarm::assign_job))
        .route("/api/v1/finding", post(swarm::report_finding))
        .with_state(state)
        .route("/health", get(|| async { "OK" }))
}
