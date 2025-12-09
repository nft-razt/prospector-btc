// libs/domain/mining-strategy/src/combinatoric.rs
// =================================================================
// APARATO: COMBINATORIC ITERATOR (BIGINT EDITION)
// RESPONSABILIDAD: GENERACIÓN SECUENCIAL DE ALTA PRECISIÓN
// ESTADO: REFACTORIZADO (U256 SUPPORT)
// =================================================================

use prospector_core_math::private_key::SafePrivateKey;
use num_bigint::BigUint;
use num_traits::One;

/// Generador de entropía secuencial capaz de manejar números arbitrariamente grandes.
/// Itera desde `current` hasta `end` incrementando en 1.
pub struct CombinatoricIterator {
    current: BigUint,
    end: BigUint,
    prefix: String,
    suffix: String,
    // Buffer reusado para minimizar allocs, aunque BigUint ya hace allocs internos
    buffer: String,
}

impl CombinatoricIterator {
    /// Crea un nuevo iterador combinatorio.
    ///
    /// # Argumentos
    /// * `start`: Número inicial (BigUint).
    /// * `end`: Límite superior exclusivo (BigUint).
    /// * `prefix`: Texto fijo al inicio.
    /// * `suffix`: Texto fijo al final.
    pub fn new(start: BigUint, end: BigUint, prefix: String, suffix: String) -> Self {
        // Estimación de capacidad: prefijo + sufijo + ~78 dígitos (2^256 en decimal)
        let capacity = prefix.len() + suffix.len() + 80;
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

        // Construcción de la frase: "Prefix" + "NumeroGigante" + "Suffix"
        self.buffer.clear();
        self.buffer.push_str(&self.prefix);
        self.buffer.push_str(&self.current.to_string());
        self.buffer.push_str(&self.suffix);

        // Incremento atómico: current = current + 1
        self.current += BigUint::one();

        let phrase = self.buffer.clone();

        // Delegamos a la lógica centralizada de brainwallet (SHA256)
        // Esto convierte la frase en una clave privada válida para secp256k1
        let pk = crate::brainwallet::phrase_to_private_key(&phrase);

        Some((phrase, pk))
    }
}
