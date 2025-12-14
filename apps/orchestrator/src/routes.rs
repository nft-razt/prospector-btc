// apps/orchestrator/src/routes.rs
// =================================================================
// APARATO: ROUTER (TRAFFIC CONTROL CENTER v4.5)
// MEJORA: RATE LIMITING & CONGESTION CONTROL
// =================================================================

use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    middleware,
    routing::{get, post},
    BoxError, Router,
};
use std::time::Duration;
use tower::{BufferLayer, LimitLayer, ServiceBuilder};
use tower::limit::RateLimitLayer;

use crate::state::AppState;
use crate::handlers::{admin, swarm};
use crate::middleware::auth_guard;

pub fn create_router(state: AppState) -> Router {

    // --- PROTECCIÓN DE TRÁFICO (DOS SHIELD) ---
    // Capa de servicios para la arteria SWARM (Alta Concurrencia)
    // 1. Manejo de errores: Si excedemos el límite, devolvemos 429 Too Many Requests.
    // 2. Buffer: Cola de espera pequeña para picos momentáneos.
    // 3. RateLimit: Máximo 50 peticiones por segundo por instancia.
    //    (300 workers / 50 req/s = ~6 segundos para procesar un latido masivo, aceptable).
    let swarm_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            tracing::warn!("⚠️ Rate Limit Excedido: {}", err);
            (StatusCode::TOO_MANY_REQUESTS, "Slow down, swarm.")
        }))
        .layer(BufferLayer::new(1024))
        .layer(RateLimitLayer::new(50, Duration::from_secs(1)));

    // 1. ARTERIA SWARM (Tráfico de Minería)
    // Se aplica Rate Limit + Auth Guard
    let swarm_routes = Router::new()
        .route("/heartbeat", post(swarm::receive_heartbeat))
        .route("/job/acquire", post(swarm::assign_job))
        .route("/job/keepalive", post(swarm::job_keep_alive))
        .route("/job/complete", post(swarm::complete_job))
        .route("/finding", post(swarm::report_finding))
        .route("/panic", post(swarm::receive_panic_alert))
        // Orden de capas: RateLimit (Externo) -> Auth (Interno)
        // Primero limitamos tráfico basura, luego verificamos credenciales.
        .layer(middleware::from_fn(auth_guard))
        .layer(swarm_layer);

    // 2. ARTERIA ADMIN (Gestión y Vigilancia)
    // Sin rate limit estricto (uso humano), solo Auth.
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
        .layer(middleware::from_fn(auth_guard));

    // 3. ENSAMBLAJE FINAL
    Router::new()
        .nest("/api/v1/swarm", swarm_routes)
        .nest("/api/v1/admin", admin_routes)
        .with_state(state)
        // El healthcheck debe ser PÚBLICO y SIN LIMITACIONES excesivas (Load Balancer)
        .route("/health", get(|| async { "OK" }))
}
