// apps/orchestrator/src/services/telemetry.rs
// =================================================================
// APARATO: TELEMETRY AGGREGATOR (v6.0)
// RESPONSABILIDAD: PULSO DEL SISTEMA (HEARTBEAT AGGREGATION)
// =================================================================

use crate::state::AppState;
use chrono::Utc;
use prospector_domain_models::SystemMetrics;
use std::time::Duration;
use tokio::time::interval;

pub async fn spawn_telemetry_loop(state: AppState) {
    // Frecuencia: 2 segundos (Estándar de UI en tiempo real)
    let mut ticker = interval(Duration::from_secs(2));

    tokio::spawn(async move {
        loop {
            ticker.tick().await;

            let metrics = {
                // Bloqueo de lectura mínimo
                let workers = match state.workers.read() {
                    Ok(guard) => guard,
                    Err(_) => continue,
                };

                let now = Utc::now();
                let active_threshold = now - chrono::Duration::seconds(60);

                let active_workers: Vec<_> = workers
                    .values()
                    .filter(|w| w.timestamp > active_threshold)
                    .collect();

                let global_hashrate: u64 = active_workers.iter().map(|w| w.hashrate).sum();
                let jobs_in_flight = active_workers
                    .iter()
                    .filter(|w| w.current_job_id.is_some())
                    .count();

                SystemMetrics {
                    active_nodes: active_workers.len(),
                    global_hashrate,
                    jobs_in_flight,
                    timestamp: now.to_rfc3339(),
                }
            };

            // ✅ Uso del EventBus desacoplado
            state.events.notify_metrics(metrics);
        }
    });
}
