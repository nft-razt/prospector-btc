// apps/orchestrator/src/handlers/lab.rs
// =================================================================
// APARATO: CRYPTO LAB HANDLERS (V13.0)
// RESPONSABILIDAD: GESTIÓN DE GOLDEN TICKETS E INTERCEPTOR
// ESTADO: NO ABBREVIATIONS // CONTRACT ALIGNED
// =================================================================

use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse};
use tracing::{info, error, instrument};

use crate::state::AppState;
use prospector_core_gen::{address_legacy::pubkey_to_address, wif::private_to_wif};
use prospector_core_math::prelude::*;
use prospector_domain_strategy::brainwallet::phrase_to_private_key;
use prospector_infra_db::repositories::{ScenarioRepository, TestScenario};

// DTOs locales (Sincronizados con api-contracts)
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateScenarioRequest {
    pub name: String,
    pub secret_phrase: String,
}

#[derive(Deserialize)]
pub struct VerifyEntropyRequest {
    pub secret: String,
}

#[derive(Serialize)]
pub struct VerifyEntropyResponse {
    pub address: String,
    pub wif: String,
    pub is_target: bool,
    pub matched_scenario: Option<String>,
}

/// Endpoint: POST /api/v1/lab/scenarios
pub async fn crystallize_new_scenario(
    State(application_state): State<AppState>,
    Json(payload): Json<CreateScenarioRequest>,
) -> impl IntoResponse {
    let private_key = phrase_to_private_key(&payload.secret_phrase);
    let public_key = SafePublicKey::from_private(&private_key);
    let address = pubkey_to_address(&public_key, false);
    let wif = private_to_wif(&private_key, false);

    let repository = ScenarioRepository::new(application_state.db.clone());

    match repository.create_atomic(&payload.name, &payload.secret_phrase, &address, &wif).await {
        Ok(scenario) => (StatusCode::CREATED, Json(scenario)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

/// Endpoint: GET /api/v1/lab/scenarios
pub async fn list_active_scenarios(
    State(application_state): State<AppState>
) -> impl IntoResponse {
    let repository = ScenarioRepository::new(application_state.db.clone());
    match repository.list_all().await {
        Ok(list) => Json(list).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

/// Endpoint: POST /api/v1/lab/verify (The Interceptor)
pub async fn verify_entropy_vector(
    State(application_state): State<AppState>,
    Json(payload): Json<VerifyEntropyRequest>,
) -> impl IntoResponse {
    let private_key = phrase_to_private_key(&payload.secret);
    let public_key = SafePublicKey::from_private(&private_key);
    let address = pubkey_to_address(&public_key, false);
    let wif = private_to_wif(&private_key, false);

    let repository = ScenarioRepository::new(application_state.db.clone());

    match repository.find_by_address(&address).await {
        Ok(match_option) => {
            Json(VerifyEntropyResponse {
                address,
                wif,
                is_target: match_option.is_some(),
                matched_scenario: match_option.map(|s| s.name),
            }).into_response()
        },
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
