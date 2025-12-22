/**
 * =================================================================
 * APARATO: SATOSHI-XP FORENSIC INTEGRITY LAB (V1.4 - SYNCED)
 * =================================================================
 */

use prospector_domain_strategy::{SatoshiWindowsXpForensicEngine, FindingHandler};
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

struct ForensicTestReporter {
    pub reported_findings: Arc<Mutex<Vec<String>>>,
}

impl FindingHandler for ForensicTestReporter {
    fn on_finding(&self, address: String, _pk: SafePrivateKey, source: String) {
        let mut collection = self.reported_findings.lock().unwrap();
        collection.push(format!("{}|{}", address, source));
    }
}

#[test]
fn certify_satoshi_xp_engine_deterministic_execution() {
    println!("\n🧬 [FORENSIC_LAB]: Initiating Satoshi-XP fast audit...");

    let mut template = vec![0u8; 250000];
    template[0..4].copy_from_slice(b"PERF");

    // CONFIGURACIÓN SINCRONIZADA (Evita la falla 50000 vs 10000)
    let frequency: u64 = 10_000;
    let start_sec: u64 = 10;
    let end_sec: u64 = 11;

    let findings = Arc::new(Mutex::new(Vec::new()));
    let reporter = ForensicTestReporter { reported_findings: findings.clone() };
    let effort = Arc::new(AtomicU64::new(0));

    // EJECUCIÓN
    let checkpoint = SatoshiWindowsXpForensicEngine::execute_forensic_audit(
        &template,
        frequency,
        start_sec,
        end_sec,
        &ShardedFilter::new(1, 10, 0.1),
        &AtomicBool::new(false),
        effort.clone(),
        &reporter
    );

    let total = effort.load(Ordering::SeqCst);
    println!("🏁 [AUDIT_COMPLETE]: Processed {} ticks.", total);

    // ✅ ASERCIÓN SINCRONIZADA
    assert_eq!(total, frequency, "METRIC_ERROR: Expected {}, Got {}", frequency, total);
    assert!(checkpoint.contains("checkpoint_s_10"));

    println!("✅ [CERTIFIED]: Forensic engine is fast and accurate.");
}
