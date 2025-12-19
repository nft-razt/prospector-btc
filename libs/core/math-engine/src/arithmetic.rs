// libs/core/math-engine/src/arithmetic.rs
/**
 * =================================================================
 * APARATO: BYTE ARITHMETIC ENGINE (V15.9 - FULL DOCS)
 * CLASIFICACIÓN: CORE MATH (L1)
 * RESPONSABILIDAD: OPERACIONES U256 DE COSTO CERO
 * ESTADO: ASM HARDENED // ZERO-BIGINT HOTPATH // ZERO-WARNINGS
 * =================================================================
 */
use crate::errors::MathError;
use std::arch::asm;
use std::cmp::Ordering;

/// Longitud constante en bytes para un entero de 256 bits (U256).
/// Utilizado para la representación Big-Endian de claves privadas en el ledger.
pub const U256_BYTE_SIZE: usize = 32;

/// Compara dos buffers U256 Big-Endian de forma eficiente.
///
/// Retorna un [Ordering] indicando si el primer buffer es menor, igual o mayor que el segundo.
#[inline(always)]
pub fn compare_u256_be(a: &[u8; 32], b: &[u8; 32]) -> Ordering {
    a.cmp(b)
}

/// Suma un valor u64 a un buffer U256 Big-Endian utilizando propagación de Carry en ASM.
///
/// Esta operación es el motor de los ataques secuenciales y combinatorios.
/// Retorna un nuevo array de 32 bytes con el resultado o [MathError] en caso de desbordamiento.
#[inline(always)]
#[allow(unsafe_code)]
pub fn add_u64_to_u256_be(buffer: &[u8; 32], value: u64) -> Result<[u8; 32], MathError> {
    let mut res = *buffer;
    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: El buffer tiene 32 bytes garantizados por el tipo [u8; 32].
        // Se accede a través de punteros u64 para sumar en bloques de 64 bits.
        unsafe {
            let ptr = res.as_mut_ptr() as *mut u64;
            let mut ovf: u8;
            asm!(
                "add qword ptr [{p} + 24], {val}",
                "adc qword ptr [{p} + 16], 0",
                "adc qword ptr [{p} + 8], 0",
                "adc qword ptr [{p}], 0",
                "setc {ovf}",
                p = in(reg) ptr,
                val = in(reg) value,
                ovf = out(reg_byte) ovf,
                options(nostack, preserves_flags)
            );
            if ovf != 0 {
                return Err(MathError::InvalidKeyFormat("U256_OVERFLOW".into()));
            }
        }
    }
    Ok(res)
}

/// Suma dos enteros U256 Big-Endian utilizando una cadena de ADC (Add with Carry) en ASM.
///
/// Diseñado para operaciones de salto en el algoritmo Pollard's Kangaroo.
#[inline(always)]
#[allow(unsafe_code)]
pub fn add_u256_be(a: &[u8; 32], b: &[u8; 32]) -> Result<[u8; 32], MathError> {
    let mut res = *a;
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            let r_ptr = res.as_mut_ptr() as *mut u64;
            let b_ptr = b.as_ptr() as *const u64;
            let mut ovf: u8;
            asm!(
                "mov {tmp}, qword ptr [{b} + 24]",
                "add qword ptr [{r} + 24], {tmp}",
                "mov {tmp}, qword ptr [{b} + 16]",
                "adc qword ptr [{r} + 16], {tmp}",
                "mov {tmp}, qword ptr [{b} + 8]",
                "adc qword ptr [{r} + 8], {tmp}",
                "mov {tmp}, qword ptr [{b}]",
                "adc qword ptr [{r}], {tmp}",
                "setc {ovf}",
                r = in(reg) r_ptr,
                b = in(reg) b_ptr,
                tmp = out(reg) _,
                ovf = out(reg_byte) ovf,
                options(nostack, preserves_flags)
            );
            if ovf != 0 {
                return Err(MathError::InvalidKeyFormat("U256_OVERFLOW".into()));
            }
        }
    }
    Ok(res)
}

/// Resta el buffer b del buffer a (U256 Big-Endian) utilizando SBB (Subtract with Borrow) en ASM.
///
/// Retorna el resultado o un error si ocurre un desbordamiento negativo (Underflow).
#[inline(always)]
#[allow(unsafe_code)]
pub fn sub_u256_be(a: &[u8; 32], b: &[u8; 32]) -> Result<[u8; 32], MathError> {
    let mut res = *a;
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            let r_ptr = res.as_mut_ptr() as *mut u64;
            let b_ptr = b.as_ptr() as *const u64;
            let mut brw: u8;
            asm!(
                "mov {tmp}, qword ptr [{b} + 24]",
                "sub qword ptr [{r} + 24], {tmp}",
                "mov {tmp}, qword ptr [{b} + 16]",
                "sbb qword ptr [{r} + 16], {tmp}",
                "mov {tmp}, qword ptr [{b} + 8]",
                "sbb qword ptr [{r} + 8], {tmp}",
                "mov {tmp}, qword ptr [{b}]",
                "sbb qword ptr [{r}], {tmp}",
                "setc {brw}",
                r = in(reg) r_ptr,
                b = in(reg) b_ptr,
                tmp = out(reg) _,
                brw = out(reg_byte) brw,
                options(nostack, preserves_flags)
            );
            if brw != 0 {
                return Err(MathError::InvalidKeyFormat("U256_UNDERFLOW".into()));
            }
        }
    }
    Ok(res)
}

/// Convierte un entero de 128 bits a una representación de buffer U256 (32 bytes) Big-Endian.
///
/// Se utiliza para normalizar las distancias de salto de los canguros.
pub fn u128_to_u256_be(val: u128) -> [u8; 32] {
    let mut res = [0u8; 32];
    let bytes = val.to_be_bytes();
    res[16..32].copy_from_slice(&bytes);
    res
}

/// Codificación hexadecimal de alto rendimiento para el motor de telemetría.
pub fn fast_hex_encode(bytes: &[u8; 32]) -> String {
    hex::encode(bytes)
}
