// apps/orchestrator/src/handlers/admin.rs
// =================================================================
// APARATO: ADMIN HANDLERS (v6.0 - NEURAL LINK)
// RESPONSABILIDAD: GESTIÓN Y VIGILANCIA EN TIEMPO REAL
// =================================================================

use crate::state::AppState;
use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use tracing::{error, info, warn};

use prospector_domain_models::{
    CreateIdentityPayload, Identity, RevokeIdentityPayload, WorkerSnapshot,
};
use prospector_infra_db::repositories::IdentityRepository;

#[derive(Deserialize)]
pub struct LeaseParams {
    pub platform: String,
}

// --- SECCIÓN 1: GESTIÓN DE IDENTIDAD ---

pub async fn upload_identity(
    State(state): State<AppState>,
    Json(payload): Json<CreateIdentityPayload>,
) -> impl IntoResponse {
    let repo = IdentityRepository::new(state.db.clone());
    match repo.upsert(&payload).await {
        Ok(_) => {
            info!("🔐 Identidad asegurada en Bóveda: {}", payload.email);
            StatusCode::CREATED
        }
        Err(e) => {
            error!("❌ Error Vault Upsert: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn revoke_identity(
    State(state): State<AppState>,
    Json(payload): Json<RevokeIdentityPayload>,
) -> impl IntoResponse {
    let repo = IdentityRepository::new(state.db.clone());
    warn!("💀 KILL SWITCH ACTIVADO para identidad: {}", payload.email);

    match repo.revoke(&payload.email).await {
        Ok(_) => {
            info!("⚰️ Identidad revocada exitosamente.");
            StatusCode::OK
        }
        Err(e) => {
            error!("❌ Error revocando identidad: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn list_identities(State(state): State<AppState>) -> Json<Vec<Identity>> {
    let repo = IdentityRepository::new(state.db.clone());
    match repo.list_all().await {
        Ok(list) => Json(list),
        Err(e) => {
            error!("❌ Error listando identidades: {}", e);
            Json(vec![])
        }
    }
}

pub async fn lease_identity(
    State(state): State<AppState>,
    Query(params): Query<LeaseParams>,
) -> impl IntoResponse {
    let repo = IdentityRepository::new(state.db.clone());
    match repo.lease_active(&params.platform).await {
        Ok(Some(identity)) => {
            info!("🎟️ Lease otorgado a nodo para: {}", identity.email);
            Json(Some(identity)).into_response()
        }
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            error!("❌ Error transaccional Lease: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// --- SECCIÓN 2: EL PANÓPTICO (VIGILANCIA VISUAL) ---

/// Recibe una captura del Provisioner, actualiza memoria y emite SSE.
pub async fn upload_snapshot(
    State(state): State<AppState>,
    Json(payload): Json<WorkerSnapshot>,
) -> impl IntoResponse {
    // 1. Actualizar Memoria RAM (Último estado conocido)
    state.update_snapshot(payload.clone());

    // 2. Emitir al Neural Link (Streaming Real-Time)
    state.events.notify_snapshot(payload);

    StatusCode::OK
}

/// Entrega todas las capturas activas (Snapshot inicial para el Dashboard).
pub async fn list_snapshots(State(state): State<AppState>) -> Json<Vec<WorkerSnapshot>> {
    Json(state.get_snapshots())
}
