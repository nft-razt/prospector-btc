// libs/core/math-engine/src/public_key.rs
/**
 * =================================================================
 * APARATO: PUBLIC KEY ENGINE (V17.0 - LINT FREE)
 * CLASIFICACIÓN: CORE MATH (L1)
 * RESPONSABILIDAD: GESTIÓN DE PUNTOS AFINES Y SERIALIZACIÓN
 * =================================================================
 */
use crate::context::global_context;
use crate::errors::MathError;
use crate::private_key::SafePrivateKey;
use secp256k1::{PublicKey, Scalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafePublicKey {
    internal_point: PublicKey,
}

impl SafePublicKey {
    #[inline(always)]
    pub fn from_private(private_key_handle: &SafePrivateKey) -> Self {
        let context = global_context();
        let point = PublicKey::from_secret_key(context, private_key_handle.as_inner());
        Self { internal_point: point }
    }

    /// Reconstruye una clave pública desde bytes (SEC1).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MathError> {
        // CORRECCIÓN LINT: Prefijo _ para variable no leída explícitamente pero necesaria para inicialización lazy.
        let _context = global_context();
        let point = PublicKey::from_slice(bytes)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self { internal_point: point })
    }

    #[inline(always)]
    pub fn increment(&self) -> Result<Self, MathError> {
        let context = global_context();
        let mut one_scalar_bytes = [0u8; 32];
        one_scalar_bytes[31] = 1;
        let scalar_one = Scalar::from_be_bytes(one_scalar_bytes)
            .map_err(|_| MathError::InvalidKeyFormat("INTERNAL_SCALAR_ERROR".into()))?;

        let updated_point = self.internal_point.add_exp_tweak(context, &scalar_one)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self { internal_point: updated_point })
    }

    #[inline(always)]
    pub fn add_scalar(&self, scalar_bytes: &[u8; 32]) -> Result<Self, MathError> {
        let context = global_context();
        let scalar_value = Scalar::from_be_bytes(*scalar_bytes)
            .map_err(|_| MathError::InvalidKeyFormat("SCALAR_OVERFLOW".into()))?;

        let updated_point = self.internal_point.add_exp_tweak(context, &scalar_value)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self { internal_point: updated_point })
    }

    #[inline(always)]
    pub fn to_bytes(&self, use_compression: bool) -> Vec<u8> {
        if use_compression {
            self.internal_point.serialize().to_vec()
        } else {
            self.internal_point.serialize_uncompressed().to_vec()
        }
    }

    #[inline(always)]
    pub fn as_inner(&self) -> &PublicKey {
        &self.internal_point
    }
}
