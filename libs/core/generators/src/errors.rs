// =================================================================
// APARATO: GENERATORS ERRORS
// RESPONSABILIDAD: DEFINICIÓN DE FALLOS EN LA GENERACIÓN DE CLAVES
// =================================================================

use thiserror::Error;

/// Errores que pueden ocurrir durante la generación de direcciones o formatos WIF.
///
/// Este enum centraliza los fallos de codificación (Base58) y los fallos
/// matemáticos subyacentes (Curva Elíptica).
#[derive(Error, Debug)]
pub enum GenError {
    /// Ocurrió un error al intentar codificar el payload en Base58Check.
    /// Generalmente sucede si el buffer de datos es inválido.
    #[error("Error codificando Base58: {0}")]
    Base58Error(#[from] bs58::encode::Error),

    /// Ocurrió un error en el motor matemático subyacente.
    /// Puede deberse a una clave privada inválida (0 o > N) o un fallo en secp256k1.
    #[error("Error matemático subyacente: {0}")]
    MathError(#[from] prospector_core_math::errors::MathError),
}
