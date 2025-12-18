// libs/core/math-engine/src/arithmetic.rs
/**
 * =================================================================
 * APARATO: BYTE ARITHMETIC ENGINE (V14.0 - KERNEL IGNITION)
 * CLASIFICACIÓN: CORE MATH (L1)
 * RESPONSABILIDAD: OPERACIONES U256 DE COSTO CERO
 * ESTADO: ELITE PERFORMANCE // ASM HARDENED
 * =================================================================
 */

use crate::errors::MathError;
use std::arch::asm;

/// Longitud constante en bytes para un entero de 256 bits.
pub const U256_BYTE_SIZE: usize = 32;

/**
 * Realiza un incremento de +1 sobre un buffer de 32 bytes (U256 Big-Endian).
 *
 * # Optimización de Élite
 * En x86_64, utiliza 'ADC' (Add with Carry) encadenado.
 * Al operar directamente sobre el puntero como qwords, eliminamos el bucle
 * de bytes y las comprobaciones de saltos del compilador.
 */
#[inline(always)]
pub fn fast_increment_u256_be(buffer: &mut [u8; U256_BYTE_SIZE]) -> Result<(), MathError> {
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            let ptr = buffer.as_mut_ptr() as *mut u64;
            let mut overflow: u8;

            // En Big-Endian [u8; 32], el qword menos significativo está en el offset 24
            asm!(
                "add qword ptr [{p} + 24], 1",
                "adc qword ptr [{p} + 16], 0",
                "adc qword ptr [{p} + 8], 0",
                "adc qword ptr [{p}], 0",
                "setc {ovf}",
                p = in(reg) ptr,
                ovf = out(reg_byte) overflow,
                options(nostack, preserves_flags)
            );

            if overflow != 0 {
                return Err(MathError::InvalidKeyFormat("U256_OVERFLOW".into()));
            }
        }
        Ok(())
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        // Fallback optimizado para arquitecturas no x86 (ej: Apple M-Series/Graviton)
        let mut carry = 1u64;
        for i in (0..4).rev() {
            let p = buffer.as_mut_ptr() as *mut u64;
            unsafe {
                let val = u64::from_be(*p.add(i));
                let (new_val, new_carry) = val.overflowing_add(carry);
                *p.add(i) = new_val.to_be();
                carry = if new_carry { 1 } else { 0 };
            }
            if carry == 0 { break; }
        }
        if carry > 0 { return Err(MathError::InvalidKeyFormat("U256_OVERFLOW".into())); }
        Ok(())
    }
}
