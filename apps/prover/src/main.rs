/**
 * =================================================================
 * APARATO: PROVER SYSTEM SHELL (V36.0 - LOGGING CERTIFIED)
 * CLASIFICACIÓN: APPLICATION LAYER (ENTRY POINT)
 * RESPONSABILIDAD: ORQUESTACIÓN DE LA FORJA DE ESCENARIOS DE PRUEBA
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la interfaz de mando para la generación de "Golden Tickets".
 * Esta versión garantiza que el sistema de observación (Tracing) esté
 * correctamente inicializado antes de disparar la derivación
 * criptográfica, permitiendo auditoría forense en tiempo real.
 * =================================================================
 */

mod forge;

use crate::forge::ScenarioForge;
use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

/**
 * Estructura de argumentos de comando para el Prover.
 */
#[derive(Parser, Debug)]
#[command(
    author = "Raz Podesta <metaShark Tech>",
    version = "10.9",
    about = "Generador de Golden Tickets: Certificación de integridad para el enjambre Hydra-Zero."
)]
struct CommandArguments {
    /// Carpeta de destino donde se guardarán los fragmentos (shards) de prueba.
    #[arg(short = 'o', long = "output-directory", default_value = "dist/filters_proof")]
    target_output_directory: PathBuf,

    /// Prefijo alfanumérico para la frase semilla (Entropy Source).
    #[arg(short = 'p', long = "prefix", default_value = "GOLDEN_VECTOR_")]
    seed_phrase_prefix: String,

    /// Identificador numérico u hexadecimal que se inyectará en el rango de búsqueda.
    #[arg(short = 't', long = "target-identifier", default_value = "777")]
    target_numerical_identifier: String,

    /// Nivel de verbosidad para la auditoría de logs.
    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    is_verbose_mode_enabled: bool,
}

/**
 * Punto de ignición principal del sistema Prover.
 */
fn main() -> Result<()> {
    // 1. CONFIGURACIÓN DEL SISTEMA DE OBSERVABILIDAD (Sincronizado con L6)
    // Se inicializa el suscriptor con soporte para variables de entorno RUST_LOG
    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false).compact())
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    info!("🧪 [PROVER_IGNITION]: Starting Sovereign Certification Sequence...");

    // 2. PARSING DE ARGUMENTOS TÁCTICOS
    let configuration = CommandArguments::parse();

    if configuration.is_verbose_mode_enabled {
        info!("🔍 [DEBUG]: Configuration loaded successfully.");
        info!("   -> Output: {:?}", configuration.target_output_directory);
        info!("   -> Prefix: {}", configuration.seed_phrase_prefix);
        info!("   -> Target: {}", configuration.target_numerical_identifier);
    }

    // 3. INSTANCIACIÓN DEL MOTOR DE FORJA (ScenarioForge)
    let forging_engine = ScenarioForge::new(
        &configuration.target_output_directory,
        &configuration.seed_phrase_prefix,
        &configuration.target_numerical_identifier,
    );

    // 4. EJECUCIÓN DE LA SECUENCIA DE CRISTALIZACIÓN
    match forging_engine.execute_forging_sequence() {
        Ok(mission_identifier) => {
            info!("🏁 [SEQUENCE_COMPLETE]: Golden Ticket '{}' crystallized successfully.", mission_identifier);
            Ok(())
        }
        Err(fatal_error) => {
            error!("🔥 [FORGE_COLLAPSE]: Critical fault detected: {}", fatal_error);
            Err(fatal_error).context("FAILED_TO_EXECUTE_FORGING_SEQUENCE")
        }
    }
}
