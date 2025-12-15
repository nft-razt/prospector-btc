#[cfg(test)]
mod tests {
    use crate::{ExecutorContext, FindingHandler, StrategyExecutor};
    use prospector_core_gen::address_legacy::pubkey_to_address;
    use prospector_core_math::private_key::SafePrivateKey;
    use prospector_core_math::public_key::SafePublicKey;
    use prospector_core_probabilistic::filter_wrapper::RichListFilter;
    use prospector_domain_models::{SearchStrategy, WorkOrder};
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

        let mut filter = RichListFilter::new(100, 0.01);
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
        let reporter = MockReporter {
            found: Arc::new(Mutex::new(false)),
        };
        let context = ExecutorContext::default();

        StrategyExecutor::execute(&job, &filter, &context, &reporter);

        // 4. Verificar que se encontró la aguja en el pajar
        assert!(
            *reporter.found.lock().unwrap(),
            "El ejecutor falló al encontrar 'Satoshi1'"
        );
    }
}
