// libs/infra/db-turso/src/repositories/identity/queries.rs
// =================================================================
// APARATO: IDENTITY SQL STORE
// RESPONSABILIDAD: CATÁLOGO DE CONSULTAS SQL PARA GESTIÓN DE IDENTIDAD
// =================================================================

/// Inserta o actualiza una identidad (Upsert).
/// Si existe (mismo email+plataforma), actualiza cookies y reactiva.
pub const UPSERT_IDENTITY: &str = r#"
    INSERT INTO identities (id, platform, email, credentials_json, user_agent, status, created_at)
    VALUES (?1, ?2, ?3, ?4, ?5, 'active', CURRENT_TIMESTAMP)
    ON CONFLICT(platform, email) DO UPDATE SET
        credentials_json = excluded.credentials_json,
        user_agent = excluded.user_agent,
        status = 'active',
        last_used_at = NULL
"#;

/// Marca una identidad como REVOCADA (Kill Switch).
pub const REVOKE_IDENTITY: &str =
    "UPDATE identities SET status = 'revoked', updated_at = CURRENT_TIMESTAMP WHERE email = ?1";

/// Lista todas las identidades ordenadas por creación.
pub const LIST_ALL_IDENTITIES: &str = "SELECT * FROM identities ORDER BY created_at DESC";

/// Arrienda una identidad activa (Atomic Lease).
/// Selecciona la menos usada recientemente y actualiza sus contadores atómicamente.
pub const LEASE_ACTIVE_IDENTITY: &str = r#"
    UPDATE identities
    SET
        usage_count = usage_count + 1,
        last_used_at = CURRENT_TIMESTAMP
    WHERE id = (
        SELECT id FROM identities
        WHERE platform = ?1 AND status = 'active'
        ORDER BY last_used_at ASC NULLS FIRST
        LIMIT 1
    )
    RETURNING *
"#;
