use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_tracing(service_name: &str) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=debug,tower_http=info,warn", service_name).into());

    // Formato humano para dev, JSON para prod (Render)
    let format = fmt::format()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .compact(); // O .json() si detectas ENV=production

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().event_format(format))
        .init();

    tracing::info!("👁️ HEIMDALL OBSERVER INITIALIZED: {}", service_name);
}
