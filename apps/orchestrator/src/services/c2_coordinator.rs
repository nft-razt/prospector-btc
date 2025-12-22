/**
 * =================================================================
 * APARATO: C2 GITHUB COORDINATOR (V110.0 - PRODUCTION READY)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L6)
 * RESPONSABILIDAD: ORQUESTACIÓN REMOTA DE CAPACIDAD COMPUTACIONAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el protocolo de comando y control (C2) hacia la nube de GitHub.
 * Permite que el orquestador dispare flujos de aprovisionamiento automáticos
 * ante la pérdida de nodos, garantizando que el hashrate del enjambre
 * permanezca constante durante la misión de auditoría.
 * =================================================================
 */

use reqwest::{Client, StatusCode};
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

/// Payload para el despacho de workflows mediante la API de GitHub.
#[derive(Serialize)]
struct GitHubWorkflowDispatchPayload {
    /// Referencia de rama o tag (ej: "main").
    #[serde(rename = "ref")]
    branch_git_reference: String,
    /// Mapa de entradas configuradas en el archivo YAML del workflow.
    workflow_input_parameters: HashMap<String, String>,
}

/// Coordinador soberano de infraestructura externa.
pub struct GitHubCommandCoordinator {
    /// Cliente de red persistente con TLS 1.3.
    network_communication_client: Client,
    /// Propietario del repositorio objetivo.
    repository_owner_identifier: String,
    /// Nombre del repositorio del proyecto.
    repository_name_identifier: String,
    /// Token de Acceso Personal (PAT) con permisos de workflow.
    github_access_token: String,
}

impl GitHubCommandCoordinator {
    /**
     * Inicializa el coordinador extrayendo las credenciales del entorno de ejecución.
     */
    pub fn from_production_environment() -> anyhow::Result<Self> {
        let github_access_token = std::env::var("GITHUB_PAT")
            .map_err(|_| anyhow::anyhow!("CRITICAL_AUTH: GITHUB_PAT is missing."))?;

        let repository_owner_identifier = std::env::var("GITHUB_OWNER")
            .unwrap_or_else(|_| "nft-razt".to_string());

        let repository_name_identifier = std::env::var("GITHUB_REPO")
            .unwrap_or_else(|_| "prospector-btc".to_string());

        Ok(Self {
            network_communication_client: Client::builder()
                .timeout(Duration::from_secs(20))
                .build()?,
            repository_owner_identifier,
            repository_name_identifier,
            github_access_token,
        })
    }

    /**
     * Dispara una secuencia de expansión del enjambre para reemplazar capacidad degradada.
     *
     * @param node_count_to_initialize Cantidad de nuevas unidades a desplegar.
     */
    pub async fn trigger_swarm_expansion_sequence(
        &self,
        node_count_to_initialize: u32
    ) -> anyhow::Result<()> {
        let target_workflow_filename = "provisioner-cron.yml";

        let api_endpoint_url = format!(
            "https://api.github.com/repos/{}/{}/actions/workflows/{}/dispatches",
            self.repository_owner_identifier,
            self.repository_name_identifier,
            target_workflow_filename
        );

        let mut workflow_inputs = HashMap::new();
        workflow_inputs.insert("worker_count_per_shard".to_string(), node_count_to_initialize.to_string());
        workflow_inputs.insert("shard_count".to_string(), "1".to_string());

        let dispatch_payload = GitHubWorkflowDispatchPayload {
            branch_git_reference: "main".to_string(),
            workflow_input_parameters: workflow_inputs,
        };

        let network_response = self.network_communication_client
            .post(&api_endpoint_url)
            .header("Authorization", format!("Bearer {}", self.github_access_token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "Prospector-Sovereign-C2-V11")
            .json(&dispatch_payload)
            .send()
            .await?;

        if network_response.status() == StatusCode::NO_CONTENT {
            // El código 204 indica que el despacho fue aceptado correctamente.
            Ok(())
        } else {
            let error_response_body = network_response.text().await?;
            Err(anyhow::anyhow!("GITHUB_API_REJECTION: {}", error_response_body))
        }
    }
}
