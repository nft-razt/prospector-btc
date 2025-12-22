/**
 * =================================================================
 * APARATO: ARCHIVAL PARITY AUDITOR (V100.0 - SOBERANO)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: CERTIFICACIÓN DE CONSISTENCIA MULTI-CLOUD
 *
 * VISION HIPER-HOLÍSTICA:
 * Ejecuta un protocolo de 'Double-Entry Verification'. Compara el
 * conteo de misiones en Turso (Motor A) con los registros en
 * Supabase (Motor B). Si se detecta un desfase (drift), notifica
 * al Neural Link para alertar al operador sobre un posible
 * fallo de integridad en el archivo de la Tesis.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::AuditRepository;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, warn, error};
use reqwest::Client;

const AUDIT_CYCLE_SECONDS: u64 = 3600; // Escrutinio cada hora

pub struct ArchivalParityAuditor {
    application_state: AppState,
    network_client: Client,
}

impl ArchivalParityAuditor {
    pub fn new(application_state: AppState) -> Self {
        Self {
            application_state,
            network_client: Client::new(),
        }
    }

    pub async fn spawn_auditor_daemon(self) {
        let mut timer = interval(Duration::from_secs(AUDIT_CYCLE_SECONDS));
        info!("⚖️  [PARITY_AUDITOR]: Strategic Consistency Service active.");

        loop {
            timer.tick().await;
            if let Err(e) = self.perform_parity_check().await {
                error!("❌ [PARITY_FAULT]: Consistency scan collapsed: {}", e);
            }
        }
    }

    async fn perform_parity_check(&self) -> anyhow::Result<()> {
        let audit_repo = AuditRepository::new(self.application_state.database_client.clone());

        // 1. ADQUISICIÓN DE CONTEO TÁCTICO (Motor A)
        let tactical_count = audit_repo.get_certified_missions_count().await?;

        // 2. ADQUISICIÓN DE CONTEO ESTRATÉGICO (Motor B)
        let strategic_count = self.fetch_strategic_count().await?;

        // 3. ANÁLISIS DE DERIVA (Drift Analysis)
        let drift_gap = tactical_count.saturating_sub(strategic_count);

        if drift_gap > 0 {
            warn!("🚨 [SYNC_DRIFT_DETECTED]: Motor B is lagging by {} missions.", drift_gap);
            self.application_state.event_bus.notify_archival_drift(drift_gap, tactical_count);
        } else {
            info!("✅ [PARITY_OK]: Multi-cloud consistency verified. Total: {} missions.", tactical_count);
        }

        Ok(())
    }

    /**
     * Consulta directamente el API REST de Supabase para obtener el conteo inmutable.
     */
    async fn fetch_strategic_count(&self) -> anyhow::Result<u64> {
        let supabase_url = std::env::var("NEXT_PUBLIC_SUPABASE_URL")?;
        let supabase_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")?;
        let endpoint = format!("{}/rest/v1/archived_audit_reports?select=count", supabase_url);

        let response = self.network_client.get(endpoint)
            .header("apikey", &supabase_key)
            .header("Authorization", format!("Bearer {}", supabase_key))
            .header("Prefer", "count=exact")
            .send()
            .await?;

        let count_header = response.headers()
            .get("Content-Range")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.split('/').last())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(count_header)
    }
}
