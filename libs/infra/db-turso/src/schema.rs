use libsql::Connection;
use anyhow::Result;
use tracing::info; // Ahora funciona gracias a Cargo.toml

pub async fn apply_schema(conn: &Connection) -> Result<()> {
    // ... (El contenido de las tablas es correcto, no cambia) ...
    // Solo corregimos el import de tracing al inicio.

    // TABLA IDENTITIES
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS identities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            provider TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            cookies TEXT NOT NULL,
            user_agent TEXT,
            status TEXT DEFAULT 'active',
            leased_until INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
        (),
    ).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_identities_lease ON identities(status, leased_until);", ()).await?;

    // TABLA JOBS
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,
            range_start TEXT NOT NULL,
            range_end TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            worker_id TEXT,
            attempt_count INTEGER DEFAULT 0,
            started_at DATETIME,
            last_heartbeat_at DATETIME,
            completed_at DATETIME
        );
        "#,
        (),
    ).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_jobs_reaper ON jobs(status, last_heartbeat_at);", ()).await?;
    conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS idx_jobs_range ON jobs(range_start, range_end);", ()).await?;

    // TABLA FINDINGS
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS findings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            address TEXT NOT NULL,
            private_key TEXT NOT NULL,
            found_by_worker TEXT,
            job_id TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(job_id) REFERENCES jobs(id)
        );
        "#,
        (),
    ).await?;

    // TABLA WORKERS
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS workers (
            id TEXT PRIMARY KEY,
            ip_address TEXT,
            version TEXT,
            status TEXT DEFAULT 'online',
            last_seen_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            hashrate_avg REAL,
            jobs_completed INTEGER DEFAULT 0
        );
        "#,
        (),
    ).await?;

    info!("✅ Schema V2 (Indestructible Ledger) applied successfully.");
    Ok(())
}
