// apps/orchestrator/src/services/flush.rs
// =================================================================
// APARATO: FLUSH SERVICE (PERSISTENCE DAEMON)
// RESPONSABILIDAD: VOLCADO ASÍNCRONO DE BUFFERS A BASE DE DATOS
// FRECUENCIA: BATCH CADA 5 SEGUNDOS
// =================================================================

use crate::state::AppState;
use prospector_infra_db::repositories::WorkerRepository;
use std::time::Duration;
use tokio::time::interval;
use tracing::{debug, error, info};

/// Inicia el proceso de persistencia en segundo plano.
pub async fn spawn_flush_service(state: AppState) {
    let mut ticker = interval(Duration::from_secs(5)); // Configurable
    let repo = WorkerRepository::new(state.db.clone());

    tokio::spawn(async move {
        info!("💾 FLUSH DAEMON: Servicio de persistencia por lotes iniciado.");

        loop {
            ticker.tick().await;

            // 1. DRENAJE DEL BUFFER (CRITICAL SECTION)
            // Tomamos el lock brevemente, extraemos todo y liberamos el lock inmediatamente.
            let pending_updates: Vec<_> = {
                match state.heartbeat_buffer.lock() {
                    Ok(mut buffer) => {
                        if buffer.is_empty() {
                            continue;
                        }
                        // drain() vacía el map y nos da un iterador
                        buffer.drain().map(|(_, v)| v).collect()
                    }
                    Err(e) => {
                        error!("❌ FLUSH ERROR: Buffer Lock Poisoned: {}", e);
                        continue;
                    }
                }
            };

            let count = pending_updates.len();
            debug!(
                "💾 FLUSH: Persistiendo {} actualizaciones de nodos...",
                count
            );

            // 2. ESCRITURA EN DB (IO BOUND)
            // Esto ocurre sin bloquear el AppState, permitiendo que la API siga respondiendo.
            match repo.upsert_bulk(pending_updates).await {
                Ok(saved) => {
                    debug!("✅ FLUSH: {} registros guardados en Turso.", saved);
                }
                Err(e) => {
                    error!("⚠️ FLUSH FALLIDO: Error escribiendo en DB: {}. Los datos se perdieron (Circuit Breaker activado).", e);
                    // Nota: En un sistema bancario reintentaríamos, pero en telemetría efímera
                    // es mejor perder un heartbeat que bloquear el sistema. Los workers enviarán otro en 30s.
                }
            }
        }
    });
}
