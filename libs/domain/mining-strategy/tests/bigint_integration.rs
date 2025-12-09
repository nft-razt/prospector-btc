// libs/domain/mining-strategy/tests/bigint_integration.rs
// =================================================================
// APARATO: BIGINT INTEGRATION TEST
// RESPONSABILIDAD: VALIDACIÓN DE RESILIENCIA MATEMÁTICA (U256)
// ESTADO: NEW (VERIFICATION LAYER)
// =================================================================

use prospector_domain_strategy::{StrategyExecutor, ExecutorContext, FindingHandler};
use prospector_domain_models::{WorkOrder, SearchStrategy};
use prospector_core_probabilistic::filter_wrapper::RichListFilter;
use prospector_core_math::private_key::SafePrivateKey;
use std::sync::{Arc, Mutex};

// Mock del Handler para capturar resultados en el test
struct TestReporter {
    pub findings: Arc<Mutex<Vec<String>>>,
}

impl FindingHandler for TestReporter {
    fn on_finding(&self, address: String, _pk: SafePrivateKey, _source: String) {
        let mut data = self.findings.lock().unwrap();
        data.push(address);
    }
}

#[test]
fn test_executor_handles_massive_numbers_without_overflow() {
    // ESCENARIO: Un rango que excede u64 (2^64 = ~1.8e19).
    // Usamos 2^70 para garantizar que un sistema de 64 bits fallaría.
    // 2^70 = 1,180,591,620,717,411,303,424
    let massive_start = "1180591620717411303424";
    let massive_end   = "1180591620717411303430"; // +6 iteraciones

    // 1. Crear WorkOrder con Strings Gigantes
    let job = WorkOrder {
        id: "job-bigint-test".to_string(),
        target_duration_sec: 10,
        strategy: SearchStrategy::Combinatoric {
            prefix: "TEST".to_string(),
            suffix: "".to_string(),
            start_index: massive_start.to_string(),
            end_index: massive_end.to_string(),
        },
    };

    // 2. Filtro Dummy (Vacío, solo para que corra)
    let filter = RichListFilter::new(100, 0.01);

    // 3. Reportero
    let reporter = TestReporter {
        findings: Arc::new(Mutex::new(Vec::new()))
    };

    let context = ExecutorContext::default();

    // 4. EJECUCIÓN (Aquí es donde explotaría si usáramos u64)
    println!("🧪 Iniciando prueba de estrés BigInt...");
    StrategyExecutor::execute(&job, &filter, &context, &reporter);

    println!("✅ Prueba completada sin Panic/Overflow.");
}
