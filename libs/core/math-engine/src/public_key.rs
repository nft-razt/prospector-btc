// libs/core/math-engine/src/public_key.rs
// =================================================================
// APARATO: PUBLIC KEY MATH (OPTIMIZED)
// RESPONSABILIDAD: ARITMÉTICA DE PUNTO EN CURVA SECP256K1
// OPTIMIZACIÓN: CONTEXTO GLOBAL EN MULTIPLICACIÓN ESCALAR
// =================================================================

use crate::context::global_context;
use crate::private_key::SafePrivateKey;
use secp256k1::PublicKey; // ✅ Singleton

/// Wrapper seguro para una Clave Pública (Punto $P$ en la curva).
/// $P = k * G$
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafePublicKey {
    inner: PublicKey,
}

impl SafePublicKey {
    /// Deriva una Clave Pública a partir de una Clave Privada.
    ///
    /// # Performance (Elite Optimization)
    /// Esta función es el "Hot Path" del minero. Utiliza el contexto global estático
    /// para acceder a tablas de multiplicación pre-computadas, evitando la inicialización
    /// costosa en cada iteración del bucle de minería.
    pub fn from_private(private: &SafePrivateKey) -> Self {
        let secp = global_context();
        let public_key = PublicKey::from_secret_key(secp, private.as_inner());
        Self { inner: public_key }
    }

    /// Serializa el punto de la curva.
    ///
    /// - `compressed = true`: 33 bytes (0x02/0x03 + X).
    /// - `compressed = false`: 65 bytes (0x04 + X + Y).
    #[inline]
    pub fn to_bytes(&self, compressed: bool) -> Vec<u8> {
        if compressed {
            self.inner.serialize().to_vec()
        } else {
            self.inner.serialize_uncompressed().to_vec()
        }
    }

    /// Retorna referencia al objeto interno.
    #[inline(always)]
    pub fn as_inner(&self) -> &PublicKey {
        &self.inner
    }
}
