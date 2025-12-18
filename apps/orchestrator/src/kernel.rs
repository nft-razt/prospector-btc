// apps/orchestrator/src/kernel.rs
// =================================================================
// APARATO: ORCHESTRATOR KERNEL (V17.0)
// CLASIFICACIÓN: APPLICATION LAYER (L1) // COMPOSITION ROOT
// RESPONSABILIDAD: ENSAMBLAJE DE SUBSISTEMAS Y GESTIÓN DE TRÁFICO
//
// ESTRATEGIA DE ÉLITE:
// - Desacoplamiento de migraciones: El esquema es validado, no alterado.
// - Arranque Asíncrono: Liveness probe inmediata para Render/K8s.
// - Higiene Aritmética: Soporte para validación de campos archivados V7.0.
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
// ✅ RESOLUCIÓN: Importación calificada de Anyhow para gestión de errores semánticos
use anyhow::{Result, Context};

/// El Núcleo central del Orquestador Hydra.
///
/// Esta estructura actúa como el punto de composición (Composition Root)
/// donde se inyectan las dependencias de infraestructura y se lanzan
/// los servicios de fondo que mantienen la salud del enjambre.
pub struct OrchestratorKernel {
    /// Puerto de red para el servidor de API.
    network_port: u16,
    /// Estado compartido de la aplicación (AppState).
    application_state: AppState,
}

impl OrchestratorKernel {
    /// Inicializa una instancia del Kernel estableciendo los enlaces tácticos.
    ///
    /// # Proceso de Ignición
    /// 1. Conecta con la Bóveda Táctica (Turso/libSQL).
    /// 2. Valida que la estructura de la base de datos sea compatible (V7.0+).
    /// 3. Inicia la secuencia de diagnóstico asíncrona.
    ///
    /// # Errores
    /// Retorna un fallo fatal si la base de datos es inalcanzable o el esquema es incompatible.
    pub async fn ignite(
        database_url: &str,
        authentication_token: Option<String>,
        network_port: u16
    ) -> Self {
        // 1. Establecimiento de enlace con Turso
        let database_client = match TursoClient::connect(database_url, authentication_token).await {
            Ok(client) => {
                info!("✅ KERNEL: Tactical Database link secured.");
                client
            }
            Err(error) => {
                error!("❌ KERNEL_FATAL: Database connection failed: {}", error);
                std::process::exit(1);
            }
        };

        // 2. Construcción del estado neural
        let application_state = AppState::new(database_client);

        // 3. Auditoría de Integridad Estructural
        // ✅ RESOLUCIÓN: Uso de Result calificado para validación de esquema
        if let Err(error) = Self::verify_database_integrity(&application_state).await {
            error!("💀 INTEGRITY_ERROR: Schema mismatch. Details: {}", error);
            std::process::exit(1);
        }

        // 4. Activación de Diagnóstico en segundo plano (No-bloqueante)
        Bootstrap::spawn_diagnostics(application_state.clone());

        Self {
            network_port,
            application_state
        }
    }

    /// Lanza los servicios de fondo y comienza a servir tráfico HTTP.
    ///
    /// Este método bloquea el hilo principal y es el responsable de la
    /// orquestación de liveness del contenedor en Render.
    pub async fn launch(self) {
        let state_handle = self.application_state.clone();

        info!("⚙️  KERNEL: Deploying background maintenance daemons...");

        // A. REAPER: Recolección de hilos y memoria huérfana
        spawn_reaper(state_handle.clone()).await;

        // B. TELEMETRY: Agregación de pulsos del enjambre para SSE
        spawn_telemetry_loop(state_handle.clone()).await;

        // C. FLUSH: Persistencia diferida (Write-Behind) para Turso
        spawn_flush_service(state_handle.clone()).await;

        // D. CHRONOS: Autopreservación y marcapasos de la instancia
        let public_endpoint = std::env::var("RENDER_EXTERNAL_URL")
            .unwrap_or_else(|_| format!("http://localhost:{}", self.network_port));
        spawn_chronos(public_endpoint).await;

        // E. ROUTING MATRIX: Configuración de Axum y recursos estáticos
        let cors_policy = CorsLayer::permissive(); // TODO: Ajustar para entornos restringidos
        let static_file_service = ServeDir::new("resources");

        let application_router = routes::create_router(state_handle)
            .nest_service("/resources", static_file_service)
            .layer(cors_policy);

        // F. BIND & SERVE
        let socket_address = SocketAddr::from(([0, 0, 0, 0], self.network_port));
        info!("📡 ORCHESTRATOR ONLINE: Awaiting traffic at {}", socket_address);

        let tcp_listener = tokio::net::TcpListener::bind(socket_address)
            .await
            .expect("FATAL: Failed to bind to network interface.");

        if let Err(error) = axum::serve(tcp_listener, application_router).await {
            error!("💀 KERNEL_CRASH: Server malfunction: {}", error);
            std::process::exit(1);
        }
    }

    /// Realiza una verificación pasiva de las tablas del Ledger.
    ///
    /// Asegura que el binario de la API sea compatible con el estado actual
    /// de la base de datos sin intentar realizar migraciones destructivas.
    async fn verify_database_integrity(state: &AppState) -> Result<()> {
        let connection = state.db.get_connection()
            .map_err(|error| anyhow::anyhow!("Pool link failure: {}", error))?;

        // Validamos la existencia del campo 'archived_at' introducido en la V7.0
        connection.query("SELECT archived_at FROM jobs LIMIT 1", ())
            .await
            .context("DATABASE_OUT_OF_SYNC: Table 'jobs' missing archival metadata.")?;

        Ok(())
    }
}
