
// =================================================================
// APARATO: CENSUS TAKER (ETL)
// ESTÁNDARES: CLI PROFESIONAL, STREAMING
// =================================================================

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use prospector_core_probabilistic::filter_wrapper::RichListFilter;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;
use anyhow::{Context, Result};

/// Estructura para mapear las filas del CSV de BigQuery.
/// BigQuery exporta: address, balance
#[derive(Debug, serde::Deserialize)]
struct CsvRecord {
    address: String,
    #[allow(dead_code)] // Solo usamos la dirección para el filtro, el balance es para la DB
    balance: String,
}

/// Argumentos de Línea de Comandos
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ruta al archivo CSV de entrada (exportado de BigQuery)
    #[arg(short, long)]
    input: PathBuf,

    /// Ruta donde guardar el Filtro de Bloom binario (.bin)
    #[arg(short, long, default_value = "utxo_filter.bin")]
    output: PathBuf,

    /// Cantidad estimada de items (para optimizar el filtro)
    /// Por defecto: 50 millones (Aprox UTXO set 2024-2025)
    #[arg(long, default_value_t = 50_000_000)]
    size: usize,

    /// Tasa de falsos positivos deseada
    #[arg(long, default_value_t = 0.0000001)]
    fp_rate: f64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let start_time = Instant::now();

    println!("🚀 INICIANDO PROSPECTOR CENSUS TAKER");
    println!("--------------------------------------");
    println!("📄 Input CSV: {:?}", args.input);
    println!("💾 Output Bin: {:?}", args.output);
    println!("wv Target Size: {}", args.size);

    // 1. Inicializar el Filtro en Memoria
    println!("Creating memory structure...");
    let mut filter = RichListFilter::new(args.size, args.fp_rate);

    // 2. Preparar el Lector CSV
    let file = File::open(&args.input).context("No se pudo abrir el archivo CSV")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true) // Asumimos que BigQuery pone headers
        .from_reader(file);

    // 3. Configurar Barra de Progreso
    let pb = ProgressBar::new(args.size as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // 4. Bucle de Procesamiento (Streaming)
    let mut count = 0;
    for result in rdr.deserialize() {
        // Manejo de errores por fila (si una fila está mal, no rompemos todo el proceso)
        let record: CsvRecord = match result {
            Ok(rec) => rec,
            Err(e) => {
                eprintln!("⚠️ Error leyendo fila CSV: {}", e);
                continue;
            }
        };

        // Inyectar en el filtro
        filter.add(&record.address);

        count += 1;
        if count % 1000 == 0 {
            pb.inc(1000);
        }
    }

    pb.finish_with_message("Lectura completada");

    // 5. Guardar a Disco
    println!("💾 Guardando filtro binario optimizado...");
    filter.save_to_file(&args.output).context("Fallo al escribir el archivo .bin")?;

    let duration = start_time.elapsed();
    println!("--------------------------------------");
    println!("✅ PROCESO COMPLETADO EXITOSAMENTE");
    println!("⏱️ Tiempo Total: {:.2?}", duration);
    println!("📦 Direcciones Procesadas: {}", count);
    println!("📁 Archivo Generado: {:?}", args.output);

    Ok(())
}
