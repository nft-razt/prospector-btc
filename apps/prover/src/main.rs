// apps/prover/src/main.rs
// =================================================================
// APARATO: PROVER SHELL
// RESPONSABILIDAD: INTERFAZ CLI PARA CERTIFICACIÓN
// =================================================================

mod forge; // ✅ MÓDULO IMPORTADO

use anyhow::Result;
use clap::Parser;
use log::info;
use std::path::PathBuf;
use crate::forge::ScenarioForge;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generador de Golden Tickets para certificación de sistema")]
struct Args {
    /// Directorio donde se guardarán los shards de prueba
    #[arg(short, long, default_value = "dist/filters_proof")]
    output: PathBuf,

    /// Prefijo para la Brainwallet (Semilla)
    #[arg(long, default_value = "GOLD")]
    prefix: String,

    /// Número objetivo dentro del rango
    #[arg(long, default_value = "777")]
    target: String,
}

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "info"); }
    }
    env_logger::init();

    let args = Args::parse();

    info!("🧪 INICIANDO SECUENCIA DE CERTIFICACIÓN (PROVER)");

    // Instanciación del Motor
    let forge = ScenarioForge::new(
        &args.output,
        &args.prefix,
        &args.target
    );

    // Ejecución
    forge.execute().map(|_| ())
}
