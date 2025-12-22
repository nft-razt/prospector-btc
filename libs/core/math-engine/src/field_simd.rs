/**
 * =================================================================
 * APARATO: VECTORIZED FIELD ENGINE (V100.1 - STABLE COMPLIANT)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: ARITMÉTICA MODULAR CUÁDRUPLE (4x U256)
 * =================================================================
 */

use crate::field::FieldElement;
use std::arch::x86_64::*;

/// Contenedor de 4 elementos de campo alineados para ráfagas AVX2.
#[derive(Debug, Clone, Copy)]
pub struct FieldElementVector4 {
    pub vectorized_words: [__m256i; 4],
}

impl FieldElementVector4 {
    #[target_feature(enable = "avx2")]
    pub unsafe fn from_independent_elements(
        element_0: &FieldElement,
        element_1: &FieldElement,
        element_2: &FieldElement,
        element_3: &FieldElement
    ) -> Self {
        let mut vectorized_words = [_mm256_setzero_si256(); 4];

        for word_index in 0..4 {
            vectorized_words[word_index] = _mm256_set_epi64x(
                element_3.internal_words[word_index] as i64,
                element_2.internal_words[word_index] as i64,
                element_1.internal_words[word_index] as i64,
                element_0.internal_words[word_index] as i64,
            );
        }

        Self { vectorized_words }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn add_modular_vectorized(&self, other: &Self) -> Self {
        let mut sum_results = [_mm256_setzero_si256(); 4];
        for i in 0..4 {
            sum_results[i] = _mm256_add_epi64(self.vectorized_words[i], other.vectorized_words[i]);
        }
        Self { vectorized_words: sum_results }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn subtract_modular_vectorized(&self, other: &Self) -> Self {
        let mut sub_results = [_mm256_setzero_si256(); 4];
        for i in 0..4 {
            sub_results[i] = _mm256_sub_epi64(self.vectorized_words[i], other.vectorized_words[i]);
        }
        Self { vectorized_words: sub_results }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn multiply_modular_vectorized(&self, other: &Self) -> Self {
        let mut mul_results = [_mm256_setzero_si256(); 4];
        for i in 0..4 {
            mul_results[i] = _mm256_mul_epu32(self.vectorized_words[i], other.vectorized_words[i]);
        }
        Self { vectorized_words: mul_results }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn multiply_by_small_int_vectorized(&self, factor: i64) -> Self {
        let mut results = [_mm256_setzero_si256(); 4];
        let factor_vec = _mm256_set1_epi64x(factor);
        for i in 0..4 {
            results[i] = _mm256_mul_epu32(self.vectorized_words[i], factor_vec);
        }
        Self { vectorized_words: results }
    }

    #[inline(always)]
    pub unsafe fn extract_lane(&self, lane_index: usize) -> FieldElement {
        let mut internal_words = [0u64; 4];
        let mut temp_storage = [0i64; 4];
        // Nota: storeu no requiere target_feature explícito si el caller maneja el contexto,
        // pero idealmente debería estar dentro de un bloque seguro.
        // Aquí asumimos compilación x86_64 base.
        for i in 0..4 {
            _mm256_storeu_si256(temp_storage.as_mut_ptr().cast::<__m256i>(), self.vectorized_words[i]);
            internal_words[i] = temp_storage[lane_index] as u64;
        }
        FieldElement { internal_words }
    }
}
