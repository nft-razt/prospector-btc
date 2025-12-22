// libs/core/math-engine/tests/arithmetic_stress.rs
/**
 * =================================================================
 * APARATO: ARITHMETIC STRESS CHAMBER (V1.0)
 * CLASIFICACIÓN: QA INFRASTRUCTURE (ESTRATO L1)
 * RESPONSABILIDAD: CERTIFICACIÓN DE INTEGRIDAD DE ADICIÓN U256
 *
 * OBJETIVO:
 * Validar que la implementación optimizada de `add_u64_to_u256_be`
 * (que utiliza ensamblador inline o bitsvis slicing) sea isomorfa
 * a la aritmética de precisión arbitraria de `num-bigint`.
 *
 * ESTRATEGIA:
 * Property-Based Testing (Fuzzing) utilizando `proptest`.
 * Se generan 10,000 casos aleatorios cubriendo bordes críticos
 * (Overflow de u64, Overflow de u256, Carry Propagation).
 * =================================================================
 */

use prospector_core_math::arithmetic::{add_u64_to_u256_be, U256_BYTE_SIZE};
use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive};
use proptest::prelude::*;

// Configuración de la suite de pruebas
proptest! {
    // Configuramos el runner para ejecutar 10,000 casos aleatorios.
    #![proptest_config(ProptestConfig::with_cases(10000))]

    #[test]
    fn certify_addition_integrity(
        // Generamos un array de 32 bytes aleatorio (Simula el acumulador actual)
        mut base_bytes in proptest::collection::vec(any::<u8>(), U256_BYTE_SIZE),
        // Generamos un u64 aleatorio (Simula el incremento del iterador)
        increment_value in any::<u64>()
    ) {
        // 1. TRANSFORMACIÓN A ARRAY ESTÁTICO (Normalización de Tipos)
        let mut buffer: [u8; 32] = [0u8; 32];
        buffer.copy_from_slice(&base_bytes);

        // 2. EJECUCIÓN EN EL MOTOR DE REFERENCIA (BigInt - Verdad Absoluta)
        // Convertimos los bytes a BigUint para realizar la suma teórica perfecta.
        let reference_bigint = BigUint::from_bytes_be(&buffer);
        let increment_bigint = BigUint::from_u64(increment_value).unwrap();
        let expected_result_bigint = reference_bigint + increment_bigint;

        // 3. EJECUCIÓN EN EL APARATO BAJO PRUEBA (Core Math - Optimizado)
        // Ejecutamos nuestra función crítica.
        let result = add_u64_to_u256_be(&mut buffer, increment_value);

        // 4. ANÁLISIS DE RESULTADOS Y DETECCIÓN DE REGRESIONES

        // CASO A: DESBORDAMIENTO (Overflow)
        // Si el resultado teórico excede 256 bits, nuestra función DEBE retornar error.
        let max_u256 = BigUint::from_bytes_be(&[0xFF; 32]);
        if expected_result_bigint > max_u256 {
            assert!(
                result.is_err(),
                "CRITICAL_FAILURE: Function did not detect 256-bit overflow. \
                 Input: {:?}, Increment: {}",
                base_bytes, increment_value
            );
        }
        // CASO B: OPERACIÓN EXITOSA
        else {
            assert!(
                result.is_ok(),
                "FALSE_NEGATIVE: Valid addition failed unexpectedly. \
                 Input: {:?}, Increment: {}",
                base_bytes, increment_value
            );

            // Verificación bit a bit del resultado
            let actual_result_bigint = BigUint::from_bytes_be(&buffer);
            assert_eq!(
                expected_result_bigint,
                actual_result_bigint,
                "CALCULATION_MISMATCH: The optimized engine produced an incorrect value.\n\
                 Expected: {}\n\
                 Actual:   {}",
                expected_result_bigint,
                actual_result_bigint
            );
        }
    }
}
