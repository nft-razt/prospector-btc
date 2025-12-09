
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Fallo en conexión: {0}")]
    ConnectionError(String),

    #[error("Error de Query: {0}")]
    QueryError(#[from] libsql::Error),

    #[error("Error de mapeo de datos: {0}")]
    MappingError(String),

    #[error("Transacción fallida")]
    TransactionError,
}
