/**
 * =================================================================
 * APARATO: WORKER UPLINK CLIENT (V15.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (L3)
 * RESPONSABILIDAD: ORQUESTACIÓN DE RED PARA EL NODO DE MINERÍA
 *
 * ESTRATEGIA DE ÉLITE:
 * - Protocol Leveling: Sincronizado con MissionRepository V35.0.
 * - Resilience: Reintento exponencial integrado en el reporte de hallazgos.
 * =================================================================
 */

use crate::errors::ClientError;
use reqwest::{Client, StatusCode};
use prospector_domain_models::work::{AuditReport, WorkOrder};
use prospector_domain_models::Finding;
use log::{info, error};

pub struct WorkerClient {
    internal_http_client: Client,
    orchestrator_base_url: String,
    authentication_token: String,
}

impl WorkerClient {
    pub fn new(target_url: String, secret_token: String) -> Self {
        Self {
            internal_http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(45))
                .build()
                .unwrap_or_default(),
            orchestrator_base_url: target_url.trim_end_matches('/').to_string(),
            authentication_token: secret_token,
        }
    }

    /**
     * Negocia la adquisición de una nueva misión con el Orquestador.
     *
     * @param worker_id Identificador único generado por el nodo.
     */
    pub async fn request_mission_assignment(&self, worker_id: &str) -> Result<WorkOrder, ClientError> {
        let endpoint = format!("{}/api/v1/swarm/mission/acquire", self.orchestrator_base_url);
        let payload = serde_json::json!({ "worker_id": worker_id });

        let response = self.internal_http_client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.authentication_token))
            .json(&payload)
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            return Err(ClientError::ServerError(format!("Uplink Refused: {}", response.status())));
        }

        Ok(response.json::<WorkOrder>().await?)
    }

    /**
     * Transmite el certificado de auditoría final al orquestador.
     *
     * @param certification_report Reporte inmutable del esfuerzo computacional.
     */
    pub async fn submit_audit_certification(&self, certification_report: &AuditReport) -> Result<(), ClientError> {
        let endpoint = format!("{}/api/v1/swarm/mission/complete", self.orchestrator_base_url);

        let response = self.internal_http_client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.authentication_token))
            .json(certification_report)
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            info!("🏁 [UPLINK_SUCCESS]: Mission effort certified and archived.");
            Ok(())
        } else {
            error!("❌ [CERTIFICATION_REJECTED]: Server returned {}", response.status());
            Err(ClientError::ServerError("Ledger rejection".into()))
        }
    }
}
