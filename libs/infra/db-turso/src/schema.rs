use libsql::Connection;
use anyhow::Result;
use tracing::info;

/// Aplica el esquema de base de datos V2 (Architecture: Indestructible Ledger).
/// Este esquema es idempotente: puede ejecutarse múltiples veces sin destruir datos.
pub async fn apply_schema(conn: &Connection) -> Result<()> {

    // -------------------------------------------------------------------------
    // 1. BÓVEDA DE IDENTIDADES (THE FUEL)
    // Almacena credenciales de Google/Kaggle para los workers.
    // Optimización: 'leased_until' permite exclusión mutua distribuida.
    // -------------------------------------------------------------------------
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS identities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            provider TEXT NOT NULL,              -- 'google', 'kaggle', 'colab'
            email TEXT NOT NULL UNIQUE,          -- Identificador único de la cuenta
            cookies TEXT NOT NULL,               -- JSON blob con la sesión
            user_agent TEXT,                     -- Fingerprint para evitar bloqueos

            status TEXT DEFAULT 'active',        -- 'active', 'burned', 'rate_limited'
            leased_until INTEGER DEFAULT 0,      -- Timestamp (Unix) para bloqueo optimista

            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
        (),
    ).await?;

    // Índice para encontrar rápidamente cuentas disponibles
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_identities_lease ON identities(status, leased_until);",
        ()
    ).await?;


    // -------------------------------------------------------------------------
    // 2. LIBRO MAYOR DE TRABAJOS (THE LEDGER)
    // Núcleo del algoritmo. Reemplaza al 'cursor' simple.
    // Permite reintentos, auditoría y recuperación de fallos.
    // -------------------------------------------------------------------------
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,                 -- UUID v4
            range_start TEXT NOT NULL,           -- BigInt como String (para evitar overflow en 64bit)
            range_end TEXT NOT NULL,             -- BigInt como String

            -- Máquina de Estados: 'pending', 'processing', 'completed', 'failed'
            status TEXT NOT NULL DEFAULT 'pending',

            worker_id TEXT,                      -- ID del nodo que tomó el trabajo
            attempt_count INTEGER DEFAULT 0,     -- Para detectar rangos 'tóxicos'

            started_at DATETIME,
            last_heartbeat_at DATETIME,          -- Vital para el 'Reaper'
            completed_at DATETIME
        );
        "#,
        (),
    ).await?;

    // Índice Crítico: Permite al Reaper encontrar trabajos zombies instantáneamente
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_jobs_reaper ON jobs(status, last_heartbeat_at);",
        ()
    ).await?;

    // Índice para evitar generar rangos duplicados
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_jobs_range ON jobs(range_start, range_end);",
        ()
    ).await?;


    // -------------------------------------------------------------------------
    // 3. HALLAZGOS (THE GOLD)
    // Almacena las colisiones exitosas. Persistencia crítica.
    // -------------------------------------------------------------------------
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS findings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            address TEXT NOT NULL,               -- Dirección BTC encontrada
            private_key TEXT NOT NULL,           -- WIF o Hex

            found_by_worker TEXT,
            job_id TEXT,                         -- Trazabilidad: ¿En qué trabajo se halló?

            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(job_id) REFERENCES jobs(id)
        );
        "#,
        (),
    ).await?;


    // -------------------------------------------------------------------------
    // 4. FLOTA DE MINEROS (THE SWARM)
    // Metadatos efímeros sobre los nodos conectados.
    // Útil para calcular Hashrate global y salud del enjambre.
    // -------------------------------------------------------------------------
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS workers (
            id TEXT PRIMARY KEY,
            ip_address TEXT,
            version TEXT,

            status TEXT DEFAULT 'online',
            last_seen_at DATETIME DEFAULT CURRENT_TIMESTAMP,

            -- Métricas de rendimiento
            hashrate_avg REAL,
            jobs_completed INTEGER DEFAULT 0
        );
        "#,
        (),
    ).await?;

    info!("✅ Schema V2 (Indestructible Ledger) applied successfully.");
    Ok(())
}
