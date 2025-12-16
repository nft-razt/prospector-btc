// libs/core/math-engine/src/arithmetic.rs
// =================================================================
// APARATO: BYTE ARITHMETIC ENGINE (ELITE OPTIMIZED)
// RESPONSABILIDAD: OPERACIONES MATEMÁTICAS SOBRE ARRAYS DE 256 BITS
// ALGORITMO: BIG-ENDIAN CARRY/BORROW CHAINING
// =================================================================

use crate::errors::MathError;
use std::cmp::Ordering;

/// Compara dos enteros de 256 bits representados como arrays de bytes en Big-Endian.
///
/// # Rendimiento
/// Utiliza comparación lexicográfica byte a byte, que es equivalente a la comparación
/// numérica para enteros sin signo en formato Big-Endian.
#[inline(always)]
pub fn compare_u256_be(a: &[u8; 32], b: &[u8; 32]) -> Ordering {
    for i in 0..32 {
        match a[i].cmp(&b[i]) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }
    Ordering::Equal
}

/// Convierte un u128 nativo a un array de 32 bytes (u256) con relleno de ceros.
///
/// Útil para convertir offsets o índices pequeños al formato compatible con las
/// claves privadas de Bitcoin.
#[inline(always)]
pub fn u128_to_u256_be(val: u128) -> [u8; 32] {
    let mut buf = [0u8; 32];
    // Escribimos los 16 bytes del u128 al final del buffer (Big-Endian alignment)
    buf[16..32].copy_from_slice(&val.to_be_bytes());
    buf
}

/// Suma dos enteros de 256 bits: $R = A + B$.
///
/// Implementa la suma escolar columna por columna propagando el acarreo (carry).
///
/// # Errores
/// Retorna `MathError::InvalidKeyFormat` si ocurre un desbordamiento (Overflow),
/// lo cual indicaría una clave privada fuera del rango válido de 256 bits.
#[inline(always)]
pub fn add_u256_be(a: &[u8; 32], b: &[u8; 32]) -> Result<[u8; 32], MathError> {
    let mut result = [0u8; 32];
    let mut carry: u16 = 0;

    // Iteramos desde el byte menos significativo (índice 31) hacia atrás
    for i in (0..32).rev() {
        // Sumamos los bytes y el acarreo anterior. Máximo valor: 255 + 255 + 1 = 511 (cabe en u16)
        let sum = (a[i] as u16) + (b[i] as u16) + carry;

        // El byte resultante es los 8 bits bajos
        result[i] = (sum & 0xFF) as u8;

        // El nuevo acarreo es el bit alto (división por 256)
        carry = sum >> 8;
    }

    if carry > 0 {
        return Err(MathError::InvalidKeyFormat("Overflow: Suma excede 256 bits".into()));
    }

    Ok(result)
}

/// Resta dos enteros de 256 bits: $R = A - B$.
///
/// Implementa la resta con préstamo (borrow).
///
/// # Errores
/// Retorna `MathError::InvalidKeyFormat` si $A < B$ (Underflow), ya que trabajamos
/// con enteros sin signo (claves privadas).
#[inline(always)]
pub fn sub_u256_be(a: &[u8; 32], b: &[u8; 32]) -> Result<[u8; 32], MathError> {
    // Verificación previa de magnitud para evitar underflow
    if compare_u256_be(a, b) == Ordering::Less {
        return Err(MathError::InvalidKeyFormat("Underflow: Resultado negativo no permitido en PrivateKey".into()));
    }

    let mut result = [0u8; 32];
    let mut borrow: i16 = 0;

    for i in (0..32).rev() {
        // Calculamos la diferencia considerando el préstamo anterior
        // Usamos i16 para manejar valores negativos temporalmente
        let diff = (a[i] as i16) - (b[i] as i16) - borrow;

        if diff < 0 {
            // Si es negativo, pedimos prestado 256 (0x100) al siguiente byte
            result[i] = (diff + 256) as u8;
            borrow = 1;
        } else {
            result[i] = diff as u8;
            borrow = 0;
        }
    }

    Ok(result)
}
