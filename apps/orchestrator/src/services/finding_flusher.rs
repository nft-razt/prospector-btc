/**
 * =================================================================
 * APARATO: FINDING FLUSHER SERVICE (V100.0)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: VOLCADO ASÍNCRONO DE HALLAZGOS A TURSO
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::finding::FindingRepository;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, debug, error};

/// Configuración de ráfagas de persistencia.
const FLUSH_INTERVAL_SECONDS: u64 = 5;

pub struct FindingFlusherService {
    application_state: AppState,
}

impl FindingFlusherService {
    pub fn new(application_state: AppState) -> Self {
        Self { application_state }
    }

    /**
     * Inicia el daemon de volcado táctico.
     */
    pub async fn spawn_flusher_daemon(self) {
        let mut timer = interval(Duration::from_secs(FLUSH_INTERVAL_SECONDS));
        info!("💾 [FINDING_FLUSHER]: Archival heart active. Every {}s", FLUSH_INTERVAL_SECONDS);

        loop {
            timer.tick().await;

            // 1. DRENAJE ATÓMICO (LOCK-FREE INTENT)
            let pending_batch = self.application_state.finding_vault.drain_vault_for_flush();

            if pending_batch.is_empty() {
                continue;
            }

            // 2. PERSISTENCIA EN MOTOR A
            let repository = FindingRepository::new(self.application_state.database_client.clone());
            match repository.batch_persist_findings(pending_batch).await {
                Ok(count) => debug!("✅ [FLUSH_SUCCESS]: Secured {} findings in Turso.", count),
                Err(error) => error!("❌ [FLUSH_CRITICAL_FAULT]: Persistence failure! Hallazgos perdidos o retenidos: {}", error),
            }
        }
    }
}
