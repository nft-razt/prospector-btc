// =================================================================
// APARATO: PRIVATE KEY MANAGER
// RESPONSABILIDAD: GESTIÓN SEGURA DE ESCALARES SECRETOS (256 BITS)
// =================================================================

use secp256k1::{SecretKey, Secp256k1};
use secp256k1::rand::rngs::OsRng;
use crate::errors::MathError;

/// Wrapper seguro para una clave privada de curva elíptica secp256k1.
///
/// Este struct garantiza que el valor interno sea siempre un escalar válido
/// (0 < k < n) y abstrae la complejidad de la librería C subyacente.
pub struct SafePrivateKey {
    inner: SecretKey,
}

impl SafePrivateKey {
    /// Genera una nueva clave privada utilizando el generador de números aleatorios
    /// criptográficamente seguro del Sistema Operativo (CSPRNG).
    ///
    /// Esto es vital para asegurar que las claves propias del sistema
    /// (si alguna vez generamos wallets para usuarios) no sean predecibles.
    pub fn new_random() -> Self {
        let secp = Secp256k1::new();
        // OsRng toma entropía de /dev/urandom (Linux/Mac) o CryptGenRandom (Windows)
        let (secret_key, _) = secp.generate_keypair(&mut OsRng);
        Self { inner: secret_key }
    }

    /// Intenta construir una clave privada a partir de una secuencia de bytes cruda.
    ///
    /// # Errores
    /// Retorna `MathError` si:
    /// - La longitud no es exactamente 32 bytes.
    /// - El valor numérico es 0 o mayor que el orden de la curva (n).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MathError> {
        if bytes.len() != 32 {
            return Err(MathError::InvalidLength {
                expected: 32,
                got: bytes.len()
            });
        }

        let sk = SecretKey::from_slice(bytes)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self { inner: sk })
    }

    /// Serializa la clave privada a un array de 32 bytes.
    /// Vital para la exportación a formato WIF o almacenamiento en frío.
    pub fn to_bytes(&self) -> [u8; 32] {
        self.inner.secret_bytes()
    }

    /// Obtiene una referencia al objeto interno `SecretKey` de la librería `secp256k1`.
    ///
    /// Útil para operaciones de bajo nivel dentro del crate (como derivación de PubKey).
    pub fn as_inner(&self) -> &SecretKey {
        &self.inner
    }
}

// =================================================================
// PROTOCOLO DE PRUEBAS DE INVARIANTE MATEMÁTICO
// =================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_random_key_generation_is_non_deterministic() {
        // INVARIANTE: Generar 100 claves seguidas no debe producir colisiones.
        // Si esto falla, el generador de entropía del SO (OsRng) está roto o comprometido.
        let mut set = HashSet::new();
        for _ in 0..100 {
            let key = SafePrivateKey::new_random();
            let bytes = key.to_bytes();

            // Verificación 1: No debe ser una clave nula (todo ceros)
            assert_ne!(bytes, [0u8; 32], "FATAL: Se generó una clave nula (Zero Key)");

            // Verificación 2: Unicidad estadística
            assert!(set.insert(bytes), "FATAL: Colisión de entropía detectada. El RNG no es seguro.");
        }
    }

    #[test]
    fn test_roundtrip_bytes() {
        // INVARIANTE: Serialización <-> Deserialización debe ser sin pérdida (Lossless).
        // key -> bytes -> key' debe resultar en key == key'
        let original = SafePrivateKey::new_random();
        let bytes = original.to_bytes();

        let recovered = SafePrivateKey::from_bytes(&bytes)
            .expect("Debería poder recuperar una clave válida desde bytes válidos");

        assert_eq!(original.to_bytes(), recovered.to_bytes(), "Fallo en la integridad de los datos");
    }

    #[test]
    fn test_invalid_bytes_handling() {
        // INVARIANTE: El sistema debe rechazar basura.

        // Caso 1: Longitud incorrecta
        let bad_len = [0u8; 31];
        assert!(SafePrivateKey::from_bytes(&bad_len).is_err());

        // Caso 2: Clave fuera de rango (0xFF...FF es mayor que el orden de secp256k1)
        let overflow_key = [0xFFu8; 32];
        // Nota: secp256k1::SecretKey::from_slice chequea el orden de la curva
        assert!(SafePrivateKey::from_bytes(&overflow_key).is_err());
    }
}
