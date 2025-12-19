// libs/domain/mining-strategy/tests/bigint_integration.rs
// =================================================================
// APARATO: BIGINT INTEGRATION TEST
// ESTADO: FIXED (SHARDING COMPLIANT)
// =================================================================

use prospector_domain_models::{SearchStrategy, WorkOrder};
use prospector_domain_strategy::{ExecutorContext, FindingHandler, StrategyExecutor};
// ✅ CORRECCIÓN: Usar ShardedFilter
use prospector_core_math::private_key::SafePrivateKey;
use prospector_core_probabilistic::sharded::ShardedFilter;
use std::sync::{Arc, Mutex};

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
    let massive_start = "1180591620717411303424";
    let massive_end = "1180591620717411303430";

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

    // ✅ CORRECCIÓN: Instanciación correcta
    let filter = ShardedFilter::new(1, 100, 0.01);

    let reporter = TestReporter {
        findings: Arc::new(Mutex::new(Vec::new())),
    };

    let context = ExecutorContext::default();

    println!("🧪 Iniciando prueba de estrés BigInt...");
    StrategyExecutor::execute(&job, &filter, &context, &reporter);

    println!("✅ Prueba completada sin Panic/Overflow.");
}
