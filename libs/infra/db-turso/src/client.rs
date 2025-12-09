// libs/infra/db-turso/src/client.rs
use libsql::{Builder, Connection, Database};
use std::sync::Arc;
use crate::errors::DbError;
use crate::schema::INITIAL_SCHEMA;

#[derive(Clone)]
pub struct TursoClient {
    db: Arc<Database>,
}

impl TursoClient {
    /// Conecta a la base de datos (Archivo local o Turso Remoto).
    pub async fn connect(url: &str, token: Option<String>) -> Result<Self, DbError> {
        // CORRECCIÓN: Construimos la DB dentro de las ramas para unificar el tipo de retorno
        let db = if let Some(t) = token {
            // Rama Remota
            Builder::new_remote(url.to_string(), t)
                .build()
                .await
                .map_err(|e| DbError::ConnectionError(e.to_string()))?
        } else {
            // Rama Local
            Builder::new_local(url)
                .build()
                .await
                .map_err(|e| DbError::ConnectionError(e.to_string()))?
        };

        // Inicializar esquema (Migración "Poor Man's")
        let conn = db.connect().map_err(|e| DbError::ConnectionError(e.to_string()))?;
        conn.execute_batch(INITIAL_SCHEMA).await?;

        Ok(Self {
            db: Arc::new(db),
        })
    }

    /// Obtiene una conexión fresca para ejecutar queries.
    pub fn get_connection(&self) -> Result<Connection, DbError> {
        self.db.connect().map_err(|e| DbError::ConnectionError(e.to_string()))
    }
}
