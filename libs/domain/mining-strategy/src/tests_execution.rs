// libs/domain/mining-strategy/src/tests_execution.rs
// =================================================================
// APARATO: EXECUTION STRATEGY TESTS (INTEGRATION)
// OBJETIVO: Verificar que el Executor orquesta la búsqueda correctamente.
// ESTADO: SHARDING COMPLIANT & THREAD-SAFE
// =================================================================

#[cfg(test)]
mod tests {
    use crate::{StrategyExecutor, ExecutorContext, FindingHandler};
    use prospector_domain_models::{WorkOrder, SearchStrategy};

    // Infraestructura de Datos
    use prospector_core_probabilistic::sharded::ShardedFilter;

    // Núcleo Matemático
    use prospector_core_math::private_key::SafePrivateKey;
    use prospector_core_math::public_key::SafePublicKey;
    use prospector_core_gen::address_legacy::pubkey_to_address;

    use std::sync::{Arc, Mutex};

    /// Mock del Reporter para capturar hallazgos en memoria durante el test.
    /// Simula el comportamiento del WorkerClient pero sin I/O de red.
    struct MockReporter {
        pub findings: Arc<Mutex<Vec<String>>>,
    }

    impl FindingHandler for MockReporter {
        fn on_finding(&self, address: String, _pk: SafePrivateKey, _source: String) {
            let mut data = self.findings.lock().unwrap();
            data.push(address);
        }
    }

    #[test]
    fn test_combinatoric_strategy_finds_target_in_sharded_filter() {
        // 1. SETUP: Crear una "Aguja" (Target) conocida
        // Frase: "Satoshi1" -> SHA256 -> PrivKey
        let known_phrase = "Satoshi1";
        let pk = crate::brainwallet::phrase_to_private_key(known_phrase);
        let pubk = SafePublicKey::from_private(&pk);
        // Usamos formato no comprimido (false) por defecto en estrategias legacy
        let target_address = pubkey_to_address(&pubk, false);

        println!("🎯 Target Address: {}", target_address);

        // 2. SETUP: Construir el Mapa (ShardedFilter) en memoria
        // Usamos 2 shards para probar que el enrutamiento interno funciona.
        let mut filter = ShardedFilter::new(2, 100, 0.01);
        filter.add(&target_address);

        // 3. SETUP: Definir el Trabajo (Job)
        // El trabajo buscará en el rango "Satoshi0" a "Satoshi5".
        // "Satoshi1" está dentro de este rango.
        let job = WorkOrder {
            id: "test-job-integration-01".to_string(),
            target_duration_sec: 10,
            strategy: SearchStrategy::Combinatoric {
                prefix: "Satoshi".to_string(),
                suffix: "".to_string(),
                start_index: "0".to_string(),
                end_index: "5".to_string(),
            },
        };

        // 4. EJECUCIÓN
        let findings_buffer = Arc::new(Mutex::new(Vec::new()));
        let reporter = MockReporter { findings: findings_buffer.clone() };
        let context = ExecutorContext::default();

        println!("🚀 Ejecutando StrategyExecutor...");
        StrategyExecutor::execute(&job, &filter, &context, &reporter);

        // 5. ASERCIÓN (Validación)
        let results = findings_buffer.lock().unwrap();

        assert!(
            !results.is_empty(),
            "El ejecutor terminó sin reportar hallazgos."
        );

        assert_eq!(
            results[0], target_address,
            "La dirección encontrada no coincide con la esperada."
        );

        println!("✅ Test Exitoso: Se encontró {} usando fuerza bruta combinatoria.", results[0]);
    }
}
