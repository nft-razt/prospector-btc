/**
 * =================================================================
 * APARATO: GEOMETRIC POINT STRUCTURE (V10.0)
 * CLASIFICACIÓN: CORE MATH (L1)
 * RESPONSABILIDAD: REPRESENTACIÓN DE COORDENADAS EN LA CURVA
 * =================================================================
 */

use crate::field::FieldElement;

/// Punto en Coordenadas Afines (Estándar para exportación).
pub struct AffinePoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub is_infinity: bool,
}

/// Punto en Coordenadas Jacobianas (Estándar para computación de alta frecuencia).
/// Proyectado para evitar el modular inverse en cada adición.
pub struct JacobianPoint {
    pub x: FieldElement, // Representa X/Z^2
    pub y: FieldElement, // Representa Y/Z^3
    pub z: FieldElement,
    pub is_infinity: bool,
}

impl JacobianPoint {
    /**
     * Transforma un punto Jacobiano a Afín para derivación de dirección.
     * Requiere UNA sola operación de inversión modular (Costo O(log P)).
     */
    pub fn to_affine(&self) -> Result<AffinePoint, crate::errors::MathError> {
        let z_inverse = self.z.invert()?;
        let z_inverse_sq = z_inverse.multiply_modular(&z_inverse);
        let z_inverse_cu = z_inverse_sq.multiply_modular(&z_inverse);

        Ok(AffinePoint {
            x: self.x.multiply_modular(&z_inverse_sq),
            y: self.y.multiply_modular(&z_inverse_cu),
            is_infinity: self.is_infinity,
        })
    }
}
