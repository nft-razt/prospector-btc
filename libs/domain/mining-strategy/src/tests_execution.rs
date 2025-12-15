// libs/domain/mining-strategy/src/tests_execution.rs
// =================================================================
// APARATO: EXECUTION TESTS (SHARDING COMPLIANT)
// ESTADO: FIXED (MATCHES EXECUTOR API)
// =================================================================

#[cfg(test)]
mod tests {
    use crate::{StrategyExecutor, ExecutorContext, FindingHandler};
    use prospector_domain_models::{WorkOrder, SearchStrategy};

    // ✅ CORRECCIÓN: Importamos ShardedFilter en lugar de RichListFilter
    use prospector_core_probabilistic::sharded::ShardedFilter;

    use prospector_core_math::private_key::SafePrivateKey;
    use prospector_core_math::public_key::SafePublicKey;
    use prospector_core_gen::address_legacy::pubkey_to_address;
    use std::sync::{Arc, Mutex};

    // Mock del Reporter para capturar hallazgos en memoria
    struct MockReporter {
        found: Arc<Mutex<bool>>,
    }
    impl FindingHandler for MockReporter {
        fn on_finding(&self, _addr: String, _pk: SafePrivateKey, _src: String) {
            let mut lock = self.found.lock().unwrap();
            *lock = true;
        }
    }

    #[test]
    fn test_combinatoric_strategy_finds_key() {
        // 1. Preparar un Filtro con una dirección conocida
        // "Satoshi1" -> SHA256 -> PrivKey -> PubKey -> Address
        let known_phrase = "Satoshi1";
        let pk = crate::brainwallet::phrase_to_private_key(known_phrase);
        let pubk = SafePublicKey::from_private(&pk);
        let target_address = pubkey_to_address(&pubk, false);

        // ✅ CORRECCIÓN: Instanciamos ShardedFilter
        // Usamos 1 shard para simular un entorno simple, equivalente a lo que hacía el test antes.
        let mut filter = ShardedFilter::new(1, 100, 0.01);
        filter.add(&target_address);

        // 2. Definir Trabajo: Buscar "Satoshi" + "0".."5" (El "1" está incluido)
        let job = WorkOrder {
            id: "test-job-unit".to_string(),
            target_duration_sec: 10,
            strategy: SearchStrategy::Combinatoric {
                prefix: "Satoshi".to_string(),
                suffix: "".to_string(),
                start_index: "0".to_string(),
                end_index: "5".to_string(),
            },
        };

        // 3. Ejecutar
        let reporter = MockReporter { found: Arc::new(Mutex::new(false)) };
        let context = ExecutorContext::default();

        // Ahora los tipos coinciden: &ShardedFilter -> &ShardedFilter
        StrategyExecutor::execute(&job, &filter, &context, &reporter);

        // 4. Verificar que se encontró la aguja en el pajar
        assert!(*reporter.found.lock().unwrap(), "El ejecutor falló al encontrar 'Satoshi1'");
    }
}
