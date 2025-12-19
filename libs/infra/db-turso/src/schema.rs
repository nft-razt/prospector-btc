// libs/infra/db-turso/src/schema.rs
/**
 * =================================================================
 * APARATO: DATABASE SCHEMA ENGINE (V15.0 - ANALYTICS READY)
 * CLASIFICACIÓN: INFRASTRUCTURE DEFINITION (L3)
 * RESPONSABILIDAD: EVOLUCIÓN IDEMPOTENTE DEL LEDGER TÁCTICO
 * ESTADO: V3.3.0 // ARCHIVAL ENABLED
 * =================================================================
 */
use anyhow::{Context, Result};
use libsql::Connection;
use tracing::{info, instrument};

pub const SCHEMA_VERSION: &str = "3.3.0";

#[instrument(skip(connection))]
pub async fn apply_full_schema(connection: &Connection) -> Result<()> {
    info!(
        "🏗️  [SCHEMA_ENGINE]: Synchronizing structural strata to v{}",
        SCHEMA_VERSION
    );

    // 1. TABLA DE TRABAJOS (RANGOS U256)
    // Se añade 'strategy_type' y 'total_hashes' para métricas doctorales.
    connection
        .execute(
            r#"CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,
            range_start TEXT NOT NULL,
            range_end TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            strategy_type TEXT DEFAULT 'Combinatoric',
            worker_id TEXT,
            total_hashes INTEGER DEFAULT 0,
            attempt_count INTEGER DEFAULT 0,
            started_at DATETIME,
            last_heartbeat_at DATETIME,
            completed_at DATETIME,
            archived_at DATETIME
        );"#,
            (),
        )
        .await
        .context("Failed to evolve 'jobs' table")?;

    // 2. TABLA DE HALLAZGOS (THE VAULT)
    connection
        .execute(
            r#"CREATE TABLE IF NOT EXISTS findings (
            id TEXT PRIMARY KEY,
            address TEXT UNIQUE NOT NULL,
            private_key_wif TEXT NOT NULL,
            source_entropy TEXT NOT NULL,
            wallet_type TEXT NOT NULL,
            found_by_worker TEXT,
            job_id TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );"#,
            (),
        )
        .await
        .context("Failed to secure 'findings' table")?;

    // 3. TABLA DE IDENTIDADES (IAM / ZK-VAULT)
    connection
        .execute(
            r#"CREATE TABLE IF NOT EXISTS identities (
            id TEXT PRIMARY KEY,
            platform TEXT NOT NULL,
            email TEXT NOT NULL,
            credentials_json TEXT NOT NULL, -- Almacena EncryptedIdentityPayload (Base64)
            user_agent TEXT NOT NULL,
            status TEXT DEFAULT 'active',
            usage_count INTEGER DEFAULT 0,
            last_used_at DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(platform, email)
        );"#,
            (),
        )
        .await
        .context("Failed to level 'identities' table")?;

    info!("✅ [SCHEMA_ENGINE]: Structural sync complete. System is L4-migration ready.");
    Ok(())
}
