// apps/orchestrator/src/main.rs
// =================================================================
// APARATO: ORCHESTRATOR ENTRY POINT (V4.5)
// MEJORA: STARTUP SELF-DIAGNOSTICS & INTEGRITY CHECK
// =================================================================

use dotenvy::dotenv;
use std::net::SocketAddr;
use std::process;
use std::path::Path;
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

/// Verifica la integridad de los activos críticos antes de abrir el puerto.
fn perform_integrity_check() {
    let filter_path = Path::new("utxo_filter.bin");

    if filter_path.exists() {
        match std::fs::metadata(filter_path) {
            Ok(metadata) => {
                let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
                if size_mb < 1.0 {
                    error!("❌ INTEGRITY CHECK FAILED: 'utxo_filter.bin' es demasiado pequeño ({:.2} MB). Posible descarga corrupta.", size_mb);
                    // En producción, esto debería detener el despliegue.
                    if cfg!(not(debug_assertions)) {
                        process::exit(1);
                    }
                } else {
                    info!("✅ INTEGRITY CHECK PASSED: Filter size {:.2} MB.", size_mb);
                }
            },
            Err(e) => error!("❌ Error leyendo metadata del filtro: {}", e),
        }
    } else {
        warn!("⚠️ INTEGRITY WARNING: 'utxo_filter.bin' no encontrado. Los mineros no podrán hidratarse desde este nodo.");
    }
}

#[tokio::main]
async fn main() {
    // 1. Entorno
    dotenv().ok();

    // 2. Observabilidad (Heimdall)
    init_tracing("prospector_orchestrator");

    info!("🚀 SYSTEM STARTUP: ORCHESTRATOR ONLINE [HYDRA-ZERO V4.5]");

    // 3. Autodiagnóstico
    perform_integrity_check();

    // 4. Infraestructura de Datos
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "file:prospector.db".to_string());
    let db_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    let db_client = match TursoClient::connect(&db_url, db_token).await {
        Ok(c) => { info!("✅ Conexión DB establecida: {}", db_url); c },
        Err(e) => {
            error!("❌ FALLO CRÍTICO DB: {}", e);
            process::exit(1);
        }
    };

    // 5. Inicialización del Estado Global
    let state = AppState::new(db_client);

    // 6. Servicios de Fondo (The Undead Logic)
    spawn_reaper(state.clone()).await;

    let public_url = std::env::var("RENDER_EXTERNAL_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    spawn_chronos(public_url).await;

    // 7. Configuración Web & Assets
    let cors = CorsLayer::permissive();

    let public_path = "public";
    if !std::path::Path::new(public_path).exists() {
        // En producción, si usamos un volumen, esto asegura que exista
        std::fs::create_dir_all(public_path).unwrap_or_default();
    }

    // Servimos el directorio raíz para permitir la descarga de 'utxo_filter.bin' si está ahí
    let static_files = ServeDir::new(".");

    // Inyectamos el estado en el router
    let app = routes::create_router(state)
        // Exponemos el filtro bajo /resources/utxo_filter.bin
        .nest_service("/resources", static_files)
        .layer(cors);

    // 8. Lanzamiento del Servidor
    let port = std::env::var("PORT").unwrap_or("3000".into()).parse().unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("📡 Orchestrator escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    if let Err(e) = axum::serve(listener, app).await {
        error!("❌ Error en runtime del servidor: {}", e);
        process::exit(1);
    }
}
