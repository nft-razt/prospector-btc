// apps/orchestrator/src/bin/migrator.rs
// =================================================================
// APARATO: DB MIGRATOR CLI
// RESPONSABILIDAD: EJECUCIÓN DE MANTENIMIENTO ESTRUCTURAL
// USO: cargo run --bin migrator
// =================================================================

use dotenvy::dotenv;
use prospector_infra_db::TursoClient;
use prospector_infra_db::schema::apply_full_schema;
use tracing::{error, info};
use prospector_shared_heimdall::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    init_tracing("prospector_migrator");

    info!("🚀 MIGRATOR: Initiating structural audit...");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    // 1. Conexión a la DB
    let client = TursoClient::connect(&db_url, db_token).await
        .map_err(|e| anyhow::anyhow!("Connection failure: {}", e))?;

    let connection = client.get_connection()
        .map_err(|e| anyhow::anyhow!("Pool failure: {}", e))?;

    // 2. Aplicación de Esquema
    match apply_full_schema(&connection).await {
        Ok(_) => {
            info!("✨ MIGRATOR: Database is now up to date. Ready for API traffic.");
            std::process::exit(0);
        }
        Err(e) => {
            error!("❌ MIGRATOR: Fatal error during schema application: {}", e);
            std::process::exit(1);
        }
    }
}
