// apps/orchestrator/src/main.rs
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::process;
use tracing::{info, error, warn};
use prospector_infra_db::TursoClient;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use prospector_shared_heimdall::init_tracing; // Utilidad centralizada

mod state;
mod handlers;
mod routes;
mod middleware;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    // 1. Entorno
    dotenv().ok();

    // 2. Observabilidad Unificada
    init_tracing("prospector_orchestrator");

    info!("🚀 SYSTEM STARTUP: ORCHESTRATOR ONLINE [HYDRA-ZERO]");

    // 3. Infraestructura de Datos
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "file:prospector.db".to_string());
    let db_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    let db_client = match TursoClient::connect(&db_url, db_token).await {
        Ok(c) => { info!("✅ Conexión DB establecida: {}", db_url); c },
        Err(e) => {
            error!("❌ FALLO CRÍTICO DB: {}", e);
            process::exit(1);
        }
    };

    let state = AppState::new(db_client);

    // Configuración CORS Permisiva para Workers Distribuidos
    let cors = CorsLayer::permissive();

    // 4. Logística de Archivos (Filtro UTXO)
    let public_path = "public";
    if !std::path::Path::new(public_path).exists() {
        warn!("⚠️  Directorio '{}' no encontrado. Creándolo vacío para evitar crash.", public_path);
        std::fs::create_dir_all(public_path).unwrap_or_default();
    }
    let static_files = ServeDir::new(public_path);

    // 5. Construcción del Router
    let app = routes::create_router(state)
        .nest_service("/resources", static_files)
        .layer(cors);

    // 6. Lanzamiento
    let port = std::env::var("PORT").unwrap_or("3000".into()).parse().unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("📡 Orchestrator escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    if let Err(e) = axum::serve(listener, app).await {
        error!("❌ Error en runtime del servidor: {}", e);
        process::exit(1);
    }
}
