// libs/domain/mining-strategy/src/brainwallet.rs
use prospector_core_math::private_key::SafePrivateKey;
use sha2::{Digest, Sha256};

/// Convierte una frase de texto (passphrase) en una Clave Privada.
///
/// Aplica SHA-256 sobre los bytes UTF-8 de la entrada.
pub fn phrase_to_private_key(phrase: &str) -> SafePrivateKey {
    let mut hasher = Sha256::new();
    hasher.update(phrase.as_bytes());
    let result = hasher.finalize();

    SafePrivateKey::from_bytes(&result).expect("Hash SHA256 inválido para curva secp256k1")
}

/// Iterador para recorrer una lista de palabras en memoria.
pub struct BrainwalletIterator<'a> {
    dictionary: &'a [String],
    index: usize,
}

impl<'a> BrainwalletIterator<'a> {
    /// Crea un nuevo iterador sobre un slice de palabras.
    pub fn new(dictionary: &'a [String]) -> Self {
        Self {
            dictionary,
            index: 0,
        }
    }
}

impl<'a> Iterator for BrainwalletIterator<'a> {
    type Item = (String, SafePrivateKey);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.dictionary.len() {
            return None;
        }

        let phrase = &self.dictionary[self.index];
        self.index += 1;

        let private_key = phrase_to_private_key(phrase);
        Some((phrase.clone(), private_key))
    }
}
