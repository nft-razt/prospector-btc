/**
 * =================================================================
 * APARATO: FORENSIC BOOTSTRAPPER (V110.0 - SOBERANO)
 * RESPONSABILIDAD: GARANTIZAR EXISTENCIA DE ADN SINTÉTICO EN TURSO
 * =================================================================
 */

use prospector_infra_db::repositories::ScenarioRegistryRepository;
use prospector_domain_models::scenario::SystemTemplateRegistry;
use crate::state::AppState;
use tracing::info;

pub async fn perform_automatic_forensic_ignition(state: &AppState) -> Result<(), String> {
    // ✅ RESOLUCIÓN E0308: Pasamos el database_client (TursoClient) en lugar de la conexión cruda.
    let repository = ScenarioRegistryRepository::new(state.database_client.clone());

    let scenarios = repository.list_all_metadata().await
        .map_err(|error| format!("DATABASE_READ_FAILURE: {}", error))?;

    if scenarios.iter().any(|s| s.template_identifier == "WIN_XP_SP3_GENESIS") {
        info!("✅ [IGNITION]: Windows XP DNA already registered in the vault.");
        return Ok(());
    }

    info!("🧬 [IGNITION]: Generating synthetic Windows XP DNA...");
    let mut synthetic_dna = vec![0u8; 250000];
    synthetic_dna[0..4].copy_from_slice(b"PERF");

    let metadata = SystemTemplateRegistry {
        template_identifier: "WIN_XP_SP3_GENESIS".into(),
        display_name: "Windows XP SP3 (Synthetic Gold Master)".into(),
        binary_integrity_hash: "v10.8_autogen".into(),
        buffer_size_bytes: 250000,
        environment_category: "Desktop".into(),
        captured_at_timestamp: chrono::Utc::now().to_rfc3339(),
    };

    repository.persist_master_template(&metadata, synthetic_dna).await
        .map_err(|error| format!("PERSISTENCE_FAILURE: {}", error))?;

    Ok(())
}
