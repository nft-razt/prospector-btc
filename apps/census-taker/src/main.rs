// apps/census-taker/src/main.rs
// =================================================================
// APARATO: CENSUS TAKER SHELL
// RESPONSABILIDAD: INTERFAZ CLI Y CONFIGURACIÓN
// =================================================================

mod pipeline; // ✅ MÓDULO IMPORTADO

use clap::Parser;
use anyhow::Result;
use std::path::PathBuf;
use crate::pipeline::IngestionPipeline;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generador de filtros particionados de alto rendimiento")]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long, default_value = "dist/filters")]
    output_dir: PathBuf,

    #[arg(long, default_value_t = 50_000_000)]
    size: usize,

    #[arg(long, default_value_t = 0.0000001)]
    fp_rate: f64,

    #[arg(long, default_value_t = 4)]
    shards: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Inyección de dependencias simple hacia el motor
    let engine = IngestionPipeline::new(
        &args.input,
        &args.output_dir,
        args.size,
        args.shards,
        args.fp_rate
    );

    // Delegación de ejecución
    engine.execute()
}
