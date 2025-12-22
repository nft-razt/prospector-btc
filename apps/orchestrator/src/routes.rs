/**
 * =================================================================
 * APARATO: SOVEREIGN ROUTING MATRIX (V15.0 - TOTAL CONTROL)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L3)
 * RESPONSABILIDAD: ORQUESTACIÓN DE ENDPOINTS Y SEGURIDAD CRIPTOGRÁFICA
 * =================================================================
 */

use crate::handlers::{admin, lab, stream, swarm, assets, visual};
use crate::middleware::{auth_guard, health_guard};
use crate::state::AppState;
use axum::{
    middleware,
    routing::{get, post},
    Router,
    http::{header, Method}
};
use tower_http::cors::{Any, CorsLayer};
use std::time::Duration;

pub fn create_router(application_shared_state: AppState) -> Router {

    let network_security_shield = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(Duration::from_secs(3600));

    // ESTRATO TÁCTICO
    let swarm_operations_stratum = Router::new()
        .route("/mission/acquire", post(swarm::SwarmHandshakeHandler::negotiate_mission_assignment_handshake))
        .route("/heartbeat", post(swarm::SwarmHandshakeHandler::register_worker_heartbeat_signal))
        .route("/finding", post(swarm::SwarmHandshakeHandler::register_cryptographic_collision_finding));

    // ESTRATO DE LABORATORIO
    let laboratory_research_stratum = Router::new()
        .route("/certification/ignite", post(lab::CertificationHandler::handle_certification_ignition))
        .route("/verify", post(lab::CertificationHandler::handle_manual_verification));

    // ESTRATO VISUAL
    let visual_intelligence_stratum = Router::new()
        .route("/snapshot", post(visual::VisualIntelligenceHandler::handle_snapshot_ingestion))
        .route("/frame/:worker_identifier", get(visual::VisualIntelligenceHandler::retrieve_worker_frame));

    // ESTRATO DE ASSETS
    let digital_assets_stratum = Router::new()
        .route("/dna/:strata_identifier/:fragment_filename", get(assets::AssetGatewayHandler::download_shard));

    // ESTRATO DE ADMINISTRACIÓN
    // ✅ RESOLUCIÓN: Nueva ruta /system/mode inyectada para control de pánico
    let administrative_control_stratum = Router::new()
        .route("/identities", get(admin::ScenarioAdministrationHandler::handle_list_scenarios))
        .route("/identities/inject", post(admin::ScenarioAdministrationHandler::handle_template_injection))
        .route("/system/mode", post(admin::ScenarioAdministrationHandler::handle_system_mode_transition));

    // ENSAMBLAJE DE LA API V1
    let api_version_one_hub = Router::new()
        .nest("/swarm", swarm_operations_stratum)
        .nest("/lab", laboratory_research_stratum)
        .nest("/visual", visual_intelligence_stratum)
        .nest("/assets", digital_assets_stratum)
        .nest("/admin", administrative_control_stratum)
        .route("/stream/metrics", get(stream::stream_metrics));

    // COMPOSICIÓN GLOBAL
    Router::new()
        .nest("/api/v1", api_version_one_hub)
        .layer(middleware::from_fn_with_state(application_shared_state.clone(), health_guard))
        .layer(middleware::from_fn(auth_guard))
        .layer(network_security_shield)
        .with_state(application_shared_state)
        .route("/health", get(|| async { "OK" }))
}
