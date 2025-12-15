use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

/// Inicializa el sistema de trazas Heimdall.
///
/// # Comportamiento
/// - En Desarrollo: Logs coloridos y compactos.
/// - En Producción (Release): Logs estructurados JSON para ingestión (Datadog/CloudWatch).
pub fn init_tracing(service_name: &str) {
    // Filtro por defecto: Info para nosotros, Warn para librerías de terceros ruidosas
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=info,tower_http=info,warn", service_name).into());

    // Detección de entorno (si estamos en release mode de Rust)
    let is_production = !cfg!(debug_assertions);

    if is_production {
        // Modo JSON para máquinas
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer().json().flatten_event(true))
            .init();
    } else {
        // Modo Humano para desarrolladores
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer().compact().with_target(false))
            .init();
    }

    tracing::info!("👁️ HEIMDALL ONLINE: Monitorizando [{}]", service_name);
}
