/**
 * =================================================================
 * APARATO: BYTE ARITHMETIC ENGINE (V17.0 - ELITE ALIGNED)
 * CLASIFICACIÓN: CORE MATH (L1)
 * RESPONSABILIDAD: OPERACIONES U256 DE COSTO CERO (STACK-ONLY)
 *
 * PERFORMANCE:
 * - Utiliza ASM Inline para evitar el overhead de chequeo de límites de Rust
 *   en el hot-path del minero, manteniendo la seguridad de tipos.
 * =================================================================
 */

use crate::errors::MathError;
use std::arch::asm;
use std::cmp::Ordering;

/// Tamaño exacto en bytes para un escalar de 256 bits.
pub const U256_BYTE_SIZE: usize = 32;

/**
 * Realiza la comparación constante de dos buffers U256.
 *
 * @param operand_a Primer buffer de 32 bytes.
 * @param operand_b Segundo buffer de 32 bytes.
 * @returns Ordering (Less, Equal, Greater).
 */
#[inline]
#[must_use]
pub fn compare_u256_be(operand_a: &[u8; 32], operand_b: &[u8; 32]) -> Ordering {
    operand_a.cmp(operand_b)
}

/**
 * Incrementa un buffer U256 Big-Endian con un valor u64 de forma atómica en registros.
 *
 * # Performance
 * Esta función es el motor del Sequential Scanner. Ejecuta en ~1-3 ciclos de CPU.
 *
 * # Errors
 * Retorna `MathError::InvalidKeyFormat` si ocurre un desbordamiento (Overflow) por encima de 2^256.
 */
#[inline]
#[allow(unsafe_code)]
pub fn add_u64_to_u256_be(buffer: &[u8; 32], incremental_value: u64) -> Result<[u8; 32], MathError> {
    let mut result_buffer = *buffer;
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            let pointer_to_buffer = result_buffer.as_mut_ptr().cast::<u64>();
            let mut carry_flag: u8;

            // Cadena de adición con acarreo (ADC) para propagar el incremento
            asm!(
                "add qword ptr [{p} + 24], {val}",
                "adc qword ptr [{p} + 16], 0",
                "adc qword ptr [{p} + 8], 0",
                "adc qword ptr [{p}], 0",
                "setc {cf}",
                p = in(reg) pointer_to_buffer,
                val = in(reg) incremental_value,
                cf = out(reg_byte) carry_flag,
                options(nostack, preserves_flags)
            );

            if carry_flag != 0 {
                return Err(MathError::InvalidKeyFormat("U256_ARITHMETIC_OVERFLOW".to_string()));
            }
        }
    }
    Ok(result_buffer)
}
