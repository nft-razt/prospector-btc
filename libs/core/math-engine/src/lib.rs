/**
 * =================================================================
 * APARATO: CORE MATH MASTER HUB (V32.0 - KANGAROO EXPOSED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: EXPOSICIÓN DE PRIMITIVAS MATEMÁTICAS SOBERANAS
 * =================================================================
 */

pub mod arithmetic;
pub mod curve;
pub mod field;
pub mod point;
pub mod field_simd;
pub mod curve_simd;
pub mod hashing;
pub mod private_key;
pub mod public_key;
pub mod errors;
pub mod context;

// ✅ CORRECCIÓN CRÍTICA: Exposición pública para el motor de estrategia
pub mod kangaroo;

pub mod prelude {
    pub use crate::arithmetic::{add_u64_to_u256_be, bytes_to_words_u256, words_to_bytes_u256};
    pub use crate::field::FieldElement;
    pub use crate::point::JacobianPoint;
    pub use crate::curve::UnifiedCurveEngine;
    pub use crate::field_simd::FieldElementVector4;
    pub use crate::curve_simd::JacobianPointVector4;
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;
    pub use crate::errors::MathError;
    pub use crate::hashing::hash160;
    // ✅ Re-exportación conveniente
    pub use crate::kangaroo::{KangarooSolver, KangarooConfig};
}
