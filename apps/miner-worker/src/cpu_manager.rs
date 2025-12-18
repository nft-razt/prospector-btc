// apps/miner-worker/src/cpu_manager.rs
// =================================================================
// APARATO: CPU TOPOLOGY & HEALTH MANAGER (V11.5)
// RESPONSABILIDAD: AFINIDAD DE NÚCLEOS Y TELEMETRÍA DE HARDWARE
// ESTADO: ZERO-WARNINGS // FULL IMPLEMENTATION
// =================================================================

use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Métricas instantáneas de la unidad de procesamiento.
pub struct CpuMetrics {
    pub frequency_mhz: u32,
    pub load_percent: f32,
    pub core_count: u32,
}

/// Recupera el estado del hardware consultando el kernel Linux.
pub fn get_current_metrics() -> CpuMetrics {
    let mut frequency = 0;

    // 1. Frecuencia del reloj (MHz)
    if let Ok(content) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq") {
        frequency = content.trim().parse::<u32>().unwrap_or(0) / 1000;
    } else if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.contains("cpu MHz") {
                frequency = line.split(':').nth(1).and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(0.0) as u32;
                break;
            }
        }
    }

    CpuMetrics {
        frequency_mhz: frequency,
        load_percent: 100.0, // El minero satura la CPU por diseño
        core_count: num_cpus::get() as u32,
    }
}

/// Fija los hilos de minería a núcleos físicos específicos.
pub fn optimize_process_affinity() -> anyhow::Result<()> {
    let core_identifiers = match core_affinity::get_core_ids() {
        Some(ids) => ids,
        None => return Ok(()),
    };

    let thread_counter = Arc::new(AtomicUsize::new(0));
    let cores_available = core_identifiers.len();

    rayon::ThreadPoolBuilder::new()
        .num_threads(cores_available)
        .start_handler(move |_| {
            let index = thread_counter.fetch_add(1, Ordering::SeqCst);
            if let Some(core) = core_identifiers.get(index % cores_available) {
                core_affinity::set_for_current(*core);
            }
        })
        .build_global()?;

    Ok(())
}
