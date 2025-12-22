/**
 * =================================================================
 * APARATO: TELEMETRY AGGREGATOR (V15.0 - OBSERVABILITY HARDENED)
 * =================================================================
 */

use crate::state::AppState;
use chrono::Utc;
use prospector_domain_models::telemetry::SystemMetrics;
use std::time::Duration;
use tokio::time::interval;
use tracing::debug;

pub async fn spawn_telemetry_loop(state: AppState) {
    let mut ticker = interval(Duration::from_secs(2));

    tokio::spawn(async move {
        loop {
            ticker.tick().await;

            // ✅ RESOLUCIÓN: Usamos get_pending_count para diagnóstico interno
            let pending_vault_volume = state.finding_vault.get_pending_count();
            if pending_vault_volume > 0 {
                debug!("Bóveda en RAM contiene {} colisiones pendientes de volcado.", pending_vault_volume);
            }

            let metrics = {
                let workers_guard = state.swarm_telemetry.active_nodes_telemetry.read().expect("Lock poisoned");
                let active_nodes_count = workers_guard.len() as u32;
                let cumulative_global_hashrate: u64 = workers_guard.values().map(|w| w.hashrate).sum();
                let active_missions_in_flight = workers_guard.values().filter(|w| w.current_job_id.is_some()).count() as u32;

                SystemMetrics {
                    active_nodes_count,
                    cumulative_global_hashrate,
                    active_missions_in_flight,
                    timestamp_ms: Utc::now().timestamp_millis() as u64,
                }
            };

            state.event_bus.notify_system_pulse_update(metrics);
        }
    });
}
