// libs/domain/mining-strategy/src/combinatoric.rs
use prospector_core_math::private_key::SafePrivateKey;
// use prospector_core_math::hashing::double_sha256; // <-- ELIMINADO

/// Generador de entropía secuencial.
pub struct CombinatoricIterator {
    current: u64,
    end: u64,
    prefix: String,
    suffix: String,
    buffer: String,
}

impl CombinatoricIterator {
    pub fn new(start: u64, end: u64, prefix: String, suffix: String) -> Self {
        let capacity = prefix.len() + suffix.len() + 20;
        Self {
            current: start,
            end,
            prefix,
            suffix,
            buffer: String::with_capacity(capacity),
        }
    }
}

impl Iterator for CombinatoricIterator {
    type Item = (String, SafePrivateKey);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        self.buffer.clear();
        self.buffer.push_str(&self.prefix);
        self.buffer.push_str(&self.current.to_string());
        self.buffer.push_str(&self.suffix);

        self.current += 1;

        let phrase = self.buffer.clone();

        // Delegamos a la lógica centralizada de brainwallet
        let pk = crate::brainwallet::phrase_to_private_key(&phrase);

        Some((phrase, pk))
    }
}
