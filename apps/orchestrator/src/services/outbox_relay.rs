/**
 * =================================================================
 * APARATO: SOVEREIGN ARCHIVAL ENGINE (V110.0 - PRODUCTION READY)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: MIGRACIÓN Y PERSISTENCIA EN EL ARCHIVO ESTRATÉGICO
 *
 * VISION HIPER-HOLÍSTICA:
 * Orquesta el flujo de datos inmutables entre los motores gemelos.
 * Consume los hallazgos criptográficos y los reportes de misión certificados
 * desde la base de datos de alta frecuencia (Turso) y los proyecta hacia
 * el almacenamiento de largo plazo (Supabase) mediante un túnel HTTP seguro.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::{ArchivalRepository, FindingRepository};
use prospector_domain_models::work::AuditReport;
use reqwest::Client;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, error};

/// Motor de sincronización asíncrona para la redundancia de datos.
pub struct SovereignArchivalEngine {
    /// Cliente de red optimizado para ráfagas de alta latencia.
    network_client: Client,
    /// Referencia al estado neural compartido de la aplicación.
    application_state: AppState,
}

impl SovereignArchivalEngine {
    /**
     * Inicializa una nueva instancia del motor de archivo soberano.
     *
     * @param application_state Instancia del estado neural inyectado.
     */
    pub fn new(application_state: AppState) -> Self {
        let network_client = Client::builder()
            .timeout(Duration::from_secs(45))
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .build()
            .expect("FATAL: Archival Network Client initialization failed.");

        Self {
            network_client,
            application_state
        }
    }

    /**
     * Lanza el daemon de sincronización en el runtime asíncrono.
     * Implementa un bucle de escrutinio infinito con intervalos de 60 segundos.
     */
    pub async fn spawn_archival_loop(self) {
        let mut synchronization_interval = interval(Duration::from_secs(60));
        info!("🏛️  [ARCHIVAL_DAEMON]: Sovereign sync system operational. Target: Motor B.");

        loop {
            synchronization_interval.tick().await;

            // FASE 1: Sincronización de Misiones Certificadas
            if let Err(mission_sync_error) = self.synchronize_certified_missions_to_archive().await {
                error!("❌ [ARCHIVAL_FAULT]: Failed to migrate mission reports: {}", mission_sync_error);
            }

            // FASE 2: Sincronización de Hallazgos Criptográficos (High Value)
            if let Err(collision_sync_error) = self.synchronize_found_collisions_to_vault().await {
                error!("❌ [ARCHIVAL_FAULT]: Failed to migrate cryptographic discoveries: {}", collision_sync_error);
            }
        }
    }

    /**
     * Recupera y transmite reportes de auditoría completados.
     */
    async fn synchronize_certified_missions_to_archive(&self) -> anyhow::Result<()> {
        let archival_repository = ArchivalRepository::new(self.application_state.database_client.clone());
        let pending_missions_batch = archival_repository.fetch_pending_strategic_migration(25).await?;

        if pending_missions_batch.is_empty() { return Ok(()); }

        info!("📤 [ARCHIVAL]: Migrating {} certified reports to strategic strata.", pending_missions_batch.len());

        for mission_payload in pending_missions_batch {
            let original_job_identifier = mission_payload["original_job_id"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("INVALID_PAYLOAD_FORMAT"))?
                .to_string();

            // Intento de transmisión al Cuartel General
            if self.transmit_payload_to_strategic_endpoint("archived_audit_reports", &mission_payload).await.is_ok() {
                // Sello local para evitar duplicidad
                archival_repository.seal_archived_records(vec![original_job_identifier]).await?;

                // Notificación reactiva al Neural Link del Dashboard
                if let Ok(deserialized_report) = serde_json::from_value::<AuditReport>(mission_payload) {
                    self.application_state.event_bus.notify_mission_audit_certified(deserialized_report);
                }
            }
        }
        Ok(())
    }

    /**
     * Recupera y transmite colisiones encontradas por el enjambre.
     */
    async fn synchronize_found_collisions_to_vault(&self) -> anyhow::Result<()> {
        let finding_repository = FindingRepository::new(self.application_state.database_client.clone());
        let pending_findings_batch = finding_repository.fetch_pending_strategic_archival(10).await?;

        if pending_findings_batch.is_empty() { return Ok(()); }

        info!("🎯 [ARCHIVAL]: Securing {} cryptographic discoveries in the strategic vault.", pending_findings_batch.len());

        for finding_payload in pending_findings_batch {
            let original_discovery_identifier = finding_payload["original_id"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("INVALID_FINDING_FORMAT"))?
                .to_string();

            if self.transmit_payload_to_strategic_endpoint("strategic_findings", &finding_payload).await.is_ok() {
                finding_repository.mark_as_archived(vec![original_discovery_identifier]).await?;
            }
        }
        Ok(())
    }

    /**
     * Túnel de comunicación genérico hacia Supabase REST API.
     */
    async fn transmit_payload_to_strategic_endpoint(
        &self,
        target_table_name: &str,
        json_payload: &serde_json::Value
    ) -> Result<(), ()> {
        let strategic_base_url = std::env::var("SUPABASE_URL").map_err(|_| ())?;
        let strategic_service_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY").map_err(|_| ())?;

        let destination_url = format!("{}/rest/v1/{}", strategic_base_url, target_table_name);

        let network_response = self.network_client.post(destination_url)
            .header("apikey", &strategic_service_key)
            .header("Authorization", format!("Bearer {}", strategic_service_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .json(json_payload)
            .send()
            .await;

        match network_response {
            Ok(response) if response.status().is_success() => Ok(()),
            _ => Err(())
        }
    }
}
