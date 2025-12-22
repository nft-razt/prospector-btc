/**
 * =================================================================
 * APARATO: SCENARIO ADMINISTRATION HANDLER (V125.0 - TOTAL CONTROL)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE IDENTIDADES Y MANDO OPERATIVO GLOBAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa los puntos de entrada administrativos del sistema.
 * Esta versión integra el control de transición de modo, permitiendo
 * al operador cambiar el estado del sistema (Run/Pause/Stop)
 * consumiendo el método 'transition_mode' del nexo operacional.
 * =================================================================
 */

use crate::state::AppState;
use crate::state::operational_nexus::SwarmOperationalMode;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse
};
use serde::Deserialize;
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose::STANDARD as BASE64_ENGINE, Engine};
use tracing::{info, error, instrument};

// --- SINAPSIS INTERNA: MODELOS Y REPOSITORIOS ---
use prospector_domain_models::scenario::SystemTemplateRegistry;
use prospector_infra_db::repositories::ScenarioRegistryRepository;

/**
 * Contrato de datos para la carga de plantillas desde el Dashboard.
 */
#[derive(Debug, Deserialize)]
pub struct TemplateInjectionPayload {
    pub template_identifier: String,
    pub display_name: String,
    pub binary_content_base64: String,
    pub environment_category: String,
}

/**
 * Contrato de datos para la transición del modo operativo.
 */
#[derive(Debug, Deserialize)]
pub struct SystemModeTransitionPayload {
    pub target_mode: SwarmOperationalMode,
}

pub struct ScenarioAdministrationHandler;

impl ScenarioAdministrationHandler {
    /**
     * Endpoint: POST /api/v1/admin/system/mode
     * Ejecuta el cambio de estado operativo del enjambre.
     *
     * ✅ RESOLUCIÓN: Consume 'transition_mode' para alcanzar Zero-Warnings.
     */
    #[instrument(skip(application_state, payload))]
    pub async fn handle_system_mode_transition(
        State(application_state): State<AppState>,
        Json(payload): Json<SystemModeTransitionPayload>,
    ) -> impl IntoResponse {
        let desired_mode = payload.target_mode;

        info!("🕹️ [SYSTEM_CONTROL]: Requesting transition to mode: {:?}", desired_mode);

        // Ejecución de la transición en el átomo de estado soberano
        application_state.operational_nexus.transition_mode(desired_mode);

        info!("✅ [SYSTEM_CONTROL]: Transition successful. Swarm now in {:?}", desired_mode);

        StatusCode::OK
    }

    /**
     * Endpoint: POST /api/v1/admin/identities/inject
     * Realiza la ingesta y sellado de una nueva plantilla forense.
     */
    #[instrument(skip(application_state, payload))]
    pub async fn handle_template_injection(
        State(application_state): State<AppState>,
        Json(payload): Json<TemplateInjectionPayload>,
    ) -> impl IntoResponse {
        info!("🧬 [INGESTION]: Initiating injection for: {}", payload.template_identifier);

        let binary_data = match BASE64_ENGINE.decode(&payload.binary_content_base64) {
            Ok(bytes) => bytes,
            Err(decoding_error) => {
                error!("❌ [DECODE_ERROR]: Invalid Base64: {}", decoding_error);
                return (StatusCode::BAD_REQUEST, "Invalid binary encoding").into_response();
            }
        };

        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&binary_data);
        let integrity_hash = format!("{:x}", sha256_hasher.finalize());

        let template_metadata = SystemTemplateRegistry {
            template_identifier: payload.template_identifier.clone(),
            display_name: payload.display_name,
            binary_integrity_hash: integrity_hash,
            buffer_size_bytes: binary_data.len() as u32,
            environment_category: payload.environment_category,
            captured_at_timestamp: chrono::Utc::now().to_rfc3339(),
        };

        let repository = ScenarioRegistryRepository::new(application_state.database_client.clone());

        match repository.persist_forensic_template(
            &template_metadata.template_identifier,
            &template_metadata.display_name,
            binary_data
        ).await {
            Ok(_) => {
                info!("✅ [INGESTION_SUCCESS]: DNA secured.");
                (StatusCode::CREATED, Json(template_metadata)).into_response()
            },
            Err(database_error) => {
                error!("❌ [DATABASE_FAULT]: {}", database_error);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /**
     * Endpoint: GET /api/v1/admin/identities
     */
    #[instrument(skip(application_state))]
    pub async fn handle_list_scenarios(
        State(application_state): State<AppState>,
    ) -> impl IntoResponse {
        let repository = ScenarioRegistryRepository::new(application_state.database_client.clone());

        match repository.fetch_all_registered_metadata().await {
            Ok(scenarios) => Json::<Vec<SystemTemplateRegistry>>(scenarios).into_response(),
            Err(database_error) => {
                error!("❌ [DATABASE_FAULT]: Inventory retrieval failed: {}", database_error);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
