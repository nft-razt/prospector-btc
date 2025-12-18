// libs/core/math-engine/src/public_key.rs
// =================================================================
// APARATO: PUBLIC KEY ENGINE (V10.0 - IMMUTABLE EDITION)
// RESPONSABILIDAD: ARITMÉTICA DE PUNTOS SOBRE LA CURVA SECP256K1
// ESTADO: RESOLUCIÓN DE WARNING rustc(unused_mut) + OPTIMIZACIÓN FUNCIONAL
// =================================================================

use crate::context::global_context;
use crate::errors::MathError;
use crate::private_key::SafePrivateKey;
use secp256k1::{PublicKey, Scalar};

/// Wrapper seguro y atómico para un Punto en la Curva Elíptica ($P$).
///
/// Representa una Clave Pública de Bitcoin. Esta estructura encapsula las
/// coordenadas $(x, y)$ que satisfacen la ecuación de Weierstrass $y^2 = x^3 + 7$.
///
/// La estructura es inmutable por diseño para garantizar la integridad en
/// entornos de minería multihilo.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct SafePublicKey {
    inner: PublicKey,
}

impl SafePublicKey {
    /// Deriva la clave pública a partir de una clave privada ($k$).
    ///
    /// # Matemáticas
    /// Realiza la multiplicación escalar del punto generador $G$:
    /// $$ P = k \cdot G $$
    ///
    /// Utiliza el contexto global estático para maximizar el rendimiento.
    #[inline(always)]
    pub fn from_private(private_key_handle: &SafePrivateKey) -> Self {
        let context = global_context();
        let internal_point = PublicKey::from_secret_key(context, private_key_handle.as_inner());
        Self { inner: internal_point }
    }

    /// Deserializa una clave pública desde un buffer de bytes en formato SEC1.
    ///
    /// Soporta formatos comprimidos (33 bytes) y no comprimidos (65 bytes).
    pub fn from_bytes(raw_bytes: &[u8]) -> Result<Self, MathError> {
        let internal_point = PublicKey::from_slice(raw_bytes)
            .map_err(MathError::EllipticCurveError)?;
        Ok(Self { inner: internal_point })
    }

    /// Realiza la operación de "Tweak Addition" (Adición de Escalar).
    ///
    /// $$ P' = P + (s \cdot G) $$
    ///
    /// Esta función es crítica para el algoritmo **Pollard's Kangaroo** y la
    /// derivación de carteras jerárquicas (BIP32).
    ///
    /// # Optimización V10.0
    /// Se eliminó la variable mutable intermedia `mut new_point` para cumplir con
    /// el estándar de inmutabilidad y resolver el warning del compilador.
    ///
    /// # Argumentos
    /// * `scalar_bytes`: El escalar $s$ de 256 bits en formato Big-Endian.
    pub fn add_scalar(&self, scalar_bytes: &[u8; 32]) -> Result<Self, MathError> {
        let context = global_context();

        // 1. Parsing del escalar con validación de rango (0 < s < n)
        let scalar_value = Scalar::from_be_bytes(*scalar_bytes)
            .map_err(|_| MathError::InvalidKeyFormat(
                "Scalar overflow: El valor excede el orden de la curva".to_string()
            ))?;

        // 2. Operación Criptográfica Directa
        // add_exp_tweak retorna una nueva PublicKey, no requiere mutabilidad local.
        let tweaked_point = self.inner.add_exp_tweak(context, &scalar_value)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self { inner: tweaked_point })
    }

    /// Serializa el punto en la curva al formato binario estándar SEC1.
    ///
    /// # Argumentos
    /// * `use_compressed_format`:
    ///     - `true`: 33 bytes (Prefijo 02/03 + X). Estándar moderno.
    ///     - `false`: 65 bytes (Prefijo 04 + X + Y). Estándar legacy de Satoshi.
    #[inline]
    pub fn to_bytes(&self, use_compressed_format: bool) -> Vec<u8> {
        if use_compressed_format {
            self.inner.serialize().to_vec()
        } else {
            self.inner.serialize_uncompressed().to_vec()
        }
    }

    /// Provee acceso directo al tipo primitivo de la librería subyacente.
    ///
    /// Útil para operaciones FFI o integraciones directas con `secp256k1`.
    #[inline(always)]
    pub fn as_inner(&self) -> &PublicKey {
        &self.inner
    }
}
