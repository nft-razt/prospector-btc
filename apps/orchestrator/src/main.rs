// apps/orchestrator/src/main.rs
// =================================================================
// APARATO: ORCHESTRATOR SHELL
// RESPONSABILIDAD: CARGA DE ENTORNO Y BOOTSTRAP DEL KERNEL
// =================================================================

mod bootstrap;
mod handlers;
mod kernel;
mod middleware;
mod routes;
mod services;
mod state; // ✅ MÓDULO IMPORTADO

#[cfg(test)]
mod tests;

use crate::kernel::OrchestratorKernel;
use dotenvy::dotenv;
use prospector_shared_heimdall::init_tracing;

#[tokio::main]
async fn main() {
    // 1. Carga de Variables de Entorno
    dotenv().ok();

    // 2. Sistema de Observabilidad
    init_tracing("prospector_orchestrator");

    // 3. Configuración
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_token = std::env::var("TURSO_AUTH_TOKEN").ok();
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);

    // 4. Inyección y Despegue
    // Delegamos toda la lógica compleja al Kernel atomizado.
    let system = OrchestratorKernel::ignite(&db_url, db_token, port).await;
    system.launch().await;
}
