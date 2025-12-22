// libs/infra/db-turso/src/repositories/identity/mod.rs
/**
 * =================================================================
 * APARATO: IDENTITY REPOSITORY (V17.1 - FIXED VISIBILITY)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (L3)
 * =================================================================
 */
pub mod queries; // ✅ EXPOSICIÓN PÚBLICA DEL SUBMÓDULO

use crate::errors::DbError;
use crate::TursoClient;
use chrono::{DateTime, Utc};
use libsql::params;
use prospector_domain_models::identity::{CreateIdentityPayload, Identity, IdentityStatus};
use tracing::{error, info, instrument};
use uuid::Uuid;

// Uso directo del submódulo expuesto arriba
use self::queries as sql;

pub struct IdentityRepository {
    client: TursoClient,
}

impl IdentityRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { client }
    }

    /// Ejecuta el Upsert de una identidad validando la integridad del JSON.
    #[instrument(skip(self, payload))]
    pub async fn upsert(&self, payload: &CreateIdentityPayload) -> Result<(), DbError> {
        let database_connection = self.client.get_connection()?;

        let credentials_string = serde_json::to_string(&payload.cookies).map_err(|error| {
            error!("🔥 [VAULT_SERIALIZATION_FAULT]: {}", error);
            DbError::MappingError(format!("Invalid Cookie Payload: {}", error))
        })?;

        let internal_id = Uuid::new_v4().to_string();

        database_connection
            .execute(
                sql::UPSERT_IDENTITY,
                params![
                    internal_id,
                    payload.platform.clone(),
                    payload.email.clone(),
                    credentials_string,
                    payload.user_agent.clone()
                ],
            )
            .await?;

        info!(
            "🔐 [VAULT_SYNC]: Identity secured for owner: {}",
            payload.email
        );
        Ok(())
    }

    /// Implementación del Arrendamiento Atómico (Atomic Lease).
    pub async fn lease_active(&self, target_platform: &str) -> Result<Option<Identity>, DbError> {
        let database_connection = self.client.get_connection()?;

        let mut rows = database_connection
            .query(sql::LEASE_ACTIVE_IDENTITY, params![target_platform])
            .await?;

        if let Some(row) = rows.next().await? {
            Ok(Some(self.map_row_to_identity(row)?))
        } else {
            Ok(None)
        }
    }

    fn map_row_to_identity(&self, row: libsql::Row) -> Result<Identity, DbError> {
        // Índice 8 basado en la query LEASE_ACTIVE_IDENTITY (SELECT *)
        // Ajustar índices según la estructura real de la tabla si cambia.
        let status_raw: String = row.get(5).unwrap_or_else(|_| "revoked".to_string()); // status está en col 5 según schema

        let status = match status_raw.as_str() {
            "active" => IdentityStatus::Active,
            "ratelimited" => IdentityStatus::RateLimited,
            "expired" => IdentityStatus::Expired,
            _ => IdentityStatus::Revoked,
        };

        let parse_utc = |index: i32| -> Option<DateTime<Utc>> {
            row.get::<Option<String>>(index)
                .ok()
                .flatten()
                .and_then(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                })
        };

        Ok(Identity {
            id: Uuid::parse_str(&row.get::<String>(0)?).unwrap_or_default(),
            platform: row.get(1)?,
            email: row.get(2)?,
            credentials_json: row.get(3)?,
            user_agent: row.get(4)?,
            usage_count: row.get::<u64>(6)?, // usage_count col 6
            last_used_at: parse_utc(7),      // last_used_at col 7
            created_at: parse_utc(8).unwrap_or_else(Utc::now), // created_at col 8
            status,
        })
    }
}
