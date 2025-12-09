
use prospector_core_probabilistic::filter_wrapper::RichListFilter;
use rand::Rng;

#[test]
fn test_false_positive_rate() {
    let n = 100_000; // Insertamos 100k elementos
    let fp_rate = 0.01; // Esperamos 1% de error

    let mut filter = RichListFilter::new(n, fp_rate);

    // 1. Insertar datos
    for i in 0..n {
        filter.add(&format!("address-{}", i));
    }

    // 2. Probar datos que NO existen
    let trials = 10_000;
    let mut collisions = 0;

    for i in 0..trials {
        // Generamos strings aleatorios que seguro no están
        let random_str = format!("random-{}", rand::thread_rng().gen::<u64>());
        if filter.contains(&random_str) {
            collisions += 1;
        }
    }

    let actual_rate = collisions as f64 / trials as f64;
    println!("Tasa de Falsos Positivos: Esperada {}, Real {}", fp_rate, actual_rate);

    // Tolerancia pequeña (probabilidad es estadística)
    assert!(actual_rate < fp_rate * 1.5, "La tasa de FP es demasiado alta");
}
