/**
 * =================================================================
 * APARATO: ARITHMETIC TORTURE CHAMBER (V1.1 - SOBERANO)
 * CLASIFICACIÓN: QA ESTRATÉGICO (ESTRATO L1)
 * RESPONSABILIDAD: CERTIFICACIÓN DE INTEGRIDAD U256 MEDIANTE FUERZA BRUTA
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa una auditoría matemática irrefutable. Utiliza Property-Based
 * Testing para colisionar la implementación optimizada en ensamblador (ASM)
 * contra la implementación teórica de BigUint (Fuente de Verdad).
 * =================================================================
 */

use prospector_core_math::arithmetic::{add_u64_to_u256_be, compare_u256_be, U256_BYTE_SIZE};
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use proptest::prelude::*;

// Configuración de la Suite de Tortura
proptest! {
    // 50,000 casos aleatorios por ejecución para detectar Cisnes Negros
    #![proptest_config(ProptestConfig::with_cases(50000))]

    /**
     * Certifica que la adición de un escalar u64 a un buffer de 256 bits
     * sea idéntica a la suma teórica perfecta y detecte desbordamientos.
     */
    #[test]
    fn certify_u256_addition_integrity_under_fuzzing(
        base_value_bytes_collection in prop::collection::vec(0..=255u8, U256_BYTE_SIZE),
        increment_value_u64 in any::<u64>()
    ) {
        let mut computational_buffer = [0u8; 32];
        computational_buffer.copy_from_slice(&base_value_bytes_collection);

        // 1. CÁLCULO DE REFERENCIA (Motor de Verdad Absoluta)
        let reference_big_integer = BigUint::from_bytes_be(&computational_buffer);
        let expected_result_big_integer = reference_big_integer + BigUint::from_u64(increment_value_u64).unwrap();
        let u256_boundary_max = BigUint::from_bytes_be(&[0xFF; 32]);

        // 2. EJECUCIÓN EN EL MOTOR TÁCTICO (Core Math L1)
        let execution_result = add_u64_to_u256_be(&mut computational_buffer, increment_value_u64);

        // 3. AUDITORÍA DE RESULTADOS Y DETECCIÓN DE REGRESIONES
        if expected_result_big_integer > u256_boundary_max {
            // El motor DEBE detectar el desbordamiento de 256 bits y retornar error
            assert!(
                execution_result.is_err(),
                "CRITICAL_FAILURE: Overflow not detected. Base: {:?}, Add: {}",
                base_value_bytes_collection,
                increment_value_u64
            );
        } else {
            // La operación debe ser exitosa y el resultado bit-a-bit idéntico
            assert!(
                execution_result.is_ok(),
                "FALSE_NEGATIVE: Addition failed within valid boundary."
            );

            let actual_result_big_integer = BigUint::from_bytes_be(&computational_buffer);
            assert_eq!(
                actual_result_big_integer,
                expected_result_big_integer,
                "MATHEMATICAL_DIVERGENCE: ASM calculation differs from theoretical value."
            );
        }
    }

    /**
     * Certifica que la comparación lexicográfica de 256 bits sea coherente
     * con el ordenamiento numérico de precisión arbitraria.
     */
    #[test]
    fn certify_lexicographical_comparison_consistency(
        bytes_collection_alpha in prop::collection::vec(0..=255u8, 32),
        bytes_collection_beta in prop::collection::vec(0..=255u8, 32)
    ) {
        let mut buffer_alpha = [0u8; 32];
        let mut buffer_beta = [0u8; 32];
        buffer_alpha.copy_from_slice(&bytes_collection_alpha);
        buffer_beta.copy_from_slice(&bytes_collection_beta);

        let big_integer_alpha = BigUint::from_bytes_be(&buffer_alpha);
        let big_integer_beta = BigUint::from_bytes_be(&buffer_beta);

        let actual_ordering_result = compare_u256_be(&buffer_alpha, &buffer_beta);
        let expected_ordering_result = big_integer_alpha.cmp(&big_integer_beta);

        assert_eq!(
            actual_ordering_result,
            expected_ordering_result,
            "COMPARISON_FAULT: U256 logic failed to determine correct hierarchy."
        );
    }
}
