// libs/domain/mining-strategy/src/kangaroo.rs
// =================================================================
// APARATO: KANGAROO STRATEGY ADAPTER (V16.0)
// RESPONSABILIDAD: ORQUESTACIÓN DEL SOLVER POLLARD'S LAMBDA
// ESTADO: RESOLUCIÓN DE ERROR rustc(macro debug)
// =================================================================

use hex;
use tracing::{debug, error, info, warn}; // ✅ RESOLUCIÓN: debug macro incluida

// --- SINAPSIS INTERNA ---
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_math::prelude::*;
use crate::executor::FindingHandler;

/// Adaptador soberano para la ejecución de la estrategia Canguro.
///
/// Permite atacar claves públicas conocidas cuando se sospecha de un
/// rango de entropía acotado, operando con una eficiencia de O(sqrt(W)).
pub struct KangarooRunner;

impl KangarooRunner {
    /// Ejecuta el proceso de resolución con validación criptográfica final.
    ///
    /// # Argumentos
    /// * `target_hex` - Clave pública objetivo (SEC1 Hex).
    /// * `start_hex` - Escalar base del rango (32 bytes Hex).
    /// * `width` - Ancho de la ventana de búsqueda.
    pub fn run<H: FindingHandler>(
        target_hex: &str,
        start_hex: &str,
        width: u64,
        handler: &H,
    ) {
        // 1. Validación de Material Criptográfico
        let target_bytes = match hex::decode(target_hex.trim()) {
            Ok(bytes) => bytes,
            Err(_) => {
                error!("🦘 KANGAROO: Target Hex decoding failure.");
                return;
            }
        };

        let target_public_key = match SafePublicKey::from_bytes(&target_bytes) {
            Ok(key) => key,
            Err(error) => {
                error!("🦘 KANGAROO: Invalid target point: {}", error);
                return;
            }
        };

        let start_scalar_bytes = match hex::decode(start_hex.trim()) {
            Ok(bytes) if bytes.len() == 32 => {
                let mut array = [0u8; 32];
                array.copy_from_slice(&bytes);
                array
            }
            _ => {
                error!("🦘 KANGAROO: Start scalar must be exactly 32 bytes.");
                return;
            }
        };

        // 2. Configuración del Entorno de Salto
        let solver_config = KangarooConfig {
            start_scalar: start_scalar_bytes,
            width,
            // Máscara adaptativa para optimizar la probabilidad de colisión en RAM
            dp_mask: if width > 100_000_000 { 0xFF } else { 0x3F },
            max_traps: 2_000_000,
        };

        info!("🦘 KANGAROO: Herd launched for target [{}...]", &target_hex[0..10]);

        // 3. Ejecución del Solver Matemático (Parallel Pollard's Lambda)
        match KangarooSolver::solve(&target_public_key, &solver_config) {
            Ok(Some(found_private_bytes)) => {
                // Éxito: Verificamos y reportamos el hallazgo
                Self::verify_and_emit(found_private_bytes, &target_public_key, handler);
            }
            Ok(None) => {
                debug!("🦘 KANGAROO: Range [{}] exhausted without collisions.", width);
            }
            Err(error) => {
                error!("🦘 KANGAROO: Solver core malfunction: {}", error);
            }
        }
    }

    /// Realiza una derivación de clave completa para certificar la colisión.
    fn verify_and_emit<H: FindingHandler>(
        private_bytes: [u8; 32],
        target_point: &SafePublicKey,
        handler: &H,
    ) {
        if let Ok(safe_private_key) = SafePrivateKey::from_bytes(&private_bytes) {
            let derived_public_key = SafePublicKey::from_private(&safe_private_key);

            // Comparación de identidad en el grupo elíptico
            if derived_public_key.as_inner() == target_point.as_inner() {
                let address = pubkey_to_address(&derived_public_key, false);

                info!("🎯 KANGAROO: Victory! Private key recovered for address [{}]", address);

                handler.on_finding(
                    address,
                    safe_private_key,
                    "pollard_lambda_herd_collision_v16".to_string()
                );
            } else {
                warn!("⚠️ KANGAROO: False collision detected. Mathematics out of sync.");
            }
        }
    }
}
