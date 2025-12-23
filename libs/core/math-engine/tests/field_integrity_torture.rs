/**
 * =================================================================
 * APARATO: FIELD INTEGRITY TORTURE CHAMBER (V17.0 - SOBERANO)
 * CLASIFICACIÓN: QA INFRASTRUCTURE (ESTRATO L1)
 * RESPONSABILIDAD: CERTIFICACIÓN DE ARITMÉTICA MODULAR SECP256K1
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa una auditoría matemática irrefutable. Utiliza 'num-bigint'
 * como el Oráculo de Verdad Absoluta para validar que el motor
 * optimizado de FieldElement no presente divergencias.
 * =================================================================
 */

use prospector_core_math::field::{FieldElement};
use num_bigint::BigUint;
use proptest::prelude::*;

/**
 * Helper: Transforma un FieldElement de Prospector en un BigUint de referencia.
 */
fn transform_to_bigint_oracle(element: &FieldElement) -> BigUint {
    let mut bytes_buffer = [0u8; 32];
    for limb_index in 0..4 {
        let start_offset = (3 - limb_index) * 8;
        bytes_buffer[start_offset..start_offset + 8]
            .copy_from_slice(&element.internal_words[limb_index].to_be_bytes());
    }
    BigUint::from_bytes_be(&bytes_buffer)
}

/**
 * Helper: Transforma un BigUint de referencia en un FieldElement de Prospector.
 */
fn transform_from_bigint_oracle(big_integer: &BigUint) -> FieldElement {
    let bytes_vector = big_integer.to_bytes_be();
    let mut final_bytes = [0u8; 32];
    let start_position = 32 - bytes_vector.len();
    if bytes_vector.len() <= 32 {
        final_bytes[start_position..].copy_from_slice(&bytes_vector);
    }

    let mut words_result = [0u64; 4];
    for limb_index in 0..4 {
        let start_offset = (3 - limb_index) * 8;
        words_result[limb_index] = u64::from_be_bytes(
            final_bytes[start_offset..start_offset + 8].try_into().unwrap()
        );
    }
    FieldElement { internal_words: words_result }
}

/// El primo p de secp256k1 como BigUint
fn get_field_prime_oracle() -> BigUint {
    BigUint::from_bytes_be(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F
    ])
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100000))]

    /**
     * CERTIFICACIÓN: Multiplicación Modular e Isomorfismo de Solinas.
     */
    #[test]
    fn certify_modular_multiplication_isomorphism(
        bytes_alpha in prop::collection::vec(0..=255u8, 32),
        bytes_beta in prop::collection::vec(0..=255u8, 32)
    ) {
        let mut buffer_alpha = [0u8; 32];
        let mut buffer_beta = [0u8; 32];
        buffer_alpha.copy_from_slice(&bytes_alpha);
        buffer_beta.copy_from_slice(&bytes_beta);

        let element_alpha = transform_from_bigint_oracle(&BigUint::from_bytes_be(&buffer_alpha));
        let element_beta = transform_from_bigint_oracle(&BigUint::from_bytes_be(&buffer_beta));

        // 1. Ejecución en Motor Táctico (L1)
        let tactical_result = element_alpha.multiply_modular(&element_beta);

        // 2. Ejecución en Oráculo (BigInt)
        let bigint_alpha = transform_to_bigint_oracle(&element_alpha);
        let bigint_beta = transform_to_bigint_oracle(&element_beta);
        let prime_oracle = get_field_prime_oracle();
        let expected_result_bigint = (bigint_alpha * bigint_beta) % prime_oracle;

        // 3. Verificación de Sincronía
        assert_eq!(
            transform_to_bigint_oracle(&tactical_result),
            expected_result_bigint,
            "DIVERGENCE_DETECTED: Solinas reduction failed isomorphism check."
        );
    }

    /**
     * CERTIFICACIÓN: Paridad del Inverso Modular (Montgomery vs Individual).
     */
    #[test]
    fn certify_montgomery_batch_inversion_parity(
        seed_bytes in prop::collection::vec(0..=255u8, 32)
    ) {
        let mut buffer = [0u8; 32];
        buffer.copy_from_slice(&seed_bytes);
        let base_element = transform_from_bigint_oracle(&BigUint::from_bytes_be(&buffer));

        if base_element.is_zero() { return Ok(()); }

        let elements_batch = vec![
            base_element,
            base_element.multiply_by_u64(3),
            base_element.square_modular(),
        ];

        // 1. Inversión Individual
        let individual_inverses: Vec<FieldElement> = elements_batch.iter()
            .map(|element| element.invert().expect("Inversion failed"))
            .collect();

        // 2. Inversión por Lote (Montgomery)
        let batch_inverses = FieldElement::batch_invert_sovereign(&elements_batch);

        // 3. Verificación de Paridad Absoluta
        for index in 0..elements_batch.len() {
            assert_eq!(
                individual_inverses[index],
                batch_inverses[index],
                "MONTGOMERY_FAULT: Batch inversion divergent at index {}", index
            );

            // Verificación Criptográfica: x * (1/x) ≡ 1 (mod p)
            let identity_check = elements_batch[index].multiply_modular(&batch_inverses[index]);
            assert_eq!(identity_check, FieldElement::from_u64(1));
        }
    }
}
