use axum::{extract::{State, Json, Query}, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use tracing::{info, error};
use crate::state::AppState;
use prospector_domain_models::{CreateIdentityPayload, Identity};
use prospector_infra_db::repositories::IdentityRepository;

#[derive(Deserialize)]
pub struct LeaseParams {
    pub platform: String,
}

pub async fn upload_identity(
    State(state): State<AppState>,
    Json(payload): Json<CreateIdentityPayload>,
) -> impl IntoResponse {
    let repo = IdentityRepository::new(state.db.clone());
    match repo.upsert(&payload).await {
        Ok(_) => {
            info!("🔐 Identidad recibida: {}", payload.email);
            StatusCode::CREATED
        },
        Err(e) => {
            error!("❌ Error Vault: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn list_identities(State(state): State<AppState>) -> Json<Vec<Identity>> {
    let repo = IdentityRepository::new(state.db.clone());
    match repo.list_all().await {
        Ok(list) => Json(list),
        Err(_) => Json(vec![])
    }
}

pub async fn lease_identity(
    State(state): State<AppState>,
    Query(params): Query<LeaseParams>,
) -> impl IntoResponse {
    let repo = IdentityRepository::new(state.db.clone());
    match repo.lease_active(&params.platform).await {
        Ok(Some(identity)) => Json(Some(identity)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            error!("❌ Error Lease: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
