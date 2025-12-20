/**
 * =================================================================
 * APARATO: CENSUS TAKER SHELL (V10.8 - SOBERANO)
 * CLASIFICACIÓN: APPLICATION LAYER (ENTRY POINT)
 * RESPONSABILIDAD: GESTIÓN DE ARGUMENTOS Y DISPARO DE PIPELINE
 *
 * ESTRATEGIA DE ÉLITE:
 * - Explicit Naming: Definición de argumentos sin abreviaciones.
 * - Zero-Regression: Mantiene la compatibilidad con el motor V40.2.
 * =================================================================
 */

mod pipeline;

use crate::pipeline::IngestionPipeline;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author = "Raz Podesta <metaShark Tech>",
    version = "10.8",
    about = "Cartógrafo Criptográfico: Cristaliza el censo UTXO en filtros binarios particionados."
)]
struct CommandArguments {
    /// Ruta física del archivo CSV descargado de BigQuery.
    #[arg(short, long)]
    input: PathBuf,

    /// Carpeta donde se guardarán los fragmentos (.bin) resultantes.
    #[arg(short, long, default_value = "dist/filters")]
    output_directory: PathBuf,

    /// Volumen nominal de registros esperados (margen de seguridad).
    #[arg(short, long, default_value_t = 1_000_000)]
    size: usize,

    /// Tasa de falsos positivos (False Positive Rate) para la red distribuida.
    /// ✅ NIVELACIÓN: Nombre explícito sin abreviaciones.
    #[arg(long = "false-positive-rate", default_value_t = 0.0000001)]
    false_positive_rate: f64,

    /// Cantidad de particiones deterministas (Sharding).
    #[arg(short, long, default_value_t = 4)]
    shards: usize,
}

fn main() -> Result<()> {
    let configuration = CommandArguments::parse();

    // Inyección de dependencias hacia el motor de procesamiento táctico
    let ingestion_engine = IngestionPipeline::new(
        &configuration.input,
        &configuration.output_directory,
        configuration.size,
        configuration.shards,
        configuration.false_positive_rate,
    );

    // Ejecución de la secuencia nivelada
    ingestion_engine.execute_ingestion_sequence()
}
