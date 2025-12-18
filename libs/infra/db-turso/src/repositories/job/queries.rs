// libs/infra/db-turso/src/repositories/job/queries.rs
/**
 * =================================================================
 * APARATO: JOB SQL STORE (V15.0 - ANALYTICS UPGRADE)
 * CLASIFICACIÓN: TACTICAL LEDGER (L3)
 * RESPONSABILIDAD: CONSULTAS PARA MÉTRICAS DE ESFUERZO
 * ESTADO: V3.3.0 COMPLIANT
 * =================================================================
 */

/// Sella un trabajo como finalizado e inyecta las métricas de cómputo real.
pub const FINALIZE_WITH_METRICS: &str = r#"
    UPDATE jobs
    SET status = 'completed',
        total_hashes = ?2,
        completed_at = CURRENT_TIMESTAMP
    WHERE id = ?1
"#;

/// Consulta para el motor Chronos de migración L4.
/// Recupera el strategy_type dinámicamente.
pub const GET_COMPLETED_FOR_ARCHIVE: &str = r#"
    SELECT id, range_start, range_end, strategy_type, total_hashes, started_at, completed_at
    FROM jobs
    WHERE status = 'completed' AND archived_at IS NULL
    LIMIT ?1
"#;
