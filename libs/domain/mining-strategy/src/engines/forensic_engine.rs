// libs/domain/mining-strategy/src/engines/forensic_engine.rs
/**
 * =================================================================
 * APARATO: FORENSIC ARCHAEOLOGY ENGINE (V31.0 - DOCUMENTED)
 * CLASIFICACIÓN: DOMAIN STRATEGY (L2)
 * RESPONSABILIDAD: SIMULACIÓN DE PRNG DEBILITADOS (DEBIAN/ANDROID)
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_forensics::debian_rng::DebianIterator;
use crate::executor::FindingHandler;

/// Motor de arqueología digital que explora patrones de vulnerabilidad conocidos.
/// Actualmente soporta el bug de Debian OpenSSL (2008).
pub struct ForensicArchaeologyEngine;

impl ForensicArchaeologyEngine {
    /// Ejecuta una auditoría sobre patrones de entropía predecibles.
    ///
    /// # Argumentos
    /// * `vulnerability_target` - Identificador de la vulnerabilidad (ej: "Debian_OpenSSL_2008").
    /// * `target_filter` - Filtro de Bloom con el censo de direcciones.
    pub fn execute_forensic_scan<H: FindingHandler>(
        vulnerability_target: &str,
        target_filter: &ShardedFilter,
        stop_signal: &AtomicBool,
        effort_counter: Arc<AtomicU64>,
        collision_handler: &H,
    ) -> String {
        let mut final_forensic_checkpoint = String::new();

        match vulnerability_target {
            "Debian_OpenSSL_2008" => {
                let forensic_iterator = DebianIterator::new(1, 32767);
                for (metadata, private_key) in forensic_iterator {
                    if stop_signal.load(Ordering::Relaxed) { break; }

                    let public_key = SafePublicKey::from_private(&private_key);
                    let address = prospector_core_gen::address_legacy::pubkey_to_address(&public_key, false);

                    if target_filter.contains(&address) {
                        collision_handler.on_finding(address, private_key, metadata.clone());
                    }

                    final_forensic_checkpoint = metadata;
                    effort_counter.fetch_add(1, Ordering::Relaxed);
                }
            },
            _ => final_forensic_checkpoint = "UNSUPPORTED_FORENSIC_PATTERN".into(),
        }

        final_forensic_checkpoint
    }
}
