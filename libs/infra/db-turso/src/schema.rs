/*!
 * =================================================================
 * APARATO: DATABASE SCHEMA ENGINE (V13.5 - STACK OPTIMIZED)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: MIGRACIÓN IDEMPOTENTE CON BAJA HUELLA DE MEMORIA
 *
 * VISION HIPER-HOLÍSTICA:
 * Refactorizado para evitar desbordamientos de pila en Windows.
 * Centraliza las consultas en una estructura estática para minimizar
 * el tamaño de la máquina de estados del Future asíncrono.
 * =================================================================
 */

use anyhow::{Context, Result};
use libsql::Connection;
use tracing::{info, instrument};

/// Colección inmutable de definiciones estructurales del Ledger Táctico.
const TACTICAL_SCHEMA_QUERIES: &[(&str, &str)] = &[
    ("TABLE_JOBS", r#"
        CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,
            range_start TEXT NOT NULL,
            range_end TEXT NOT NULL,
            status TEXT DEFAULT 'pending',
            worker_id TEXT,
            strategy_type TEXT DEFAULT 'Sequential',
            total_hashes_effort TEXT,
            execution_duration_ms INTEGER,
            audit_footprint_checkpoint TEXT,
            integrity_hash TEXT,
            scenario_template_identifier TEXT,
            uptime_seconds_start INTEGER,
            uptime_seconds_end INTEGER,
            hardware_clock_frequency INTEGER,
            required_strata TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            completed_at DATETIME,
            archived_at DATETIME
        );
    "#),
    ("TABLE_FINDINGS", r#"
        CREATE TABLE IF NOT EXISTS findings (
            id TEXT PRIMARY KEY,
            address TEXT NOT NULL,
            private_key_wif TEXT NOT NULL,
            source_entropy TEXT,
            wallet_type TEXT,
            found_by_worker TEXT,
            job_id TEXT,
            detected_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            archived_at DATETIME DEFAULT NULL
        );
    "#),
    ("TABLE_IDENTITIES", r#"
        CREATE TABLE IF NOT EXISTS identities (
            id TEXT PRIMARY KEY,
            platform TEXT NOT NULL,
            email TEXT NOT NULL,
            credentials_json TEXT NOT NULL,
            user_agent TEXT,
            status TEXT DEFAULT 'active',
            usage_count INTEGER DEFAULT 0,
            last_used_at DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(platform, email)
        );
    "#),
    ("TABLE_SYSTEM_STATE", r#"
        CREATE TABLE IF NOT EXISTS system_state (
            key TEXT PRIMARY KEY,
            value_text TEXT,
            value_int INTEGER,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#),
    ("TABLE_SCENARIO_TEMPLATES", r#"
        CREATE TABLE IF NOT EXISTS scenario_templates (
            identifier TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            blob_data BLOB,
            size_bytes INTEGER,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#),
    ("TABLE_TEST_SCENARIOS", r#"
        CREATE TABLE IF NOT EXISTS test_scenarios (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            secret_phrase TEXT,
            target_address TEXT NOT NULL,
            target_private_key TEXT,
            status TEXT DEFAULT 'idle',
            verified_at DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#),
    ("TABLE_WORKERS", r#"
        CREATE TABLE IF NOT EXISTS workers (
            id TEXT PRIMARY KEY,
            ip_address TEXT,
            version TEXT,
            status TEXT,
            last_seen_at DATETIME,
            hashrate_avg REAL,
            jobs_completed INTEGER DEFAULT 0
        );
    "#),
    ("INDEX_JOBS_STATUS", "CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status);"),
    ("INDEX_FINDINGS_SYNC", "CREATE INDEX IF NOT EXISTS idx_findings_sync ON findings(archived_at) WHERE archived_at IS NULL;"),
    ("INDEX_JOBS_SYNC", "CREATE INDEX IF NOT EXISTS idx_jobs_sync ON jobs(archived_at) WHERE status = 'completed' AND archived_at IS NULL;")
];

/**
 * Ejecuta la validación y aplicación del esquema táctico.
 *
 * # Performance
 * Utiliza una iteración lineal sobre un slice estático para mantener
 * el frame de la pila por debajo de los 4KB, evitando overflow en Windows.
 */
#[instrument(skip(connection))]
pub async fn apply_full_schema(connection: &Connection) -> Result<()> {
    info!("🏗️ [SCHEMA_ENGINE]: Verifying tactical ledger structure...");

    for (name, query) in TACTICAL_SCHEMA_QUERIES {
        connection.execute(query, ())
            .await
            .with_context(|| format!("CRITICAL_SCHEMA_FAULT: Failed to apply {}", name))?;
    }

    info!("✅ [SCHEMA_ENGINE]: All strata verified and synchronized.");
    Ok(())
}

pub async fn apply_archival_schema_evolution(_connection: &Connection) -> Result<()> {
    // Reservado para futuras migraciones incrementales
    Ok(())
}
