// apps/orchestrator/src/middleware.rs
use crate::state::AppState;
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::warn;

pub async fn health_guard(State(state): State<AppState>, req: Request, next: Next) -> Response {
    if let Err(reason) = state.is_operational() {
        warn!(
            "⛔ Acceso denegado a {} (Mantenimiento: {})",
            req.uri(),
            reason
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": "System Maintenance Mode",
                "reason": reason,
                "retry_after": 60
            })),
        )
            .into_response();
    }
    next.run(req).await
}

pub async fn auth_guard(req: Request, next: Next) -> Result<Response, StatusCode> {
    let secret = std::env::var("WORKER_AUTH_TOKEN").unwrap_or_default();
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    match auth_header {
        Some(h) if h.starts_with("Bearer ") && &h[7..] == secret => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
