/**
 * =================================================================
 * APARATO: SATOSHI XP INTEGRITY TEST (V16.6 - MIT LEVEL)
 * CLASIFICACIÓN: LABORATORIO FORENSE (L2)
 * RESPONSABILIDAD: CERTIFICACIÓN DE THROUGHPUT Y DETERMINISMO
 * =================================================================
 */

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::private_key::SafePrivateKey;
use prospector_domain_strategy::{SatoshiWindowsXpForensicEngine, FindingHandler};

/// Reportero táctico para capturar hallazgos en memoria durante la prueba.
struct ForensicTestReporter {
    pub reported_findings: Arc<Mutex<Vec<String>>>,
}

impl FindingHandler for ForensicTestReporter {
    fn on_finding(&self, address: String, _private_key: SafePrivateKey, source: String) {
        let mut findings = self.reported_findings.lock().expect("Lock poisoned");
        findings.push(format!("ADDR: {} | SRC: {}", address, source));
    }
}

#[test]
fn certify_satoshi_xp_engine_deterministic_execution() {
    println!("\n🧬 [FORENSIC_LAB]: Initiating Satoshi-XP ultra-fast audit...");

    // 1. SETUP: Plantilla de ADN sintético (PERF_DATA_BLOCK)
    let mut template = vec![0u8; 250000];
    template[0..4].copy_from_slice(b"PERF");

    // 2. PARÁMETROS: Segmento de 1 segundo a 10,000 Hz
    let clock_frequency: u64 = 10_000;
    let start_uptime_sec: u64 = 500;
    let end_uptime_sec: u64 = 501;

    let findings_buffer = Arc::new(Mutex::new(Vec::new()));
    let spy_reporter = ForensicTestReporter { reported_findings: findings_buffer.clone() };
    let effort_accumulator = Arc::new(AtomicU64::new(0));
    let termination_signal = AtomicBool::new(false);

    // 3. EJECUCIÓN DEL MOTOR TÁCTICO
    let start_timestamp = std::time::Instant::now();

    let _checkpoint = SatoshiWindowsXpForensicEngine::execute_forensic_audit(
        &template,
        clock_frequency,
        start_uptime_sec,
        end_uptime_sec,
        &ShardedFilter::new(1, 10, 0.1),
        &termination_signal,
        effort_accumulator.clone(),
        &spy_reporter
    );

    let duration = start_timestamp.elapsed();
    let total_hashes = effort_accumulator.load(Ordering::SeqCst);

    println!("🏁 [AUDIT_COMPLETE]: Processed {} ticks in {:?}", total_hashes, duration);

    // 4. ASERCIONES DE ÉLITE
    // El conteo debe ser exacto a la frecuencia por el tiempo (1s * 10k)
    assert_eq!(total_hashes, clock_frequency, "TELEMETRY_DRIFT: Effort volume mismatch.");

    // Performance: 10,000 iteraciones deben ser casi instantáneas
    assert!(duration.as_secs() < 1, "PERFORMANCE_FAILURE: Engine below target hashrate.");
}
