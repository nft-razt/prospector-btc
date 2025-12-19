/**
 * =================================================================
 * APARATO: DATABASE SCHEMA ENGINE (V3.4.0 - FORENSIC READY)
 * CLASIFICACIÓN: INFRASTRUCTURE DEFINITION (L3)
 * RESPONSABILIDAD: EVOLUCIÓN IDEMPOTENTE DEL LEDGER TÁCTICO
 *
 * ESTRATEGIA DE ÉLITE:
 * - Deterministic Indexing: Índice sobre range_end_hex para adquisición O(1).
 * - Metadata Stratification: Campos dedicados para la huella forense.
 * - Zero-Regression: Mantiene compatibilidad con identidades ZK.
 * =================================================================
 */

use anyhow::{Context, Result};
use libsql::Connection;
use tracing::info;

pub const SCHEMA_VERSION: &str = "3.4.0";

pub async fn apply_full_schema_evolution(connection: &Connection) -> Result<()> {
    info!("🏗️  [SCHEMA_ENGINE]: Synchronizing structural strata to v{}", SCHEMA_VERSION);

    // 1. TABLA DE MISIONES (LEDGER TÁCTICO)
    // Refactorizada para soportar U256 y Checkpoints Forenses.
    connection.execute(
        r#"CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,
            range_start_hex TEXT NOT NULL,
            range_end_hex TEXT NOT NULL,
            strategy_type TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'active',
            worker_id TEXT NOT NULL,
            total_hashes_effort TEXT DEFAULT '0',
            audit_footprint_checkpoint TEXT,
            execution_duration_ms INTEGER DEFAULT 0,
            started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            completed_at DATETIME
        );"#,
        (),
    ).await.context("Failed to evolve 'jobs' table")?;

    // 2. ÍNDICE DE FRONTERA (CRÍTICO PARA PERFORMANCE)
    // Permite al MissionRepository encontrar el final del censo en milisegundos.
    connection.execute(
        "CREATE INDEX IF NOT EXISTS idx_jobs_frontier ON jobs (range_end_hex DESC) WHERE status = 'completed';",
        (),
    ).await.context("Failed to create frontier index")?;

    info!("✅ [SCHEMA_ENGINE]: Structural sync complete. System is V8.5 compliant.");
    Ok(())
}
