/**
 * =================================================================
 * APARATO: SCENARIO ADMINISTRATION HANDLER (V115.0 - SOBERANO)
 * CLASIFICACIÓN: ADMIN LAYER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN SOBERANA DEL CATÁLOGO DE ENTROPÍA
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa los puntos de entrada administrativos para la expansión
 * de la Tesis Doctoral. Permite la carga de plantillas binarias y
 * la validación de integridad mediante sumas de verificación SHA-256.
 * =================================================================
 */

use crate::state::AppState;
use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use tracing::{info, error, instrument};

// --- SINAPSIS INTERNA: MODELOS Y REPOSITORIOS ---
use prospector_domain_models::scenario::SystemTemplateRegistry;
use prospector_infra_db::repositories::scenario_repository::ScenarioRegistryRepository;

/// DTO para la carga de plantillas desde el Dashboard.
#[derive(Deserialize)]
pub struct TemplateInjectionPayload {
    pub template_identifier: String,
    pub display_name: String,
    /// Los 250KB codificados en Base64 para transferencia JSON segura.
    pub binary_content_base64: String,
    pub environment_category: String,
}

pub struct ScenarioAdministrationHandler;

impl ScenarioAdministrationHandler {
    /**
     * Endpoint: POST /api/v1/admin/scenarios/inject
     * Realiza la ingesta, decodificación y persistencia de una plantilla XP en Turso.
     */
    #[instrument(skip(application_state, payload))]
    pub async fn handle_template_injection(
        State(application_state): State<AppState>,
        Json(payload): Json<TemplateInjectionPayload>,
    ) -> impl IntoResponse {
        info!("🧬 [INGESTION]: Initiating injection sequence for template: {}", payload.template_identifier);

        // 1. DECODIFICACIÓN Y VALIDACIÓN DE PAYLOAD
        let binary_data = match BASE64.decode(&payload.binary_content_base64) {
            Ok(bytes) => bytes,
            Err(error) => {
                error!("❌ [DECODE_ERROR]: Invalid Base64 payload: {}", error);
                return (StatusCode::BAD_REQUEST, "Invalid binary encoding").into_response();
            }
        };

        // 2. CÁLCULO DE HUELLA DE INTEGRIDAD (SHA-256)
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&binary_data);
        let integrity_hash = format!("{:x}", sha256_hasher.finalize());

        // 3. CONSTRUCCIÓN DEL REGISTRO SOBERANO
        let template_metadata = SystemTemplateRegistry {
            template_identifier: payload.template_identifier,
            display_name: payload.display_name,
            binary_integrity_hash: integrity_hash,
            buffer_size_bytes: binary_data.len() as u32,
            environment_category: payload.environment_category,
            captured_at_timestamp: chrono::Utc::now().to_rfc3339(),
        };

        // 4. PERSISTENCIA ACÍDICA EN TURSO
        let database_connection = match application_state.database_client.get_connection() {
            Ok(connection) => connection,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let repository = ScenarioRegistryRepository::new(database_connection);

        match repository.persist_master_template(&template_metadata, binary_data).await {
            Ok(_) => {
                info!("✅ [INGESTION_SUCCESS]: Scenario {} DNA secured.", template_metadata.template_identifier);
                (StatusCode::CREATED, Json(template_metadata)).into_response()
            },
            Err(error) => {
                error!("❌ [DATABASE_FAULT]: Failed to persist template: {}", error);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /**
     * Endpoint: GET /api/v1/admin/scenarios
     * Lista todos los escenarios registrados en la Bóveda Genética.
     */
    pub async fn handle_list_scenarios(
        State(application_state): State<AppState>,
    ) -> impl IntoResponse {
        let database_connection = match application_state.database_client.get_connection() {
            Ok(connection) => connection,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let repository = ScenarioRegistryRepository::new(database_connection);

        match repository.list_all_metadata().await {
            Ok(scenarios) => Json(scenarios).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
