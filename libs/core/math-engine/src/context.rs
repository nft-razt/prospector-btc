/**
 * =================================================================
 * APARATO: GLOBAL CRYPTO CONTEXT (V18.0 - NATIVE LOCK)
 * CLASIFICACIÓN: CORE MATH (L1)
 * RESPONSABILIDAD: SINGLETON DEL CONTEXTO SECP256K1
 *
 * ESTRATEGIA DE ÉLITE:
 * - Native Performance: Migración a std::sync::LazyLock para costo cero.
 * - Memory Safety: Garantiza una única asignación de tablas en el heap.
 * =================================================================
 */

use secp256k1::{All, Secp256k1};
use std::sync::LazyLock;

/// Instancia global y estática del contexto de Curva Elíptica.
///
/// Este Singleton pre-computa las tablas de multiplicación escalar (G * k)
/// durante el primer acceso, optimizando todas las operaciones subsiguientes.
pub static GLOBAL_CONTEXT: LazyLock<Secp256k1<All>> = LazyLock::new(Secp256k1::new);

/**
 * Provee acceso seguro y de alto rendimiento al contexto global de criptografía.
 *
 * @returns Una referencia estática al motor de secp256k1.
 */
#[inline]
#[must_use]
pub fn global_context() -> &'static Secp256k1<All> {
    &GLOBAL_CONTEXT
}
