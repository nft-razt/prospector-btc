// libs/domain/mining-strategy/src/combinatoric.rs
// =================================================================
// APARATO: COMBINATORIC ITERATOR (V16.5)
// RESPONSABILIDAD: GENERACIÓN SECUENCIAL DE ENTROPÍA U256
// ESTADO: ZERO-WARNINGS // NO ABBREVIATIONS
// =================================================================

use hex;
use prospector_core_math::arithmetic::{
    add_u64_to_u256_be,
    compare_u256_be,
    fast_hex_encode,
    U256_BYTE_SIZE
};
use prospector_core_math::private_key::SafePrivateKey;
use std::cmp::Ordering;

pub struct CombinatoricIterator {
    current_state_bytes: [u8; U256_BYTE_SIZE],
    end_state_bytes: [u8; U256_BYTE_SIZE], // ✅ RESOLUCIÓN: Ahora se lee para validación
    prefix_string: String,
    suffix_string: String,
    total_iterations: u64,
    current_iteration: u64,
}

impl CombinatoricIterator {
    pub fn new(start_hex: &str, end_hex: &str, prefix: String, suffix: String) -> Self {
        let mut start_buffer = [0u8; U256_BYTE_SIZE];
        let mut end_buffer = [0u8; U256_BYTE_SIZE];

        if let Ok(d) = hex::decode(start_hex.trim()) { if d.len() == 32 { start_buffer.copy_from_slice(&d); } }
        if let Ok(d) = hex::decode(end_hex.trim()) { if d.len() == 32 { end_buffer.copy_from_slice(&d); } }

        let iteration_delta = if compare_u256_be(&end_buffer, &start_buffer) == Ordering::Greater {
            let mut steps_raw = [0u8; 8];
            steps_raw.copy_from_slice(&end_buffer[24..32]);
            let end_val = u64::from_be_bytes(steps_raw);
            steps_raw.copy_from_slice(&start_buffer[24..32]);
            let start_val = u64::from_be_bytes(steps_raw);
            end_val.saturating_sub(start_val)
        } else {
            0
        };

        Self {
            current_state_bytes: start_buffer,
            end_state_bytes: end_buffer,
            prefix_string: prefix,
            suffix_string: suffix,
            total_iterations: iteration_delta,
            current_iteration: 0,
        }
    }
}

impl Iterator for CombinatoricIterator {
    type Item = (String, SafePrivateKey);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        // Validación de frontera doble: Conteo y Magnitud
        if self.current_iteration >= self.total_iterations { return None; }
        if compare_u256_be(&self.current_state_bytes, &self.end_state_bytes) == Ordering::Greater { return None; }

        let entropy_hex = fast_hex_encode(&self.current_state_bytes);
        let mut candidate = String::with_capacity(self.prefix_string.len() + self.suffix_string.len() + 64);
        candidate.push_str(&self.prefix_string);
        candidate.push_str(&entropy_hex);
        candidate.push_str(&self.suffix_string);

        let key = crate::brainwallet::phrase_to_private_key(&candidate);
        self.current_state_bytes = add_u64_to_u256_be(&self.current_state_bytes, 1).ok()?;
        self.current_iteration += 1;

        Some((candidate, key))
    }
}
