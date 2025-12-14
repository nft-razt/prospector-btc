// libs/infra/worker-client/src/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Error de red HTTP: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Error de I/O (Disco): {0}")]
    IoError(#[from] std::io::Error),

    #[error("El servidor rechazó la petición: {0}")]
    ServerError(String),

    #[error("Error de autenticación (Token inválido)")]
    Unauthorized,

    #[error("Imposible hidratar filtro tras múltiples intentos")]
    HydrationFailed,
}
