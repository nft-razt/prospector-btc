use bincode::{DefaultOptions, Options};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Error de serialización: {0}")]
    EncodeError(#[from] bincode::Error),
}

const MAX_PACKET_SIZE: u64 = 10 * 1024 * 1024; // 10 MB Límite

/// Configuración Estándar Elite
fn get_config() -> impl Options {
    DefaultOptions::new()
        .with_little_endian() // Estándar CPU moderno
        .with_fixint_encoding() // Enteros tamaño fijo (u64 = 8 bytes siempre)
        .with_limit(MAX_PACKET_SIZE) // Protección de Memoria
}

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, TransportError> {
    let bytes = get_config().serialize(value)?;
    Ok(bytes)
}

pub fn from_bytes<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T, TransportError> {
    let value = get_config().deserialize(bytes)?;
    Ok(value)
}
