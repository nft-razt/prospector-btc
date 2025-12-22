/**
 * =================================================================
 * APARATO: SWARM RESURRECTION SERVICE (V130.0 - ELITE SYNC)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: DETECCIÓN Y REEMPLAZO DE NODOS DEGRADADOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el lazo de control cerrado para la persistencia del hashrate.
 * Escanea el Ledger Táctico en busca de misiones abandonadas (zombies),
 * orquestando el despacho de nuevos trabajadores mediante el C2
 * de GitHub para mantener la capacidad de cómputo nominal.
 * =================================================================
 */

use crate::state::AppState;
use crate::services::c2_coordinator::GitHubCommandCoordinator;
use prospector_infra_db::repositories::MissionRepository;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, warn, error, instrument};

/// Umbral de inactividad para considerar un nodo como abandonado (300 segundos).
const INACTIVITY_ABANDONMENT_THRESHOLD_SECONDS: i64 = 300;
/// Ciclo de escrutinio del monitor de resurrección (cada 60 segundos).
const MONITORING_CYCLE_DURATION_SECONDS: u64 = 60;

pub struct SwarmResurrectionService {
    /// Referencia al estado neural atomizado de la aplicación.
    application_state: AppState,
}

impl SwarmResurrectionService {
    /**
     * Construye una nueva instancia del servicio de autocuración de flota.
     * @param application_state Estado compartido inyectado.
     */
    pub fn new(application_state: AppState) -> Self {
        Self { application_state }
    }

    /**
     * Lanza el daemon de vigilancia autónoma en el runtime asíncrono.
     */
    pub async fn spawn_resurrection_daemon(self) {
        let mut monitoring_timer = interval(Duration::from_secs(MONITORING_CYCLE_DURATION_SECONDS));

        info!(
            "🩺 [RESURRECTION]: Swarm Monitor engaged. Inactivity limit: {}s",
            INACTIVITY_ABANDONMENT_THRESHOLD_SECONDS
        );

        loop {
            monitoring_timer.tick().await;

            // Verificamos si el enjambre está autorizado para operar
            if !self.application_state.is_mission_acquisition_authorized() {
                continue;
            }

            if let Err(recovery_error) = self.execute_recovery_sweep_sequence().await {
                error!("❌ [RECOVERY_FAULT]: Sweep sequence collapsed: {}", recovery_error);
            }
        }
    }

    /**
     * Ejecuta un barrido forense para identificar y reemplazar misiones huérfanas.
     */
    #[instrument(skip(self))]
    async fn execute_recovery_sweep_sequence(&self) -> anyhow::Result<()> {
        let mission_repository = MissionRepository::new(self.application_state.database_client.clone());
        let database_connection = self.application_state.database_client.get_connection()?;

        // 1. IDENTIFICACIÓN DE MISIONES ABANDONADAS (ESTRATO L3)
        let abandoned_mission_identifiers = mission_repository
            .identify_abandoned_missions(&database_connection, INACTIVITY_ABANDONMENT_THRESHOLD_SECONDS)
            .await?;

        if abandoned_mission_identifiers.is_empty() {
            return Ok(());
        }

        let total_lost_units_count = abandoned_mission_identifiers.len() as u32;
        warn!(
            "💀 [ZOMBIES_DETECTED]: Found {} huérfanas. Requesting infrastructure replacement...",
            total_lost_units_count
        );

        // 2. DISPARO DE REEMPLAZO VÍA C2 (ESTRATO L6)
        // ✅ RESOLUCIÓN E0599: Sincronizado con GitHubCommandCoordinator V110.0
        let command_coordinator = GitHubCommandCoordinator::from_production_environment()?;

        match command_coordinator.trigger_swarm_expansion_sequence(total_lost_units_count).await {
            Ok(_) => {
                info!("🚀 [RECOVERY_IGNITION]: Replacement signal dispatched successfully to GitHub.");

                // 3. RE-ENCOLADO ATÓMICO DE MISIONES
                mission_repository.requeue_missions(&database_connection, abandoned_mission_identifiers).await?;

                info!("✅ [RECOVERY_COMPLETE]: Workforce expansion confirmed.");
            },
            Err(github_error) => {
                error!(
                    "❌ [C2_HANDSHAKE_FAULT]: Failed to trigger replacement: {}. Targets remain in zombie state.",
                    github_error
                );
            }
        }

        Ok(())
    }
}
