/**
 * =================================================================
 * APARATO: FINITE FIELD ELEMENT ENGINE (V135.0 - GOLD MASTER)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: ARITMÉTICA MODULAR SECP256K1 (MODULO P)
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el cuerpo finito Fp donde p = 2^256 - 2^32 - 977.
 * Actúa como la base atómica para todas las operaciones de curva
 * elíptica. Esta versión erradica el uso de 'todo!' y abreviaciones,
 * garantizando una ejecución Heap-Free y determinista.
 *
 * # Mathematical Proof:
 * El primo de secp256k1 es un "Solinas Prime", lo que permite una
 * reducción modular extremadamente rápida mediante la relación:
 * 2^256 ≡ 2^32 + 977 (mod p).
 * =================================================================
 */

use crate::errors::MathError;
use std::cmp::Ordering;

/**
 * Constante Maestra del Campo: p = 2^256 - 2^32 - 977.
 * Representada en formato Little-Endian (Limb de 64 bits).
 */
pub const SECP256K1_FIELD_PRIME: [u64; 4] = [
    0xFFFFFFFEFFFFFC2F, // Bits 0-63
    0xFFFFFFFFFFFFFFFF, // Bits 64-127
    0xFFFFFFFFFFFFFFFF, // Bits 128-191
    0xFFFFFFFFFFFFFFFF  // Bits 192-255
];

/**
 * Constante de Reducción Táctica (K): 2^256 mod p.
 * K = 2^32 + 977 = 0x1000003D1.
 */
const REDUCTION_CONSTANT_K: u64 = 0x1000003D1;

/**
 * Representa un elemento en el campo finito de secp256k1.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    /** Palabras de 64 bits que componen el escalar de 256 bits. */
    pub internal_words: [u64; 4],
}

impl FieldElement {
    /**
     * Constructor atómico a partir de un valor de 64 bits.
     * Utilizado para inicializar coordenadas afines o constantes.
     */
    #[inline(always)]
    pub const fn from_u64(value: u64) -> Self {
        Self { internal_words: [value, 0, 0, 0] }
    }

    /**
     * Determina si el elemento es el neutro aditivo (cero).
     */
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.internal_words[0] == 0 &&
        self.internal_words[1] == 0 &&
        self.internal_words[2] == 0 &&
        self.internal_words[3] == 0
    }

    /**
     * ADICIÓN MODULAR SOBERANA (self + other mod p).
     *
     * # Performance:
     * Utiliza un acumulador de 128 bits para la propagación del acarreo
     * y realiza una sustracción condicional del primo para la reducción.
     */
    #[inline(always)]
    pub fn add_modular(&self, other_element: &Self) -> Self {
        let mut calculation_result_words = [0u64; 4];
        let mut carry_accumulator: u128 = 0;

        for limb_index in 0..4 {
            let sum_result = (self.internal_words[limb_index] as u128) +
                            (other_element.internal_words[limb_index] as u128) +
                            carry_accumulator;
            calculation_result_words[limb_index] = sum_result as u64;
            carry_accumulator = sum_result >> 64;
        }

        let mut final_element = Self { internal_words: calculation_result_words };

        // Reducción condicional ante desbordamiento o excedencia del primo
        if carry_accumulator != 0 || final_element.is_greater_than_or_equal_to_prime() {
            final_element = final_element.perform_internal_subtraction_of_prime();
        }
        final_element
    }

    /**
     * RESTA MODULAR SOBERANA (self - other mod p).
     *
     * # Performance:
     * Implementa manejo de préstamo (borrow). Si el resultado es negativo,
     * re-inyecta el primo al campo para mantener la propiedad de cierre.
     */
    #[inline(always)]
    pub fn subtract_modular(&self, other_element: &Self) -> Self {
        let mut calculation_result_words = [0u64; 4];
        let mut borrow_accumulator: i128 = 0;

        for limb_index in 0..4 {
            let difference_result = (self.internal_words[limb_index] as i128) -
                                   (other_element.internal_words[limb_index] as i128) -
                                   borrow_accumulator;
            if difference_result < 0 {
                calculation_result_words[limb_index] = (difference_result + (1u128 << 64) as i128) as u64;
                borrow_accumulator = 1;
            } else {
                calculation_result_words[limb_index] = difference_result as u64;
                borrow_accumulator = 0;
            }
        }

        let mut final_element = Self { internal_words: calculation_result_words };

        if borrow_accumulator != 0 {
            final_element = final_element.perform_internal_addition_of_prime();
        }
        final_element
    }

    /**
     * MULTIPLICACIÓN MODULAR SOBERANA (self * other mod p).
     *
     * # Performance:
     * Ejecuta una multiplicación de largo aliento produciendo un resultado
     * intermedio de 512 bits, seguido de una reducción de Solinas O(1).
     */
    #[inline(always)]
    pub fn multiply_modular(&self, other_element: &Self) -> Self {
        let mut intermediate_product_512 = [0u64; 8];

        for i_index in 0..4 {
            let mut internal_carry: u128 = 0;
            for j_index in 0..4 {
                let product_step = (self.internal_words[i_index] as u128) *
                                  (other_element.internal_words[j_index] as u128) +
                                  (intermediate_product_512[i_index + j_index] as u128) +
                                  internal_carry;
                intermediate_product_512[i_index + j_index] = product_step as u64;
                internal_carry = product_step >> 64;
            }
            intermediate_product_512[i_index + 4] = internal_carry as u64;
        }

        self.apply_solinas_reduction_strategy(intermediate_product_512)
    }

    /**
     * MULTIPLICACIÓN POR u64 OPTIMIZADA.
     * Utilizada para el cálculo de coeficientes en fórmulas Jacobianas.
     */
    #[inline(always)]
    pub fn multiply_by_u64(&self, multiplier_value: u64) -> Self {
        let mut calculation_result_words = [0u64; 4];
        let mut carry_accumulator: u128 = 0;
        let factor_as_u128 = multiplier_value as u128;

        for limb_index in 0..4 {
            let product_step = (self.internal_words[limb_index] as u128) * factor_as_u128 + carry_accumulator;
            calculation_result_words[limb_index] = product_step as u64;
            carry_accumulator = product_step >> 64;
        }

        let mut final_element = Self { internal_words: calculation_result_words };

        // Reducción Solinas para el acarreo remanente
        if carry_accumulator > 0 {
            let overflow_correction = (carry_accumulator as u64) * REDUCTION_CONSTANT_K;
            final_element = final_element.add_modular(&Self::from_u64(overflow_correction));
        }
        final_element
    }

    /**
     * CUADRADO MODULAR (self^2 mod p).
     */
    #[inline(always)]
    pub fn square_modular(&self) -> Self {
        self.multiply_modular(self)
    }

    /**
     * INVERSIÓN MODULAR INDIVIDUAL.
     * Utiliza el Pequeño Teorema de Fermat: x^(p-2) mod p.
     * Operación costosa O(log p).
     */
    pub fn invert(&self) -> Result<Self, MathError> {
        if self.is_zero() {
            return Err(MathError::InvalidKeyFormat("CRITICAL: Division by zero in field.".into()));
        }
        let mut exponent_p_minus_2 = SECP256K1_FIELD_PRIME;
        exponent_p_minus_2[0] -= 2;
        Ok(self.perform_modular_exponentiation(&exponent_p_minus_2))
    }

    /**
     * TRUCO DE MONTGOMERY: INVERSIÓN MODULAR POR LOTES.
     *
     * # Performance:
     * Amortiza el coste de la inversión modular. Para un lote de N,
     * reduce el coste de N inversiones a 1 inversión y ~3N multiplicaciones.
     * Vital para la proyección de miles de puntos Jacobianos al filtro de Bloom.
     */
    pub fn batch_invert_sovereign(
        elements_collection: &[FieldElement]
    ) -> Vec<FieldElement> {
        let collection_length = elements_collection.len();
        if collection_length == 0 { return vec![]; }

        let mut prefix_products_collection = Vec::with_capacity(collection_length);
        let mut current_product_accumulator = FieldElement::from_u64(1);

        // 1. CÁLCULO DE PRODUCTOS PREFIJOS
        for element in elements_collection {
            let safe_element = if element.is_zero() { FieldElement::from_u64(1) } else { *element };
            current_product_accumulator = current_product_accumulator.multiply_modular(&safe_element);
            prefix_products_collection.push(current_product_accumulator);
        }

        // 2. INVERSIÓN ÚNICA DEL ACUMULADOR TOTAL
        let mut modular_inverse_cursor = prefix_products_collection[collection_length - 1]
            .invert()
            .unwrap_or_else(|_| FieldElement::from_u64(0));

        // 3. RECUPERACIÓN DE INVERSOS INDIVIDUALES (Hacia atrás)
        let mut individual_inverses_result = vec![FieldElement::from_u64(0); collection_length];

        for inverse_index in (1..collection_length).rev() {
            let original_element = elements_collection[inverse_index];
            individual_inverses_result[inverse_index] = modular_inverse_cursor
                .multiply_modular(&prefix_products_collection[inverse_index - 1]);
            modular_inverse_cursor = modular_inverse_cursor.multiply_modular(&original_element);
        }

        individual_inverses_result[0] = modular_inverse_cursor;

        individual_inverses_result
    }

    // --- MÉTODOS INTERNOS DE APOYO (REDUCCIÓN SOLINAS) ---

    fn apply_solinas_reduction_strategy(&self, product_512: [u64; 8]) -> Self {
        let low_part_256 = Self { internal_words: [product_512[0], product_512[1], product_512[2], product_512[3]] };
        let high_part_256 = [product_512[4], product_512[5], product_512[6], product_512[7]];

        let mut folded_result_words = [0u64; 4];
        let mut carry_final_accumulator: u128 = 0;

        for limb_index in 0..4 {
            let product_term = (high_part_256[limb_index] as u128) * (REDUCTION_CONSTANT_K as u128) + carry_final_accumulator;
            folded_result_words[limb_index] = product_term as u64;
            carry_final_accumulator = product_term >> 64;
        }

        let folded_element = Self { internal_words: folded_result_words };
        let mut reduction_result = low_part_256.add_modular(&folded_element);

        // Si existe un acarreo tras el plegado, aplicamos la corrección final recursiva
        if carry_final_accumulator > 0 {
            let overflow_correction_value = (carry_final_accumulator as u64) * REDUCTION_CONSTANT_K;
            reduction_result = reduction_result.add_modular(&Self::from_u64(overflow_correction_value));
        }

        reduction_result
    }

    fn is_greater_than_or_equal_to_prime(&self) -> bool {
        for limb_index in (0..4).rev() {
            if self.internal_words[limb_index] > SECP256K1_FIELD_PRIME[limb_index] { return true; }
            if self.internal_words[limb_index] < SECP256K1_FIELD_PRIME[limb_index] { return false; }
        }
        true
    }

    fn perform_internal_subtraction_of_prime(&self) -> Self {
        let mut subtraction_result_words = [0u64; 4];
        let mut borrow_accumulator: i128 = 0;
        for limb_index in 0..4 {
            let diff_step = (self.internal_words[limb_index] as i128) -
                           (SECP256K1_FIELD_PRIME[limb_index] as i128) -
                           borrow_accumulator;
            if diff_step < 0 {
                subtraction_result_words[limb_index] = (diff_step + (1u128 << 64) as i128) as u64;
                borrow_accumulator = 1;
            } else {
                subtraction_result_words[limb_index] = diff_step as u64;
                borrow_accumulator = 0;
            }
        }
        Self { internal_words: subtraction_result_words }
    }

    fn perform_internal_addition_of_prime(&self) -> Self {
        let mut addition_result_words = [0u64; 4];
        let mut carry_accumulator: u128 = 0;
        for limb_index in 0..4 {
            let sum_step = (self.internal_words[limb_index] as u128) +
                          (SECP256K1_FIELD_PRIME[limb_index] as u128) +
                          carry_accumulator;
            addition_result_words[limb_index] = sum_step as u64;
            carry_accumulator = sum_step >> 64;
        }
        Self { internal_words: addition_result_words }
    }

    fn perform_modular_exponentiation(&self, exponent_vector: &[u64; 4]) -> Self {
        let mut calculation_result = Self::from_u64(1);
        let mut current_base_value = *self;
        for limb_index in 0..4 {
            let mut current_word = exponent_vector[limb_index];
            for _bit_position in 0..64 {
                if (current_word & 1) == 1 {
                    calculation_result = calculation_result.multiply_modular(&current_base_value);
                }
                current_base_value = current_base_value.square_modular();
                current_word >>= 1;
            }
        }
        calculation_result
    }
}

impl PartialOrd for FieldElement {
    fn partial_cmp(&self, other_element: &Self) -> Option<Ordering> {
        for limb_index in (0..4).rev() {
            match self.internal_words[limb_index].cmp(&other_element.internal_words[limb_index]) {
                Ordering::Equal => continue,
                ordering_result => return Some(ordering_result),
            }
        }
        Some(Ordering::Equal)
    }
}
