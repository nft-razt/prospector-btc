/**
 * =================================================================
 * APARATO: ORCHESTRATOR COMPOSITION ROOT (V105.0 - SOBERANO)
 * CLASIFICACIÓN: APPLICATION LAYER (ESTRATO L1)
 * RESPONSABILIDAD: ENSAMBLAJE DE SUBSISTEMAS Y GESTIÓN DE IGNICIÓN
 *
 * VISION HIPER-HOLÍSTICA:
 * El Kernel actúa como el sistema nervioso central del proyecto.
 * Coordina la persistencia táctica (Turso - Motor A) para el enjambre
 * y la persistencia estratégica (Supabase - Motor B) para el archivo
 * inmutable requerido por la Tesis Doctoral.
 *
 * ESTRATEGIA DE ÉLITE:
 * - Public State Sovereignty: Permite la auto-hidratación forense antes del lanzamiento.
 * - Service Segregation: Daemons independientes para mantenimiento de RAM y Base de Datos.
 * - Zero-Abbreviation: Cumplimiento total de la nomenclatura descriptiva académica.
 * =================================================================
 */

use crate::state::AppState;
use crate::services::{
    chronos::spawn_chronos,
    flush::spawn_flush_service,
    reaper::spawn_reaper,
    telemetry::spawn_telemetry_loop,
    chronos_archive::spawn_strategic_archival_bridge,
};
use crate::routes;
use prospector_infra_db::TursoClient;
use std::net::SocketAddr;
use tracing::{info, error};

/// Estructura central que encapsula el estado y la configuración del servidor orquestador.
pub struct OrchestratorKernel {
    /// Puerto de red asignado para la escucha de peticiones HTTP del enjambre y el panel de control.
    pub server_network_port: u16,
    /// Instancia soberana del estado de la aplicación que coordina los enlaces de datos y el bus de eventos.
    pub application_state: AppState,
}

impl OrchestratorKernel {
    /**
     * Inicializa el Kernel estableciendo los enlaces de datos primarios y el estado neural.
     * Realiza el apretón de manos (handshake) inicial con el Ledger Táctico.
     *
     * # Argumentos
     * * `database_connection_url` - Localizador universal para el Motor Táctico (Turso).
     * * `database_authentication_token` - Credencial de seguridad para el túnel libSQL.
     * * `server_network_port` - Puerto de destino para la interfaz de red.
     *
     * # Errors
     * Retorna un pánico controlado si la conexión con la base de datos táctica falla en el arranque.
     */
    pub async fn ignite(
        database_connection_url: &str,
        database_authentication_token: Option<String>,
        server_network_port: u16
    ) -> Self {
        info!("🔌 [KERNEL_IGNITION]: Establishing tactical data link with Motor A...");

        // 1. Establecimiento del enlace con la Bóveda Táctica (L3 Infrastructure)
        let database_client = TursoClient::connect(database_connection_url, database_authentication_token)
            .await
            .expect("FATAL: Tactical Database link failure. System cannot establish persistence strata.");

        // 2. Construcción del Estado de la Aplicación (Neural Base)
        let application_state = AppState::new(database_client);

        Self {
            server_network_port,
            application_state,
        }
    }

    /**
     * Despliega la red de servicios y comienza a servir tráfico para el enjambre Hydra-Zero.
     * Lanza los Daemons de soporte vital en hilos asíncronos desacoplados del flujo principal.
     *
     * # Responsabilidades de Lanzamiento
     * 1. REAPER: Higiene de la memoria volátil (snapshots obsoletos).
     * 2. TELEMETRY: Agregación del pulso global para el panel visual.
     * 3. FLUSH: Persistencia diferida (Write-Behind) para optimización de I/O.
     * 4. CHRONOS ARCHIVE: El puente inmutable hacia el Motor Estratégico (Supabase).
     * 5. CHRONOS LIVENESS: Marcapasos para la preservación de la instancia en la nube.
     */
    pub async fn launch(self) {
        let shared_application_state = self.application_state.clone();

        info!("🛡️ [KERNEL_LAUNCH]: Activating Swarm Life-Support Services...");

        // --- LANZAMIENTO DE ESTRATOS DE MANTENIMIENTO Y ANALÍTICA ---

        // ESTRATO A: Mantenimiento de Memoria RAM (Recolección de basura lógica)
        spawn_reaper(shared_application_state.clone()).await;

        // ESTRATO B: Procesamiento de Telemetría (Poder de cómputo y salud de nodos)
        spawn_telemetry_loop(shared_application_state.clone()).await;

        // ESTRATO C: Motor de Persistencia Diferida (Batch Writing a Turso)
        spawn_flush_service(shared_application_state.clone()).await;

        // ESTRATO D: Puente de Archivo Estratégico (Sincronización L3 -> L4 para la Tesis)
        spawn_strategic_archival_bridge(shared_application_state.clone()).await;

        // ESTRATO E: Servicio de Preservación de Instancia (Evitar suspensión de Render)
        let public_external_url = std::env::var("RENDER_EXTERNAL_URL")
            .unwrap_or_else(|_| format!("http://localhost:{}", self.server_network_port));
        spawn_chronos(public_external_url).await;

        // --- CONFIGURACIÓN DE LA MATRIZ DE RUTAS Y SERVIDOR HTTP ---
        let application_router = routes::create_router(shared_application_state);
        let socket_address = SocketAddr::from(([0, 0, 0, 0], self.server_network_port));

        info!("🚀 [ORCHESTRATOR_ONLINE]: Swarm Control Interface active at {}", socket_address);

        let tcp_listener = tokio::net::TcpListener::bind(socket_address)
            .await
            .expect("CRITICAL: Failed to bind network interface. Port might be occupied.");

        axum::serve(tcp_listener, application_router)
            .await
            .expect("CRITICAL: API Server Malfunction during operational serving.");
    }
}
