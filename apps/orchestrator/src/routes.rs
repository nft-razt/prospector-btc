// apps/orchestrator/src/routes.rs
// =================================================================
// APARATO: ROUTING MATRIX (v6.2 - LAB INTEGRATED)
// RESPONSABILIDAD: GESTIÓN DE TRÁFICO, SEGURIDAD Y ENRUTAMIENTO
// ESTRATEGIA: SEGREGACIÓN DE TRÁFICO (SWARM vs INGEST vs LAB)
// =================================================================

use axum::{
    error_handling::HandleErrorLayer,
    extract::DefaultBodyLimit,
    http::StatusCode,
    middleware,
    routing::{get, post},
    BoxError, Router,
};
use std::time::Duration;

// Middleware de Torre para Resiliencia (Rate Limiting & Backpressure)
use tower::buffer::BufferLayer;
use tower::limit::{ConcurrencyLimitLayer, RateLimitLayer};
use tower::ServiceBuilder;

// Importaciones del Estado y Lógica de Negocio
use crate::state::AppState;
// ✅ IMPORTANTE: Se añade 'lab' a los handlers expuestos
use crate::handlers::{admin, lab, stream, swarm};
use crate::middleware::{auth_guard, health_guard};

/// Construye el router principal con segregación de tráfico por perfil de carga.
pub fn create_router(state: AppState) -> Router {
    // -------------------------------------------------------------------------
    // 1. ESCUDO SWARM (Alta Frecuencia / Payload Ligero)
    // -------------------------------------------------------------------------
    // Protege contra DDOS accidental de los propios workers.
    // Límite: 100 req/s. Buffer bajo para fallar rápido (Fail-Fast).
    let swarm_shield = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            tracing::warn!("⚠️ Swarm Rate Limit: {}", err);
            (StatusCode::TOO_MANY_REQUESTS, "Swarm Saturation")
        }))
        .layer(BufferLayer::new(1024))
        .layer(RateLimitLayer::new(100, Duration::from_secs(1)));

    // -------------------------------------------------------------------------
    // 2. ESCUDO INGEST (Baja Frecuencia / Payload Pesado)
    // -------------------------------------------------------------------------
    // Para subida de imágenes (Snapshots).
    // Límite: 20 req/s. Concurrencia limitada para proteger RAM.
    let ingest_shield = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            tracing::warn!("⚠️ Ingest Overload: {}", err);
            (StatusCode::TOO_MANY_REQUESTS, "Visual Feed Busy")
        }))
        .layer(BufferLayer::new(50))
        .layer(ConcurrencyLimitLayer::new(10))
        .layer(RateLimitLayer::new(20, Duration::from_secs(1)));

    // =========================================================================
    // DEFINICIÓN DE RUTAS POR ESTRATO
    // =========================================================================

    // --- A. SWARM ROUTES (Tráfico de Minería) ---
    let swarm_routes = Router::new()
        .route("/heartbeat", post(swarm::receive_heartbeat))
        .route("/job/acquire", post(swarm::assign_job))
        .route("/job/keepalive", post(swarm::job_keep_alive))
        .route("/job/complete", post(swarm::complete_job))
        .route("/finding", post(swarm::report_finding)) // 🚨 ALERTA CRÍTICA
        .route("/panic", post(swarm::receive_panic_alert))
        .layer(middleware::from_fn_with_state(state.clone(), health_guard)) // Circuit Breaker Lógico
        .layer(middleware::from_fn(auth_guard)) // Seguridad de Token
        .layer(swarm_shield); // Protección Volumétrica

    // --- B. INGEST ROUTES (Tráfico de Datos/Binarios) ---
    // Aislado para que la subida de imágenes no bloquee los heartbeats.
    let ingest_routes = Router::new()
        .route("/worker-snapshot", post(admin::upload_snapshot))
        .layer(DefaultBodyLimit::max(512 * 1024)) // Límite estricto 512KB por Request
        .layer(middleware::from_fn(auth_guard))
        .layer(ingest_shield);

    // --- C. ADMIN ROUTES (Gestión Operativa) ---
    let admin_routes = Router::new()
        .route("/status", get(swarm::get_system_status))
        .route("/identities", post(admin::upload_identity))
        .route("/identities", get(admin::list_identities))
        .route("/identities/lease", get(admin::lease_identity))
        .route("/identities/revoke", post(admin::revoke_identity))
        .route("/worker-snapshots", get(admin::list_snapshots))
        .layer(middleware::from_fn(auth_guard));

    // --- D. LAB ROUTES (The Crypto Lab & Interceptor) ✅ NUEVO ---
    // Rutas para la creación de escenarios de prueba y verificación manual.
    let lab_routes = Router::new()
        .route("/scenarios", post(lab::create_scenario)) // Crear Golden Ticket
        .route("/scenarios", get(lab::list_scenarios)) // Listar Pruebas
        .route("/verify", post(lab::verify_entropy)) // The Interceptor Tool
        .layer(middleware::from_fn(auth_guard));

    // --- E. STREAM ROUTES (Server-Sent Events) ---
    let stream_routes = Router::new()
        .route("/metrics", get(stream::stream_metrics))
        .layer(middleware::from_fn(auth_guard));

    // =========================================================================
    // ENSAMBLAJE FINAL (FRACTAL COMPOSITION)
    // =========================================================================
    Router::new()
        .nest("/api/v1/swarm", swarm_routes)
        .nest("/api/v1/admin", admin_routes)
        .nest("/api/v1/ingest", ingest_routes)
        .nest("/api/v1/stream", stream_routes)
        .nest("/api/v1/lab", lab_routes) // ✅ Módulo Lab Registrado
        .with_state(state)
        // Healthcheck público para K8s/Render (Sin Auth)
        .route("/health", get(|| async { "OK" }))
}
