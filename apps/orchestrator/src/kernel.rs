// apps/orchestrator/src/kernel.rs
// =================================================================
// APARATO: ORCHESTRATOR KERNEL (V3.1 - CLEAN BOOTSTRAP)
// RESPONSABILIDAD: ENSAMBLAJE DE SERVICIOS Y RUTAS HTTP
// PATRÓN: BUILDER / COMPOSITION ROOT
// ESTADO: OPTIMIZED (UNUSED IMPORTS REMOVED)
// =================================================================

use crate::bootstrap::Bootstrap;
use crate::routes;
use crate::services::{
    chronos::spawn_chronos,
    flush::spawn_flush_service,
    reaper::spawn_reaper,
    telemetry::spawn_telemetry_loop,
};
use crate::state::AppState;
use prospector_infra_db::TursoClient;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing::{error, info};

/// El Núcleo del Orquestador.
/// Encapsula la configuración y el ciclo de vida de la aplicación.
pub struct OrchestratorKernel {
    port: u16,
    state: AppState,
}

impl OrchestratorKernel {
    /// Inicializa el Kernel conectando a la infraestructura crítica (DB).
    pub async fn ignite(db_url: &str, db_token: Option<String>, port: u16) -> Self {
        // 1. Conexión a Base de Datos (Persistencia)
        let db_client = match TursoClient::connect(db_url, db_token).await {
            Ok(c) => {
                info!("✅ Conexión DB establecida (Turso/libSQL).");
                c
            }
            Err(e) => {
                error!("❌ FALLO CRÍTICO DB: {}", e);
                std::process::exit(1);
            }
        };

        // 2. Inicialización del Estado Global (Memoria Compartida)
        let state = AppState::new(db_client);

        // 3. Diagnóstico de Integridad (Pre-Flight Check)
        Bootstrap::run_diagnostics(&state);

        Self { port, state }
    }

    /// Lanza los subsistemas y bloquea el hilo principal sirviendo tráfico HTTP.
    pub async fn launch(self) {
        let state = self.state.clone();

        // A. ACTIVACIÓN DE DEMONIOS (Background Services)
        info!("⚙️  Iniciando subsistemas en segundo plano...");

        // Limpiador de memoria RAM
        spawn_reaper(state.clone()).await;

        // Agregador de métricas SSE
        spawn_telemetry_loop(state.clone()).await;

        // Persistencia diferida (Write-Behind)
        spawn_flush_service(state.clone()).await;

        // Marcapasos para entornos Serverless (Render/Koyeb)
        let public_url = std::env::var("RENDER_EXTERNAL_URL")
            .unwrap_or_else(|_| format!("http://localhost:{}", self.port));
        spawn_chronos(public_url).await;

        // B. CONFIGURACIÓN DEL SERVIDOR HTTP (Axum)
        // La configuración de rutas y middleware ocurre dentro de `routes::create_router`
        let cors = CorsLayer::permissive(); // TODO: Restringir en Producción
        let static_files = ServeDir::new("."); // Para descarga de filtros

        let app = routes::create_router(state)
            .nest_service("/resources", static_files)
            .layer(cors);

        // C. BIND & SERVE
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!("📡 ORCHESTRATOR ONLINE: Escuchando tráfico en {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        if let Err(e) = axum::serve(listener, app).await {
            error!("💀 FALLO CRÍTICO DEL SERVIDOR HTTP: {}", e);
            std::process::exit(1);
        }
    }
}
