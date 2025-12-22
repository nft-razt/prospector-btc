/**
 * =================================================================
 * APARATO: SYSTEM STATE REPOSITORY (V110.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: PERSISTENCIA ACÍDICA DE CONFIGURACIÓN Y ESTADO GLOBAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la autoridad de persistencia para los metadatos críticos
 * que rigen el comportamiento del sistema distribuido. Gestiona el
 * 'Audit Token' del censo, asegurando que todos los estratos de la
 * arquitectura (L1 a L6) reconozcan de forma unívoca la versión de
 * los datos arqueológicos en uso.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use tracing::{info, instrument, error};

/// Repositorio especializado en la gestión de la tabla 'system_state'.
pub struct SystemStateRepository {
    /// Cliente de enlace táctico con el Motor A (Turso).
    database_client: TursoClient,
}

impl SystemStateRepository {
    /**
     * Inicializa una nueva instancia del repositorio inyectando el cliente de base de datos.
     *
     * @param database_client Instancia activa del cliente Turso/libSQL.
     */
    pub fn new(database_client: TursoClient) -> Self {
        Self { database_client }
    }

    /**
     * Recupera el Token de Auditoría del censo activo desde el Ledger Táctico.
     *
     * # Mathematical Proof
     * Este token garantiza la integridad referencial entre el filtro de Bloom
     * cargado en los workers y la base de datos de misiones del Orquestador.
     *
     * @returns Result con el token opcional o error de base de datos.
     */
    #[instrument(skip(self))]
    pub async fn retrieve_active_census_audit_token(&self) -> Result<Option<String>, DbError> {
        let database_connection = self.database_client.get_connection()?;

        let query_statement = "
            SELECT value_text FROM system_state
            WHERE key = 'active_census_audit_token'
            LIMIT 1
        ";

        let mut query_result = database_connection
            .query(query_statement, ())
            .await
            .map_err(|database_error| {
                error!("❌ [QUERY_FAULT]: Failed to fetch census audit token: {}", database_error);
                DbError::QueryError(database_error)
            })?;

        if let Some(data_row) = query_result.next().await? {
            let audit_token: String = data_row.get(0)?;
            Ok(Some(audit_token))
        } else {
            Ok(None)
        }
    }

    /**
     * Persiste o actualiza el token de integridad del sistema de forma atómica.
     * Invocado durante la secuencia de ignición si se detecta un nuevo manifiesto de estratos.
     *
     * # Protocolo de Sincronización
     * Realiza un UPSERT (INSERT OR UPDATE) para garantizar que la llave única
     * 'active_census_audit_token' sea la Fuente Única de Verdad.
     *
     * @param new_census_audit_token El nuevo hash SHA-256 del manifiesto de estratos.
     * @returns Result indicando el éxito de la operación.
     */
    #[instrument(skip(self, new_census_audit_token))]
    pub async fn persist_system_integrity_audit_token(
        &self,
        new_census_audit_token: &str
    ) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        let sql_statement = "
            INSERT INTO system_state (key, value_text, updated_at)
            VALUES ('active_census_audit_token', ?1, CURRENT_TIMESTAMP)
            ON CONFLICT(key) DO UPDATE SET
                value_text = excluded.value_text,
                updated_at = CURRENT_TIMESTAMP
        ";

        database_connection
            .execute(sql_statement, params![new_census_audit_token])
            .await
            .map_err(|database_error| {
                error!("❌ [PERSISTENCE_FAULT]: Failed to seal audit token: {}", database_error);
                DbError::QueryError(database_error)
            })?;

        info!(
            "🛡️ [SYSTEM_STATE]: Integrity Audit Token crystallized: [{}]",
            new_census_audit_token
        );

        Ok(())
    }
}
