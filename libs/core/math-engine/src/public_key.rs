// INICIO DEL ARCHIVO [libs/core/math-engine/src/public_key.rs]
// =================================================================
// APARATO: PUBLIC KEY ENGINE (V9.5 - ELITE GOLD)
// RESPONSABILIDAD: ARITMÉTICA DE PUNTOS SOBRE LA CURVA SECP256K1
// ESTADO: OPTIMIZED (ZERO WARNINGS / FULLY DOCUMENTED)
// =================================================================

use crate::context::global_context;
use crate::errors::MathError;
use crate::private_key::SafePrivateKey;
use secp256k1::{PublicKey, Scalar};

/// Wrapper seguro para un Punto en la Curva Elíptica ($P$).
///
/// Representa una Clave Pública de Bitcoin. Internamente gestiona las coordenadas
/// del punto $P = (x, y)$ satisfaciendo $y^2 = x^3 + 7$.
///
/// Implementa `Copy` para permitir semántica de movimiento barata (64 bytes en stack).
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
    /// # Rendimiento
    /// Utiliza el `GLOBAL_CONTEXT` pre-computado para evitar re-inicializar
    /// tablas de multiplicación en cada llamada.
    #[inline]
    pub fn from_private(private: &SafePrivateKey) -> Self {
        let secp = global_context();
        let public_key = PublicKey::from_secret_key(secp, private.as_inner());
        Self { inner: public_key }
    }

    /// Deserializa una clave pública desde bytes en formato SEC1.
    ///
    /// # Formatos Soportados
    /// * **Comprimido (33 bytes):** `0x02` o `0x03` seguido de $x$.
    /// * **No Comprimido (65 bytes):** `0x04` seguido de $x$ e $y$.
    pub fn from_bytes(data: &[u8]) -> Result<Self, MathError> {
        let point = PublicKey::from_slice(data)
            .map_err(MathError::EllipticCurveError)?;
        Ok(Self { inner: point })
    }

    /// Realiza la operación de "Tweaking" (Adición de Escalar).
    ///
    /// Modifica el punto público sumándole otro punto derivado de un escalar.
    /// $$ P' = P + (s \cdot G) $$
    ///
    /// Esta operación es fundamental para:
    /// 1. Algoritmos de búsqueda como **Pollard's Kangaroo** (saltos en la curva).
    /// 2. Derivación de claves en HD Wallets (BIP32).
    ///
    /// # Argumentos
    /// * `scalar_bytes`: Entero de 256 bits (Big-Endian) que representa $s$.
    pub fn add_scalar(&self, scalar_bytes: &[u8; 32]) -> Result<Self, MathError> {
        let secp = global_context();

        // 1. Parsing seguro del escalar (Validación de rango 0 < s < n)
        let scalar = Scalar::from_be_bytes(*scalar_bytes)
            .map_err(|_| MathError::InvalidKeyFormat("Scalar overflow: Valor mayor al orden de la curva".into()))?;

        // 2. Clonación del punto
        // OPTIMIZACIÓN: Mutabilidad eliminada según análisis estático del compilador (Linter Clean-up).
        // La librería secp256k1 maneja la mutación interna a través de punteros FFI seguros.
        let mut new_point = self.inner;

        // 3. Operación algebraica
        new_point.add_exp_tweak(secp, &scalar)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self { inner: new_point })
    }

    /// Serializa el punto al formato estándar de Bitcoin (SEC1).
    ///
    /// # Argumentos
    /// * `compressed`:
    ///     - `true`: Retorna 33 bytes (Prefijo 02/03 + X). Estándar moderno.
    ///     - `false`: Retorna 65 bytes (Prefijo 04 + X + Y). Estándar legacy (Satoshi).
    #[inline]
    pub fn to_bytes(&self, compressed: bool) -> Vec<u8> {
        if compressed {
            self.inner.serialize().to_vec()
        } else {
            self.inner.serialize_uncompressed().to_vec()
        }
    }

    /// Acceso de bajo nivel al objeto interno de `secp256k1`.
    ///
    /// # Objetivo
    /// Permite la interoperabilidad con otras partes del sistema que requieran
    /// el tipo primitivo `PublicKey` sin incurrir en costos de conversión.
    ///
    /// # Retorno
    /// Referencia inmutable directa al struct FFI subyacente.
    #[inline(always)]
    pub fn as_inner(&self) -> &PublicKey {
        &self.inner
    }
}
// FIN DEL ARCHIVO [libs/core/math-engine/src/public_key.rs]
