/**
 * =================================================================
 * APARATO: CENSUS INGESTION PIPELINE (V40.3 - SOBERANO)
 * CLASIFICACIÓN: APPLICATION LOGIC / ETL ENGINE
 * RESPONSABILIDAD: TRANSFORMACIÓN DE DATOS MASIVOS (CSV -> SHARDS)
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la lectura en streaming del censo UTXO para cristalizarlo
 * en fragmentos binarios de búsqueda probabilística, garantizando un
 * consumo de RAM constante en hardware antiguo (VAIO).
 * =================================================================
 */

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use prospector_core_probabilistic::sharded::ShardedFilter;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Modelo de datos para la deserialización de filas del Censo UTXO.
#[derive(Debug, serde::Deserialize)]
struct BitcoinAddressRecord {
    /// La dirección Bitcoin Legacy extraída.
    address: String,
    /// Balance acumulado (no utilizado en el filtro, marcado para evitar ruidos).
    #[allow(dead_code)]
    balance: String,
}

pub struct IngestionPipeline {
    input_file_path: PathBuf,
    output_directory: PathBuf,
    target_capacity: usize,
    partition_count: usize,
    false_positive_rate: f64,
}

impl IngestionPipeline {
    /**
     * Construye una nueva instancia del Pipeline con parámetros de élite.
     */
    pub fn new(
        input: &Path,
        output: &Path,
        capacity: usize,
        shards: usize,
        rate: f64
    ) -> Self {
        Self {
            input_file_path: input.to_path_buf(),
            output_directory: output.to_path_buf(),
            target_capacity: capacity,
            partition_count: shards,
            false_positive_rate: rate,
        }
    }

    /**
     * Ejecuta la secuencia completa de cristalización.
     */
    pub fn execute_ingestion_sequence(&self) -> Result<()> {
        let global_timer_start = Instant::now();
        println!("⚙️  [PIPELINE]: Iniciando secuencia de cristalización V10.8...");

        // 1. ALLOCATION: Matriz probabilística
        let mut filter_orchestrator = ShardedFilter::new(
            self.partition_count,
            self.target_capacity,
            self.false_positive_rate
        );

        // 2. EXTRACTION: Stream del archivo físico
        let census_file = File::open(&self.input_file_path).with_context(|| {
            format!("CRITICAL_IO_ERROR: No se pudo abrir {:?}", self.input_file_path)
        })?;

        let mut csv_stream = ReaderBuilder::new()
            .has_headers(true)
            .buffer_capacity(128 * 1024) // Optimización para discos mecánicos
            .from_reader(census_file);

        // 3. MONITORING: Telemetría de terminal
        let progress_bar = ProgressBar::new(self.target_capacity as u64);
        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta}) {msg}")
            .unwrap()
            .progress_chars("##-"));

        // 4. TRANSFORMATION: Ingestión determinista
        let mut processed_records: usize = 0;
        let mut error_count: usize = 0;

        for result in csv_stream.deserialize() {
            let record: BitcoinAddressRecord = match result {
                Ok(data) => data,
                Err(_) => {
                    error_count += 1;
                    continue;
                }
            };

            // Marcado en fragmento SipHash estable
            filter_orchestrator.add(&record.address);

            processed_records += 1;
            if processed_records % 10000 == 0 {
                progress_bar.set_message(format!("Procesando (Err: {})", error_count));
                progress_bar.inc(10000);
            }
        }

        progress_bar.finish_with_message("✅ Mapa de bits sincronizado en RAM.");

        // 5. LOADING: Persistencia inmutable
        println!("💾 [DISK]: Escribiendo fragmentos en {:?}...", self.output_directory);

        filter_orchestrator
            .save_to_directory(&self.output_directory)
            .context("WRITE_FAULT: No se pudieron guardar los Shards binarios")?;

        println!("--------------------------------------------------");
        println!("🏁 [INFORME FINAL V10.8]");
        println!("⏱️  Tiempo Total:    {:.2?}", global_timer_start.elapsed());
        println!("📦 Registros:       {}", processed_records);
        println!("📂 Artefactos:      {} shards binarios", self.partition_count);
        println!("--------------------------------------------------");

        Ok(())
    }
}
