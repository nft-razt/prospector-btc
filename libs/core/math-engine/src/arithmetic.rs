// libs/core/math-engine/src/arithmetic.rs
// =================================================================
// APARATO: BYTE ARITHMETIC ENGINE
// RESPONSABILIDAD: OPERACIONES MATEMÁTICAS SOBRE BUFFERS DE BYTES
// OPTIMIZACIÓN: CARRY PROPAGATION SIN ALLOCACIONES
// =================================================================

use crate::errors::MathError;

/// Suma un entero de 128 bits (`u128`) a un escalar de 256 bits (`[u8; 32]`) en formato BigEndian.
///
/// Realiza la propagación de acarreo (carry) desde el byte menos significativo hasta el más
/// significativo, asegurando corrección matemática en todo el espacio de 256 bits.
///
/// # Argumentos
/// * `scalar`: El número base de 32 bytes (BigEndian).
/// * `addend`: El número a sumar (u128).
///
/// # Retorno
/// Retorna el nuevo array de 32 bytes o error si hay overflow total (excede 256 bits).
#[inline]
pub fn add_u128_to_u256_be(scalar: &[u8; 32], addend: u128) -> Result<[u8; 32], MathError> {
    let mut result = *scalar;
    let mut carry = addend;

    // Iteramos desde el último byte (31) hasta el primero (0)
    for i in (0..32).rev() {
        // Si no hay nada que sumar, terminamos temprano (Optimización)
        if carry == 0 {
            break;
        }

        // Tomamos el byte actual del 'carry' (u128)
        let value_to_add = (carry & 0xFF) as u16;

        // Desplazamos el carry para la siguiente iteración
        carry >>= 8;

        // Sumamos al byte actual del escalar
        let sum = result[i] as u16 + value_to_add;

        // Guardamos el byte (módulo 256)
        result[i] = (sum & 0xFF) as u8;

        // Si hubo overflow en este byte (sum > 255), sumamos al carry
        carry += (sum >> 8) as u128;
    }

    // Si al terminar el bucle todavía queda carry, significa que desbordamos los 256 bits
    if carry > 0 {
        return Err(MathError::InvalidKeyFormat("Overflow: El resultado excede 256 bits".into()));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition_with_propagation() {
        // Caso: [0, ... 0, 255] + 1 => [0, ... 1, 0]
        let mut base = [0u8; 32];
        base[31] = 0xFF;

        let res = add_u128_to_u256_be(&base, 1).unwrap();
        assert_eq!(res[31], 0x00);
        assert_eq!(res[30], 0x01);
    }

    #[test]
    fn test_addition_large_carry() {
        // Caso: Sumar un número grande que afecta múltiples bytes
        let mut base = [0u8; 32];
        let addend = 0xAABBCC; // 3 bytes

        let res = add_u128_to_u256_be(&base, addend).unwrap();
        assert_eq!(res[31], 0xCC);
        assert_eq!(res[30], 0xBB);
        assert_eq!(res[29], 0xAA);
    }
}
