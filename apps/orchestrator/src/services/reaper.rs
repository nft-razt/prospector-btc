// apps/orchestrator/src/services/reaper.rs
// =================================================================
// APARATO: THE REAPER (MEMORY GARBAGE COLLECTOR)
// RESPONSABILIDAD: MANTENIMIENTO DE HIGIENE EN MEMORIA VOLÁTIL (RAM)
// ESTRATEGIA: LAZY DB RECOVERY (La DB se limpia sola al asignar trabajos)
// ESTADO: CLEAN (UNUSED IMPORTS PURGED)
// =================================================================

use crate::state::AppState;
use std::time::Duration;
use tokio::time::interval;
use tracing::info;

/// Inicia el servicio de limpieza en segundo plano.
///
/// Este servicio opera exclusivamente sobre la memoria RAM (`AppState`).
/// La limpieza de la base de datos (trabajos zombies) se delega al
/// `JobRepository::assign_work` para optimizar las transacciones ACID.
pub async fn spawn_reaper(state: AppState) {
    // Frecuencia de ejecución: Cada 60 segundos
    let mut ticker = interval(Duration::from_secs(60));

    tokio::spawn(async move {
        info!("💀 THE REAPER: Servicio de limpieza de memoria iniciado.");

        loop {
            ticker.tick().await;

            // 1. LIMPIEZA DE MEMORIA RAM (SNAPSHOTS VISUALES)
            // Eliminamos imágenes de workers que no han reportado en los últimos 5 minutos (300s).
            // Esto evita que la RAM del contenedor se sature con Base64 strings viejos.
            let pruned_count = state.prune_stale_snapshots(300);

            if pruned_count > 0 {
                info!("💀 THE REAPER: Poda de memoria completada. {} snapshots obsoletos eliminados.", pruned_count);
            }

            // 2. LIMPIEZA DE MAPA DE WORKERS (HEARTBEATS NUMÉRICOS)
            // Limpiamos la lista de workers para que el dashboard no muestre fantasmas.
            {
                // Un bloque pequeño para minimizar el tiempo de bloqueo del Write Lock
                let mut workers_map = state.workers.write().expect("RwLock workers poisoned");
                let initial = workers_map.len();
                let threshold = chrono::Utc::now() - chrono::Duration::seconds(300); // 5 min

                workers_map.retain(|_, hb| {
                    hb.timestamp > threshold
                });

                let removed = initial - workers_map.len();
                if removed > 0 {
                     info!("💀 THE REAPER: {} workers inactivos eliminados del radar.", removed);
                }
            }
        }
    });
}
