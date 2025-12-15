// apps/miner-worker/src/cpu_manager.rs
// =================================================================
// APARATO: CPU TOPOLOGY MANAGER (HARDWARE ABSTRACTION)
// RESPONSABILIDAD: GESTIÓN DE HILOS Y AFINIDAD DE NÚCLEOS
// ESTRATEGIA: RAYON THREAD POOL BUILDER + CORE AFFINITY
// =================================================================

use log::{info, warn};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Configura el pool global de hilos de Rayon con afinidad de CPU estricta.
///
/// # Lógica de Optimización
/// 1. Detecta los núcleos físicos/lógicos disponibles.
/// 2. Configura Rayon para lanzar exactamente un hilo por núcleo disponible.
/// 3. En el arranque de cada hilo (`start_handler`), lo fija a un núcleo específico.
///
/// Esto previene que el Scheduler del SO mueva los hilos de minería,
/// preservando la localidad de caché L1/L2 para las tablas pre-computadas de `secp256k1`.
pub fn optimize_process_affinity() -> anyhow::Result<()> {
    // 1. Obtener IDs de núcleos del sistema
    let core_ids = match core_affinity::get_core_ids() {
        Some(ids) => ids,
        None => {
            warn!("⚠️ No se pudo detectar la topología de CPU. La afinidad está desactivada.");
            return Ok(());
        }
    };

    let available_cores = core_ids.len();
    info!(
        "🧠 Hardware Detectado: {} núcleos lógicos.",
        available_cores
    );

    // Creamos un contador atómico compartido para asignar índices a los hilos
    // Rayon no pasa el índice del hilo en el start_handler directamente de forma determinista,
    // así que lo gestionamos manualmente.
    let counter = Arc::new(AtomicUsize::new(0));

    // 2. Construcción del Pool de Rayon
    rayon::ThreadPoolBuilder::new()
        .num_threads(available_cores)
        .start_handler(move |_| {
            // Obtenemos un índice único para este hilo
            let thread_idx = counter.fetch_add(1, Ordering::SeqCst);

            // Seguridad: Aseguramos que el índice esté dentro de los límites (modulo)
            if let Some(core_id) = core_ids.get(thread_idx % core_ids.len()) {
                // 3. FIJACIÓN (PINNING)
                if core_affinity::set_for_current(*core_id) {
                    // Log a nivel debug para no saturar la salida en producción
                    // println!("🧵 Hilo de minería #{} fijado al núcleo {:?}", thread_idx, core_id);
                } else {
                    warn!("⚠️ Fallo al fijar el hilo #{} al núcleo", thread_idx);
                }
            }
        })
        .build_global() // Configuramos el pool global que usará `par_iter`
        .map_err(|e| anyhow::anyhow!("Fallo crítico al inicializar Rayon: {}", e))?;

    info!("🚀 Motor de Paralelismo (Rayon) inicializado con Afinidad de CPU activada.");
    Ok(())
}
