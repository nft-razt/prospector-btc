/**
 * =================================================================
 * APARATO: SEQUENTIAL INTEGRITY CERTIFICATION SUITE (V3.0)
 * CLASIFICACIÓN: ESTRATO DE VALIDACIÓN ESTRATÉGICA (L2)
 * RESPONSABILIDAD: CERTIFICACIÓN INTEGRAL DEL MOTOR JACOBIANO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa un entorno de pruebas endurecido para garantizar que el
 * 'ProjectiveSequentialEngine' opere con precisión quirúrgica.
 * Valida la recuperación de claves, la integridad de la telemetría
 * atómica y la respuesta a señales de terminación del sistema.
 * =================================================================
 */

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// --- SINAPSIS CON EL NÚCLEO MATEMÁTICO Y DE GENERACIÓN ---
use prospector_core_math::prelude::*;
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_probabilistic::sharded::ShardedFilter;

// --- SINAPSIS CON EL DOMINIO DE ESTRATEGIA ---
use prospector_domain_strategy::{ProjectiveSequentialEngine, FindingHandler};

/**
 * IMPLEMENTACIÓN: REPORTERO DE CAPTURA TÁCTICA
 * Responsabilidad Única: Almacenar colisiones detectadas en un buffer
 * seguro para su posterior auditoría forense en los tests.
 */
struct TacticalSpyReporter {
    pub captured_findings_collection: Arc<Mutex<Vec<String>>>,
}

impl FindingHandler for TacticalSpyReporter {
    /**
     * Invocado por el motor cuando se detecta una coincidencia en el filtro.
     */
    fn on_finding(
        &self,
        bitcoin_address: String,
        _recovered_private_key: SafePrivateKey,
        source_metadata: String
    ) {
        let mut collection_guard = self.captured_findings_collection
            .lock()
            .expect("CRITICAL_FAULT: Tactical collection lock poisoned.");

        collection_guard.push(format!(
            "ADDRESS: {} | SOURCE: {}",
            bitcoin_address,
            source_metadata
        ));
    }
}

/**
 * TEST ESTRATÉGICO 01: CERTIFICACIÓN DE RECUPERACIÓN Y TELEMETRÍA EXACTA
 * Objetivo: Validar que el motor encuentra una clave conocida y reporta
 * el volumen de hashes sin pérdida de precisión (Remnant Correction).
 */
#[test]
fn certify_full_recovery_and_telemetry_precision() {
    println!("\n🔍 [AUDIT_01]: Commencing Positive Recovery & Telemetry precision test...");

    // 1. GENERACIÓN DE LA AGUJA CRIPTOGRÁFICA
    // Escalar 0xABC = 2748.
    let target_scalar_hexadecimal = "0000000000000000000000000000000000000000000000000000000000000ABC";
    let target_bytes_array = hex::decode(target_scalar_hexadecimal).expect("HEX_DECODE_ERROR");

    let target_private_key_instance = SafePrivateKey::from_bytes(&target_bytes_array)
        .expect("MATH_FAULT: Invalid private key bytes.");
    let target_public_key_instance = SafePublicKey::from_private(&target_private_key_instance);
    let target_bitcoin_address = pubkey_to_address(&target_public_key_instance, false);

    // 2. PREPARACIÓN DEL CENSO TÁCTICO (EL PAJAR)
    let mut tactical_sharded_filter = ShardedFilter::new(1, 1000, 0.00001);
    tactical_sharded_filter.add(&target_bitcoin_address);

    // 3. CONFIGURACIÓN DE LA MISIÓN
    // Iniciamos 50 posiciones antes: 0xABC - 50 = 0xA8A (2698)
    let start_range_hexadecimal = "0000000000000000000000000000000000000000000000000000000000000A8A";
    let iteration_limit_count: u64 = 120; // Cruzará el target y terminará.

    let findings_buffer = Arc::new(Mutex::new(Vec::new()));
    let spy_reporter = TacticalSpyReporter {
        captured_findings_collection: findings_buffer.clone(),
    };
    let global_termination_signal = Arc::new(AtomicBool::new(false));
    let computational_effort_telemetry = Arc::new(AtomicU64::new(0));

    // 4. EJECUCIÓN DEL MOTOR
    let final_checkpoint_hex = ProjectiveSequentialEngine::execute_optimized_audit(
        start_range_hexadecimal,
        iteration_limit_count,
        &tactical_sharded_filter,
        &global_termination_signal,
        computational_effort_telemetry.clone(),
        &spy_reporter
    );

    // 5. AUDITORÍA DE RESULTADOS
    let total_scanned_hashes = computational_effort_telemetry.load(Ordering::SeqCst);
    let findings = findings_buffer.lock().unwrap();

    println!("📊 [METRICS]: Scanned: {} hashes | Checkpoint: {}", total_scanned_hashes, final_checkpoint_hex);

    // ASERCIÓN DE TELEMETRÍA: El conteo debe ser EXACTO al límite configurado.
    assert_eq!(total_scanned_hashes, iteration_limit_count, "TELEMETRY_DRIFT: Scanned count is not exact.");

    // ASERCIÓN DE HALLAZGO: El motor DEBE haber capturado la colisión.
    assert!(!findings.is_empty(), "RECOVERY_FAULT: Target key was bypassed.");
    assert!(findings[0].contains(&target_bitcoin_address), "DATA_MISMATCH: Collision address integrity failed.");

    println!("✅ [AUDIT_01_SUCCESS]: Recovery and Telemetry certified.");
}

/**
 * TEST ESTRATÉGICO 02: CERTIFICACIÓN DE RESPUESTA A SEÑAL (KILL-SWITCH)
 * Objetivo: Asegurar que el motor detiene el cálculo Jacobiano inmediatamente
 * cuando recibe la señal de terminación, evitando el desperdicio de ciclos.
 */
#[test]
fn certify_immediate_termination_protocol() {
    println!("\n🔍 [AUDIT_02]: Commencing Termination Signal responsiveness test...");

    let empty_filter = ShardedFilter::new(1, 100, 0.01);
    let effort_telemetry = Arc::new(AtomicU64::new(0));
    let termination_signal = Arc::new(AtomicBool::new(true)); // Señal activa desde el inicio.

    let spy_reporter = TacticalSpyReporter {
        captured_findings_collection: Arc::new(Mutex::new(Vec::new())),
    };

    // Intentamos procesar un rango de 1 millón, pero con señal de stop.
    ProjectiveSequentialEngine::execute_optimized_audit(
        "0000000000000000000000000000000000000000000000000000000000000001",
        1_000_000,
        &empty_filter,
        &termination_signal,
        effort_telemetry.clone(),
        &spy_reporter
    );

    let total_hashes_after_stop = effort_telemetry.load(Ordering::SeqCst);

    // El motor debe detectar el stop antes de la primera iteración o inmediatamente después.
    // Toleramos 0 o el primer incremento si el check está al final del bucle.
    assert!(total_hashes_after_stop <= 1, "SIGNAL_RECOIL_FAULT: Engine failed to stop on signal.");

    println!("✅ [AUDIT_02_SUCCESS]: Kill-Switch protocol certified.");
}

/**
 * TEST ESTRATÉGICO 03: CERTIFICACIÓN DE INTEGRIDAD DE FRONTERA
 * Objetivo: Validar que el checkpoint devuelto es el siguiente escalar matemático.
 * 0xA8A (2698) + 10 iteraciones = 2708 (0xA94).
 */
#[test]
fn certify_mathematical_boundary_consistency() {
    println!("\n🔍 [AUDIT_03]: Commencing Mathematical Boundary audit...");

    let effort_telemetry = Arc::new(AtomicU64::new(0));
    let start_hex = "0000000000000000000000000000000000000000000000000000000000000A8A";
    let steps: u64 = 10;

    let expected_checkpoint = "0000000000000000000000000000000000000000000000000000000000000a94";

    let final_hex = ProjectiveSequentialEngine::execute_optimized_audit(
        start_hex,
        steps,
        &ShardedFilter::new(1, 10, 0.1),
        &Arc::new(AtomicBool::new(false)),
        effort_telemetry,
        &TacticalSpyReporter { captured_findings_collection: Arc::new(Mutex::new(Vec::new())) }
    );

    assert_eq!(final_hex.to_lowercase(), expected_checkpoint, "BOUNDARY_DRIFT: Checkpoint math is incorrect.");

    println!("✅ [AUDIT_03_SUCCESS]: Boundary consistency certified.");
}
