// libs/core/math-engine/src/public_key.rs
// =================================================================
// APARATO: PUBLIC KEY MATH (OPTIMIZED + ALGEBRAIC)
// RESPONSABILIDAD: ARITMÉTICA DE PUNTO EN CURVA SECP256K1
// OPTIMIZACIÓN: ZERO-COST ABSTRACTIONS & GLOBAL CONTEXT
// =================================================================

use crate::context::global_context;
use crate::errors::MathError;
use crate::private_key::SafePrivateKey;
use secp256k1::{PublicKey, Scalar};

/// Wrapper seguro para una Clave Pública (Punto $P$ en la curva).
/// Matemáticamente: $P = k * G$
///
/// # Optimización de Memoria
/// Deriva `Copy` para permitir el paso por valor en registros de CPU,
/// vital para algoritmos iterativos de alto rendimiento como Pollard's Kangaroo.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct SafePublicKey {
    inner: PublicKey,
}

impl SafePublicKey {
    /// Deriva una Clave Pública a partir de una Clave Privada.
    ///
    /// # Performance (Elite Optimization)
    /// Utiliza el contexto global estático para acceder a tablas de multiplicación
    /// pre-computadas, evitando la inicialización costosa (malloc) en cada llamada.
    pub fn from_private(private: &SafePrivateKey) -> Self {
        let secp = global_context();
        let public_key = PublicKey::from_secret_key(secp, private.as_inner());
        Self { inner: public_key }
    }

    /// Construye una Clave Pública desde sus bytes serializados.
    ///
    /// Soporta ambos formatos estándar de Bitcoin:
    /// - **Comprimido (33 bytes):** 0x02/0x03 + X coordinate.
    /// - **No Comprimido (65 bytes):** 0x04 + X + Y coordinate.
    pub fn from_bytes(data: &[u8]) -> Result<Self, MathError> {
        let point = PublicKey::from_slice(data)
            .map_err(MathError::EllipticCurveError)?;
        Ok(Self { inner: point })
    }

    /// Realiza una suma de punto con un escalar ("Tweaking").
    /// Operación algebraica: $P' = P + (scalar * G)$.
    ///
    /// # Caso de Uso: Protocolo Canguro
    /// Permite "saltar" en la curva pública simulando la adición de una clave privada
    /// desconocida, sin necesidad de conocer $k$ original.
    ///
    /// # Argumentos
    /// * `scalar_bytes`: Entero de 256 bits (32 bytes BigEndian) a sumar.
    pub fn add_scalar(&self, scalar_bytes: &[u8; 32]) -> Result<Self, MathError> {
        let secp = global_context();

        // Parseo seguro del escalar (verifica que < Order de la curva)
        let scalar = Scalar::from_be_bytes(*scalar_bytes)
            .map_err(|_| MathError::InvalidKeyFormat("Scalar inválido (overflow de curva)".into()))?;

        // Clonamos el punto interno (operación barata de copia de 64 bytes)
        let mut new_point = self.inner;

        // Ejecución optimizada en C (libsecp256k1)
        // add_exp_assign realiza la operación in-place: P = P + scalar * G
        new_point.add_exp_assign(secp, &scalar)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self { inner: new_point })
    }

    /// Serializa el punto de la curva a formato binario.
    ///
    /// # Argumentos
    /// * `compressed`:
    ///     - `true`: Retorna 33 bytes (Estándar moderno).
    ///     - `false`: Retorna 65 bytes (Estándar Legacy/Satoshi).
    #[inline]
    pub fn to_bytes(&self, compressed: bool) -> Vec<u8> {
        if compressed {
            self.inner.serialize().to_vec()
        } else {
            self.inner.serialize_uncompressed().to_vec()
        }
    }

    /// Retorna referencia al objeto interno de `secp256k1`.
    /// Útil para interoperabilidad con otras librerías del ecosistema Rust.
    #[inline(always)]
    pub fn as_inner(&self) -> &PublicKey {
        &self.inner
    }
}
