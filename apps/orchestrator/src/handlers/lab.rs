// apps/orchestrator/src/handlers/lab.rs
// =================================================================
// APARATO: LAB HANDLERS (V2.2 - LINT FREE)
// RESPONSABILIDAD: GESTIÓN DE EXPERIMENTOS Y VALIDACIÓN DE ENTROPÍA
// ESTADO: OPTIMIZED (DEAD CODE FIXED VIA LOGGING)
// =================================================================

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};

use crate::state::AppState;
use prospector_core_gen::{address_legacy::pubkey_to_address, wif::private_to_wif};
use prospector_core_math::public_key::SafePublicKey;
use prospector_domain_strategy::brainwallet::phrase_to_private_key;
use prospector_infra_db::repositories::ScenarioRepository;

// --- DTOs (Data Transfer Objects) ---

#[derive(Deserialize, Debug)]
pub struct CreateScenarioRequest {
    pub name: String,
    pub secret_phrase: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyRequest {
    pub secret: String,
    #[serde(default = "default_verify_type")]
    pub r#type: String, // Campo ahora activo en telemetría
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

// --- HANDLERS ---

/// Crea un "Golden Ticket" o escenario de prueba en la base de datos.
#[instrument(skip(state, payload))]
pub async fn create_scenario(
    State(state): State<AppState>,
    Json(payload): Json<CreateScenarioRequest>,
) -> impl IntoResponse {
    info!("🧪 LAB: Iniciando cristalización de escenario: '{}'", payload.name);

    // 1. Derivación Criptográfica Determinista (Source of Truth)
    let pk = phrase_to_private_key(&payload.secret_phrase);
    let pubk = SafePublicKey::from_private(&pk);

    // Generamos los artefactos esperados
    let target_address = pubkey_to_address(&pubk, false); // Legacy P2PKH
    let target_wif = private_to_wif(&pk, false);

    // 2. Persistencia Atómica
    let repo = ScenarioRepository::new(state.db.clone());

    match repo
        .create(
            &payload.name,
            &payload.secret_phrase,
            &target_address,
            &target_wif,
        )
        .await
    {
        Ok(scenario) => {
            info!(
                "✅ ESCENARIO CREADO: {} -> {} (ID: {})",
                scenario.name, scenario.target_address, scenario.id
            );
            (StatusCode::CREATED, Json(scenario)).into_response()
        }
        Err(e) => {
            error!("❌ ERROR CRÍTICO AL CREAR ESCENARIO: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Write Failure").into_response()
        }
    }
}

/// Lista todos los escenarios activos e históricos.
#[instrument(skip(state))]
pub async fn list_scenarios(State(state): State<AppState>) -> impl IntoResponse {
    let repo = ScenarioRepository::new(state.db.clone());

    match repo.list_all().await {
        Ok(list) => Json(list).into_response(),
        Err(e) => {
            error!("❌ ERROR LEYENDO ESCENARIOS: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Read Failure").into_response()
        }
    }
}

/// "The Interceptor": Herramienta de validación manual en tiempo real.
#[instrument(skip(state, payload))]
pub async fn verify_entropy(
    State(state): State<AppState>,
    Json(payload): Json<VerifyRequest>,
) -> impl IntoResponse {
    // ✅ CORRECCIÓN: Consumimos el campo 'type' en el log para eliminar el warning de código muerto.
    // Esto también ayuda a depurar qué tipo de entrada está enviando el frontend.
    info!("🔎 INTERCEPTOR: Analizando vector de entrada [Modo: {}]", payload.r#type);

    // 1. Recalculamos la criptografía al vuelo
    let pk = phrase_to_private_key(&payload.secret);
    let pubk = SafePublicKey::from_private(&pk);
    let address = pubkey_to_address(&pubk, false);
    let wif = private_to_wif(&pk, false);

    // 2. Consultamos a la Bóveda si esta dirección es un objetivo
    let repo = ScenarioRepository::new(state.db.clone());

    let match_result = repo.find_by_address(&address).await.unwrap_or_else(|e| {
        error!("⚠️ Error consultando interceptor: {}", e);
        None
    });

    let is_target = match_result.is_some();
    let matched_scenario = match_result.map(|s| s.name);

    if is_target {
        info!("🎯 INTERCEPTOR MATCH CONFIRMADO: {}", address);
    } else {
        info!("💨 INTERCEPTOR MISS: {}", address);
    }

    Json(VerifyResponse {
        address,
        wif,
        is_target,
        matched_scenario,
    })
    .into_response()
}
