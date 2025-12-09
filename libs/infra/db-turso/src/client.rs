// libs/infra/db-turso/src/client.rs
// =================================================================
// APARATO: TURSO CLIENT
// RESPONSABILIDAD: GESTIÓN DE CONEXIONES Y POOLING
// ESTADO: REPARADO (ERROR MAPPING FIX)
// =================================================================

use libsql::{Builder, Connection, Database};
use std::sync::Arc;
use crate::errors::DbError;
use crate::schema::apply_schema;

/// Cliente encapsulado para operaciones con Turso/libSQL.
/// Maneja la conexión subyacente y asegura que el esquema esté aplicado al iniciar.
#[derive(Clone)]
pub struct TursoClient {
    db: Arc<Database>,
}

impl TursoClient {
    /// Establece una conexión con la base de datos (Local o Remota).
    ///
    /// # Argumentos
    /// * `url`: Cadena de conexión (ej: "file:local.db" o "libsql://...").
    /// * `token`: Token de autenticación (solo requerido para conexiones remotas).
    ///
    /// # Errores
    /// Retorna `DbError::ConnectionError` si falla el handshake o la aplicación del esquema.
    pub async fn connect(url: &str, token: Option<String>) -> Result<Self, DbError> {
        // 1. Construcción del Database Driver
        let db = if let Some(t) = token {
            Builder::new_remote(url.to_string(), t)
                .build()
                .await
                .map_err(|e| DbError::ConnectionError(format!("Remote build failed: {}", e)))?
        } else {
            Builder::new_local(url)
                .build()
                .await
                .map_err(|e| DbError::ConnectionError(format!("Local build failed: {}", e)))?
        };

        // 2. Establecimiento de Conexión
        let conn = db.connect()
            .map_err(|e| DbError::ConnectionError(format!("Connect failed: {}", e)))?;

        // 3. Hidratación del Esquema (Schema Migration)
        // CORRECCIÓN: Mapeamos el error de anyhow directamente a ConnectionError
        // en lugar de intentar construir un libsql::Error manual.
        apply_schema(&conn).await
            .map_err(|e| DbError::ConnectionError(format!("Schema initialization failed: {}", e)))?;

        Ok(Self {
            db: Arc::new(db),
        })
    }

    /// Obtiene una nueva conexión ligera del pool interno.
    pub fn get_connection(&self) -> Result<Connection, DbError> {
        self.db.connect()
            .map_err(|e| DbError::ConnectionError(format!("Pool connection failed: {}", e)))
    }
}
