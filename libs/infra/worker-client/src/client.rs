/**
 * =================================================================
 * APARATO: WORKER UPLINK CLIENT (V250.0 - PURIFIED)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: COMUNICACIÓN SOBERANA Y GESTIÓN DE ARTEFACTOS
 * =================================================================
 */

use crate::errors::ClientError;
use prospector_domain_models::work::{
    WorkOrder,
    AuditReport,
    MissionRequestPayload,
    TargetStrata
};
use prospector_domain_models::finding::Finding;
use reqwest::{Client, StatusCode};
use std::path::Path;
use tokio::fs;
use futures::future::join_all;
use tracing::{info, instrument};

pub struct WorkerClient {
    network_session: Client,
    orchestrator_endpoint: String,
    authentication_token: String,
}

impl WorkerClient {
    pub fn new(base_url: String, secret_token: String) -> Self {
        Self {
            network_session: Client::new(),
            orchestrator_endpoint: base_url.trim_end_matches('/').to_string(),
            authentication_token: secret_token,
        }
    }

    #[instrument(skip(self, handshake))]
    pub async fn negotiate_mission_assignment(
        &self,
        handshake: &MissionRequestPayload
    ) -> Result<WorkOrder, ClientError> {
        let endpoint_url = format!("{}/api/v1/swarm/mission/acquire", self.orchestrator_endpoint);
        let response = self.network_session
            .post(&endpoint_url)
            .header("Authorization", format!("Bearer {}", self.authentication_token))
            .json(handshake)
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            Ok(response.json::<WorkOrder>().await?)
        } else {
            Err(ClientError::ServerError(format!("UPLINK_REJECTED: {}", response.status())))
        }
    }

    /**
     * Hidrata los estratos del censo UTXO descargando shards en paralelo.
     */
    pub async fn hydrate_mission_census_strata(
        &self,
        mission_order: &WorkOrder,
        cache_directory: &Path
    ) -> Result<(), ClientError> {
        let strata_label = match mission_order.required_strata {
            TargetStrata::SatoshiEra => "satoshi_era",
            TargetStrata::VulnerableLegacy => "vulnerable_legacy",
            TargetStrata::StandardLegacy => "standard_legacy",
            TargetStrata::FullTacticalSet => "full_tactical_set",
        };

        info!("🧊 [HYDRATION]: Initiating mesh download for strata [{}].", strata_label);

        let local_vault_path = cache_directory.join(strata_label);
        if !local_vault_path.exists() {
            fs::create_dir_all(&local_vault_path).await.map_err(ClientError::IoError)?;
        }

        let mut transfer_tasks = Vec::with_capacity(4);
        for shard_index in 0..4 {
            transfer_tasks.push(self.download_census_fragment(
                strata_label,
                shard_index,
                &local_vault_path
            ));
        }

        let results = join_all(transfer_tasks).await;
        for result in results {
            if let Err(error) = result { return Err(error); }
        }

        Ok(())
    }

    async fn download_census_fragment(
        &self,
        strata_id: &str,
        shard_id: usize,
        destination_path: &Path
    ) -> Result<(), ClientError> {
        let file_name = format!("filter_shard_{}.bin", shard_id);
        let target_url = format!("{}/api/v1/assets/dna/{}/{}",
            self.orchestrator_endpoint, strata_id, file_name);

        let local_file = destination_path.join(&file_name);
        if local_file.exists() { return Ok(()); }

        let response = self.network_session
            .get(&target_url)
            .header("Authorization", format!("Bearer {}", self.authentication_token))
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let binary_payload = response.bytes().await?;
            fs::write(local_file, binary_payload).await.map_err(ClientError::IoError)?;
            Ok(())
        } else {
            Err(ClientError::ServerError(format!("SHARD_{}_FETCH_FAULT", shard_id)))
        }
    }

    pub async fn transmit_found_collision(&self, discovery: &Finding) -> Result<(), ClientError> {
        let endpoint_url = format!("{}/api/v1/swarm/finding", self.orchestrator_endpoint);
        let _ = self.network_session.post(&endpoint_url)
            .header("Authorization", format!("Bearer {}", self.authentication_token))
            .json(discovery).send().await?;
        Ok(())
    }

    pub async fn submit_audit_certification(&self, report: &AuditReport) -> Result<(), ClientError> {
        let endpoint_url = format!("{}/api/v1/swarm/mission/complete", self.orchestrator_endpoint);
        let _ = self.network_session.post(&endpoint_url)
            .header("Authorization", format!("Bearer {}", self.authentication_token))
            .json(report).send().await?;
        Ok(())
    }
}
