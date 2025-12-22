/**
 * =================================================================
 * APARATO: SOVEREIGN ROUTING MATRIX (V15.5 - TOTAL SINAPSIS)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L3)
 * RESPONSABILIDAD: ORQUESTACIÓN DE ENDPOINTS Y SEGURIDAD PERIMETRAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la topología definitiva de la API del Orquestador.
 * Esta versión sincroniza:
 * 1. Swarm Operations: Negociación de misiones y pulso de hardware.
 * 2. Laboratory Research: Certificación de Tesis y Auditoría Real (33 Vectores).
 * 3. Visual Intelligence: Panóptico de vigilancia visual del enjambre.
 * 4. Digital Assets: Distribución paralela de fragmentos del censo.
 * 5. Administrative Control: Mando de modo global y Bóveda de Identidad.
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

/**
 * Punto de ignición para la configuración de rutas del sistema.
 *
 * # Protocolo de Seguridad
 * Aplica una capa de 'Health Guard' para prevenir la ejecución en modo degradado
 * y un 'Auth Guard' para asegurar que solo los agentes certificados Hydra-Zero
 * accedan al material de la misión.
 *
 * @param application_shared_state Referencia al estado neural atomizado.
 * @returns Instancia de Router configurada y blindada.
 */
pub fn create_router(application_shared_state: AppState) -> Router {

    // 1. ESCUDO DE SEGURIDAD PERIMETRAL (CORS)
    // Permite la sinapsis fluida entre el Dashboard (Vercel) y el Orquestador (Render).
    let network_security_shield = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(Duration::from_secs(3600));

    // 2. ESTRATO TÁCTICO: Gestión del Enjambre (Swarm)
    // Encargado de la asignación O(1) desde RAM y la recepción de telemetría.
    let swarm_operations_stratum = Router::new()
        .route("/mission/acquire", post(swarm::SwarmHandshakeHandler::negotiate_mission_assignment_handshake))
        .route("/heartbeat", post(swarm::SwarmHandshakeHandler::register_worker_heartbeat_signal))
        .route("/finding", post(swarm::SwarmHandshakeHandler::register_cryptographic_collision_finding));

    // 3. ESTRATO DE LABORATORIO: Investigación Forense y Certificación
    // Incluye el nuevo motor de auditoría real para los 33 vectores Brainwallet.
    let laboratory_research_stratum = Router::new()
        .route("/certification/ignite", post(lab::CertificationHandler::handle_certification_ignition))
        .route("/verify", post(lab::CertificationHandler::handle_manual_verification))
        // ✅ NIVELACIÓN: Endpoint de auditoría real para el dataset de los 33
        .route("/audit/brainwallet-dataset", get(lab::CertificationHandler::handle_brainwallet_dataset_audit));

    // 4. ESTRATO VISUAL: Panóptico de Vigilancia Visual
    // Túnel de ingesta y visualización de frames de video de los workers.
    let visual_intelligence_stratum = Router::new()
        .route("/snapshot", post(visual::VisualIntelligenceHandler::handle_snapshot_ingestion))
        .route("/frame/:worker_identifier", get(visual::VisualIntelligenceHandler::retrieve_worker_frame));

    // 5. ESTRATO DE ACTIVOS: Distribución de ADN Sistémico
    // Sirve los fragmentos binarios del censo UTXO para la hidratación del worker.
    let digital_assets_stratum = Router::new()
        .route("/dna/:strata_identifier/:fragment_filename", get(assets::AssetGatewayHandler::download_shard));

    // 6. ESTRATO DE ADMINISTRACIÓN: Control Maestro y Bóveda ZK
    // Permite la gestión de identidades y la transición del modo operativo (Pánico/Halt).
    let administrative_control_stratum = Router::new()
        .route("/identities", get(admin::ScenarioAdministrationHandler::handle_list_scenarios))
        .route("/identities/inject", post(admin::ScenarioAdministrationHandler::handle_template_injection))
        // ✅ NIVELACIÓN: Endpoint de control de modo global (Run/Pause/Stop)
        .route("/system/mode", post(admin::ScenarioAdministrationHandler::handle_system_mode_transition));

    // 7. ENSAMBLAJE DE LA API V1 (Neural Link)
    // Consolida todos los estratos bajo un prefijo común y activa el flujo SSE.
    let api_version_one_hub = Router::new()
        .nest("/swarm", swarm_operations_stratum)
        .nest("/lab", laboratory_research_stratum)
        .nest("/visual", visual_intelligence_stratum)
        .nest("/assets", digital_assets_stratum)
        .nest("/admin", administrative_control_stratum)
        .route("/stream/metrics", get(stream::stream_metrics));

    // 8. COMPOSICIÓN GLOBAL Y APLICACIÓN DE CAPAS DE SEGURIDAD
    // Nota: El health_guard es la primera línea de defensa antes de la autenticación.
    Router::new()
        .nest("/api/v1", api_version_one_hub)
        .layer(middleware::from_fn_with_state(application_shared_state.clone(), health_guard))
        .layer(middleware::from_fn(auth_guard))
        .layer(network_security_shield)
        .with_state(application_shared_state)
        .route("/health", get(|| async { "OK" }))
}
