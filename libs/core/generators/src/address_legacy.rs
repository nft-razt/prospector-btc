
use prospector_core_math::hashing::hash160;
use prospector_core_math::public_key::SafePublicKey;
use bs58;

/// Prefijo de red para Bitcoin Mainnet (0x00).
/// Referencia: https://en.bitcoin.it/wiki/List_of_address_prefixes
const MAINNET_PREFIX: u8 = 0x00;

/// Transforma una Clave Pública en una Dirección Bitcoin Legacy (P2PKH).
///
/// Este proceso implementa el algoritmo estándar de Bitcoin v1.
///
/// # Algoritmo Matemático
/// $$ Address = Base58Check(Version \concat RIPEMD160(SHA256(PubKey))) $$
///
/// # Argumentos
/// * `pubkey` - La clave pública encapsulada y segura.
/// * `compressed` - Bandera booleana.
///     - `true`: Genera direcciones comprimidas (Estándar moderno, empiezan con '1').
///     - `false`: Genera direcciones no comprimidas (Estándar Satoshi 2009-2010).
///
/// # Retorno
/// Retorna un `String` con la dirección codificada en Base58Check.
pub fn pubkey_to_address(pubkey: &SafePublicKey, compressed: bool) -> String {
    // 1. Serialización del Punto en la Curva (33 o 65 bytes)
    let pubkey_bytes = pubkey.to_bytes(compressed);

    // 2. Doble Hashing (Identifier)
    // Hash160 = RIPEMD160(SHA256(K))
    let pubkey_hash = hash160(&pubkey_bytes);

    // 3. Construcción del Payload
    // [Versión (1 byte)] + [Hash (20 bytes)]
    let mut payload = Vec::with_capacity(21);
    payload.push(MAINNET_PREFIX);
    payload.extend_from_slice(&pubkey_hash);

    // 4. Codificación Base58Check
    // La librería bs58 con feature "check" calcula el checksum (doble SHA256)
    // y lo anexa automáticamente antes de codificar.
    bs58::encode(payload)
        .with_check()
        .into_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use prospector_core_math::private_key::SafePrivateKey;

    #[test]
    fn test_legacy_address_generation_genesis_block() {
        // Validación contra vectores conocidos de la red.
        // Clave privada (Hex): 18e14a7b6a307f426a94f8114701e7c8e774e7f9a47e2c2035db29a206321725
        // Dirección esperada (Uncompressed): 16UwLL9Risc3QfPqBUvKofHmBQ7wMtjvM

        let secret_hex = "18e14a7b6a307f426a94f8114701e7c8e774e7f9a47e2c2035db29a206321725";
        let secret_bytes = hex::decode(secret_hex).unwrap();
        let sk = SafePrivateKey::from_bytes(&secret_bytes).unwrap();
        let pk = SafePublicKey::from_private(&sk);

        let address = pubkey_to_address(&pk, false);
        assert_eq!(address, "16UwLL9Risc3QfPqBUvKofHmBQ7wMtjvM");
    }
}
