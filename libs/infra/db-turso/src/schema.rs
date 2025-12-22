// libs/infra/db-turso/src/schema.rs
/**
 * =================================================================
 * APARATO: DATABASE SCHEMA ENGINE (V13.1 - FULL SPECTRUM)
 * RESPONSABILIDAD: DEFINICIÓN Y MIGRACIÓN DEL ESQUEMA TÁCTICO
 * =================================================================
 */

use anyhow::Result;
use libsql::Connection;
use tracing::info;

/// Aplica el esquema completo de base de datos.
/// Idempotente: Solo crea tablas si no existen.
pub async fn apply_full_schema(connection: &Connection) -> Result<()> {
    info!("🏗️ [SCHEMA_ENGINE]: Verifying tactical ledger structure...");

    // 1. TABLA DE TRABAJOS (MISIONES)
    // Soporta rangos U256 (como texto) y metadatos de auditoría.
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,
            range_start TEXT NOT NULL,
            range_end TEXT NOT NULL,
            status TEXT DEFAULT 'pending', -- pending, active, completed
            worker_id TEXT,
            strategy_type TEXT DEFAULT 'Sequential',

            -- Telemetría de Esfuerzo
            total_hashes_effort TEXT,
            execution_duration_ms INTEGER,
            audit_footprint_checkpoint TEXT,
            integrity_hash TEXT,

            -- Metadatos de Escenario (Forensic)
            scenario_template_identifier TEXT,
            uptime_seconds_start INTEGER,
            uptime_seconds_end INTEGER,
            hardware_clock_frequency INTEGER,
            required_strata TEXT,

            -- Timestamps
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            completed_at DATETIME,
            archived_at DATETIME -- Sincronización con L4
        );
        "#,
        (),
    ).await?;

    // 2. TABLA DE HALLAZGOS (TESORO)
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS findings (
            id TEXT PRIMARY KEY,
            address TEXT NOT NULL,
            private_key_wif TEXT NOT NULL,
            source_entropy TEXT,
            wallet_type TEXT,
            found_by_worker TEXT,
            job_id TEXT,
            detected_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            archived_at DATETIME DEFAULT NULL -- Sincronización con L4
        );
        "#,
        (),
    ).await?;

    // 3. TABLA DE IDENTIDADES (VAULT)
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS identities (
            id TEXT PRIMARY KEY,
            platform TEXT NOT NULL,
            email TEXT NOT NULL,
            credentials_json TEXT NOT NULL, -- Cifrado o Plano
            user_agent TEXT,
            status TEXT DEFAULT 'active',
            usage_count INTEGER DEFAULT 0,
            last_used_at DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(platform, email)
        );
        "#,
        (),
    ).await?;

    // 4. TABLA DE ESTADO DEL SISTEMA (CONFIGURACIÓN GLOBAL)
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS system_state (
            key TEXT PRIMARY KEY,
            value_text TEXT,
            value_int INTEGER,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
        (),
    ).await?;

    // 5. TABLA DE PLANTILLAS DE ESCENARIOS (FORENSIC DNA)
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS scenario_templates (
            identifier TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            blob_data BLOB,
            size_bytes INTEGER,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
        (),
    ).await?;

    // 6. TABLA DE ESCENARIOS DE PRUEBA (LABORATORIO)
    connection.execute(
        r#"
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
        "#,
        (),
    ).await?;

    // 7. TABLA DE WORKERS (TELEMETRÍA PERSISTENTE)
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS workers (
            id TEXT PRIMARY KEY,
            ip_address TEXT,
            version TEXT,
            status TEXT,
            last_seen_at DATETIME,
            hashrate_avg REAL,
            jobs_completed INTEGER DEFAULT 0
        );
        "#,
        (),
    ).await?;

    // ÍNDICES ESTRATÉGICOS PARA RENDIMIENTO O(1)
    connection.execute("CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status);", ()).await?;
    connection.execute("CREATE INDEX IF NOT EXISTS idx_findings_sync ON findings(archived_at) WHERE archived_at IS NULL;", ()).await?;
    connection.execute("CREATE INDEX IF NOT EXISTS idx_jobs_sync ON jobs(archived_at) WHERE status = 'completed' AND archived_at IS NULL;", ()).await?;

    info!("✅ [SCHEMA_ENGINE]: Tactical strata verified and synchronized.");
    Ok(())
}

pub async fn apply_archival_schema_evolution(connection: &Connection) -> Result<()> {
    // Función de mantenimiento para evoluciones futuras (Stub)
    // Ya incluida en apply_full_schema para fresh installs
    let _ = connection;
    Ok(())
}
