// =================================================================
// APARATO: ADMIN HANDLERS (COMMAND & CONTROL)
// RESPONSABILIDAD: GESTIÓN DE IDENTIDAD Y VIGILANCIA (PANÓPTICO)
// =================================================================

use axum::{extract::{State, Json, Query}, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use tracing::{info, error};
use crate::state::AppState;

// IMPORTACIÓN DE LA VERDAD ÚNICA (MODELOS)
use prospector_domain_models::{
    CreateIdentityPayload,
    Identity,
    WorkerSnapshot // <--- El modelo visual que definimos
};
use prospector_infra_db::repositories::IdentityRepository;

#[derive(Deserialize)]
pub struct LeaseParams {
    pub platform: String,
}

// --- SECCIÓN 1: GESTIÓN DE IDENTIDAD (THE VAULT) ---

/// Carga nuevas credenciales.
pub async fn upload_identity(
    State(state): State<AppState>,
    Json(payload): Json<CreateIdentityPayload>,
) -> impl IntoResponse {
    let repo = IdentityRepository::new(state.db.clone());
    match repo.upsert(&payload).await {
        Ok(_) => {
            info!("🔐 Identidad asegurada en Bóveda: {}", payload.email);
            StatusCode::CREATED
        },
        Err(e) => {
            error!("❌ Error Vault Upsert: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Lista inventario de cuentas.
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

/// Entrega una identidad a un Provisioner (Lease).
pub async fn lease_identity(
    State(state): State<AppState>,
    Query(params): Query<LeaseParams>,
) -> impl IntoResponse {
    let repo = IdentityRepository::new(state.db.clone());
    match repo.lease_active(&params.platform).await {
        Ok(Some(identity)) => {
            info!("🎟️ Lease otorgado a nodo para: {}", identity.email);
            Json(Some(identity)).into_response()
        },
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            error!("❌ Error transaccional Lease: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// --- SECCIÓN 2: EL PANÓPTICO (VIGILANCIA VISUAL) ---

/// Recibe una captura de pantalla del Provisioner (Worker).
/// POST /api/v1/admin/worker-snapshot
pub async fn upload_snapshot(
    State(state): State<AppState>,
    Json(payload): Json<WorkerSnapshot>,
) -> impl IntoResponse {
    // Almacenamos en memoria volátil (RAM) para velocidad extrema.
    // No persiste en DB para no inflar el almacenamiento con imágenes efímeras.
    state.update_snapshot(payload);
    StatusCode::OK
}

/// Entrega todas las capturas activas al Dashboard.
/// GET /api/v1/admin/worker-snapshots
pub async fn list_snapshots(
    State(state): State<AppState>
) -> Json<Vec<WorkerSnapshot>> {
    Json(state.get_snapshots())
}
