/**
 * =================================================================
 * APARATO: CHRONOS ARCHIVAL BRIDGE (V20.0 - STRATEGIC SINC)
 * CLASIFICACIÓN: BACKGROUND SERVICE (L1-APP)
 * RESPONSABILIDAD: PERSISTENCIA PERMANENTE DE LA TESIS DOCTORAL
 *
 * ESTRATEGIA DE ÉLITE:
 * - Idempotent Migration: Evita la duplicidad de registros en Supabase.
 * - Fault Isolation: El fallo del archivo no detiene la minería táctica.
 * - Bulk Transmission: Optimizado para reducir latencia de red.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::ArchivalRepository;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, error, debug};
use reqwest::Client;

/// Frecuencia de sincronización estratégica (10 minutos).
const ARCHIVAL_SYNC_INTERVAL_SEC: u64 = 600;

pub async fn spawn_strategic_archival_service(application_state: AppState) {
    let mut synchronization_ticker = interval(Duration::from_secs(ARCHIVAL_SYNC_INTERVAL_SEC));
    let networking_client = Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .unwrap_or_default();

    // Adquisición de secretos estratégicos (Engine B)
    let supabase_url = std::env::var("SUPABASE_URL").unwrap_or_default();
    let supabase_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default();

    if supabase_url.is_empty() {
        error!("🛑 [CHRONOS_ARCHIVE]: Supabase credentials missing. Archival bridge suspended.");
        return;
    }

    let strategic_endpoint = format!("{}/rest/v1/archived_jobs", supabase_url);

    tokio::spawn(async move {
        info!("🏛️  [CHRONOS_ARCHIVE]: Strategic Archival Service operational.");

        loop {
            synchronization_ticker.tick().await;

            let archival_repository = ArchivalRepository::new(application_state.db.clone());

            // 1. EXTRACCIÓN DE REPORTES CERTIFICADOS (L3 -> RAM)
            match archival_repository.get_pending_migration(100).await {
                Ok(migration_batch) if !migration_batch.is_empty() => {
                    info!("📤 [ARCHIVAL]: Transmitting {} certified reports to Engine B...", migration_batch.len());

                    // 2. TRANSMISIÓN ESTRATÉGICA (L4 Uplink)
                    let transmission_response = networking_client.post(&strategic_endpoint)
                        .header("apikey", &supabase_key)
                        .header("Authorization", format!("Bearer {}", supabase_key))
                        .header("Content-Type", "application/json")
                        .json(&migration_batch)
                        .send()
                        .await;

                    match transmission_response {
                        Ok(response) if response.status().is_success() => {
                            // 3. SELLADO INMUTABLE EN LEDGER TÁCTICO
                            let identifiers: Vec<String> = migration_batch.iter()
                                .map(|val| val["original_job_id"].as_str().unwrap_or_default().to_string())
                                .collect();

                            if let Err(err) = archival_repository.mark_as_archived(identifiers).await {
                                error!("❌ [ARCHIVAL_FAULT]: Could not seal tactical records: {}", err);
                            } else {
                                info!("✅ [ARCHIVAL_SUCCESS]: Strategic Ledger is in sync.");
                            }
                        },
                        Ok(response) => error!("❌ [ARCHIVAL_REJECTED]: Supabase Status {}", response.status()),
                        Err(e) => error!("❌ [ARCHIVAL_NETWORK_FAULT]: {}", e),
                    }
                },
                Ok(_) => debug!("🏛️ [CHRONOS_ARCHIVE]: No pending missions for archival."),
                Err(e) => error!("❌ [ARCHIVAL_READ_FAULT]: {}", e),
            }
        }
    });
}
