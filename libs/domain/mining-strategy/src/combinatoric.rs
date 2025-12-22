// libs/domain/mining-strategy/src/combinatoric.rs
/**
 * =================================================================
 * APARATO: COMBINATORIC ITERATOR (V18.0 - CLEANED)
 * RESPONSABILIDAD: GENERACIÓN SECUENCIAL DE ENTROPÍA U256
 * =================================================================
 */

use hex;
use prospector_core_math::arithmetic::{
    add_u64_to_u256_be,
    compare_u256_be,
    fast_hex_encode,
    U256_BYTE_SIZE
};
use prospector_core_math::private_key::SafePrivateKey;
use std::cmp::Ordering;

/// Iterador para recorrer un rango numérico U256 con prefijos/sufijos fijos.
pub struct CombinatoricIterator {
    current_state_bytes: [u8; U256_BYTE_SIZE],
    end_state_bytes: [u8; U256_BYTE_SIZE],
    prefix_string: String,
    suffix_string: String,
    // total_iterations eliminado por dead code warning
}

impl CombinatoricIterator {
    /// Crea un nuevo iterador combinatorio.
    pub fn new(start_hex: &str, end_hex: &str, prefix: String, suffix: String) -> Self {
        let mut start_buffer = [0u8; U256_BYTE_SIZE];
        let mut end_buffer = [0u8; U256_BYTE_SIZE];

        if let Ok(d) = hex::decode(start_hex.trim()) { if d.len() == 32 { start_buffer.copy_from_slice(&d); } }
        if let Ok(d) = hex::decode(end_hex.trim()) { if d.len() == 32 { end_buffer.copy_from_slice(&d); } }

        Self {
            current_state_bytes: start_buffer,
            end_state_bytes: end_buffer,
            prefix_string: prefix,
            suffix_string: suffix,
        }
    }
}

impl Iterator for CombinatoricIterator {
    type Item = (String, SafePrivateKey);

    fn next(&mut self) -> Option<Self::Item> {
        if compare_u256_be(&self.current_state_bytes, &self.end_state_bytes) == Ordering::Greater {
            return None;
        }

        let entropy_hex = fast_hex_encode(&self.current_state_bytes);
        let mut candidate = String::with_capacity(self.prefix_string.len() + self.suffix_string.len() + 64);
        candidate.push_str(&self.prefix_string);
        candidate.push_str(&entropy_hex);
        candidate.push_str(&self.suffix_string);

        let key = crate::brainwallet::phrase_to_private_key(&candidate);

        if add_u64_to_u256_be(&mut self.current_state_bytes, 1).is_err() {
            return None;
        }

        Some((candidate, key))
    }
}
