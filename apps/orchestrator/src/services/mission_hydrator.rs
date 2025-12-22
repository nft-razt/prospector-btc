/**
 * =================================================================
 * APARATO: MISSION HYDRATOR SERVICE (V200.0 - DYNAMIC SOBERANO)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: MANTENIMIENTO AUTÓNOMO DEL INVENTARIO DE TRABAJO
 *
 * VISION HIPER-HOLÍSTICA:
 * Monitorea el nivel de saturación del MissionControlManager en RAM.
 * Cuando detecta un nivel bajo (Low Watermark), orquesta una
 * pre-asignación masiva en el Motor A (Turso). Implementa la resolución
 * dinámica de escenarios consultando el nexo operativo en cada ciclo,
 * permitiendo cambios de estrategia en tiempo de ejecución.
 * =================================================================
 */

use crate::state::AppState;
use crate::state::operational_nexus::SystemIntegrityStatus;
use prospector_infra_db::repositories::MissionRepository;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, warn, error, instrument};

/// Umbral crítico de misiones en RAM antes de disparar hidratación.
const MISSION_LOW_WATERMARK: usize = 100;
/// Volumen de misiones a generar por ráfaga de hidratación.
const HYDRATION_BATCH_VOLUME: usize = 500;
/// Intervalo de escrutinio del buffer.
const MONITORING_CYCLE_SECONDS: u64 = 15;

pub struct MissionHydratorService {
    /// Referencia al estado neural atomizado de la aplicación.
    application_state: AppState,
}

impl MissionHydratorService {
    /**
     * Inicializa el servicio de hidratación inyectando el estado compartido.
     */
    pub fn new(application_state: AppState) -> Self {
        Self { application_state }
    }

    /**
     * Lanza el daemon de hidratación en un hilo asíncrono persistente.
     */
    pub async fn spawn_hydrator_daemon(self) {
        let mut monitoring_timer = interval(Duration::from_secs(MONITORING_CYCLE_SECONDS));

        info!("🚰 [MISSION_HYDRATOR]: Autonomous replenishment service is online.");

        loop {
            monitoring_timer.tick().await;

            // 1. VERIFICACIÓN DE AUTORIDAD OPERATIVA
            // No generamos misiones si el sistema está en espera de certificación o en stop.
            let integrity_status = self.application_state.operational_nexus.get_integrity_status();
            if integrity_status == SystemIntegrityStatus::AwaitingCertification {
                continue;
            }

            // 2. ESCRUTINIO DEL BUFFER DE DESPACHO
            let current_inventory_size = self.application_state.mission_control.get_available_buffer_size();

            if current_inventory_size < MISSION_LOW_WATERMARK {
                warn!("⚠️ [BUFFER_LOW]: Inventory at {}. Initiating bulk replenishment...", current_inventory_size);

                if let Err(error) = self.execute_dynamic_hydration_sequence().await {
                    error!("❌ [HYDRATION_FAULT]: Failed to replenish mission buffer: {}", error);
                }
            }
        }
    }

    /**
     * Ejecuta la lógica de adquisición y carga de misiones.
     */
    #[instrument(skip(self))]
    async fn execute_dynamic_hydration_sequence(&self) -> anyhow::Result<()> {
        let mission_repository = MissionRepository::new(self.application_state.database_client.clone());

        // 1. ADQUISICIÓN DE LOTE DINÁMICO (L3)
        // Se resuelve el TODO: fetch_dynamic_mission_batch recupera el escenario
        // activo desde la tabla 'system_state' de Turso.
        let fresh_missions_batch = mission_repository
            .fetch_dynamic_mission_batch(HYDRATION_BATCH_VOLUME)
            .await?;

        // 2. INYECCIÓN EN EL BUFFER DE MANDO (L1-RAM)
        let batch_count = fresh_missions_batch.len();
        self.application_state.mission_control.hydrate_queue(fresh_missions_batch);

        info!("✅ [HYDRATION_SUCCESS]: Injected {} strategic missions into the active queue.", batch_count);

        Ok(())
    }
}
