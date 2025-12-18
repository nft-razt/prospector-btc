// apps/orchestrator/src/services/chronos_archive.rs
/**
 * =================================================================
 * APARATO: CHRONOS ARCHIVAL BRIDGE (V15.0 - DYNAMIC STRATEGY)
 * CLASIFICACIÓN: BACKGROUND SERVICE (L1 - APP)
 * RESPONSABILIDAD: MIGRACIÓN TÁCTICA -> ESTRATÉGICA (TURSO -> SUPABASE)
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::ArchivalRepository;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, error, warn, debug};
use reqwest::Client;

/// Intervalo de sincronización (10 minutos para optimizar cuotas API).
const ARCHIVAL_SYNC_INTERVAL_SECONDS: u64 = 600;

pub async fn spawn_archival_bridge(application_state: AppState) {
    let mut sync_ticker = interval(Duration::from_secs(ARCHIVAL_SYNC_INTERVAL_SECONDS));
    let networking_client = Client::new();

    // Adquisición de credenciales estratégicas inyectadas por Render/Koyeb.
    let supabase_endpoint = std::env::var("SUPABASE_URL").unwrap_or_default();
    let supabase_service_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default();

    if supabase_endpoint.is_empty() || supabase_service_key.is_empty() {
        warn!("⚠️  [CHRONOS_ARCHIVE]: Missing strategic credentials. Archival bridge suspended.");
        return;
    }

    let strategic_url = format!("{}/rest/v1/archived_jobs", supabase_endpoint);

    tokio::spawn(async move {
        info!("🏛️  [CHRONOS_ARCHIVE]: Strategic bridge active. Monitoring Tactical Ledger.");

        loop {
            sync_ticker.tick().await;

            let repository = ArchivalRepository::new(application_state.db.clone());

            // 1. DRENAJE DE REGISTROS COMPLETADOS
            match repository.get_pending_migration(50).await {
                Ok(migration_batch) if !migration_batch.is_empty() => {
                    info!("📤 [ARCHIVAL]: Transmitting {} finalized jobs to cold storage...", migration_batch.len());

                    // 2. TRANSMISIÓN ESTRATÉGICA (UPLINK)
                    let transmission_result = networking_client.post(&strategic_url)
                        .header("apikey", &supabase_service_key)
                        .header("Authorization", format!("Bearer {}", supabase_service_key))
                        .header("Content-Type", "application/json")
                        .header("Prefer", "return=minimal")
                        .json(&migration_batch)
                        .send()
                        .await;

                    match transmission_result {
                        Ok(response) if response.status().is_success() => {
                            // 3. SELLO ATÓMICO EN TURSO
                            // Recuperamos los identificadores originales para marcarlos como archivados.
                            let job_identifiers: Vec<String> = migration_batch.iter()
                                .map(|v| v["original_job_id"].as_str().unwrap_or_default().to_string())
                                .collect();

                            if let Err(error) = repository.mark_as_archived(job_identifiers).await {
                                error!("❌ [ARCHIVAL_FINALIZATION_FAULT]: {}", error);
                            } else {
                                info!("✅ [ARCHIVAL_SUCCESS]: Tactical records synchronized and sealed.");
                            }
                        }
                        Ok(response) => {
                            error!("❌ [STRATEGIC_REJECTION]: Supabase returned status {}", response.status());
                        }
                        Err(error) => {
                            error!("❌ [STRATEGIC_UPLINK_SEVERED]: Connection failure: {}", error);
                        }
                    }
                }
                Ok(_) => debug!("🏛️  [CHRONOS_ARCHIVE]: Tactical ledger is synchronized. No pending migrations."),
                Err(error) => error!("❌ [TACTICAL_READ_FAULT]: Failed to scan jobs table: {}", error),
            }
        }
    });
}
