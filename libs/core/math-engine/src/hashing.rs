use sha2::{Sha256, Digest};
use ripemd::Ripemd160;

/// Realiza un doble SHA-256 (SHA256(SHA256(data))).
/// Usado comúnmente en Bitcoin para Checksums y Merkle Trees.
#[inline]
pub fn double_sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result1 = hasher.finalize();

    let mut hasher2 = Sha256::new();
    hasher2.update(result1);
    hasher2.finalize().into()
}

/// Realiza un HASH160 (RIPEMD160(SHA256(data))).
/// Usado para generar direcciones Bitcoin (P2PKH).
#[inline]
pub fn hash160(data: &[u8]) -> [u8; 20] {
    let mut sha_hasher = Sha256::new();
    sha_hasher.update(data);
    let sha_result = sha_hasher.finalize();

    let mut ripe_hasher = Ripemd160::new();
    ripe_hasher.update(sha_result);
    ripe_hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash160_known_vector() {
        let input = b"hello";
        let expected = hex::decode("b6a9c8c230722b7c748331a8b450f05566dc7d0f").unwrap();
        let result = hash160(input);
        assert_eq!(result.to_vec(), expected);
    }
}
