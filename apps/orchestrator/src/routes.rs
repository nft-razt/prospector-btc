// apps/orchestrator/src/routes.rs
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
use crate::middleware::{auth_guard, health_guard};

pub fn create_router(state: AppState) -> Router {
    let swarm_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            (StatusCode::TOO_MANY_REQUESTS, "Rate Limit Exceeded")
        }))
        .layer(BufferLayer::new(1024))
        .layer(RateLimitLayer::new(50, Duration::from_secs(1)));

    let swarm_routes = Router::new()
        .route("/heartbeat", post(swarm::receive_heartbeat))
        .route("/job/acquire", post(swarm::assign_job))
        .route("/job/keepalive", post(swarm::job_keep_alive))
        .route("/job/complete", post(swarm::complete_job))
        .route("/finding", post(swarm::report_finding))
        .route("/panic", post(swarm::receive_panic_alert))
        .layer(middleware::from_fn_with_state(state.clone(), health_guard)) // 🛡️
        .layer(middleware::from_fn(auth_guard))
        .layer(swarm_layer);

    let admin_routes = Router::new()
        .route("/status", get(swarm::get_system_status))
        .route("/identities", post(admin::upload_identity))
        .route("/identities", get(admin::list_identities))
        .route("/identities/lease", get(admin::lease_identity))
        .route("/identities/revoke", post(admin::revoke_identity))
        .route("/worker-snapshot", post(admin::upload_snapshot))
        .route("/worker-snapshots", get(admin::list_snapshots))
        .layer(middleware::from_fn(auth_guard));

    Router::new()
        .nest("/api/v1/swarm", swarm_routes)
        .nest("/api/v1/admin", admin_routes)
        .with_state(state)
        .route("/health", get(|| async { "OK" }))
}
