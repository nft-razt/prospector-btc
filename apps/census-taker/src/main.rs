// apps/census-taker/src/main.rs
// =================================================================
// APARATO: CENSUS TAKER (SHARDING EDITION)
// RESPONSABILIDAD: GENERACIÓN DE ARTEFACTOS PARTICIONADOS
// CAMBIO: Output es ahora un Directorio, no un Archivo único.
// =================================================================

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
// ✅ Importamos el nuevo ShardedFilter
use anyhow::{Context, Result};
use prospector_core_probabilistic::sharded::ShardedFilter;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, serde::Deserialize)]
struct CsvRecord {
    address: String,
    #[allow(dead_code)]
    balance: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Ruta al archivo CSV de entrada (BigQuery Export).
    #[arg(short, long)]
    input: PathBuf,

    /// Directorio de salida para los shards.
    #[arg(short, long, default_value = "dist/filters")]
    output_dir: PathBuf,

    /// Cantidad estimada de items totales.
    #[arg(long, default_value_t = 50_000_000)]
    size: usize,

    /// Tasa de falsos positivos deseada.
    #[arg(long, default_value_t = 0.0000001)]
    fp_rate: f64,

    /// Número de shards (particiones) a generar.
    /// Recomendado: 4 para balancear I/O y conexiones HTTP.
    #[arg(long, default_value_t = 4)]
    shards: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let start_time = Instant::now();

    println!("🚀 INICIANDO CENSUS TAKER [SHARDING MODE]");
    println!("--------------------------------------");
    println!("📄 Input: {:?}", args.input);
    println!("wd Output Dir: {:?}", args.output_dir);
    println!("wv Target Size: {}", args.size);
    println!("Hx Shards: {}", args.shards);

    // 1. Inicializar Filtro Particionado en Memoria
    println!("🧠 Allocating memory structures...");
    let mut filter = ShardedFilter::new(args.shards, args.size, args.fp_rate);

    // 2. Preparar Lector CSV
    let file = File::open(&args.input).context("Fallo al abrir CSV")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    // 3. Configurar UI
    let pb = ProgressBar::new(args.size as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta}) {msg}")
        .unwrap()
        .progress_chars("##-"));

    // 4. Procesamiento Streaming
    let mut count = 0;
    for result in rdr.deserialize() {
        let record: CsvRecord = match result {
            Ok(rec) => rec,
            Err(_) => continue, // Skip filas corruptas
        };

        // El ShardedFilter se encarga de rutear al shard correcto internamente
        filter.add(&record.address);

        count += 1;
        if count % 5000 == 0 {
            pb.set_message(format!("{} addrs processed", count));
            pb.inc(5000);
        }
    }

    pb.finish_with_message("Ingestión completada");

    // 5. Volcado a Disco (Paralelo)
    println!("💾 Escribiendo shards a disco...");
    filter
        .save_to_dir(&args.output_dir)
        .context("Fallo crítico al guardar shards")?;

    let duration = start_time.elapsed();
    println!("--------------------------------------");
    println!("✅ PROCESO COMPLETADO");
    println!("⏱️ Tiempo: {:.2?}", duration);
    println!("📦 Direcciones: {}", count);
    println!("📂 Directorio: {:?}", args.output_dir);

    Ok(())
}
