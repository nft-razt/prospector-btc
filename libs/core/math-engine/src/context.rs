// libs/core/math-engine/src/context.rs
// =================================================================
// APARATO: GLOBAL MATH CONTEXT (SINGLETON)
// RESPONSABILIDAD: GESTIÓN DE MEMORIA PRE-COMPUTADA DE CURVA ELÍPTICA
// PATRÓN: LAZY STATIC SINGLETON
// =================================================================

use once_cell::sync::Lazy;
use secp256k1::{All, Secp256k1};

/// Contexto Global de Secp256k1.
///
/// Contiene las tablas pre-computadas para la multiplicación de puntos (G * k).
/// Inicializar esto es costoso (CPU/Memoria), por lo que lo hacemos una sola vez
/// y lo compartimos entre todos los hilos del minero de forma segura (Sync).
///
/// Al usar `Lazy`, la inicialización ocurre en el primer acceso, no al cargar el binario.
pub static GLOBAL_CONTEXT: Lazy<Secp256k1<All>> = Lazy::new(|| {
    // Secp256k1::new() asigna memoria en el Heap para las tablas de optimización.
    // Al hacerlo estático, evitamos malloc/free en el bucle caliente de minería.
    Secp256k1::new()
});

/// Retorna una referencia estática al contexto global.
/// Operación de costo cero (dereferencia de puntero).
#[inline(always)]
pub fn global_context() -> &'static Secp256k1<All> {
    &GLOBAL_CONTEXT
}
