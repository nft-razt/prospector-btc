/**
 * =================================================================
 * APARATO: ARITHMETIC STRESS CHAMBER (V1.2 - CLEANED)
 * CLASIFICACIÓN: QA INFRASTRUCTURE (ESTRATO L1)
 * RESPONSABILIDAD: CERTIFICACIÓN DE INTEGRIDAD DE ADICIÓN U256
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa una auditoría cruzada entre la aritmética optimizada del
 * core y la implementación de referencia BigUint.
 * =================================================================
 */

use prospector_core_math::arithmetic::{add_u64_to_u256_be, U256_BYTE_SIZE};
use num_bigint::BigUint;
use num_traits::FromPrimitive; // ✅ RESOLUCIÓN: ToPrimitive eliminado por desuso
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]

    #[test]
    fn certify_addition_integrity(
        // ✅ RESOLUCIÓN: 'mut' eliminado para cumplir con higiene de lints
        base_bytes in proptest::collection::vec(any::<u8>(), U256_BYTE_SIZE),
        increment_value in any::<u64>()
    ) {
        // 1. TRANSFORMACIÓN A ARRAY ESTÁTICO
        let mut computational_buffer: [u8; 32] = [0u8; 32];
        computational_buffer.copy_from_slice(&base_bytes);

        // 2. EJECUCIÓN EN MOTOR DE REFERENCIA (BIGINT)
        let reference_big_integer = BigUint::from_bytes_be(&computational_buffer);
        let increment_big_integer = BigUint::from_u64(increment_value).unwrap();
        let expected_result_big_integer = reference_big_integer + increment_big_integer;

        // 3. EJECUCIÓN EN EL APARATO BAJO PRUEBA
        let execution_result = add_u64_to_u256_be(&mut computational_buffer, increment_value);

        // 4. AUDITORÍA DE RESULTADOS
        let max_u256_boundary = BigUint::from_bytes_be(&[0xFF; 32]);

        if expected_result_big_integer > max_u256_boundary {
            assert!(
                execution_result.is_err(),
                "CRITICAL_FAILURE: Function did not detect 256-bit overflow."
            );
        } else {
            assert!(
                execution_result.is_ok(),
                "FALSE_NEGATIVE: Valid addition failed unexpectedly."
            );

            let actual_result_big_integer = BigUint::from_bytes_be(&computational_buffer);
            assert_eq!(
                expected_result_big_integer,
                actual_result_big_integer,
                "CALCULATION_MISMATCH: The optimized engine produced a divergent value."
            );
        }
    }
}
