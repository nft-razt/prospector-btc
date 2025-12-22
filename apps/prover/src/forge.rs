/**
 * =================================================================
 * APARATO: SCENARIO FORGE ENGINE (V30.0 - CONTRACT ALIGNED)
 * CLASIFICACIÓN: OPS UTILITY (ESTRATO L6)
 * RESPONSABILIDAD: CREACIÓN DETERMINISTA DE ARTEFACTOS DE PRUEBA
 *
 * ESTRATEGIA DE ÉLITE:
 * - Deterministic Derivation: Genera material criptográfico reproducible.
 * - Sharding Compliance: Sincronizado con el motor de particionamiento V10.8.
 * - Zero-Abbreviation: Cumplimiento total de nomenclatura descriptiva.
 * =================================================================
 */

use anyhow::{Context, Result};
use tracing::info;
use std::path::{Path, PathBuf};
use uuid::Uuid;

// --- SINAPSIS INTERNA ---
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_strategy::brainwallet::phrase_to_private_key;

pub struct ScenarioForge {
    output_directory: PathBuf,
    seed_phrase_prefix: String,
    target_numerical_identifier: String,
}

impl ScenarioForge {
    /**
     * Construye una nueva instancia de la forja de escenarios.
     */
    pub fn new(output_path: &Path, prefix: &str, target_id: &str) -> Self {
        Self {
            output_directory: output_path.to_path_buf(),
            seed_phrase_prefix: prefix.to_string(),
            target_numerical_identifier: target_id.to_string(),
        }
    }

    /**
     * Ejecuta la secuencia completa de forja criptográfica.
     * Genera la aguja (clave) y el pajar (filtro sintético).
     *
     * @returns Result con el ID de la misión de prueba generada.
     */
    pub fn execute_forging_sequence(&self) -> Result<String> {
        // 1. GENERACIÓN DEL SECRETO MAESTRO (LA AGUJA)
        let constant_suffix = "TEST_VECTOR_GOLDEN";
        let complete_phrase = format!("{}{}{}",
            self.seed_phrase_prefix,
            self.target_numerical_identifier,
            constant_suffix
        );

        info!("🔑 [FORGE]: Generating cryptographic material for phrase: '{}'", complete_phrase);

        // 2. DERIVACIÓN CRIPTOGRÁFICA SOBERANA
        let private_key_instance = phrase_to_private_key(&complete_phrase);
        let public_key_instance = SafePublicKey::from_private(&private_key_instance);
        let bitcoin_target_address = pubkey_to_address(&public_key_instance, false); // Legacy Uncompressed
        let wallet_import_format = prospector_core_gen::wif::private_to_wif(&private_key_instance, false);

        println!("\n--- 📝 [GENERATED_TEST_ARTIFACTS] ---");
        println!("Target Address:   {}", bitcoin_target_address);
        println!("WIF Private Key:  {}", wallet_import_format);
        println!("Entropy Source:   SHA256(\"{}\")", complete_phrase);
        println!("-------------------------------------\n");

        // 3. GENERACIÓN DEL FILTRO SINTÉTICO PARTICIONADO (EL MAPA)
        self.crystallize_synthetic_filter(&bitcoin_target_address)?;

        // 4. GENERACIÓN DE INSTRUCCIONES TÁCTICAS SQL
        let mission_identifier = self.emit_sql_injection_instructions()?;

        Ok(mission_identifier)
    }

    /**
     * Cristaliza un filtro de Bloom particionado que contiene el objetivo.
     * ✅ RESOLUCIÓN: Corregido de 'save_to_dir' a 'save_to_directory'.
     */
    fn crystallize_synthetic_filter(&self, target_address: &str) -> Result<()> {
        if self.output_directory.exists() {
            std::fs::remove_dir_all(&self.output_directory)?;
        }
        std::fs::create_dir_all(&self.output_directory)?;

        info!("🧠 [FORGE]: Constructing 4-shard synthetic filter...");
        let mut filter_orchestrator = ShardedFilter::new(4, 1000, 0.00001);

        // Inyección de la aguja en el mapa de bits
        filter_orchestrator.add(target_address);

        // Persistencia física sincronizada con el contrato L1
        filter_orchestrator
            .save_to_directory(&self.output_directory)
            .context("IO_FAULT: Failed to persist synthetic shards.")?;

        Ok(())
    }

    /**
     * Genera la consulta SQL necesaria para inyectar la misión en Turso.
     */
    fn emit_sql_injection_instructions(&self) -> Result<String> {
        let mission_id = Uuid::new_v4().to_string();

        println!("\n✅ [FORGE_COMPLETE]: Execution artifacts ready.");
        println!("🚀 INJECTION QUERY FOR TACTICAL LEDGER:");
        println!("==================================================");
        println!(
            "INSERT INTO jobs (id, range_start, range_end, status, strategy_type, created_at) \n\
             VALUES ('{}', '0', '1000000', 'pending', 'Sequential', CURRENT_TIMESTAMP);",
            mission_id
        );
        println!("==================================================");

        Ok(mission_id)
    }
}
