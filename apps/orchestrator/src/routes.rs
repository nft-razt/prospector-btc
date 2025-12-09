// apps/orchestrator/src/routes.rs
// =================================================================
// APARATO: ROUTER (TRAFFIC CONTROL)
// RESPONSABILIDAD: ENRUTAMIENTO Y SEGURIDAD PERIMETRAL
// ESTADO: SECURED (AUTH GUARD APPLIED)
// =================================================================

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use crate::state::AppState;
use crate::handlers::{admin, swarm};
use crate::middleware::auth_guard; // Importamos el guardián

pub fn create_router(state: AppState) -> Router {

    // 1. ARTERIA SWARM (Tráfico de Minería - Alta Frecuencia)
    // Protegida por Token de Worker
    let swarm_routes = Router::new()
        .route("/heartbeat", post(swarm::receive_heartbeat))
        .route("/job/acquire", post(swarm::assign_job))
        .route("/job/keepalive", post(swarm::job_keep_alive))
        .route("/job/complete", post(swarm::complete_job))
        .route("/finding", post(swarm::report_finding))
        // 🔥 CAPA DE SEGURIDAD APLICADA
        .route_layer(middleware::from_fn(auth_guard));

    // 2. ARTERIA ADMIN (Gestión y Vigilancia)
    // Protegida por Token de Admin (Mismo secreto por ahora en V3.5)
    let admin_routes = Router::new()
        // Telemetría Numérica
        .route("/status", get(swarm::get_system_status))

        // The Iron Vault (Identidades)
        .route("/identities", post(admin::upload_identity))
        .route("/identities", get(admin::list_identities))
        .route("/identities/lease", get(admin::lease_identity))
        .route("/identities/revoke", post(admin::revoke_identity))

        // El Panóptico (Imágenes en tiempo real)
        .route("/worker-snapshot", post(admin::upload_snapshot)) // Upload
        .route("/worker-snapshots", get(admin::list_snapshots))  // Download
        // 🔥 CAPA DE SEGURIDAD APLICADA
        .route_layer(middleware::from_fn(auth_guard));

    // 3. ENSAMBLAJE FINAL
    Router::new()
        .nest("/api/v1/swarm", swarm_routes)
        .nest("/api/v1/admin", admin_routes)

        .with_state(state)

        // Ruta de Salud PÚBLICA (Sin Auth para Render/Koyeb/UptimeRobot)
        .route("/health", get(|| async { "OK" }))
}
