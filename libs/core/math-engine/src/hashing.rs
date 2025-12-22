/**
 * =================================================================
 * APARATO: SIMD HASHING ENGINE (V12.0 - BITCOIN STANDARD)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: HASHING CRIPTOGRÁFICO DE ALTO RENDIMIENTO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa las funciones de resumen (Digest) requeridas por el
 * protocolo Bitcoin. Utiliza implementaciones Rust puras optimizadas
 * para evitar dependencias de C (OpenSSL) y facilitar la compilación
 * cruzada (MUSL).
 * =================================================================
 */

use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

/// Realiza un HASH160 estándar de Bitcoin.
/// Algoritmo: $RIPEMD160(SHA256(data))$
///
/// Utilizado para generar el Identificador de Llave Pública (KeyID)
/// que forma parte de las direcciones P2PKH.
#[inline(always)]
pub fn hash160(data: &[u8]) -> [u8; 20] {
    // Paso 1: SHA-256
    let mut sha_hasher = Sha256::new();
    sha_hasher.update(data);
    let sha_result = sha_hasher.finalize();

    // Paso 2: RIPEMD-160
    let mut ripe_hasher = Ripemd160::new();
    ripe_hasher.update(sha_result);

    // Copia segura al buffer de salida
    let mut output = [0u8; 20];
    output.copy_from_slice(&ripe_hasher.finalize());
    output
}

/// Genera hashes SHA-256 en lote.
/// Diseñado para la vectorización automática del compilador en bucles.
pub fn batch_sha256(inputs: &[String]) -> Vec<[u8; 32]> {
    inputs
        .iter()
        .map(|input| {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            let mut res = [0u8; 32];
            res.copy_from_slice(&hasher.finalize());
            res
        })
        .collect()
}
