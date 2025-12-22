/**
 * =================================================================
 * APARATO: VISUAL INTELLIGENCE HANDLER (V52.0 - HYGIENE)
 * =================================================================
 */

use crate::state::AppState;
use axum::{extract::{Json, State, Path}, http::StatusCode, response::IntoResponse};
use prospector_domain_models::worker::WorkerSnapshot;
use tracing::instrument; // ✅ RESOLUCIÓN: 'info' eliminado por ocioso

pub struct VisualIntelligenceHandler;

impl VisualIntelligenceHandler {
    #[instrument(skip(application_state, snapshot_payload))]
    pub async fn handle_snapshot_ingestion(
        State(application_state): State<AppState>,
        Json(snapshot_payload): Json<WorkerSnapshot>,
    ) -> impl IntoResponse {
        let mut frames_guard = application_state.swarm_telemetry.visual_surveillance_frames.write().expect("Lock poisoned");
        frames_guard.insert(snapshot_payload.worker_id.clone(), snapshot_payload.clone());
        StatusCode::ACCEPTED
    }

    pub async fn retrieve_worker_frame(
        State(application_state): State<AppState>,
        Path(worker_identifier): Path<String>,
    ) -> impl IntoResponse {
        let frames_guard = application_state.swarm_telemetry.visual_surveillance_frames.read().expect("Lock poisoned");
        match frames_guard.get(&worker_identifier) {
            Some(snapshot) => (
                [("Content-Type", "image/jpeg")],
                snapshot.snapshot_base64.clone()
            ).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
