// apps/census-taker/src/pipeline.rs
// =================================================================
// APARATO: INGESTION PIPELINE (ETL ENGINE)
// RESPONSABILIDAD: TRANSFORMACIÓN DE STREAMS DE DATOS (CSV -> SHARDS)
// RENDIMIENTO: ZERO-COPY PARSING DONDE SEA POSIBLE
// =================================================================

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use prospector_core_probabilistic::sharded::ShardedFilter;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

/// Estructura de registro crudo del CSV (BigQuery Export).
#[derive(Debug, serde::Deserialize)]
struct CsvRecord {
    address: String,
    #[allow(dead_code)] // Mantenemos el campo por compatibilidad con el esquema CSV
    balance: String,
}

/// Motor de Ingestión.
/// Encapsula el estado del proceso de transformación.
pub struct IngestionPipeline {
    input_path: std::path::PathBuf,
    output_dir: std::path::PathBuf,
    target_size: usize,
    shard_count: usize,
    fp_rate: f64,
}

impl IngestionPipeline {
    /// Construye un nuevo pipeline de ingestión.
    pub fn new(input: &Path, output: &Path, size: usize, shards: usize, fp_rate: f64) -> Self {
        Self {
            input_path: input.to_path_buf(),
            output_dir: output.to_path_buf(),
            target_size: size,
            shard_count: shards,
            fp_rate,
        }
    }

    /// Ejecuta el proceso ETL (Extract, Transform, Load).
    pub fn execute(&self) -> Result<()> {
        let start_time = Instant::now();
        println!("⚙️  PIPELINE: Iniciando secuencia de ingestión...");

        // 1. ALLOCATION (Memoria RAM)
        println!(
            "🧠 Allocating: {} items en {} shards (FP: {})",
            self.target_size, self.shard_count, self.fp_rate
        );
        let mut filter = ShardedFilter::new(self.shard_count, self.target_size, self.fp_rate);

        // 2. EXTRACTION (Streaming Read)
        let file = File::open(&self.input_path).with_context(|| {
            format!(
                "No se pudo abrir el archivo de entrada: {:?}",
                self.input_path
            )
        })?;

        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .buffer_capacity(64 * 1024) // 64KB Buffer para I/O rápido
            .from_reader(file);

        // 3. MONITORING (User Interface)
        let pb = ProgressBar::new(self.target_size as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta}) {msg}")
            .unwrap()
            .progress_chars("##-"));

        // 4. TRANSFORMATION (Processing Loop)
        let mut count = 0;
        let mut errors = 0;

        for result in rdr.deserialize() {
            let record: CsvRecord = match result {
                Ok(rec) => rec,
                Err(_) => {
                    errors += 1;
                    continue; // Skip filas corruptas sin pánico
                }
            };

            // La magia ocurre aquí: Address -> Hash -> Shard
            filter.add(&record.address);

            count += 1;
            if count % 5000 == 0 {
                pb.set_message(format!("{} processed ({} err)", count, errors));
                pb.inc(5000);
            }
        }

        pb.finish_with_message("✅ Ingestión en memoria completada.");

        // 5. LOADING (Disk Write)
        println!("💾 Volcando estado de memoria a disco (Serialización)...");
        filter
            .save_to_dir(&self.output_dir)
            .context("Fallo crítico durante el volcado a disco")?;

        let duration = start_time.elapsed();

        println!("--------------------------------------");
        println!("🏁 INFORME DE EJECUCIÓN");
        println!("⏱️  Tiempo Total: {:.2?}", duration);
        println!("📦 Registros Procesados: {}", count);
        println!("⚠️  Errores/Saltos: {}", errors);
        println!("📂 Artefactos: {:?}", self.output_dir);
        println!("--------------------------------------");

        Ok(())
    }
}
