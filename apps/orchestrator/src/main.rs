/**
 * =================================================================
 * APARATO: ORCHESTRATOR MAIN ENTRY POINT (V110.0 - SOBERANO)
 * CLASIFICACIÓN: APPLICATION SHELL (ESTRATO L3)
 * RESPONSABILIDAD: BOOTSTRAP DE INFRAESTRUCTURA E IGNICIÓN FORENSE
 *
 * VISION HIPER-HOLÍSTICA:
 * Este archivo es el disparador primario del ecosistema Prospector.
 * Realiza una secuencia de arranque en tres fases:
 * 1. Auditoría de Entorno: Carga de secretos y variables operativas.
 * 2. Hidratación Forense: Garantiza el registro del ADN de Windows XP.
 * 3. Lanzamiento del Kernel: Despliegue de la red asíncrona de mando.
 * =================================================================
 */

mod bootstrap;
mod bootstrap_forensics;
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
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. CARGA DE ENTORNO Y SISTEMA DE OBSERVABILIDAD
    dotenv().ok();
    init_tracing("prospector_orchestrator");

    info!("🛰️ [COMMAND_CENTER]: Initiating global ignition sequence...");

    // 2. ADQUISICIÓN DE PARÁMETROS DE CONFIGURACIÓN
    let database_connection_url = std::env::var("DATABASE_URL")
        .expect("CRITICAL: DATABASE_URL must be defined in the environment.");

    let database_authentication_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    let server_network_port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);

    // 3. IGNICIÓN DEL KERNEL Y ENLACE TÁCTICO
    let orchestrator_system_kernel = OrchestratorKernel::ignite(
        &database_connection_url,
        database_authentication_token,
        server_network_port
    ).await;

    // 4. PROTOCOLO DE ARQUEOLOGÍA FORENSE (AUTO-HYDRATION)
    info!("🧬 [FORENSIC_SHIELD]: Verifying system template registry...");
    if let Err(ignition_error) = perform_automatic_forensic_ignition(
        &orchestrator_system_kernel.application_state
    ).await {
        error!("❌ [FATAL_IGNITION_ERROR]: Forensic auto-hydration failed: {}", ignition_error);
        std::process::exit(1);
    }

    // 5. LANZAMIENTO DEL NEURAL LINK (API & DAEMONS)
    info!("🚀 [ORCHESTRATOR_ONLINE]: Swarm Control Protocol active on port {}", server_network_port);

    // ✅ RESOLUCIÓN E0599: Sincronización con el nombre de método nivelado en kernel.rs
    orchestrator_system_kernel.launch_autonomous_ops().await;

    Ok(())
}
