// apps/orchestrator/src/main.rs
// =================================================================
// APARATO: ORCHESTRATOR KERNEL (v5.6 - RESILIENCE EDITION)
// RESPONSABILIDAD: BOOTSTRAP, INYECCIÓN DE DEPENDENCIAS Y STARTUP
// ESTADO: FULL ASYNC RUNTIME & BACKGROUND DAEMONS (FLUSH ENABLED)
// =================================================================

use dotenvy::dotenv;
use std::net::SocketAddr;
use std::process;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing::{error, info};

// --- MÓDULOS INTERNOS ---
// Definición de la estructura modular del monolito
mod bootstrap;
mod handlers;
mod middleware;
mod routes;
mod services;
mod state;

#[cfg(test)]
mod tests;

// --- IMPORTACIONES DE INFRAESTRUCTURA Y DOMINIO ---
use prospector_infra_db::TursoClient;
use prospector_shared_heimdall::init_tracing;

use crate::bootstrap::Bootstrap;
use crate::state::AppState;

// --- IMPORTACIÓN DE SERVICIOS DE FONDO (DAEMONS) ---
use crate::services::{
    chronos::spawn_chronos,
    flush::spawn_flush_service, // ✅ NUEVO: Servicio de persistencia por lotes
    reaper::spawn_reaper,
    telemetry::spawn_telemetry_loop,
};

#[tokio::main]
async fn main() {
    // 1. INICIALIZACIÓN DE ENTORNO
    // Carga variables del .env si existe (Desarrollo Local)
    dotenv().ok();

    // Inicializa el sistema de logging estructurado (Heimdall)
    init_tracing("prospector_orchestrator");

    info!("🚀 SYSTEM STARTUP: ORCHESTRATOR ONLINE [HYDRA-ZERO V5.6]");

    // 2. CONEXIÓN A BASE DE DATOS (PERSISTENCIA)
    // Establecemos la conexión crítica con Turso. Si esto falla, el sistema
    // no puede garantizar la integridad del ledger, por lo que abortamos.
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    let db_client = match TursoClient::connect(&db_url, db_token).await {
        Ok(c) => {
            info!("✅ Conexión DB establecida (Turso/libSQL).");
            c
        }
        Err(e) => {
            error!("❌ FALLO CRÍTICO DB: {}", e);
            process::exit(1);
        }
    };

    // 3. INICIALIZACIÓN DEL ESTADO GLOBAL
    // Memoria compartida (RwLock), Bus de Eventos y Buffer de Escritura.
    let state = AppState::new(db_client);

    // 4. DIAGNÓSTICO DE INTEGRIDAD (PRE-FLIGHT)
    // Verifica la existencia del filtro UTXO y la configuración crítica.
    // Si falla, el sistema entra en modo "Mantenimiento" pero no crashea.
    Bootstrap::run_diagnostics(&state);

    // 5. ACTIVACIÓN DE SERVICIOS EN SEGUNDO PLANO (BACKGROUND DAEMONS)
    // Estos procesos corren concurrentemente al servidor HTTP.

    // A. REAPER: Limpiador de memoria (Garbage Collector de Snapshots).
    spawn_reaper(state.clone()).await;

    // B. TELEMETRY: Agregador de métricas en tiempo real y emisor SSE.
    spawn_telemetry_loop(state.clone()).await;

    // C. FLUSH: Persistencia diferida (Circuit Breaker DB).
    // Drena el buffer de heartbeats hacia la base de datos en lotes.
    spawn_flush_service(state.clone()).await;

    // D. CHRONOS: Marcapasos para evitar suspensión en entornos Serverless.
    let public_url = std::env::var("RENDER_EXTERNAL_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    spawn_chronos(public_url).await;

    // 6. CONFIGURACIÓN DEL SERVIDOR HTTP (AXUM)
    // Configuración de capas de red y seguridad básica.
    let cors = CorsLayer::permissive(); // TODO: Restringir en Producción estricta

    // Servicio de archivos estáticos para que los workers descarguen 'utxo_filter.bin'
    let static_files = ServeDir::new(".");

    let app = routes::create_router(state)
        .nest_service("/resources", static_files)
        .layer(cors);

    // 7. BIND & SERVE
    // Escucha en el puerto definido por la nube (Render/Koyeb) o default 3000.
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("📡 Orchestrator escuchando tráfico en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Bucle principal del servidor
    if let Err(e) = axum::serve(listener, app).await {
        error!("💀 FALLO CRÍTICO DEL SERVIDOR HTTP: {}", e);
        process::exit(1);
    }
}
