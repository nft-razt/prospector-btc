/**
 * =================================================================
 * APARATO: ORCHESTRATOR MAIN ENTRY POINT (V110.5 - SOBERANO)
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
 * Punto de ignición principal con configuración de memoria optimizada.
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. CARGA DE ENTORNO (Soporte para .env local)
    dotenv().ok();

    // 2. INICIALIZACIÓN DEL SISTEMA DE TRAZADO (HEIMDALL)
    // Se ejecuta primero para capturar logs de configuración.
    init_tracing("prospector_orchestrator");

    // 3. CONFIGURACIÓN DEL RUNTIME SOBERANO (TOKIO)
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(4 * 1024 * 1024) // 4MB para evitar desbordamientos en cálculos profundos
        .build()?;

    runtime.block_on(async {
        info!("🛰️ [COMMAND_CENTER]: Global ignition sequence starting...");

        // 4. ADQUISICIÓN Y VALIDACIÓN DE PARÁMETROS CRÍTICOS
        // Si fallan aquí, tenemos logs gracias a init_tracing.
        let database_url = std::env::var("DATABASE_URL")
            .expect("CRITICAL_FAULT: DATABASE_URL not defined.");

        let database_token = std::env::var("TURSO_AUTH_TOKEN").ok();

        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        // 5. CONSTRUCCIÓN DEL KERNEL
        // El Kernel orquestará el estado, la DB y los daemons de fondo.
        let kernel = OrchestratorKernel::ignite(
            &database_url,
            database_token,
            port
        ).await;

        // 6. PROTOCOLO DE ARQUEOLOGÍA (AUTO-HYDRATION)
        // Garantiza que la base de datos tenga las semillas iniciales de Windows XP.
        info!("🧬 [FORENSIC_SHIELD]: Verifying cryptographic registries...");
        if let Err(hydration_error) = perform_automatic_forensic_ignition(
            &kernel.application_state
        ).await {
            error!("❌ [HYDRATION_FAILED]: Forensic initialization collapsed: {}", hydration_error);
            std::process::exit(1);
        }

        // 7. IGNICIÓN DE OPERACIONES AUTÓNOMAS
        info!("🚀 [PROSPECTOR_ONLINE]: System fully operational on port {}", port);
        kernel.launch_autonomous_ops().await;

        Ok(())
    })
}
