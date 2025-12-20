/**
 * =================================================================
 * APARATO: ORCHESTRATOR MAIN ENTRY POINT (V105.0 - SOBERANO)
 * CLASIFICACIÓN: APPLICATION SHELL (ESTRATO L3)
 * RESPONSABILIDAD: BOOTSTRAP DE INFRAESTRUCTURA Y IGNICIÓN FORENSE
 *
 * VISION HIPER-HOLÍSTICA:
 * Este archivo es el disparador primario del ecosistema Prospector.
 * Realiza una secuencia de arranque en tres fases:
 * 1. Carga y validación del entorno (Environment Audit).
 * 2. Hidratación de la Bóveda Genética (Forensic Ignition).
 * 3. Lanzamiento del Kernel de servicios (Neural Link Launch).
 *
 * ESTRATEGIA DE ÉLITE:
 * - Zero-Abbreviations: Nomenclatura descriptiva total para rigor académico.
 * - Fault Isolation: Si la ignición forense falla, el sistema aborta para
 *   prevenir misiones corruptas.
 * - Async-Runtime Orchestration: Gestión de hilos mediante Tokio.
 * =================================================================
 */

mod bootstrap;
mod bootstrap_forensics; // Nuevo aparato de auto-hidratación Satoshi-XP
mod handlers;
mod kernel;
mod middleware;
mod routes;
mod services;
mod state;

use crate::kernel::OrchestratorKernel;
use crate::bootstrap_forensics::perform_automatic_forensic_ignition;
use dotenvy::dotenv;
use prospector_shared_heimdall::init_tracing;
use tracing::{info, error};

/**
 * Punto de ignición principal del servidor Orquestador.
 * Ejecuta la secuencia imperativa de preparación antes de servir tráfico.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. AUDITORÍA DE ENTORNO
    // Carga las variables desde el archivo .env (Desarrollo) o entorno (Render).
    dotenv().ok();

    // 2. SISTEMA DE OBSERVABILIDAD (HEIMDALL)
    // Inicializa los logs estructurados JSON para monitoreo estratégico.
    init_tracing("prospector_orchestrator");

    info!("🛰️ [COMMAND_CENTER]: Initiating global ignition sequence...");

    // 3. ADQUISICIÓN DE PARÁMETROS DE CONFIGURACIÓN
    let database_connection_url = std::env::var("DATABASE_URL")
        .expect("CRITICAL: DATABASE_URL must be defined in the environment.");

    let database_authentication_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    let server_network_port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);

    // 4. IGNICIÓN DEL KERNEL Y ENLACE TÁCTICO (Turso)
    // Se establece la conexión con la base de datos de alta frecuencia.
    let orchestrator_system_kernel = OrchestratorKernel::ignite(
        &database_connection_url,
        database_authentication_token,
        server_network_port
    ).await;

    // 5. PROTOCOLO DE ARQUEOLOGÍA FORENSE (AUTO-HYDRATION)
    // Antes de lanzar el servidor, garantizamos que el ADN de Windows XP esté en la DB.
    // Esto cumple la visión de "Ignición Autónoma" discutida.
    info!("🧬 [FORENSIC_SHIELD]: Verifying system template registry...");

    if let Err(ignition_error) = perform_automatic_forensic_ignition(
        &orchestrator_system_kernel.application_state
    ).await {
        error!("❌ [FATAL_IGNITION_ERROR]: Forensic auto-hydration failed: {}", ignition_error);
        std::process::exit(1);
    }

    // 6. LANZAMIENTO DEL NEURAL LINK (API & DAEMONS)
    // El sistema comienza a escuchar peticiones del enjambre y el Dashboard.
    info!("🚀 [ORCHESTRATOR_ONLINE]: Swarm Control Protocol active on port {}", server_network_port);

    orchestrator_system_kernel.launch().await;

    Ok(())
}
