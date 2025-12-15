// =================================================================
// APARATO: CHRONOS SERVICE (SELF-PRESERVATION)
// RESPONSABILIDAD: EVITAR EL "SPIN-DOWN" DE RENDER (15 MIN TIMEOUT)
// CONSUMO: MÍNIMO (1 REQUEST CADA 14 MIN)
// =================================================================

use reqwest::Client;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info, warn};

/// Inicia el marcapasos del sistema.
/// Requiere la URL pública del servicio (inyectada por Render).
pub async fn spawn_chronos(public_url: String) {
    if public_url.contains("localhost") {
        info!("🕰️ CHRONOS: Modo local detectado. Marcapasos desactivado.");
        return;
    }

    info!(
        "🕰️ CHRONOS: Iniciando secuencia de preservación para: {}",
        public_url
    );

    // Render suspende a los 15 min. Disparamos a los 14 min para seguridad.
    let mut ticker = interval(Duration::from_secs(14 * 60));
    let client = Client::new();
    let target = format!("{}/health", public_url);

    tokio::spawn(async move {
        // El primer tick es inmediato, lo saltamos para no duplicar logs de arranque
        ticker.tick().await;

        loop {
            ticker.tick().await;

            info!("💓 CHRONOS: Enviando pulso de vida a {}", target);

            match client.get(&target).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        info!("✅ CHRONOS: Pulso exitoso. Sistema despierto.");
                    } else {
                        warn!(
                            "⚠️ CHRONOS: El sistema respondió con error: {}",
                            resp.status()
                        );
                    }
                }
                Err(e) => {
                    error!("❌ CHRONOS: Fallo crítico en el pulso: {}", e);
                    // Si fallamos al pinguearnos a nosotros mismos, algo grave pasa con la red de Render
                }
            }
        }
    });
}
