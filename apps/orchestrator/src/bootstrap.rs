// apps/orchestrator/src/bootstrap.rs
// =================================================================
// APARATO: ASYNC SYSTEM BOOTSTRAP (V13.0)
// RESPONSABILIDAD: HIDRATACIÓN DE DATOS SIN BLOQUEO DE LIVENESS
// ESTADO: CLOUD-NATIVE READY
// =================================================================

use crate::state::{AppState, SystemMode};
use std::path::Path;
use tokio::fs;
use tracing::{error, info, warn};

pub struct Bootstrap;

impl Bootstrap {
    /// Inicia la secuencia de diagnóstico en segundo plano.
    ///
    /// Esta función no bloquea el hilo principal, permitiendo que Axum
    /// comience a servir el endpoint /health inmediatamente.
    pub fn spawn_diagnostics(state: AppState) {
        tokio::spawn(async move {
            info!("🩺 BOOTSTRAP: Starting asynchronous data hydration...");

            // 1. Verificación del Filtro UTXO (Artefacto Crítico)
            let filter_path = Path::new("utxo_filter.bin");

            if !filter_path.exists() {
                let msg =
                    "Missing utxo_filter.bin. Mining operations will be restricted.".to_string();
                warn!("⚠️ {}", msg);
                state.set_mode(SystemMode::Maintenance(msg));
                return;
            }

            // 2. Validación de Integridad Estructural
            match fs::metadata(filter_path).await {
                Ok(metadata) => {
                    let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);

                    if size_mb < 0.1 {
                        let msg = format!(
                            "Integrity Check Failed: Filter too small ({:.2} MB)",
                            size_mb
                        );
                        error!("❌ {}", msg);
                        state.set_mode(SystemMode::Maintenance(msg));
                    } else {
                        info!("✅ BOOTSTRAP: UTXO Filter verified ({:.2} MB). System fully operational.", size_mb);
                        state.set_mode(SystemMode::Operational);
                    }
                }
                Err(e) => {
                    let msg = format!("I/O Error during filter validation: {}", e);
                    error!("❌ {}", msg);
                    state.set_mode(SystemMode::Maintenance(msg));
                }
            }
        });
    }
}
