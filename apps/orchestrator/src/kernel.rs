/**
 * =================================================================
 * APARATO: ORCHESTRATOR SOVEREIGN KERNEL (V340.0 - FULL SYNC)
 * CLASIFICACIÓN: COMPOSITION ROOT (ESTRATO L1-APP)
 * RESPONSABILIDAD: ENSAMBLAJE E IGNICIÓN SIN RUIDOS NI ERRORES
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el centro neurálgico de ignición. Coordina el arranque
 * de diagnósticos, guardianes de integridad y daemons de fondo,
 * asegurando que la API soberana esté lista para el mando distribuido.
 * =================================================================
 */

use crate::state::AppState;
use crate::routes::create_router;
use crate::bootstrap::Bootstrap;
use crate::services::{
    mission_hydrator::MissionHydratorService,
    finding_flusher::FindingFlusherService,
    swarm_resurrection::SwarmResurrectionService,
    certification_authority::CertificationAuthorityService,
    parity_auditor::ArchivalParityAuditor,
    chronos_archive::spawn_strategic_archival_bridge,
    OutboxRelayService, // Alias de SovereignArchivalEngine
    spawn_chronos,
    spawn_flush_service,
    spawn_reaper,
    spawn_telemetry_loop,
};
use prospector_infra_db::TursoClient;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

pub struct OrchestratorKernel {
    /// Puerto físico asignado para el servicio de red.
    pub server_network_port: u16,
    /// Estado neural atomizado de la aplicación.
    pub application_state: AppState,
}

impl OrchestratorKernel {
    /**
     * Establece el enlace táctico inicial con el Ledger Táctico (Turso).
     */
    pub async fn ignite(
        database_connection_url: &str,
        database_access_token: Option<String>,
        listening_port: u16
    ) -> Self {
        let database_client = TursoClient::connect(database_connection_url, database_access_token)
            .await
            .expect("FATAL: Database tactical link failure.");

        Self {
            server_network_port: listening_port,
            application_state: AppState::new(database_client),
        }
    }

    /**
     * Lanza la red de servicios autónomos y el servidor de mando central.
     *
     * # Protocolo de Ignición
     * 1. Lanza diagnósticos asíncronos de pre-vuelo.
     * 2. Activa la Autoridad de Certificación para validación de Golden Vectors.
     * 3. Despliega daemons de hidratación, persistencia y archivo estratégico.
     * 4. Inicia el servidor Axum en el puerto configurado.
     */
    pub async fn launch_autonomous_ops(self) {
        let shared_application_state = self.application_state.clone();

        // 1. DIAGNÓSTICO Y PRE-VUELO
        Bootstrap::spawn_diagnostics(shared_application_state.clone());

        // 2. GUARDIÁN DE INTEGRIDAD (L4)
        let certification_authority = Arc::new(
            CertificationAuthorityService::new(shared_application_state.clone())
        );
        certification_authority.spawn_integrity_listener().await;

        // 3. DAEMONS DE MISIÓN Y TÁCTICA
        let mission_hydrator = MissionHydratorService::new(shared_application_state.clone());
        tokio::spawn(async move { mission_hydrator.spawn_hydrator_daemon().await; });

        let finding_flusher = FindingFlusherService::new(shared_application_state.clone());
        tokio::spawn(async move { finding_flusher.spawn_flusher_daemon().await; });

        let swarm_resurrection = SwarmResurrectionService::new(shared_application_state.clone());
        tokio::spawn(async move { swarm_resurrection.spawn_resurrection_daemon().await; });

        // 4. ARCHIVO ESTRATÉGICO Y SINAPSIS CON MOTOR B
        // ✅ RESOLUCIÓN E0599: Sincronizado con SovereignArchivalEngine V110.0
        let archival_relay = OutboxRelayService::new(shared_application_state.clone());
        tokio::spawn(async move { archival_relay.spawn_archival_loop().await; });

        let archival_parity_auditor = ArchivalParityAuditor::new(shared_application_state.clone());
        tokio::spawn(async move { archival_parity_auditor.spawn_auditor_daemon().await; });

        spawn_strategic_archival_bridge(shared_application_state.clone()).await;

        // 5. MANTENIMIENTO VITAL Y TELEMETRÍA (L4)
        spawn_flush_service(shared_application_state.clone()).await;
        spawn_reaper(shared_application_state.clone()).await;
        spawn_telemetry_loop(shared_application_state.clone()).await;

        // Preservación de instancia ante timeouts de la nube
        let render_url = std::env::var("RENDER_EXTERNAL_URL")
            .unwrap_or_else(|_| format!("http://localhost:{}", self.server_network_port));
        spawn_chronos(render_url).await;

        // 6. IGNICIÓN DEL SERVIDOR DE RED (AXUM)
        let sovereign_router = create_router(shared_application_state);
        let bind_address = SocketAddr::from(([0, 0, 0, 0], self.server_network_port));

        info!("🚀 [KERNEL_ONLINE]: Sovereign C2 ready at {}", bind_address);

        let tcp_listener = tokio::net::TcpListener::bind(bind_address)
            .await
            .expect("CRITICAL: Network port binding fault");

        axum::serve(tcp_listener, sovereign_router)
            .await
            .expect("CRITICAL: Server runtime collapse");
    }
}
