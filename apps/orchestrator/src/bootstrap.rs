// apps/orchestrator/src/bootstrap.rs
use std::path::Path;
use tracing::{info, error, warn};
use crate::state::{AppState, SystemMode};

pub struct Bootstrap;

impl Bootstrap {
    pub fn run_diagnostics(state: &AppState) {
        info!("🩺 SYSTEM DIAGNOSTICS INITIATED...");

        // 1. Integridad del Filtro
        let filter_path = Path::new("utxo_filter.bin");
        if filter_path.exists() {
            match std::fs::metadata(filter_path) {
                Ok(metadata) => {
                    let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
                    if size_mb < 1.0 {
                        let msg = format!("Filtro corrupto detectado ({:.2} MB).", size_mb);
                        error!("❌ {}", msg);
                        state.set_mode(SystemMode::Maintenance(msg));
                    } else {
                        info!("✅ Filtro UTXO verificado: {:.2} MB.", size_mb);
                    }
                },
                Err(e) => {
                    let msg = format!("Error I/O: {}", e);
                    error!("❌ {}", msg);
                    state.set_mode(SystemMode::Maintenance(msg));
                }
            }
        } else {
            let msg = "Archivo 'utxo_filter.bin' no encontrado.".to_string();
            warn!("⚠️ {}", msg);
            state.set_mode(SystemMode::Maintenance(msg));
        }
    }
}
