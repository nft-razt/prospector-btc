// libs/domain/mining-strategy/src/kangaroo.rs
// =================================================================
// APARATO: KANGAROO STRATEGY ADAPTER (V3.0 - ROBUST)
// RESPONSABILIDAD: CONFIGURACIÓN SEGURA DEL SOLVER DE LOGARITMO DISCRETO
// ALGORITMO: POLLARD'S LAMBDA (PARALLEL KANGAROO)
// ESTADO: TYPE-SAFE & OBSERVABLE
// =================================================================

use hex;
use tracing::{error, info, warn};

use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_math::kangaroo::{KangarooConfig, KangarooSolver};
use prospector_core_math::private_key::SafePrivateKey;
use prospector_core_math::public_key::SafePublicKey;

use crate::executor::FindingHandler;

/// Adaptador para la ejecución de la estrategia Canguro.
/// Encapsula la complejidad de configuración y parsing de datos hexadecimales.
pub struct KangarooRunner;

impl KangarooRunner {
    /// Ejecuta la búsqueda del Logaritmo Discreto en el rango especificado.
    ///
    /// # Argumentos
    /// * `target_pubkey_hex`: Clave pública a crackear (Compressed o Uncompressed Hex).
    /// * `start_scalar_hex`: Límite inferior del rango de búsqueda (Hex 256-bit).
    /// * `width`: Tamaño del intervalo de búsqueda ($W$).
    /// * `handler`: Callback para reportar el éxito.
    pub fn run<H: FindingHandler>(
        target_pubkey_hex: &str,
        start_scalar_hex: &str,
        width: u64,
        handler: &H,
    ) {
        // 1. Decodificación y Validación de la Clave Pública Objetivo
        let target_bytes = match hex::decode(target_pubkey_hex) {
            Ok(b) => b,
            Err(e) => {
                error!("🦘 KANGAROO: Error decodificando Target Hex: {}", e);
                return;
            }
        };

        // El motor matemático valida si el punto está en la curva automáticamente
        let target_pub = match SafePublicKey::from_bytes(&target_bytes) {
            Ok(p) => p,
            Err(e) => {
                error!("🦘 KANGAROO: Target PubKey inválida (fuera de curva): {}", e);
                return;
            }
        };

        // 2. Decodificación del Escalar de Inicio (Base del Rango)
        let scalar_vec = match hex::decode(start_scalar_hex) {
            Ok(b) => b,
            Err(e) => {
                error!("🦘 KANGAROO: Error decodificando Start Scalar: {}", e);
                return;
            }
        };

        if scalar_vec.len() != 32 {
            error!(
                "🦘 KANGAROO: Longitud de escalar incorrecta. Esperado 32 bytes, recibido {}",
                scalar_vec.len()
            );
            return;
        }

        let mut start_scalar = [0u8; 32];
        start_scalar.copy_from_slice(&scalar_vec);

        // 3. Configuración Adaptativa (Heurística de Memoria)
        // Ajustamos la máscara de "Puntos Distinguidos" (DP) según el ancho del rango.
        // - Rango Grande (>50M): Máscara estricta (0xFF) -> Menos puntos guardados -> Ahorro RAM.
        // - Rango Pequeño: Máscara laxa (0x1F) -> Más puntos -> Detección rápida.
        let dp_mask = if width > 50_000_000 { 0xFF } else { 0x1F };

        let config = KangarooConfig {
            start_scalar,
            width,
            dp_mask,
            max_traps: 2_000_000, // Límite de seguridad para evitar OOM (Out of Memory)
        };

        // info!("🦘 KANGAROO: Iniciando manada... [Width: {}, DP: 0x{:X}]", width, dp_mask);

        // 4. Ejecución del Solver Matemático (Core)
        match KangarooSolver::solve(&target_pub, &config) {
            Ok(Some(priv_bytes)) => {
                // ¡ÉXITO POTENCIAL! El solver retornó un escalar.
                Self::verify_and_report(priv_bytes, &target_bytes, handler);
            }
            Ok(None) => {
                // Rango agotado sin hallazgos. Esto es normal si la clave no estaba ahí.
            }
            Err(e) => {
                error!("🦘 KANGAROO: Error crítico en el motor matemático: {}", e);
            }
        }
    }

    /// Verificación Criptográfica Final.
    ///
    /// Asegura que $k_{encontrado} \cdot G == P_{objetivo}$ antes de alertar al sistema.
    /// Esto elimina cualquier posibilidad de falso positivo por colisión de hash en los puntos distinguidos.
    fn verify_and_report<H: FindingHandler>(
        priv_bytes: [u8; 32],
        expected_pub_bytes: &[u8],
        handler: &H,
    ) {
        if let Ok(pk) = SafePrivateKey::from_bytes(&priv_bytes) {
            let derived_pub = SafePublicKey::from_private(&pk);

            // Determinamos si el target era comprimido o no para comparar bytes crudos
            let is_compressed = expected_pub_bytes.len() == 33;
            let derived_bytes = derived_pub.to_bytes(is_compressed);

            if derived_bytes == expected_pub_bytes {
                // GENERACIÓN DE ARTEFACTOS
                let addr = pubkey_to_address(&derived_pub, is_compressed);

                info!("🚀 KANGAROO: ¡VICTORIA CONFIRMADA! Key recuperada para {}", addr);

                handler.on_finding(
                    addr,
                    pk,
                    "kangaroo_matrix_solve_v1".to_string()
                );
            } else {
                warn!("⚠️ KANGAROO: Falso positivo matemático detectado. La clave derivada no coincide con el objetivo.");
                // Esto teóricamente no debería pasar si la matemática está bien, pero en sistemas distribuidos nunca se confía ciegamente.
            }
        }
    }
}
