/**
 * =================================================================
 * APARATO: CORE ARITHMETIC KERNEL (V110.0 - FULL SPECTRUM)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: OPERACIONES U256 DE BAJO NIVEL Y UTILIDADES
 *
 * VISION HIPER-HOLÍSTICA:
 * Provee la interfaz de bajo nivel para la manipulación de escalares
 * de 256 bits (Big-Endian Arrays). Implementa adición con acarreo
 * extendido (ADC) en ensamblador inline para el Hot-Path y
 * utilidades de comparación/serialización para los motores de estrategia.
 * =================================================================
 */

use crate::errors::MathError;
use std::arch::asm;
use std::cmp::Ordering;

/// Longitud canónica de una clave privada de secp256k1 (256 bits).
pub const U256_BYTE_SIZE: usize = 32;

/**
 * Incrementa un buffer Big-Endian de 32 bytes de forma atómica sumándole un u64.
 *
 * # Performance
 * Utiliza encadenamiento de registros (Chain ADC) mediante ensamblador inline x86_64
 * para evitar el overhead de conversiones excesivas en el bucle caliente.
 *
 * # Argumentos
 * * `buffer` - Referencia mutable al array de 32 bytes (Big-Endian).
 * * `value` - Valor u64 a sumar.
 */
#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn add_u64_to_u256_be(buffer: &mut [u8; 32], value: u64) -> Result<(), MathError> {
    unsafe {
        let ptr = buffer.as_mut_ptr() as *mut u64;
        let mut carry: u8;

        // Inversión de Endianness para aritmética nativa x86_64 (Little Endian)
        // Cargamos los 4 "limbs" de 64 bits.
        // Nota: En Big-Endian, ptr[0] son los bits más significativos (High),
        // ptr[3] son los menos significativos (Low).
        // Para sumar un u64, debemos operar sobre ptr[3] primero.

        let mut w3 = u64::from_be(*ptr.add(3)); // Low 64 bits
        let mut w2 = u64::from_be(*ptr.add(2));
        let mut w1 = u64::from_be(*ptr.add(1));
        let mut w0 = u64::from_be(*ptr.add(0)); // High 64 bits

        // Ejecución de la cadena de suma con acarreo (ADD -> ADC -> ADC -> ADC)
        asm!(
            "add {0}, {4}",      // w3 += value
            "adc {1}, 0",        // w2 += carry
            "adc {2}, 0",        // w1 += carry
            "adc {3}, 0",        // w0 += carry
            "setc {5}",          // Guardar overflow final en carry flag
            inout(reg) w3,
            inout(reg) w2,
            inout(reg) w1,
            inout(reg) w0,
            in(reg) value,
            out(reg_byte) carry,
            options(nostack, preserves_flags)
        );

        if carry != 0 {
            return Err(MathError::InvalidKeyFormat("SCALAR_OVERFLOW_256".to_string()));
        }

        // Reconversión a Big-Endian para persistencia
        *ptr.add(3) = w3.to_be();
        *ptr.add(2) = w2.to_be();
        *ptr.add(1) = w1.to_be();
        *ptr.add(0) = w0.to_be();
    }
    Ok(())
}

/// Fallback seguro para arquitecturas no x86_64 (ej: ARM/Apple Silicon).
#[inline(always)]
#[cfg(not(target_arch = "x86_64"))]
pub fn add_u64_to_u256_be(buffer: &mut [u8; 32], value: u64) -> Result<(), MathError> {
    let mut carry = value as u128;
    for chunk in buffer.rchunks_exact_mut(8) {
        let mut chunk_val = [0u8; 8];
        chunk_val.copy_from_slice(chunk);
        let val = u64::from_be_bytes(chunk_val) as u128;
        let sum = val + carry;
        chunk.copy_from_slice(&(sum as u64).to_be_bytes());
        carry = sum >> 64;
    }
    if carry > 0 {
        return Err(MathError::InvalidKeyFormat("SCALAR_OVERFLOW_256".to_string()));
    }
    Ok(())
}

/**
 * Compara dos números de 256 bits representados como arrays de bytes Big-Endian.
 *
 * # Uso
 * Crítico para `CombinatoricIterator` para detectar cuando se alcanza el final del rango.
 */
#[inline]
pub fn compare_u256_be(a: &[u8; 32], b: &[u8; 32]) -> Ordering {
    // Comparación lexicográfica directa funciona para Big-Endian unsigned integers
    for i in 0..32 {
        match a[i].cmp(&b[i]) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }
    Ordering::Equal
}

/**
 * Codificación hexadecimal optimizada.
 * Envoltorio sobre `hex::encode` para estandarizar la interfaz del dominio.
 */
pub fn fast_hex_encode(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/**
 * Suma completa de dos números de 256 bits (A + B).
 *
 * # Requerido por
 * - `KangarooSolver`: Para saltos distantes en la tabla hash.
 */
pub fn add_u256_be(a: &[u8; 32], b: &[u8; 32]) -> Result<[u8; 32], MathError> {
    let mut result = [0u8; 32];
    let mut carry = 0u16;

    // Iteramos desde el byte menos significativo (índice 31) hacia atrás
    for i in (0..32).rev() {
        let sum = (a[i] as u16) + (b[i] as u16) + carry;
        result[i] = (sum & 0xFF) as u8;
        carry = sum >> 8;
    }

    if carry > 0 {
        return Err(MathError::InvalidKeyFormat("U256_ADDITION_OVERFLOW".to_string()));
    }

    Ok(result)
}

/**
 * Resta completa de dos números de 256 bits (A - B).
 *
 * # Requerido por
 * - `KangarooSolver`: Para calcular la distancia entre huellas.
 */
pub fn sub_u256_be(a: &[u8; 32], b: &[u8; 32]) -> Result<[u8; 32], MathError> {
    let mut result = [0u8; 32];
    let mut borrow = 0i16;

    for i in (0..32).rev() {
        let diff = (a[i] as i16) - (b[i] as i16) - borrow;
        if diff < 0 {
            result[i] = (diff + 256) as u8;
            borrow = 1;
        } else {
            result[i] = diff as u8;
            borrow = 0;
        }
    }

    if borrow > 0 {
        return Err(MathError::InvalidKeyFormat("U256_SUBTRACTION_UNDERFLOW".to_string()));
    }

    Ok(result)
}

/**
 * Conversión auxiliar de u128 a u256 (Big-Endian).
 */
pub fn u128_to_u256_be(value: u128) -> [u8; 32] {
    let mut buffer = [0u8; 32];
    let bytes = value.to_be_bytes();
    // Copiamos los 16 bytes del u128 al final del buffer de 32 bytes (relleno de ceros a la izquierda)
    buffer[16..32].copy_from_slice(&bytes);
    buffer
}

/**
 * Convierte un array de bytes a palabras u64.
 * Útil para la conversión hacia el motor de campo (FieldElement).
 */
#[inline(always)]
pub fn bytes_to_words_u256(bytes: &[u8; 32]) -> [u64; 4] {
    let mut words = [0u64; 4];
    for i in 0..4 {
        // En FieldElement usamos little-endian para los limbs [u0, u1, u2, u3]
        // donde u0 son los bits menos significativos.
        // Pero el input bytes es Big-Endian [High ... Low].
        // Por tanto, words[0] (low) corresponde a los últimos 8 bytes.
        let start = (3 - i) * 8;
        let end = start + 8;
        words[i] = u64::from_be_bytes(bytes[start..end].try_into().unwrap());
    }
    words
}

/**
 * Convierte palabras u64 de vuelta a array de bytes Big-Endian.
 */
#[inline(always)]
pub fn words_to_bytes_u256(words: &[u64; 4]) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    for i in 0..4 {
        let word_bytes = words[3 - i].to_be_bytes();
        bytes[i * 8..(i + 1) * 8].copy_from_slice(&word_bytes);
    }
    bytes
}
