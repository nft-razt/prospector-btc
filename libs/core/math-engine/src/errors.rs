// =================================================================
// APARATO: MATH ERRORS
// RESPONSABILIDAD: CATALOGACIÓN DE FALLOS CRIPTOGRÁFICOS
// =================================================================

use thiserror::Error;

/// Errores posibles dentro del motor matemático y criptográfico.
#[derive(Error, Debug)]
pub enum MathError {
    /// El formato de la clave proporcionada no es válido (ej: caracteres no hex).
    #[error("Formato de clave inválido: {0}")]
    InvalidKeyFormat(String),

    /// Error propagado desde la librería criptográfica subyacente (secp256k1).
    /// Generalmente ocurre si la clave privada es 0, mayor que el orden de la curva, o el punto es inválido.
    #[error("Error en operación criptográfica de curva elíptica")]
    EllipticCurveError(#[from] secp256k1::Error),

    /// La longitud de los datos en bytes no coincide con la esperada por el algoritmo.
    #[error("Longitud de datos incorrecta. Esperado {expected}, recibido {got}")]
    InvalidLength {
        /// Cantidad de bytes requeridos estrictamente.
        expected: usize,
        /// Cantidad de bytes recibidos.
        got: usize,
    },
}
