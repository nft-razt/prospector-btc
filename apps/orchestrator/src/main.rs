// apps/orchestrator/src/main.rs
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::process;
use tracing::{info, error};
use prospector_infra_db::TursoClient;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use prospector_shared_heimdall::init_tracing;

mod state;
mod handlers;
mod routes;
mod middleware;
mod services;
mod bootstrap; // NUEVO MODULO

#[cfg(test)]
mod tests;

use crate::state::AppState;
use crate::services::{reaper::spawn_reaper, chronos::spawn_chronos};
use crate::bootstrap::Bootstrap;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing("prospector_orchestrator");

    info!("🚀 SYSTEM STARTUP: ORCHESTRATOR ONLINE [HYDRA-ZERO V5.0]");

    // 1. Conexión a Base de Datos (Critico: Si falla, exit)
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    let db_client = match TursoClient::connect(&db_url, db_token).await {
        Ok(c) => { info!("✅ Conexión DB establecida."); c },
        Err(e) => { error!("❌ FALLO CRÍTICO DB: {}", e); process::exit(1); }
    };

    // 2. Inicialización de Estado
    let state = AppState::new(db_client);

    // 3. Diagnóstico no bloqueante
    Bootstrap::run_diagnostics(&state);

    // 4. Servicios Background
    spawn_reaper(state.clone()).await;
    let public_url = std::env::var("RENDER_EXTERNAL_URL").unwrap_or("http://localhost:3000".into());
    spawn_chronos(public_url).await;

    // 5. Configuración Web
    let cors = CorsLayer::permissive(); // TODO: Restringir en Prod si es necesario
    let static_files = ServeDir::new(".");

    let app = routes::create_router(state)
        .nest_service("/resources", static_files)
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or("3000".into()).parse().unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("📡 Orchestrator escuchando en {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
