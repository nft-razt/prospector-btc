/**
 * =================================================================
 * APARATO: THE REAPER SYSTEM SERVICE (V120.0 - ELITE HYGIENE)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE (ESTRATO L4)
 * RESPONSABILIDAD: MANTENIMIENTO DE HIGIENE EN RAM Y PURGA DE ZOMBIES
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el recolector de basura especializado del Orquestador.
 * Realiza barridos cíclicos sobre la memoria RAM para eliminar:
 * 1. Nodos Desconectados: Basado en el umbral de inactividad de latidos.
 * 2. Instantáneas Obsoletas: Limpieza del Panóptico Visual.
 * =================================================================
 */

use crate::state::AppState;
use std::time::Duration;
use tokio::time::interval;
use tracing::info;

/**
 * Inicia el servicio de limpieza en segundo plano.
 *
 * @param application_state Referencia soberana al estado neural de la aplicación.
 */
pub async fn spawn_reaper(application_state: AppState) {
    // Frecuencia de escrutinio: 60 segundos para minimizar impacto en CPU.
    let mut maintenance_timer = interval(Duration::from_secs(60));

    tokio::spawn(async move {
        info!("💀 [REAPER_ACTIVE]: Memory hygiene daemon initiated.");

        loop {
            maintenance_timer.tick().await;

            // 1. PURGA DE SNAPSHOTS VISUALES (L5 UI Optimization)
            // Invocamos al método atómico del AppState nivelado en V14.5.
            let purged_frames_count = application_state.prune_stale_snapshots(300);

            if purged_frames_count > 0 {
                info!("💀 [REAPER_CLEANUP]: Evicted {} stale visual frames from RAM.", purged_frames_count);
            }

            // 2. PURGA DE TELEMETRÍA DE NODOS (L3 Swarm Health)
            // ✅ RESOLUCIÓN E0615: Llamada al método workers() con paréntesis.
            {
                let mut active_nodes_guard = application_state.workers()
                    .active_nodes_telemetry
                    .write()
                    .expect("FATAL: Swarm Telemetry Lock Poisoned");

                let initial_node_count = active_nodes_guard.len();
                let expiration_threshold = chrono::Utc::now() - chrono::Duration::seconds(300);

                // Retenemos solo los trabajadores que han reportado en los últimos 5 minutos.
                active_nodes_guard.retain(|_, heartbeat_data| {
                    heartbeat_data.timestamp > expiration_threshold
                });

                let removed_nodes_count = initial_node_count - active_nodes_guard.len();
                if removed_nodes_count > 0 {
                    info!("💀 [REAPER_SWARM]: Removed {} inactive units from tactical radar.", removed_nodes_count);
                }
            }
        }
    });
}
