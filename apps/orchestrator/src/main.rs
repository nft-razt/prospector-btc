/**
 * =================================================================
 * APARATO: ORCHESTRATOR MAIN ENTRY POINT (V110.1 - HARDENED)
 * CLASIFICACIÓN: APPLICATION SHELL (ESTRATO L3)
 * RESPONSABILIDAD: BOOTSTRAP DE INFRAESTRUCTURA E IGNICIÓN SEGURA
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
 * Punto de ignición principal con configuración de pila extendida.
 *
 * # Nota de Ingeniería
 * En Windows (modo debug), el tamaño de pila predeterminado puede ser insuficiente
 * para la profundidad de los futures asíncronos. Se utiliza un constructor manual
 * del runtime para garantizar la estabilidad del sistema.
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. CARGA DE ENTORNO
    dotenv().ok();

    // 2. CONFIGURACIÓN DEL RUNTIME SOBERANO
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(4 * 1024 * 1024) // 4MB de pila por hilo (Doble del estándar)
        .build()?;

    runtime.block_on(async {
        init_tracing("prospector_orchestrator");
        info!("🛰️ [COMMAND_CENTER]: Initiating global ignition sequence...");

        // 3. ADQUISICIÓN DE PARÁMETROS
        let database_connection_url = std::env::var("DATABASE_URL")
            .expect("CRITICAL: DATABASE_URL must be defined.");

        let database_authentication_token = std::env::var("TURSO_AUTH_TOKEN").ok();

        let server_network_port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        // 4. IGNICIÓN DEL KERNEL
        let orchestrator_system_kernel = OrchestratorKernel::ignite(
            &database_connection_url,
            database_authentication_token,
            server_network_port
        ).await;

        // 5. PROTOCOLO DE ARQUEOLOGÍA (AUTO-HYDRATION)
        info!("🧬 [FORENSIC_SHIELD]: Verifying system template registry...");
        if let Err(ignition_error) = perform_automatic_forensic_ignition(
            &orchestrator_system_kernel.application_state
        ).await {
            error!("❌ [FATAL_IGNITION_ERROR]: Forensic auto-hydration failed: {}", ignition_error);
            std::process::exit(1);
        }

        // 6. LANZAMIENTO DE OPERACIONES AUTÓNOMAS
        info!("🚀 [ORCHESTRATOR_ONLINE]: Swarm Control active on port {}", server_network_port);
        orchestrator_system_kernel.launch_autonomous_ops().await;

        Ok(())
    })
}
