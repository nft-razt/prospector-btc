/**
 * =================================================================
 * APARATO: ORCHESTRATOR COMPOSITION ROOT (V30.0 - FINAL SEAL)
 * CLASIFICACIÓN: APPLICATION LAYER (L1)
 * RESPONSABILIDAD: ENSAMBLAJE DE SUBSISTEMAS Y GESTIÓN DE IGNICIÓN
 *
 * ESTRATEGIA DE ÉLITE:
 * - Stratum Injection: Vincula la persistencia Táctica (Turso) con la Estratégica (Supabase).
 * - Asynchronous Life-Support: Lanza Daemons de mantenimiento sin bloquear el I/O.
 * - Fault Isolation: El fallo de un servicio de fondo no compromete el Kernel.
 * =================================================================
 */

use crate::state::AppState;
use crate::services::{
    chronos::spawn_chronos,
    flush::spawn_flush_service,
    reaper::spawn_reaper,
    telemetry::spawn_telemetry_loop,
    chronos_archive::spawn_strategic_archival_service, // Nuevo puente L4
};
use crate::routes;
use prospector_infra_db::TursoClient;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::{info, error};

pub struct OrchestratorKernel {
    network_port: u16,
    application_state: AppState,
}

impl OrchestratorKernel {
    /**
     * Inicializa el Kernel estableciendo los enlaces de datos y el estado neural.
     */
    pub async fn ignite(database_url: &str, auth_token: Option<String>, port: u16) -> Self {
        // 1. Enlace con Bóveda Táctica (L3)
        let database_client = TursoClient::connect(database_url, auth_token)
            .await
            .expect("FATAL: Tactical Database link failure.");

        // 2. Construcción del AppState (Neural Base)
        let application_state = AppState::new(database_client);

        Self {
            network_port: port,
            application_state,
        }
    }

    /**
     * Despliega la red y comienza a servir tráfico del enjambre.
     */
    pub async fn launch(self) {
        let state_handle = self.application_state.clone();

        info!("🛡️ [KERNEL]: Initiating Swarm Control Protocol...");

        // --- LANZAMIENTO DE SERVICIOS DE MANTENIMIENTO ---

        // A. REAPER: Higiene de memoria RAM (L1-APP)
        spawn_reaper(state_handle.clone()).await;

        // B. TELEMETRY: Agregación de pulso global para el Dashboard (L5)
        spawn_telemetry_loop(state_handle.clone()).await;

        // C. FLUSH: Persistencia diferida (Write-Behind) para optimización de I/O
        spawn_flush_service(state_handle.clone()).await;

        // D. CHRONOS ARCHIVE: El puente inmutable hacia Supabase (L4)
        // ✅ NIVELACIÓN FINAL: Sincronización del archivo histórico para la tesis
        spawn_strategic_archival_service(state_handle.clone()).await;

        // E. CHRONOS LIVENESS: Marcapasos para evitar spin-down en Render
        let public_url = std::env::var("RENDER_EXTERNAL_URL")
            .unwrap_or_else(|_| format!("http://localhost:{}", self.network_port));
        spawn_chronos(public_url).await;

        // --- CONFIGURACIÓN DE RED Y RUTAS ---
        let application_router = routes::create_router(state_handle);
        let socket_address = SocketAddr::from(([0, 0, 0, 0], self.network_port));

        info!("🚀 [IGNITION_COMPLETE]: Orchestrator online at {}", socket_address);

        let tcp_listener = tokio::net::TcpListener::bind(socket_address)
            .await
            .expect("CRITICAL: Failed to bind network interface.");

        axum::serve(tcp_listener, application_router)
            .await
            .expect("CRITICAL: Server Malfunction.");
    }
}
