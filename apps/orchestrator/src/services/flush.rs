/**
 * =================================================================
 * APARATO: TACTICAL PERSISTENCE FLUSH DAEMON (V110.0 - ELITE)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: VOLCADO ASÍNCRONO DE BUFFERS HACIA TURSO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el patrón 'Write-Behind' para proteger al Motor A (Turso)
 * de ráfagas de escritura masiva. Este servicio drena atómicamente el
 * 'heartbeat_buffer' de la memoria RAM y lo cristaliza en la base de
 * datos en una única transacción masiva, maximizando el throughput.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::WorkerRepository;
use std::time::Duration;
use tokio::time::interval;
use tracing::{debug, error, info};

/**
 * Inicia el proceso de persistencia en segundo plano para latidos de nodos.
 *
 * @param application_state Referencia al estado neural atomizado de la aplicación.
 */
pub async fn spawn_flush_service(application_state: AppState) {
    // Frecuencia de ciclo: 5 segundos para balancear latencia y carga de DB.
    let mut synchronization_timer = interval(Duration::from_secs(5));

    // Inyección de dependencia del repositorio táctico.
    let worker_repository = WorkerRepository::new(application_state.db());

    tokio::spawn(async move {
        info!("💾 [FLUSH_DAEMON]: Tactical persistence engine is now operational.");

        loop {
            synchronization_timer.tick().await;

            // 1. DRENAJE ATÓMICO DEL BUFFER (CRITICAL SECTION)
            // Extraemos los datos del buffer de memoria rápida para procesarlos en lote.
            let pending_updates_collection: Vec<_> = {
                match application_state.heartbeat_buffer.lock() {
                    Ok(mut buffer_guard) => {
                        if buffer_guard.is_empty() {
                            continue;
                        }
                        // Drenamos el mapa: transferencia de propiedad de RAM a variable local.
                        buffer_guard.drain().map(|(_, heartbeat_data)| heartbeat_data).collect()
                    }
                    Err(poison_error) => {
                        error!("❌ [FLUSH_CRITICAL_FAULT]: RAM Buffer Lock Poisoned: {}", poison_error);
                        continue;
                    }
                }
            };

            let update_volume = pending_updates_collection.len();
            debug!("💾 [FLUSH_EXECUTION]: Persisting {} node heartbeat updates...", update_volume);

            // 2. CRISTALIZACIÓN EN EL LEDGER TÁCTICO (IO BOUND)
            // Ejecución de la transacción ACID masiva en Turso.
            match worker_repository.upsert_bulk(pending_updates_collection).await {
                Ok(successfully_saved_count) => {
                    debug!("✅ [FLUSH_SUCCESS]: {} records secured in Turso Strata.", successfully_saved_count);
                }
                Err(database_error) => {
                    error!(
                        "⚠️  [FLUSH_REJECTED]: Database persistence failed: {}. Signal integrity loss detected.",
                        database_error
                    );
                    // Nota de Tesis: El sistema favorece el hashrate sobre la telemetría histórica.
                    // Los datos no persistidos se descartan para evitar fugas de memoria.
                }
            }
        }
    });
}
