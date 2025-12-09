// =================================================================
// APARATO: IDENTITY REPOSITORY
// RESPONSABILIDAD: PERSISTENCIA, ROTACIÓN Y ATOMICIDAD DE CREDENCIALES
// NIVEL: ELITE (ATOMIC TRANSACTIONS)
// =================================================================

use crate::TursoClient;
use crate::errors::DbError;
use prospector_domain_models::identity::{Identity, IdentityStatus, CreateIdentityPayload};
use libsql::params;
use uuid::Uuid;
use chrono::{Utc, TimeZone, DateTime};

pub struct IdentityRepository {
    client: TursoClient,
}

impl IdentityRepository {
    /// Constructor del repositorio con cliente inyectado.
    pub fn new(client: TursoClient) -> Self {
        Self { client }
    }

    /// Guarda o actualiza una identidad (Upsert).
    /// Si la combinación (platform + email) ya existe, actualiza las credenciales,
    /// el User-Agent y resetea el estado a 'active' para reintroducirla en la rotación.
    pub async fn upsert(&self, payload: &CreateIdentityPayload) -> Result<(), DbError> {
        let conn = self.client.get_connection()?;

        // Serialización segura del JSON de cookies a String para almacenamiento TEXT
        let credentials_str = serde_json::to_string(&payload.cookies)
            .map_err(|e| DbError::MappingError(format!("Error serializando Cookies JSON: {}", e)))?;

        let id = Uuid::new_v4().to_string();

        // SQL Upsert: INSERT ... ON CONFLICT DO UPDATE
        let query = r#"
            INSERT INTO identities (id, platform, email, credentials_json, user_agent, status, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, 'active', CURRENT_TIMESTAMP)
            ON CONFLICT(platform, email) DO UPDATE SET
                credentials_json = excluded.credentials_json,
                user_agent = excluded.user_agent,
                status = 'active', -- Reactivamos la cuenta si estaba marcada como expirada
                last_used_at = NULL -- Reiniciamos la prioridad de rotación (como nueva)
        "#;

        conn.execute(query, params![
            id,
            payload.platform.clone(),
            payload.email.clone(),
            credentials_str,
            payload.user_agent.clone()
        ]).await?;

        Ok(())
    }

    /// Obtiene el inventario completo de identidades para el Dashboard de Administración.
    /// Ordenado por fecha de creación descendente.
    pub async fn list_all(&self) -> Result<Vec<Identity>, DbError> {
        let conn = self.client.get_connection()?;

        let mut rows = conn.query(
            "SELECT * FROM identities ORDER BY created_at DESC",
            ()
        ).await?;

        let mut identities = Vec::new();

        while let Some(row) = rows.next().await? {
            identities.push(self.map_row(row)?);
        }

        Ok(identities)
    }

    /// ALGORITMO DE ARRENDAMIENTO ATÓMICO (ATOMIC LEASE).
    /// Esta es la joya de la corona del repositorio.
    ///
    /// Encuentra una identidad activa que haya sido usada menos recientemente (Least Recently Used),
    /// incrementa su contador de uso, actualiza la fecha de uso y devuelve el objeto,
    /// TODO EN UNA SOLA TRANSACCIÓN ATÓMICA DE BASE DE DATOS.
    ///
    /// Evita colisiones cuando múltiples workers solicitan identidad simultáneamente.
    pub async fn lease_active(&self, platform: &str) -> Result<Option<Identity>, DbError> {
        let conn = self.client.get_connection()?;

        // SQL ELITE:
        // 1. Subquery: Selecciona el ID de la identidad 'active' con 'last_used_at' más antiguo (o NULL).
        // 2. Update: Actualiza esa fila específica.
        // 3. Returning: Devuelve los datos actualizados inmediatamente.
        let query = r#"
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

        let mut rows = conn.query(query, params![platform]).await?;

        if let Some(row) = rows.next().await? {
            Ok(Some(self.map_row(row)?))
        } else {
            // No hay identidades disponibles (Stock agotado o todas expiradas)
            Ok(None)
        }
    }

    /// Helper privado para mapear una fila cruda de libSQL a un Struct de Dominio Rust.
    /// Maneja la conversión de tipos SQL (TEXT, INTEGER) a Rust (Enum, DateTime, u64).
    fn map_row(&self, row: libsql::Row) -> Result<Identity, DbError> {
        // Índices basados en el orden de columnas del Schema:
        // 0: id, 1: platform, 2: email, 3: credentials_json, 4: user_agent,
        // 5: usage_count, 6: last_used_at, 7: created_at, 8: status

        // Mapeo seguro de String -> Enum Status
        let status_str: String = row.get(8).unwrap_or("revoked".to_string());
        let status = match status_str.as_str() {
            "active" => IdentityStatus::Active,
            "ratelimited" => IdentityStatus::RateLimited,
            "expired" => IdentityStatus::Expired,
            _ => IdentityStatus::Revoked,
        };

        // Mapeo seguro de String ISO8601 -> DateTime
        let parse_date = |idx: i32| -> Option<DateTime<Utc>> {
            row.get::<Option<String>>(idx).ok().flatten().and_then(|s|
                DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
                    // Fallback para formatos SQLite simples sin TZ si fuera necesario
                    .or_else(|| {
                        // Intento simplificado o fallback a fecha epoch si falla parseo crítico
                        None
                    })
            )
        };

        let last_used_at = parse_date(6);

        // Para created_at, si falla (no debería), usamos current time como fallback visual
        let created_at = parse_date(7).unwrap_or_else(Utc::now);

        Ok(Identity {
            id: Uuid::parse_str(&row.get::<String>(0)?).unwrap_or_default(),
            platform: row.get(1)?,
            email: row.get(2)?,
            credentials_json: row.get(3)?,
            user_agent: row.get(4)?,
            usage_count: row.get::<u64>(5)?, // Turso devuelve enteros como i64/u64
            last_used_at,
            created_at,
            status,
        })
    }
}
