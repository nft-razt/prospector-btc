/**
 * =================================================================
 * APARATO: FINITE FIELD ENGINE (V116.0 - SOLINAS HARDENED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: ARITMÉTICA MODULAR SECP256K1 (MOD P)
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la aritmética de campo finito necesaria para las operaciones
 * de la curva elíptica. Utiliza un acumulador u128 para evitar desbordamientos
 * de acarreo y una reducción de Solinas optimizada para el primo de Bitcoin.
 *
 * MEJORAS:
 * - Restauración de `subtract_modular` con lógica de préstamo (borrow).
 * - Restauración de `square_modular` optimizado.
 * - Sincronización de API con curve.rs y point.rs.
 * =================================================================
 */

use crate::errors::MathError;
use std::cmp::Ordering;

/// Constante Primaria del Campo: p = 2^256 - 2^32 - 977 (Little-Endian)
pub const SECP256K1_PRIME: [u64; 4] = [
    0xFFFFFFFEFFFFFC2F, // Word 0: Bits bajos
    0xFFFFFFFFFFFFFFFF, // Word 1
    0xFFFFFFFFFFFFFFFF, // Word 2
    0xFFFFFFFFFFFFFFFF  // Word 3: Bits altos
];

/// Constante de Reducción K: 2^256 mod p = 2^32 + 977
const REDUCTION_CONSTANT_K: u64 = 0x1000003D1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    /// Palabras de 64 bits que representan el número de 256 bits (Little-Endian).
    pub internal_words: [u64; 4],
}

impl FieldElement {
    /**
     * Constructor atómico desde u64.
     */
    #[inline(always)]
    pub const fn from_u64(value: u64) -> Self {
        Self { internal_words: [value, 0, 0, 0] }
    }

    /**
     * ADICIÓN MODULAR (self + other mod p).
     */
    #[inline(always)]
    pub fn add_modular(&self, other: &Self) -> Self {
        let mut result_words = [0u64; 4];
        let mut carry_accumulator: u128 = 0;

        for i in 0..4 {
            let sum = (self.internal_words[i] as u128) + (other.internal_words[i] as u128) + carry_accumulator;
            result_words[i] = sum as u64;
            carry_accumulator = sum >> 64;
        }

        let mut element = Self { internal_words: result_words };

        // Reducción condicional: si hay acarreo final o el valor supera el primo.
        if carry_accumulator != 0 || element.is_greater_than_or_equal_to_prime() {
            element = element.perform_internal_subtraction_of_prime();
        }
        element
    }

    /**
     * RESTA MODULAR (self - other mod p).
     * ✅ RESOLUCIÓN: Restaura la funcionalidad perdida requerida por curve.rs.
     */
    #[inline(always)]
    pub fn subtract_modular(&self, other: &Self) -> Self {
        let mut result_words = [0u64; 4];
        let mut borrow_accumulator: i128 = 0;

        for i in 0..4 {
            let difference = (self.internal_words[i] as i128) - (other.internal_words[i] as i128) - borrow_accumulator;
            if difference < 0 {
                result_words[i] = (difference + (1u128 << 64) as i128) as u64;
                borrow_accumulator = 1;
            } else {
                result_words[i] = difference as u64;
                borrow_accumulator = 0;
            }
        }

        let mut element = Self { internal_words: result_words };

        // Si hubo préstamo (resultado negativo), sumamos el primo para volver al campo.
        if borrow_accumulator != 0 {
            element = element.perform_internal_addition_of_prime();
        }
        element
    }

    /**
     * MULTIPLICACIÓN MODULAR (self * other mod p).
     */
    #[inline(always)]
    pub fn multiply_modular(&self, other: &Self) -> Self {
        let mut intermediate_product = [0u64; 8];

        for i in 0..4 {
            let mut carry_chain = 0u128;
            for j in 0..4 {
                let product = (self.internal_words[i] as u128) * (other.internal_words[j] as u128)
                            + (intermediate_product[i + j] as u128)
                            + carry_chain;
                intermediate_product[i + j] = product as u64;
                carry_chain = product >> 64;
            }
            intermediate_product[i + 4] = carry_chain as u64;
        }

        self.apply_solinas_reduction(intermediate_product)
    }

    /**
     * CUADRADO MODULAR (self^2 mod p).
     * ✅ RESOLUCIÓN: Restaura la funcionalidad requerida por curve.rs y point.rs.
     */
    #[inline(always)]
    pub fn square_modular(&self) -> Self {
        self.multiply_modular(self)
    }

    /**
     * Reducción de Solinas optimizada para el primo de secp256k1.
     */
    fn apply_solinas_reduction(&self, product_512: [u64; 8]) -> Self {
        let low_part = Self { internal_words: [product_512[0], product_512[1], product_512[2], product_512[3]] };
        let high_part = [product_512[4], product_512[5], product_512[6], product_512[7]];

        let mut folded_words = [0u64; 4];
        let mut carry_final: u128 = 0;

        for i in 0..4 {
            let term = (high_part[i] as u128) * (REDUCTION_CONSTANT_K as u128) + carry_final;
            folded_words[i] = term as u64;
            carry_final = term >> 64;
        }

        let folded_element = Self { internal_words: folded_words };
        let mut result = low_part.add_modular(&folded_element);

        if carry_final > 0 {
            let overflow_correction = (carry_final as u64) * REDUCTION_CONSTANT_K;
            result = result.add_modular(&Self::from_u64(overflow_correction));
        }

        result
    }

    /**
     * Compara el elemento contra el primo de la curva.
     */
    #[inline(always)]
    fn is_greater_than_or_equal_to_prime(&self) -> bool {
        for i in (0..4).rev() {
            if self.internal_words[i] > SECP256K1_PRIME[i] { return true; }
            if self.internal_words[i] < SECP256K1_PRIME[i] { return false; }
        }
        true
    }

    fn perform_internal_subtraction_of_prime(&self) -> Self {
        let mut result = [0u64; 4];
        let mut borrow: i128 = 0;
        for i in 0..4 {
            let diff = (self.internal_words[i] as i128) - (SECP256K1_PRIME[i] as i128) - borrow;
            if diff < 0 {
                result[i] = (diff + (1u128 << 64) as i128) as u64;
                borrow = 1;
            } else {
                result[i] = diff as u64;
                borrow = 0;
            }
        }
        Self { internal_words: result }
    }

    fn perform_internal_addition_of_prime(&self) -> Self {
        let mut result = [0u64; 4];
        let mut carry: u128 = 0;
        for i in 0..4 {
            let sum = (self.internal_words[i] as u128) + (SECP256K1_PRIME[i] as u128) + carry;
            result[i] = sum as u64;
            carry = sum >> 64;
        }
        Self { internal_words: result }
    }

    /**
     * Inverso Modular mediante Pequeño Teorema de Fermat.
     */
    pub fn invert(&self) -> Result<Self, MathError> {
        if self.is_zero() {
            return Err(MathError::InvalidKeyFormat("DIVISION_BY_ZERO_IN_FIELD".to_string()));
        }
        let mut p_minus_2 = SECP256K1_PRIME;
        p_minus_2[0] -= 2;
        Ok(self.perform_modular_exponentiation(&p_minus_2))
    }

    pub fn is_zero(&self) -> bool {
        self.internal_words == [0, 0, 0, 0]
    }

    fn perform_modular_exponentiation(&self, exponent: &[u64; 4]) -> Self {
        let mut result = Self::from_u64(1);
        let mut base = *self;
        for i in 0..4 {
            let mut word = exponent[i];
            for _ in 0..64 {
                if (word & 1) == 1 {
                    result = result.multiply_modular(&base);
                }
                base = base.square_modular();
                word >>= 1;
            }
        }
        result
    }
}

// Implementación de PartialOrd para comparaciones condicionales de campo
impl PartialOrd for FieldElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in (0..4).rev() {
            match self.internal_words[i].cmp(&other.internal_words[i]) {
                Ordering::Equal => continue,
                ord => return Some(ord),
            }
        }
        Some(Ordering::Equal)
    }
}
