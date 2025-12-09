// apps/orchestrator/src/main.rs
// =================================================================
// APARATO: ORCHESTRATOR ENTRY POINT
// RESPONSABILIDAD: BOOTSTRAPPING Y ORQUESTACIÓN DE SERVICIOS
// ESTADO: REPARADO (DEPENDENCY INJECTION ORDER FIXED)
// =================================================================

use dotenvy::dotenv;
use std::net::SocketAddr;
use std::process;
use tracing::{info, error, warn};
use prospector_infra_db::TursoClient;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use prospector_shared_heimdall::init_tracing;

mod state;
mod handlers;
mod routes;
mod middleware;
mod services;

#[cfg(test)]
mod tests;

use crate::state::AppState;
use crate::services::reaper::spawn_reaper;
use crate::services::chronos::spawn_chronos;

#[tokio::main]
async fn main() {
    // 1. Entorno
    dotenv().ok();

    // 2. Observabilidad (Heimdall)
    init_tracing("prospector_orchestrator");

    info!("🚀 SYSTEM STARTUP: ORCHESTRATOR ONLINE [HYDRA-ZERO]");

    // 3. Infraestructura de Datos (Conexión Cruda)
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "file:prospector.db".to_string());
    let db_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    let db_client = match TursoClient::connect(&db_url, db_token).await {
        Ok(c) => { info!("✅ Conexión DB establecida: {}", db_url); c },
        Err(e) => {
            error!("❌ FALLO CRÍTICO DB: {}", e);
            process::exit(1);
        }
    };

    // 4. Inicialización del Estado Global (Memoria + DB)
    // CORRECCIÓN: Creamos el estado AQUÍ, antes de lanzar los servicios.
    let state = AppState::new(db_client);

    // 5. Servicios de Fondo (The Undead Logic)

    // A. THE REAPER (Limpia trabajos zombies y RAM)
    // CORRECCIÓN: Ahora pasamos 'state' (AppState), no 'db_client'.
    spawn_reaper(state.clone()).await;

    // B. CHRONOS (Evita que Render duerma al servidor)
    let public_url = std::env::var("RENDER_EXTERNAL_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    spawn_chronos(public_url).await;

    // 6. Configuración Web & Assets
    let cors = CorsLayer::permissive();

    let public_path = "public";
    if !std::path::Path::new(public_path).exists() {
        warn!("⚠️  Directorio '{}' no encontrado. Creándolo vacío.", public_path);
        std::fs::create_dir_all(public_path).unwrap_or_default();
    }
    let static_files = ServeDir::new(public_path);

    // Inyectamos el estado en el router
    let app = routes::create_router(state)
        .nest_service("/resources", static_files)
        .layer(cors);

    // 7. Lanzamiento del Servidor
    let port = std::env::var("PORT").unwrap_or("3000".into()).parse().unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("📡 Orchestrator escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    if let Err(e) = axum::serve(listener, app).await {
        error!("❌ Error en runtime del servidor: {}", e);
        process::exit(1);
    }
}
