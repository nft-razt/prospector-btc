
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilterError {
    #[error("Error de serialización/deserialización: {0}")]
    SerializationError(#[from] bincode::Error),

    #[error("Error de I/O: {0}")]
    IoError(#[from] std::io::Error),

    #[error("El filtro no ha sido inicializado")]
    NotInitialized,
}
