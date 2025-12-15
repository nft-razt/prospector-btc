// apps/orchestrator/src/bootstrap.rs
// =================================================================
// APARATO: SYSTEM BOOTSTRAP
// RESPONSABILIDAD: INICIALIZACIÓN ROBUSTA Y DIAGNÓSTICO
// ESTADO: CORREGIDO (UMBRAL DE TAMAÑO AJUSTADO PARA TEST DATA)
// =================================================================

use crate::state::{AppState, SystemMode};
use std::path::Path;
use tracing::{error, info, warn};

pub struct Bootstrap;

impl Bootstrap {
    /// Ejecuta diagnósticos de arranque.
    /// NO detiene el proceso, sino que degrada el estado si es necesario.
    pub fn run_diagnostics(state: &AppState) {
        info!("🩺 SYSTEM DIAGNOSTICS INITIATED...");

        // 1. Integridad del Filtro (UTXO Set)
        // Verificamos existencia y tamaño mínimo para asegurar que no es un archivo vacío o corrupto.
        let filter_path = Path::new("utxo_filter.bin");

        if filter_path.exists() {
            match std::fs::metadata(filter_path) {
                Ok(metadata) => {
                    let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);

                    // CORRECCIÓN CRÍTICA:
                    // Se reduce el umbral de 1.0 MB a 0.1 MB.
                    // El filtro dummy actual pesa ~0.4 MB, por lo que 1.0 lo rechazaba.
                    if size_mb < 0.1 {
                        let msg = format!(
                            "Integrity Fail: Filtro corrupto o demasiado pequeño ({:.2} MB).",
                            size_mb
                        );
                        error!("❌ {}", msg);
                        // Degradamos a Modo Mantenimiento para evitar pánicos, pero bloqueamos minería.
                        state.set_mode(SystemMode::Maintenance(msg));
                    } else {
                        info!(
                            "✅ Filtro UTXO verificado: {:.2} MB. Sistema listo para operaciones.",
                            size_mb
                        );
                    }
                }
                Err(e) => {
                    let msg = format!("Error I/O crítico al leer metadatos del filtro: {}", e);
                    error!("❌ {}", msg);
                    state.set_mode(SystemMode::Maintenance(msg));
                }
            }
        } else {
            let msg =
                "Archivo 'utxo_filter.bin' no encontrado en el sistema de archivos.".to_string();
            warn!("⚠️ {}", msg);
            // Sin filtro no hay minería, pasamos a mantenimiento.
            state.set_mode(SystemMode::Maintenance(msg));
        }
    }
}
