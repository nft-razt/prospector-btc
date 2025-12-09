// libs/infra/db-turso/src/repositories/job/queries.rs
// =================================================================
// APARATO: JOB SQL STORE
// RESPONSABILIDAD: CATÁLOGO DE CONSULTAS SQL OPTIMIZADAS
// =================================================================

/// Busca un trabajo "Zombie" (en proceso pero sin latido reciente) o Pendiente.
/// Prioriza cerrar huecos en el espacio de búsqueda (ORDER BY range_start ASC).
pub const ACQUIRE_ZOMBIE_OR_PENDING: &str = r#"
    SELECT id, range_start, range_end
    FROM jobs
    WHERE (status = 'processing' AND last_heartbeat_at < ?1)
       OR status = 'pending'
    ORDER BY range_start ASC
    LIMIT 1
"#;

/// "Resucita" un trabajo asignándolo a un nuevo worker.
pub const REVIVE_JOB: &str = r#"
    UPDATE jobs
    SET worker_id = ?1,
        status = 'processing',
        last_heartbeat_at = CURRENT_TIMESTAMP,
        attempt_count = attempt_count + 1
    WHERE id = ?2
"#;

/// Obtiene el límite superior explorado hasta el momento.
/// Usamos 'created_at' para ordenamiento cronológico del puntero.
pub const GET_MAX_RANGE: &str = r#"
    SELECT range_end FROM jobs ORDER BY created_at DESC LIMIT 1
"#;

/// Inserta un nuevo territorio virgen en el ledger.
pub const INSERT_NEW_JOB: &str = r#"
    INSERT INTO jobs (id, range_start, range_end, status, worker_id, started_at, last_heartbeat_at)
    VALUES (?1, ?2, ?3, 'processing', ?4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
"#;

pub const HEARTBEAT: &str = r#"
    UPDATE jobs SET last_heartbeat_at = CURRENT_TIMESTAMP WHERE id = ?1
"#;

pub const COMPLETE: &str = r#"
    UPDATE jobs SET status = 'completed', completed_at = CURRENT_TIMESTAMP WHERE id = ?1
"#;
