// apps/prover/src/forge.rs
// =================================================================
// APARATO: SCENARIO FORGE (CRYPTO GENERATOR)
// RESPONSABILIDAD: CREACIÓN DETERMINISTA DE ARTEFACTOS DE PRUEBA
// =================================================================

use anyhow::{Context, Result};
use log::info;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_strategy::brainwallet::phrase_to_private_key;

pub struct ScenarioForge {
    output_dir: PathBuf,
    seed_prefix: String,
    target_identifier: String,
}

impl ScenarioForge {
    pub fn new(output: &Path, prefix: &str, target: &str) -> Self {
        Self {
            output_dir: output.to_path_buf(),
            seed_prefix: prefix.to_string(),
            target_identifier: target.to_string(),
        }
    }

    /// Ejecuta la forja del escenario.
    /// Retorna el ID del trabajo generado para referencia.
    pub fn execute(&self) -> Result<String> {
        // 1. GENERACIÓN DEL SECRETO (LA AGUJA)
        let suffix = "TEST";
        let phrase = format!("{}{}{}", self.seed_prefix, self.target_identifier, suffix);

        info!("🔑 Generando material criptográfico para frase: '{}'", phrase);

        // 2. DERIVACIÓN CRIPTOGRÁFICA
        let pk = phrase_to_private_key(&phrase);
        let pubk = SafePublicKey::from_private(&pk);
        let address = pubkey_to_address(&pubk, false); // Legacy Uncompressed
        let wif = prospector_core_gen::wif::private_to_wif(&pk, false);

        println!("\n--- 📝 ARTEFACTOS GENERADOS ---");
        println!("Address:      {}", address);
        println!("Private Key:  {}", wif);
        println!("Entropy:      SHA256(\"{}\")", phrase);
        println!("---------------------------------\n");

        // 3. GENERACIÓN DEL FILTRO SINTÉTICO (EL MAPA)
        self.generate_synthetic_filter(&address)?;

        // 4. GENERACIÓN DE INSTRUCCIONES SQL
        let job_id = self.generate_sql_instructions()?;

        Ok(job_id)
    }

    fn generate_synthetic_filter(&self, target_address: &str) -> Result<()> {
        if self.output_dir.exists() {
            std::fs::remove_dir_all(&self.output_dir)?;
        }
        std::fs::create_dir_all(&self.output_dir)?;

        info!("🧠 Construyendo ShardedFilter sintético (4 particiones)...");
        let mut filter = ShardedFilter::new(4, 1000, 0.00001);

        // Inyectamos la aguja
        filter.add(target_address);

        info!("💾 Persistiendo filtros en {:?}...", self.output_dir);
        filter.save_to_dir(&self.output_dir).context("Fallo al guardar shards")?;

        Ok(())
    }

    fn generate_sql_instructions(&self) -> Result<String> {
        let target_num: u64 = self.target_identifier.parse().unwrap_or(777);
        let range_start = target_num.saturating_sub(50);
        let range_end = target_num + 50;
        let job_id = Uuid::new_v4().to_string();

        println!("\n✅ ESCENARIO LISTO. EJECUTA ESTA QUERY EN TURSO:");
        println!("==================================================");
        println!(
            r#"
INSERT INTO jobs (id, range_start, range_end, status, created_at)
VALUES (
    '{}',
    '{}',
    '{}',
    'pending',
    CURRENT_TIMESTAMP
);
        "#,
            job_id, range_start, range_end
        );

        Ok(job_id)
    }
}
