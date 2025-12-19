// libs/core/math-engine/src/public_key.rs
/**
 * =================================================================
 * APARATO: PUBLIC KEY ENGINE (V15.0 - HIGH PERFORMANCE EDITION)
 * CLASIFICACIÓN: CORE MATH (L1)
 * RESPONSABILIDAD: ARITMÉTICA DE PUNTOS Y OPTIMIZACIÓN SECUENCIAL
 *
 * ESTRATEGIA DE ÉLITE:
 * - Incremento Constante: Implementa la adición de puntos para evitar multiplicaciones escalares.
 * - Memoria Estática: Utiliza el generador G pre-computado.
 * - Zero-Copy: Retorno de tipos por valor optimizado para registros de CPU.
 * =================================================================
 */
use crate::context::global_context;
use crate::errors::MathError;
use crate::private_key::SafePrivateKey;
use secp256k1::{PublicKey, Scalar};

/// Representación atómica de una Clave Pública de Bitcoin.
/// Encapsula un punto en la curva secp256k1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafePublicKey {
    internal_point: PublicKey,
}

impl SafePublicKey {
    /**
     * Construye una clave pública a partir de una clave privada.
     * Operación: P = k * G
     * Nota: Utilizar solo para el punto de inicio de un rango.
     */
    #[inline(always)]
    pub fn from_private(private_key_handle: &SafePrivateKey) -> Self {
        let context = global_context();
        let point = PublicKey::from_secret_key(context, private_key_handle.as_inner());
        Self {
            internal_point: point,
        }
    }

    /**
     * Incrementa el punto actual en una unidad (P = P + G).
     *
     * ESTRATEGIA DE ÉLITE:
     * En lugar de realizar una multiplicación escalar completa (O(log n)),
     * realizamos una adición de punto contra el generador G (O(1)).
     * Esto incrementa el Hashrate en un orden de magnitud.
     */
    #[inline(always)]
    pub fn increment(&self) -> Result<Self, MathError> {
        let context = global_context();
        // El escalar "1" codificado en 32 bytes Big-Endian
        let mut one_scalar_bytes = [0u8; 32];
        one_scalar_bytes[31] = 1;

        let scalar_one = Scalar::from_be_bytes(one_scalar_bytes)
            .map_err(|_| MathError::InvalidKeyFormat("INTERNAL_SCALAR_ERROR".into()))?;

        // add_exp_tweak realiza P + (1 * G) internamente de forma optimizada
        let updated_point = self
            .internal_point
            .add_exp_tweak(context, &scalar_one)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self {
            internal_point: updated_point,
        })
    }

    /**
     * Realiza una adición escalar arbitraria (P = P + s*G).
     * Útil para saltos en el algoritmo de Pollard's Kangaroo.
     */
    #[inline(always)]
    pub fn add_scalar(&self, scalar_bytes: &[u8; 32]) -> Result<Self, MathError> {
        let context = global_context();
        let scalar_value = Scalar::from_be_bytes(*scalar_bytes)
            .map_err(|_| MathError::InvalidKeyFormat("SCALAR_OVERFLOW".into()))?;

        let updated_point = self
            .internal_point
            .add_exp_tweak(context, &scalar_value)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self {
            internal_point: updated_point,
        })
    }

    /**
     * Serializa el punto en formato SEC1.
     * @param use_compression true para 33 bytes (Estándar), false para 65 bytes (Legacy).
     */
    #[inline(always)]
    pub fn to_bytes(&self, use_compression: bool) -> Vec<u8> {
        if use_compression {
            self.internal_point.serialize().to_vec()
        } else {
            self.internal_point.serialize_uncompressed().to_vec()
        }
    }

    /// Acceso directo al tipo nativo para operaciones de bajo nivel.
    #[inline(always)]
    pub fn as_inner(&self) -> &PublicKey {
        &self.internal_point
    }
}
