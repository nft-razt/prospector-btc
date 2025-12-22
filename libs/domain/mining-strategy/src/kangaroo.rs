// libs/domain/mining-strategy/src/kangaroo.rs
/**
 * =================================================================
 * APARATO: KANGAROO STRATEGY ENGINE (V20.1 - DOCUMENTED)
 * CLASIFICACIÓN: DOMAIN STRATEGY (L2)
 * RESPONSABILIDAD: RESOLUCIÓN DE ECDLP MEDIANTE POLLARD'S LAMBDA
 * =================================================================
 */

use tracing::{info, error, instrument};
use prospector_core_math::prelude::*;
use prospector_core_math::arithmetic::U256_BYTE_SIZE;
use prospector_core_math::kangaroo::{KangarooSolver, KangarooConfig};
use crate::executor::FindingHandler;

/// Orquestador del algoritmo Pollard's Kangaroo.
pub struct KangarooRunner;

impl KangarooRunner {
    /// Ejecuta una resolución de rango corto para una clave pública objetivo.
    ///
    /// # Argumentos
    /// * `target_pubkey_hex` - Clave pública objetivo en formato hexadecimal.
    /// * `start_scalar_hex` - Inicio del rango de búsqueda (Private Key) en hex.
    /// * `width_val` - Anchura del intervalo de búsqueda (N).
    /// * `handler` - Callback para reportar el éxito.
    #[instrument(skip(handler))]
    pub fn run<H: FindingHandler>(
        target_pubkey_hex: &str,
        start_scalar_hex: &str,
        width_val: u64,
        handler: &H,
    ) {
        info!("🦘 [KANGAROO_INIT]: Starting resolution for {}", &target_pubkey_hex[0..10]);

        let target_bytes = match hex::decode(target_pubkey_hex) {
            Ok(b) => b,
            Err(_) => {
                error!("❌ INVALID_TARGET: Hex decoding failed.");
                return;
            }
        };

        let target_point = match SafePublicKey::from_bytes(&target_bytes) {
            Ok(p) => p,
            Err(e) => {
                error!("❌ MATH_FAULT: Point parsing error: {}", e);
                return;
            }
        };

        let mut start_scalar_bytes = [0u8; U256_BYTE_SIZE];
        if let Ok(decoded) = hex::decode(start_scalar_hex) {
            if decoded.len() == U256_BYTE_SIZE {
                start_scalar_bytes.copy_from_slice(&decoded);
            }
        }

        let config = KangarooConfig {
            start_scalar: start_scalar_bytes,
            width: width_val,
            dp_mask: 0x0F,
            max_traps: 10000,
        };

        match KangarooSolver::solve(&target_point, &config) {
            Ok(Some(private_key_bytes)) => {
                info!("🎯 [COLLISION_L1]: Discrete logarithm solved.");

                if let Ok(sk) = SafePrivateKey::from_bytes(&private_key_bytes) {
                    let derived_pub = SafePublicKey::from_private(&sk);
                    let address = prospector_core_gen::address_legacy::pubkey_to_address(&derived_pub, false);

                    handler.on_finding(address, sk, "kangaroo_lambda_v19".into());
                }
            }
            Ok(None) => {
                info!("🏁 [SCAN_CLEAN]: No collision in range.");
            }
            Err(e) => {
                error!("💀 [SOLVER_FAULT]: {}", e);
            }
        }
    }
}
