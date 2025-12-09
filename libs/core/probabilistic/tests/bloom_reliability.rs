// libs/core/probabilistic/tests/bloom_reliability.rs
// =================================================================
// APARATO: BLOOM RELIABILITY TEST
// RESPONSABILIDAD: VERIFICACIÓN EMPÍRICA DE TASA DE FALSOS POSITIVOS
// ESTADO: CLEAN (UNUSED VARIABLES FIXED)
// =================================================================

use prospector_core_probabilistic::filter_wrapper::RichListFilter;
use rand::Rng;

#[test]
fn test_false_positive_rate() {
    let n = 100_000; // Insertamos 100k elementos
    let fp_rate = 0.01; // Esperamos 1% de error máximo teórico

    let mut filter = RichListFilter::new(n, fp_rate);

    // 1. Insertar datos conocidos
    for i in 0..n {
        filter.add(&format!("address-{}", i));
    }

    // 2. Probar datos aleatorios que NO deberían existir
    let trials = 10_000;
    let mut collisions = 0;

    // CORRECCIÓN: Usamos '_' porque no necesitamos el índice en este bucle
    for _ in 0..trials {
        // Generamos strings aleatorios (u64) que estadísticamente no colisionan con "address-{i}"
        let random_str = format!("random-{}", rand::thread_rng().gen::<u64>());

        // Si el filtro dice "true", es un Falso Positivo (porque sabemos que no lo insertamos)
        if filter.contains(&random_str) {
            collisions += 1;
        }
    }

    let actual_rate = collisions as f64 / trials as f64;
    println!("Tasa de Falsos Positivos: Esperada {}, Real {}", fp_rate, actual_rate);

    // Tolerancia: Permitimos una desviación de hasta 1.5x sobre la tasa teórica
    // debido a la varianza estadística en muestras pequeñas.
    assert!(actual_rate < fp_rate * 1.5, "La tasa de FP es demasiado alta ({})", actual_rate);
}
