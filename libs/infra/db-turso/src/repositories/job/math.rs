// libs/infra/db-turso/src/repositories/job/math.rs
// =================================================================
// APARATO: JOB RANGE CALCULATOR (V15.8 - DEFINITIVE)
// RESPONSABILIDAD: CÁLCULO DE SEGMENTOS DE BÚSQUEDA U256
// ESTADO: NO-REGRESSIONS // ZERO-BIGINT IN HOT PATH
// =================================================================

use anyhow::{Context, Result};
use hex;
use prospector_core_math::arithmetic::{U256_BYTE_SIZE, fast_hex_encode, add_u64_to_u256_be};

/// Tamaño del bloque de búsqueda por defecto (1 Billón de claves).
const DEFAULT_RANGE_STEP: u64 = 1_000_000_000;

/// Longitud decimal para padding (2^256 requiere 78 dígitos).
const RANGE_PADDING_WIDTH: usize = 78;

pub struct RangeCalculator;

impl RangeCalculator {
    /// Calcula el siguiente segmento de búsqueda [Inicio, Fin] sincronizado con el Core.
    pub fn calculate_next(last_boundary_hex: Option<String>) -> Result<(String, String)> {
        // Determinar punto de partida
        let current_start_buffer = match last_boundary_hex {
            Some(hex_pointer) => {
                let decoded_bytes = hex::decode(hex_pointer.trim())
                    .context("FAILED_TO_DECODE_BOUNDARY: Invalid hex string in Ledger")?;

                let mut u256_array = [0u8; U256_BYTE_SIZE];
                u256_array.copy_from_slice(&decoded_bytes);

                // Incrementamos 1 para garantizar exclusividad: Start = LastEnd + 1
                add_u64_to_u256_be(&u256_array, 1)
                    .map_err(|e| anyhow::anyhow!("ARITHMETIC_OVERFLOW: {}", e))?
            }
            None => [0u8; U256_BYTE_SIZE], // Comienzo en el espacio cero
        };

        // Calculamos el límite final (End = Start + Step)
        let current_end_buffer = add_u64_to_u256_be(
            &current_start_buffer,
            DEFAULT_RANGE_STEP
        ).map_err(|e| anyhow::anyhow!("ARITHMETIC_OVERFLOW_STEP: {}", e))?;

        // Serialización optimizada
        Ok((
            fast_hex_encode(&current_start_buffer),
            fast_hex_encode(&current_end_buffer)
        ))
    }

    /// Transforma el valor a cadena decimal con padding para indexación en SQLite.
    pub fn to_lexicographical_string(hex_value: &str) -> Result<String> {
        let binary_bytes = hex::decode(hex_value.trim())
            .context("CONVERSION_ERROR: Invalid hex input")?;

        let mut temp_buffer = [0u8; U256_BYTE_SIZE];
        temp_buffer.copy_from_slice(&binary_bytes);

        // Conversión a BigUint aceptable fuera del bucle crítico
        let big_int = num_bigint::BigUint::from_bytes_be(&temp_buffer);
        let decimal_str = big_int.to_string();

        Ok(format!("{:0>width$}", decimal_str, width = RANGE_PADDING_WIDTH))
    }
}
