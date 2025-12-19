/**
 * =================================================================
 * APARATO: SWARM HANDLERS (V21.0 - AUDIT LOG ENABLED)
 * CLASIFICACIÓN: API LAYER (L3)
 * RESPONSABILIDAD: GESTIÓN DE TELEMETRÍA DE ESFUERZO
 * =================================================================
 */
use crate::state::AppState;
use ax_extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::{error, info, instrument};

// --- MODELOS DE DOMINIO ---
use prospector_domain_models::{AuditReport, Finding};
use prospector_infra_db::repositories::JobRepository;

/**
 * Endpoint: POST /api/v1/swarm/job/complete
 *
 * Recibe el reporte de auditoría final de un nodo y lo persiste en el Ledger Táctico.
 * El motor Chronos posteriormente migrará estos datos a Supabase (L4).
 */
#[instrument(skip(state, report))]
pub async fn finalize_audit_sequence(
    State(state): State<AppState>,
    Json(report): Json<AuditReport>,
) -> impl IntoResponse {
    let repo = JobRepository::new(state.db.get_connection().unwrap());

    info!(
        "🏁 [AUDIT_COMPLETE]: Worker {} finished Job {}. Effort: {} hashes.",
        report.worker_id, report.job_id, report.total_hashes
    );

    // 1. Persistencia del Esfuerzo Computacional
    match repo
        .finalize_with_metrics(
            &report.job_id,
            &report.total_hashes,
            report.duration_ms,
            &report.exit_status,
        )
        .await
    {
        Ok(_) => {
            // 2. Notificación al Neural Link (Dashboard) vía SSE
            state.events.notify_audit_progress(report);
            StatusCode::OK
        }
        Err(e) => {
            error!("❌ [LEDGER_FAULT]: Failed to persist audit metrics: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
