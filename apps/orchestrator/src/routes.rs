// =================================================================
// APARATO: ROUTER (TRÁFICO CENTRAL)
// RESPONSABILIDAD: ENRUTAMIENTO DE ARTERIAS SWARM Y ADMIN
// =================================================================

use axum::{
    routing::{get, post},
    Router,
};
use crate::state::AppState;
use crate::handlers::{admin, swarm};

pub fn create_router(state: AppState) -> Router {

    // 1. ARTERIA SWARM (Tráfico de Minería - Alta Frecuencia)
    let swarm_routes = Router::new()
        .route("/heartbeat", post(swarm::receive_heartbeat))
        .route("/job/acquire", post(swarm::assign_job))
        .route("/job/keepalive", post(swarm::job_keep_alive))
        .route("/job/complete", post(swarm::complete_job))
        .route("/finding", post(swarm::report_finding));

    // 2. ARTERIA ADMIN (Gestión y Vigilancia)
    let admin_routes = Router::new()
        // Telemetría Numérica
        .route("/status", get(swarm::get_system_status))

        // The Iron Vault (Identidades)
        .route("/identities", post(admin::upload_identity))
        .route("/identities", get(admin::list_identities))
        .route("/identities/lease", get(admin::lease_identity))

        // El Panóptico (Imágenes en tiempo real)
        .route("/worker-snapshot", post(admin::upload_snapshot)) // Upload
        .route("/worker-snapshots", get(admin::list_snapshots)); // Download

    // 3. ENSAMBLAJE FINAL
    Router::new()
        .nest("/api/v1/swarm", swarm_routes)
        .nest("/api/v1/admin", admin_routes)

        // Rutas Legacy (Compatibilidad hacia atrás si hay workers viejos)
        .route("/api/v1/job", post(swarm::assign_job))
        .route("/api/v1/finding", post(swarm::report_finding))

        .with_state(state)
        .route("/health", get(|| async { "OK" }))
}
