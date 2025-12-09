// libs/core/generators/src/wif.rs
use prospector_core_math::private_key::SafePrivateKey;
use bs58;

const MAINNET_PRIVATE_KEY_PREFIX: u8 = 0x80;

/// Convierte una clave privada en formato WIF (Wallet Import Format).
///
/// # Formato
/// * Byte 1: 0x80 (Mainnet)
/// * Bytes 2-33: Clave Privada (32 bytes)
/// * Byte 34 (Opcional): 0x01 si corresponde a una PubKey comprimida.
/// * Checksum: 4 bytes finales.
pub fn private_to_wif(secret: &SafePrivateKey, compressed: bool) -> String {
    // CORRECCIÓN: Usamos el método público to_bytes()
    let secret_bytes = secret.to_bytes();

    let mut payload = Vec::with_capacity(34);
    payload.push(MAINNET_PRIVATE_KEY_PREFIX);
    payload.extend_from_slice(&secret_bytes);

    if compressed {
        payload.push(0x01);
    }

    bs58::encode(payload)
        .with_check()
        .into_string()
}
