// libs/domain/forensics/src/android_rng.rs
// =================================================================
// APARATO: ANDROID LCG SIMULATOR (CVE-2013-7372)
// RESPONSABILIDAD: SIMULACIÓN DEL GENERADOR LINEAL CONGRUENTE DÉBIL
// ALGORITMO: Java's java.util.Random (Knuth's LCG)
// =================================================================

use byteorder::{BigEndian, ByteOrder};
use prospector_core_math::private_key::SafePrivateKey;

/// Constantes del LCG de Java.
/// Formula: next = (seed * MULTIPLIER + ADDEND) & MASK
const JAVA_LCG_MULTIPLIER: u64 = 0x5DEECE66D;
const JAVA_LCG_ADDEND: u64 = 0xB;
const JAVA_LCG_MASK: u64 = (1u64 << 48) - 1;

/// Iterador que simula la generación de claves privadas usando un PRNG débil.
///
/// En el incidente de 2013, algunas aplicaciones de Android usaban `SecureRandom`
/// pero este caía internamente en un estado de entropía baja o nula,
/// comportándose de facto como un LCG predecible.
pub struct AndroidLcgIterator {
    current_seed: u64,
    end_seed: u64,
}

impl AndroidLcgIterator {
    /// Inicializa el iterador con un rango de semillas.
    /// Las semillas típicas débiles suelen ser timestamps (ms) de la época 2010-2013.
    pub fn new(start_seed: u64, end_seed: u64) -> Self {
        Self {
            current_seed: start_seed,
            end_seed,
        }
    }

    /// Avanza el estado del LCG y retorna un entero de 32 bits (pseudo-aleatorio).
    /// Simula `java.util.Random.next(32)`.
    fn next_int(seed: &mut u64) -> u32 {
        *seed = (*seed * JAVA_LCG_MULTIPLIER + JAVA_LCG_ADDEND) & JAVA_LCG_MASK;
        (*seed >> 16) as u32
    }

    /// Genera una clave privada de 256 bits consumiendo 8 enteros del LCG.
    ///
    /// Esta es una reconstrucción de cómo una librería Java ingenua generaría
    /// 32 bytes de "entropía" llamando repetidamente al generador.
    fn generate_weak_key(mut seed: u64) -> SafePrivateKey {
        let mut bytes = [0u8; 32];

        // Llenamos el buffer de 32 bytes con 8 llamadas a nextInt() (4 bytes c/u)
        for chunk in bytes.chunks_mut(4) {
            let rand_int = Self::next_int(&mut seed);
            BigEndian::write_u32(chunk, rand_int);
        }

        // Intentamos instanciar la clave. En el improbable caso de ser inválida (0 o >= N),
        // fallback a una clave segura para no romper el iterador.
        SafePrivateKey::from_bytes(&bytes).unwrap_or_else(|_| SafePrivateKey::new_random())
    }
}

impl Iterator for AndroidLcgIterator {
    type Item = (String, SafePrivateKey);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_seed >= self.end_seed {
            return None;
        }

        let seed = self.current_seed;
        self.current_seed += 1;

        let pk = Self::generate_weak_key(seed);
        let source = format!("forensic_android_lcg:seed_{}", seed);

        Some((source, pk))
    }
}
