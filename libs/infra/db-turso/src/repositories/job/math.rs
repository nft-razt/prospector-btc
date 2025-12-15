// libs/infra/db-turso/src/repositories/job/math.rs
// =================================================================
// APARATO: JOB MATH ENGINE (v2.0 - ZERO PADDING PROTOCOL)
// RESPONSABILIDAD: ARITMÉTICA DE RANGOS DE 256 BITS
// GARANTÍA: ORDENAMIENTO LEXICOGRÁFICO CORRECTO EN SQLITE
// =================================================================

use anyhow::{anyhow, Context, Result};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

/// Tamaño del paso para cada unidad de trabajo (1 Billón de claves).
const RANGE_STEP_SIZE: &str = "1000000000";

/// Longitud fija para cadenas numéricas de 256 bits.
/// 2^256 tiene 78 dígitos decimales. Rellenamos con ceros para
/// garantizar que el ordenamiento alfabético de SQL equivalga al numérico.
const U256_DECIMAL_LENGTH: usize = 78;

/// Rellena una cadena numérica con ceros a la izquierda hasta alcanzar 78 caracteres.
/// Ejemplo: "123" -> "000...000123"
pub fn pad_u256(value: &BigUint) -> String {
    let raw = value.to_string();
    if raw.len() >= U256_DECIMAL_LENGTH {
        return raw;
    }
    format!("{:0>width$}", raw, width = U256_DECIMAL_LENGTH)
}

/// Calcula el siguiente rango de búsqueda basado en el último punto final conocido.
///
/// # Argumentos
/// * `last_end_opt`: El valor `range_end` más alto encontrado en la DB.
///
/// # Retorno
/// Tupla `(start, end)` formateada con Zero-Padding lista para inserción.
pub fn calculate_next_range(last_end_opt: Option<String>) -> Result<(String, String)> {
    // 1. Determinar el punto de partida (Start)
    // Si no hay trabajos previos, empezamos desde 0 (Génesis).
    // Si hay, start = last_end + 1.
    let next_start: BigUint = match last_end_opt {
        Some(s) => {
            let last_val = BigUint::from_str(&s)
                .map_err(|e| anyhow!("Error parseando BigInt de DB: {}", e))?;
            last_val + BigUint::one()
        }
        None => BigUint::zero(),
    };

    // 2. Determinar el tamaño del paso (Step)
    let step = BigUint::from_str(RANGE_STEP_SIZE).context("Constante RANGE_STEP_SIZE inválida")?;

    // 3. Calcular el final (End)
    // El rango es [start, end), donde end es exclusivo para el siguiente bloque.
    let next_end = &next_start + &step;

    // 4. Aplicar Protocolo de Padding
    // Vital para mantener la integridad del índice SQL.
    let start_str = pad_u256(&next_start);
    let end_str = pad_u256(&next_end);

    Ok((start_str, end_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding_invariant() {
        let n = BigUint::from(123u32);
        let padded = pad_u256(&n);
        assert_eq!(padded.len(), 78);
        assert!(padded.ends_with("123"));
        assert!(padded.starts_with("000"));
    }

    #[test]
    fn test_next_range_genesis() {
        let (start, end) = calculate_next_range(None).unwrap();

        let expected_start = format!("{:0>78}", "0");
        let expected_end = format!("{:0>78}", RANGE_STEP_SIZE);

        assert_eq!(start, expected_start);
        assert_eq!(end, expected_end);
    }

    #[test]
    fn test_next_range_consecutive() {
        let previous_end_val = BigUint::from_str("1000000000").unwrap();
        let previous_end_str = pad_u256(&previous_end_val);

        let (start, end) = calculate_next_range(Some(previous_end_str)).unwrap();

        // Start debe ser previous_end + 1
        let expected_start_val = previous_end_val + BigUint::one();
        assert_eq!(start, pad_u256(&expected_start_val));

        // End debe ser start + step
        let step = BigUint::from_str(RANGE_STEP_SIZE).unwrap();
        let expected_end_val = expected_start_val + step;
        assert_eq!(end, pad_u256(&expected_end_val));
    }
}
