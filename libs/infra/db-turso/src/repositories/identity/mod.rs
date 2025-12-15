// libs/infra/db-turso/src/repositories/identity/mod.rs
// =================================================================
// APARATO: IDENTITY REPOSITORY (ORCHESTRATOR)
// RESPONSABILIDAD: LÓGICA DE NEGOCIO PARA CREDENCIALES
// ESTADO: ATOMIZADO & CLEAN (UNUSED IMPORT REMOVED)
// =================================================================

pub mod queries;

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_models::identity::{CreateIdentityPayload, Identity, IdentityStatus};
use uuid::Uuid;
// CORRECCIÓN: Eliminado `TimeZone` de los imports
use chrono::{DateTime, Utc};

use self::queries as sql;

pub struct IdentityRepository {
    client: TursoClient,
}

impl IdentityRepository {
    /// Constructor del repositorio con cliente inyectado.
    pub fn new(client: TursoClient) -> Self {
        Self { client }
    }

    /// Guarda o actualiza una identidad (Upsert).
    pub async fn upsert(&self, payload: &CreateIdentityPayload) -> Result<(), DbError> {
        let conn = self.client.get_connection()?;

        let credentials_str = serde_json::to_string(&payload.cookies).map_err(|e| {
            DbError::MappingError(format!("Error serializando Cookies JSON: {}", e))
        })?;

        let id = Uuid::new_v4().to_string();

        conn.execute(
            sql::UPSERT_IDENTITY,
            params![
                id,
                payload.platform.clone(),
                payload.email.clone(),
                credentials_str,
                payload.user_agent.clone()
            ],
        )
        .await?;

        Ok(())
    }

    /// Marca una identidad como REVOCADA (Kill Switch).
    pub async fn revoke(&self, email: &str) -> Result<(), DbError> {
        let conn = self.client.get_connection()?;
        conn.execute(sql::REVOKE_IDENTITY, params![email]).await?;
        Ok(())
    }

    /// Obtiene el inventario completo de identidades.
    pub async fn list_all(&self) -> Result<Vec<Identity>, DbError> {
        let conn = self.client.get_connection()?;

        let mut rows = conn.query(sql::LIST_ALL_IDENTITIES, ()).await?;
        let mut identities = Vec::new();

        while let Some(row) = rows.next().await? {
            identities.push(self.map_row(row)?);
        }

        Ok(identities)
    }

    /// ALGORITMO DE ARRENDAMIENTO ATÓMICO (ATOMIC LEASE).
    pub async fn lease_active(&self, platform: &str) -> Result<Option<Identity>, DbError> {
        let conn = self.client.get_connection()?;

        let mut rows = conn
            .query(sql::LEASE_ACTIVE_IDENTITY, params![platform])
            .await?;

        if let Some(row) = rows.next().await? {
            Ok(Some(self.map_row(row)?))
        } else {
            Ok(None)
        }
    }

    /// Helper privado para mapear filas.
    fn map_row(&self, row: libsql::Row) -> Result<Identity, DbError> {
        // Índices basados en el orden de columnas del Schema
        let status_str: String = row.get(8).unwrap_or("revoked".to_string());
        let status = match status_str.as_str() {
            "active" => IdentityStatus::Active,
            "ratelimited" => IdentityStatus::RateLimited,
            "expired" => IdentityStatus::Expired,
            _ => IdentityStatus::Revoked,
        };

        let parse_date = |idx: i32| -> Option<DateTime<Utc>> {
            row.get::<Option<String>>(idx).ok().flatten().and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
                    .or_else(|| None)
            })
        };

        let last_used_at = parse_date(6);
        let created_at = parse_date(7).unwrap_or_else(Utc::now);

        Ok(Identity {
            id: Uuid::parse_str(&row.get::<String>(0)?).unwrap_or_default(),
            platform: row.get(1)?,
            email: row.get(2)?,
            credentials_json: row.get(3)?,
            user_agent: row.get(4)?,
            usage_count: row.get::<u64>(5)?,
            last_used_at,
            created_at,
            status,
        })
    }
}
