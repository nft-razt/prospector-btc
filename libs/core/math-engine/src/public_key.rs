// =================================================================
// APARATO: PUBLIC KEY MATH
// RESPONSABILIDAD: ARITMÉTICA DE PUNTO EN CURVA SECP256K1
// =================================================================

use secp256k1::{PublicKey, Secp256k1};
use crate::private_key::SafePrivateKey;

/// Wrapper seguro para una Clave Pública (Punto en la curva).
///
/// Representa el resultado de la multiplicación escalar $P = k * G$, donde:
/// - $k$ es la clave privada.
/// - $G$ es el punto generador de secp256k1.
pub struct SafePublicKey {
    inner: PublicKey,
}

impl SafePublicKey {
    /// Deriva una Clave Pública a partir de una Clave Privada segura.
    ///
    /// Esta operación es computacionalmente costosa. Utiliza optimizaciones
    /// de la librería C `libsecp256k1` para realizar la multiplicación de puntos.
    pub fn from_private(private: &SafePrivateKey) -> Self {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, private.as_inner());
        Self { inner: public_key }
    }

    /// Serializa el punto de la curva a una secuencia de bytes.
    ///
    /// # Argumentos
    /// * `compressed`:
    ///     - `true`: Formato comprimido (33 bytes). Prefijo `0x02` o `0x03` + coordenada X.
    ///       Usado en direcciones modernas y Segwit.
    ///     - `false`: Formato no comprimido (65 bytes). Prefijo `0x04` + X + Y.
    ///       Usado en las direcciones "Legacy" originales (Satoshi Era, 2009-2012).
    ///       **Crucial para la tesis de arqueología.**
    pub fn to_bytes(&self, compressed: bool) -> Vec<u8> {
        if compressed {
            self.inner.serialize().to_vec()
        } else {
            self.inner.serialize_uncompressed().to_vec()
        }
    }
}
