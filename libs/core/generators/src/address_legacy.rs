use bs58;
use prospector_core_math::hashing::hash160;
use prospector_core_math::public_key::SafePublicKey;

/// Prefijo de red para Bitcoin Mainnet (0x00).
const MAINNET_PREFIX: u8 = 0x00;

/// Transforma una Clave Pública en una Dirección Bitcoin Legacy (P2PKH).
///
/// # Argumentos
/// * `pubkey` - La clave pública encapsulada.
/// * `compressed` - Booleano para formato comprimido/descomprimido.
pub fn pubkey_to_address(pubkey: &SafePublicKey, compressed: bool) -> String {
    let pubkey_bytes = pubkey.to_bytes(compressed);
    encode_address_payload(&pubkey_bytes)
}

/// Genera una dirección directamente desde la coordenada X afín.
///
/// **Nota de Rendimiento:** Esta función asume una reconstrucción de
/// clave pública comprimida (0x02 || X) para evitar el costo de
/// calcular Y en el bucle caliente.
///
/// # Argumentos
/// * `affine_x` - Coordenada X pura (32 bytes).
/// * `_compressed` - Placeholder para compatibilidad futura de API (ignorado, siempre compressed).
pub fn pubkey_from_x_to_address(affine_x: &[u8; 32], _compressed: bool) -> String {
    let mut payload = Vec::with_capacity(33);
    payload.push(0x02); // Prefijo par por defecto
    payload.extend_from_slice(affine_x);
    encode_address_payload(&payload)
}

// Helper interno para DRY
fn encode_address_payload(pubkey_bytes: &[u8]) -> String {
    let pubkey_hash = hash160(pubkey_bytes);
    let mut payload = Vec::with_capacity(21);
    payload.push(MAINNET_PREFIX);
    payload.extend_from_slice(&pubkey_hash);
    bs58::encode(payload).with_check().into_string()
}
