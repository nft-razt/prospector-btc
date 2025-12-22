/**
 * =================================================================
 * APARATO: GEOMETRIC POINT ENGINE (V48.0 - FIELD INTEGRATED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: MANIPULACIÓN DE PUNTOS EN ESPACIO PROYECTIVO
 *
 * VISION HIPER-HOLÍSTICA:
 * Elimina las dependencias de 'todo!' y vincula el punto Jacobiano
 * directamente con el Solinas Field Engine. Provee la base para el
 * algoritmo de adición P + G que duplica el hashrate.
 * =================================================================
 */

use crate::field::FieldElement;
use crate::errors::MathError;

/// Punto en la curva secp256k1 usando coordenadas Jacobianas (X, Y, Z).
/// Donde la coordenada afín x = X/Z^2 e y = Y/Z^3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JacobianPoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub z: FieldElement,
    pub is_infinity: bool,
}

impl JacobianPoint {
    /**
     * Crea un punto Jacobiano a partir de coordenadas afines (X, Y).
     * Se utiliza típicamente para cargar el Punto Generador G o llaves base.
     */
    pub fn from_affine(x_raw: [u64; 4], y_raw: [u64; 4]) -> Self {
        Self {
            x: FieldElement { internal_words: x_raw },
            y: FieldElement { internal_words: y_raw },
            z: FieldElement::from_u64(1), // Z=1 define el plano afín
            is_infinity: false,
        }
    }

    /**
     * Transforma el punto Jacobiano a coordenadas afines.
     * Requiere una inversión modular de Z, seguido de multiplicaciones de campo.
     *
     * # Errors
     * Retorna MathError si el punto está en el infinito (no invertible).
     */
    pub fn to_affine_bytes(&self) -> Result<([u8; 32], [u8; 32]), MathError> {
        if self.is_infinity {
            return Err(MathError::InvalidKeyFormat("POINT_AT_INFINITY".into()));
        }

        // 1. Calcular Z^-1 mod p
        let z_inv = self.z.invert()?;
        let z_inv_sq = z_inv.square_modular();
        let z_inv_cu = z_inv_sq.multiply_modular(&z_inv);

        // 2. Recuperar X = X/Z^2, Y = Y/Z^3
        let x_affine = self.x.multiply_modular(&z_inv_sq);
        let y_affine = self.y.multiply_modular(&z_inv_cu);

        // 3. Serialización a Big-Endian para compatibilidad Bitcoin
        Ok((
            self.serialize_field_element(&x_affine),
            self.serialize_field_element(&y_affine)
        ))
    }

    fn serialize_field_element(&self, element: &FieldElement) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        for i in 0..4 {
            let word_bytes = element.internal_words[3 - i].to_be_bytes();
            bytes[i * 8..(i + 1) * 8].copy_from_slice(&word_bytes);
        }
        bytes
    }

    pub fn infinity() -> Self {
        Self {
            x: FieldElement::from_u64(0),
            y: FieldElement::from_u64(0),
            z: FieldElement::from_u64(0),
            is_infinity: true,
        }
    }
}
