// libs/infra/db-turso/src/repositories/job/math.rs
// =================================================================
// APARATO: JOB MATH ENGINE
// UBICACIÓN: PROYECTO LOCAL (NO EN REGISTRY)
// =================================================================

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;
use anyhow::{Result, anyhow};

// 1 Billón de claves por trabajo
const RANGE_STEP_SIZE: &str = "1000000000";

/// Calcula el siguiente rango de búsqueda basado en el último punto final conocido.
pub fn calculate_next_range(last_end_opt: Option<String>) -> Result<(String, String)> {
    // 1. Determinar el punto de partida (Start)
    let next_start: BigUint = match last_end_opt {
        Some(s) => {
            let last_val = BigUint::from_str(&s)
                .unwrap_or_else(|_| BigUint::zero());
            last_val + BigUint::one()
        },
        None => BigUint::zero(), // Génesis
    };

    // 2. Determinar el tamaño del paso (Step)
    let step = BigUint::from_str(RANGE_STEP_SIZE)
        .map_err(|_| anyhow!("Constante RANGE_STEP_SIZE inválida"))?;

    // 3. Calcular el final (End)
    let next_end = &next_start + &step;

    Ok((next_start.to_string(), next_end.to_string()))
}
