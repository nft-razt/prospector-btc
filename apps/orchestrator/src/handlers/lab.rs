// apps/orchestrator/src/handlers/lab.rs
// =================================================================
// APARATO: LAB HANDLERS
// ESTADO: TYPE-SAFE & EXPLICIT
// =================================================================

use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::{info, error, instrument};
use crate::state::AppState;

use prospector_core_math::public_key::SafePublicKey;
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_gen::wif::private_to_wif;
use prospector_domain_strategy::brainwallet::phrase_to_private_key;

// Importamos el repositorio y el struct de datos
use prospector_infra_db::repositories::{ScenarioRepository, scenarios::TestScenario};

#[derive(Deserialize, Debug)]
pub struct CreateScenarioRequest {
    pub name: String,
    pub secret_phrase: String,
}

#[derive(Serialize, Debug)]
pub struct CreateScenarioResponse {
    pub id: String,
    pub status: String,
    pub derived_address: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyRequest {
    pub secret: String,
    #[serde(default = "default_verify_type")]
    pub r#type: String,
}

fn default_verify_type() -> String {
    "phrase".to_string()
}

#[derive(Serialize, Debug)]
pub struct VerifyResponse {
    pub address: String,
    pub wif: String,
    pub is_target: bool,
    pub matched_scenario: Option<String>,
}

#[instrument(skip(state, payload))]
pub async fn create_scenario(
    State(state): State<AppState>,
    Json(payload): Json<CreateScenarioRequest>,
) -> impl IntoResponse {
    info!("🧪 LAB: Creando escenario '{}'", payload.name);

    let pk = phrase_to_private_key(&payload.secret_phrase);
    let pubk = SafePublicKey::from_private(&pk);
    let target_address = pubkey_to_address(&pubk, false);
    let target_wif = private_to_wif(&pk, false);

    let repo = ScenarioRepository::new(state.db.clone());

    match repo.create(
        &payload.name,
        &payload.secret_phrase,
        &target_address,
        &target_wif
    ).await {
        Ok(id) => {
            (StatusCode::CREATED, Json(CreateScenarioResponse {
                id,
                status: "created".to_string(),
                derived_address: target_address,
            })).into_response()
        },
        Err(e) => {
            error!("❌ Error DB create scenario: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
        }
    }
}

pub async fn list_scenarios(State(state): State<AppState>) -> impl IntoResponse {
    let repo = ScenarioRepository::new(state.db.clone());

    match repo.list_all().await {
        // ✅ CORRECCIÓN: Tipo explícito para eliminar ambigüedad
        Ok(list) => Json::<Vec<TestScenario>>(list).into_response(),
        Err(e) => {
            error!("❌ Error listing scenarios: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
        }
    }
}

#[instrument(skip(state, payload))]
pub async fn verify_entropy(
    State(state): State<AppState>,
    Json(payload): Json<VerifyRequest>,
) -> impl IntoResponse {
    let pk = phrase_to_private_key(&payload.secret);
    let pubk = SafePublicKey::from_private(&pk);
    let address = pubkey_to_address(&pubk, false);
    let wif = private_to_wif(&pk, false);

    let repo = ScenarioRepository::new(state.db.clone());

    let match_result = repo.find_by_address(&address).await.unwrap_or(None);

    let is_target = match_result.is_some();
    let matched_scenario = match_result.map(|s| s.name);

    if is_target {
        info!("🎯 MATCH: {}", address);
    } else {
        info!("💨 MISS: {}", address);
    }

    Json(VerifyResponse {
        address,
        wif,
        is_target,
        matched_scenario,
    }).into_response()
}
