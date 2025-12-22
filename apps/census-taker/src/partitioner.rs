/**
 * =================================================================
 * APARATO: FORENSIC BLOOM PARTITIONER (V150.0 - MANIFEST ENABLED)
 * CLASIFICACIÓN: CORE ETL ENGINE (ESTRATO L1)
 * RESPONSABILIDAD: SEGMENTACIÓN Y SELLADO CRIPTOGRÁFICO DEL CENSO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la segmentación cronológica del set UTXO y genera un
 * 'StratumManifest'. Este manifiesto contiene los hashes de integridad
 * de cada fragmento, permitiendo que el sistema distribuido detecte
 * desincronizaciones de datos antes de iniciar la auditoría.
 * =================================================================
 */

use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::stratum::StratumManifest;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use sha2::{Sha256, Digest};
use std::fs;
use tracing::{info, error, instrument};

#[derive(Debug, Deserialize)]
pub struct RawUtxoRecord {
    pub address: String,
    pub block_timestamp: String,
}

pub struct ForensicPartitioner {
    /// Carpeta base donde se generará la jerarquía de estratos.
    output_directory: PathBuf,
}

impl ForensicPartitioner {
    pub fn new(output_path: &Path) -> Self {
        Self {
            output_directory: output_path.to_path_buf(),
        }
    }

    /**
     * Ejecuta la partición y genera el sello de integridad global.
     */
    #[instrument(skip(self, records))]
    pub fn partition_and_crystallize(&self, records: Vec<RawUtxoRecord>) -> anyhow::Result<()> {
        info!("🔮 [PARTITIONER]: Segmenting {} records into archaeological strata...", records.len());

        // 1. INICIALIZACIÓN DE ESTRATOS
        let mut satoshi_filter = ShardedFilter::new(4, 2_000_000, 0.000001);
        let mut vulnerable_filter = ShardedFilter::new(4, 10_000_000, 0.000001);
        let mut standard_filter = ShardedFilter::new(4, 30_000_000, 0.00001);

        // 2. CLASIFICACIÓN CRONOLÓGICA
        for record in records {
            let year = record.block_timestamp[0..4].parse::<u32>().unwrap_or(2025);
            match year {
                2009..=2010 => satoshi_filter.add(&record.address),
                2011..=2013 => vulnerable_filter.add(&record.address),
                _ => standard_filter.add(&record.address),
            }
        }

        // 3. PERSISTENCIA Y CÁLCULO DE INTEGRIDAD
        let mut manifest = StratumManifest::new();

        manifest.add_strata(
            "satoshi_era",
            self.save_and_hash_strata("satoshi_era", &satoshi_filter)?
        );
        manifest.add_strata(
            "vulnerable_legacy",
            self.save_and_hash_strata("vulnerable_legacy", &vulnerable_filter)?
        );
        manifest.add_strata(
            "standard_legacy",
            self.save_and_hash_strata("standard_legacy", &standard_filter)?
        );

        // 4. SELLADO DEL MANIFIESTO (Audit Token Generation)
        let manifest_path = self.output_directory.join("stratum_manifest.json");
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        fs::write(&manifest_path, manifest_json)?;

        info!("✅ [PARTITIONER_COMPLETE]: Censo sellado con Audit Token: {}", manifest.audit_token);
        Ok(())
    }

    /**
     * Guarda los fragmentos y calcula el hash combinado de los archivos generados.
     */
    fn save_and_hash_strata(&self, name: &str, filter: &ShardedFilter) -> anyhow::Result<String> {
        let target_path = self.output_directory.join(name);
        if !target_path.exists() {
            fs::create_dir_all(&target_path)?;
        }

        filter.save_to_directory(&target_path)?;

        // Calcular hash de integridad del estrato (basado en sus 4 shards)
        let mut combined_hasher = Sha256::new();
        for i in 0..4 {
            let shard_path = target_path.join(format!("filter_shard_{}.bin", i));
            let shard_bytes = fs::read(shard_path)?;
            combined_hasher.update(&shard_bytes);
        }

        Ok(format!("{:x}", combined_hasher.finalize()))
    }
}
