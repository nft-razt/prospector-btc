/**
 * =================================================================
 * APARATO: SOVEREIGN SEED SCRIPT (V105.0 - FIXED)
 * CLASIFICACIÓN: OPS UTILITY (ESTRATO L3)
 * RESPONSABILIDAD: POBLAMIENTO INICIAL DEL LEDGER TÁCTICO
 * =================================================================
 */

use prospector_infra_db::TursoClient;
use tracing::info;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    info!("🌱 [SEED]: Initiating tactical data injection...");

    let database_url = std::env::var("DATABASE_URL")
        .expect("CRITICAL: DATABASE_URL not set.");
    let database_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    // 1. CONEXIÓN AL MOTOR A
    let client = TursoClient::connect(&database_url, database_token).await?;
    let connection = client.get_connection()?;

    // 2. INYECCIÓN DE CONFIGURACIÓN DINÁMICA
    // ✅ RESOLUCIÓN: Uso de params directos para evitar colisión de imports
    info!("⚙️ [SEED]: Setting active_scenario_config...");
    connection.execute(
        "INSERT INTO system_state (key, value_text, value_int, updated_at)
         VALUES ('active_scenario_config', 'WIN_XP_SP3_GOLD', 3579545, CURRENT_TIMESTAMP)
         ON CONFLICT(key) DO UPDATE SET
            value_text = excluded.value_text,
            value_int = excluded.value_int,
            updated_at = CURRENT_TIMESTAMP",
        ()
    ).await?;

    // 3. REGISTRO DEL GOLDEN TICKET
    info!("🎯 [SEED]: Injecting certification target (Block 1)...");
    connection.execute(
        "INSERT INTO test_scenarios (id, name, target_address, status, created_at)
         VALUES ('cert-alpha-001', 'SMOKE_TEST_GENESIS', '12cbqSREwGrvtd3LsBhymWvCX9A9Snd9E7', 'idle', CURRENT_TIMESTAMP)
         ON CONFLICT(id) DO NOTHING",
        ()
    ).await?;

    info!("✅ [SEED_COMPLETE]: Tactical strata is now operational.");
    Ok(())
}
