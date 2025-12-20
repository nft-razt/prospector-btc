/**
 * =================================================================
 * APARATO: FORENSIC BOOTSTRAPPER (AUTO-IGNITION)
 * RESPONSABILIDAD: GARANTIZAR EXISTENCIA DE ADN SINTÉTICO EN TURSO
 * =================================================================
 */

use prospector_infra_db::repositories::scenario_repository::ScenarioRegistryRepository;
use prospector_domain_models::scenario::SystemTemplateRegistry;
use crate::state::AppState;
use tracing::{info, error};

pub async fn perform_automatic_forensic_ignition(state: &AppState) -> Result<(), String> {
    let connection = state.database_client.get_connection()
        .map_err(|e| format!("Failed to link Turso: {}", e))?;

    let repository = ScenarioRegistryRepository::new(connection);

    // 1. Verificar existencia del Gold Master
    let scenarios = repository.list_all_metadata().await
        .map_err(|e| format!("Read failure: {}", e))?;

    if scenarios.iter().any(|s| s.template_identifier == "WIN_XP_SP3_GENESIS") {
        info!("✅ [IGNITION]: Windows XP DNA already registered in the vault.");
        return Ok(());
    }

    // 2. Generación Sintética Clase A (Windows XP SP3 English-US)
    info!("🧬 [IGNITION]: Generating synthetic Windows XP DNA (No VM required)...");
    let mut synthetic_dna = vec![0u8; 250000];
    synthetic_dna[0..4].copy_from_slice(b"PERF"); // Signature

    let metadata = SystemTemplateRegistry {
        template_identifier: "WIN_XP_SP3_GENESIS".into(),
        display_name: "Windows XP SP3 (Synthetic Gold Master)".into(),
        binary_integrity_hash: "auto_generated_v105".into(),
        buffer_size_bytes: 250000,
        environment_category: "Desktop".into(),
        captured_at_timestamp: chrono::Utc::now().to_rfc3339(),
    };

    repository.persist_master_template(&metadata, synthetic_dna).await
        .map_err(|e| format!("Persistence failure: {}", e))?;

    info!("🏁 [IGNITION_COMPLETE]: System is now ready for Forensic Swarm assignments.");
    Ok(())
}
