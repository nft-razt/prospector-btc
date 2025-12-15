// libs/domain/mining-strategy/src/kangaroo.rs
// =================================================================
// APARATO: KANGAROO STRATEGY ADAPTER
// RESPONSABILIDAD: PUENTE TÁCTICO CON MANEJO DE ERRORES ROBUSTO
// =================================================================

use hex;
use prospector_core_math::kangaroo::{KangarooConfig, KangarooSolver};
use prospector_core_math::private_key::SafePrivateKey;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_gen::address_legacy::pubkey_to_address;
use crate::executor::FindingHandler;

pub struct KangarooRunner;

impl KangarooRunner {
    pub fn run<H: FindingHandler>(
        target_pubkey_hex: &str,
        start_scalar_hex: &str,
        width: u64,
        handler: &H,
    ) {
        // 1. DECODIFICACIÓN SEGURA
        let target_bytes = match hex::decode(target_pubkey_hex) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("❌ KANGAROO: Error decodificando Target Hex: {}", e);
                return;
            }
        };

        // Usamos from_bytes para instanciar la clave pública (requiere core actualizado)
        let target_pub = match SafePublicKey::from_bytes(&target_bytes) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("❌ KANGAROO: Clave Pública inválida: {}", e);
                return;
            }
        };

        let mut start_scalar = [0u8; 32];
        match hex::decode(start_scalar_hex) {
            Ok(bytes) if bytes.len() == 32 => {
                start_scalar.copy_from_slice(&bytes);
            },
            Ok(_) => {
                eprintln!("❌ KANGAROO: Start Scalar longitud incorrecta (debe ser 32 bytes)");
                return;
            },
            Err(e) => {
                eprintln!("❌ KANGAROO: Error decodificando Start Scalar: {}", e);
                return;
            }
        }

        // 2. CONFIGURACIÓN DINÁMICA
        // Ajustamos la máscara según el ancho para optimizar memoria
        let dp_mask = if width > 10_000_000 {
            0xFF // Máscara agresiva (1/256) para rangos muy grandes
        } else {
            0x1F // Máscara suave (1/32) para rangos cortos
        };

        let config = KangarooConfig {
            start_scalar,
            width,
            dp_mask,
        };

        // 3. EJECUCIÓN FORENSE
        // println!("🦘 KANGAROO: Iniciando caza en rango de ancho {}", width);

        match KangarooSolver::solve(&target_pub, &config) {
            Ok(Some(private_bytes)) => {
                if let Ok(pk) = SafePrivateKey::from_bytes(&private_bytes) {
                    let recovered_pub = SafePublicKey::from_private(&pk);
                    let address = pubkey_to_address(&recovered_pub, true); // Asumimos comprimida

                    handler.on_finding(
                        address,
                        pk,
                        format!("kangaroo:{}", target_pubkey_hex)
                    );
                    println!("🎉 KANGAROO: ¡Objetivo abatido!");
                }
            }
            Ok(None) => {
                // Rango limpio
            }
            Err(e) => {
                eprintln!("💀 KANGAROO CRASH: Error matemático interno: {}", e);
            }
        }
    }
}
