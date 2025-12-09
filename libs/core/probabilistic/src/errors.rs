// libs/core/probabilistic/src/errors.rs
// =================================================================
// APARATO: PROBABILISTIC ERRORS
// RESPONSABILIDAD: CATÁLOGO DE FALLOS EN EL FILTRO DE BLOOM
// ESTADO: ELITE COMPLIANCE (FULL RUSTDOC)
// =================================================================

use thiserror::Error;

/// Define los errores posibles durante la operación del Filtro de Bloom.
///
/// Este enum centraliza los fallos que pueden ocurrir al manipular la estructura
/// probabilística, cubriendo desde problemas de sistema de archivos (I/O) hasta
/// corrupción de datos binarios.
#[derive(Error, Debug)]
pub enum FilterError {
    /// Error crítico durante la serialización o deserialización binaria.
    ///
    /// Generalmente ocurre si:
    /// - El archivo `.bin` está corrupto.
    /// - La versión de `bincode` o la estructura del struct ha cambiado (Breaking Change).
    /// - El archivo está truncado (EOF inesperado).
    #[error("Error de serialización/deserialización: {0}")]
    SerializationError(#[from] bincode::Error),

    /// Error de Entrada/Salida del sistema operativo.
    ///
    /// Ocurre si:
    /// - No se encuentra el archivo del filtro (`NotFound`).
    /// - No hay permisos de lectura/escritura (`PermissionDenied`).
    /// - El disco está lleno.
    #[error("Error de I/O: {0}")]
    IoError(#[from] std::io::Error),

    /// Intento de uso del filtro antes de su carga completa en memoria RAM.
    ///
    /// El filtro debe ser hidratado explícitamente usando `load_from_file`
    /// antes de ejecutar consultas `contains`.
    #[error("El filtro no ha sido inicializado")]
    NotInitialized,
}
