/**
 * =================================================================
 * APARATO: KANGAROO STRATEGY ENGINE (V18.0 - PROJECTIVE SYNC)
 * CLASIFICACIÓN: DOMAIN STRATEGY (L2)
 * RESPONSABILIDAD: RESOLUCIÓN DE ECDLP MEDIANTE POLLARD'S LAMBDA
 *
 * ESTRATEGIA DE ÉLITE:
 * - Cross-Library Linkage: Vinculación correcta con prospector_core_math.
 * - Point Addition Optimization: Uso de SafePublicKey::add_scalar para saltos.
 * - Zero-Regression: Mantiene compatibilidad con el contrato FindingHandler.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{info, error, instrument};

// --- SINAPSIS DE INFRAESTRUCTURA (CORE MATH) ---
// ✅ RESOLUCIÓN: Corregidos los imports de crate:: a prospector_core_math::
use prospector_core_math::prelude::*;
use prospector_core_math::arithmetic::{U256_BYTE_SIZE, add_u256_be, sub_u256_be, u128_to_u256_be};

// --- SINAPSIS DE DOMINIO ---
use crate::executor::FindingHandler;

/// Orquestador del algoritmo Pollard's Kangaroo.
pub struct KangarooRunner;

impl KangarooRunner {
    /**
     * Ejecuta una resolución de rango corto para una clave pública objetivo.
     *
     * @param target_pubkey_hex Hex de la clave pública (P) a resolver.
     * @param start_scalar_hex Hex del inicio del rango de búsqueda.
     * @param width_val Longitud del intervalo de búsqueda.
     * @param handler Delegado para el reporte de la clave privada encontrada.
     */
    #[instrument(skip(handler))]
    pub fn run<H: FindingHandler>(
        target_pubkey_hex: &str,
        start_scalar_hex: &str,
        width_val: u64,
        handler: &H,
    ) {
        info!("🦘 [KANGAROO_INIT]: Starting short-range resolution for target {}", &target_pubkey_hex[0..10]);

        // 1. HIDRATACIÓN DEL TARGET (Punto en la curva)
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
                error!("❌ MATH_FAULT: Could not parse target point: {}", e);
                return;
            }
        };

        // 2. CONFIGURACIÓN DEL SOLVER (L1)
        let mut start_scalar_bytes = [0u8; U256_BYTE_SIZE];
        if let Ok(decoded) = hex::decode(start_scalar_hex) {
            if decoded.len() == U256_BYTE_SIZE {
                start_scalar_bytes.copy_from_slice(&decoded);
            }
        }

        let config = KangarooConfig {
            start_scalar: start_scalar_bytes,
            width: width_val,
            dp_mask: 0x0F, // Propiedad de punto distinguido ajustable
            max_traps: 10000,
        };

        // 3. IGNICIÓN DEL MOTOR MATEMÁTICO (ECDLP Solver)
        match KangarooSolver::solve(&target_point, &config) {
            Ok(Some(private_key_bytes)) => {
                info!("🎯 [COLLISION_L1]: Discrete logarithm solved successfully.");

                if let Ok(sk) = SafePrivateKey::from_bytes(&private_key_bytes) {
                    let derived_pub = SafePublicKey::from_private(&sk);
                    let address = prospector_core_gen::address_legacy::pubkey_to_address(&derived_pub, false);

                    handler.on_finding(address, sk, "kangaroo_lambda_v18".into());
                }
            }
            Ok(None) => {
                info!("🏁 [SCAN_CLEAN]: No collision detected in the specified width.");
            }
            Err(e) => {
                error!("💀 [CRITICAL_SOLVER_FAULT]: {}", e);
            }
        }
    }
}

/**
 * Nota de Arquitectura:
 * Este componente actúa como el "Comandante de Campo" para el KangarooSolver de L1.
 * Traduce los requerimientos de la orden de trabajo (Strings/Hex) a primitivas
 * matemáticas puras y orquesta la respuesta al flujo de hallazgos del sistema.
 */
