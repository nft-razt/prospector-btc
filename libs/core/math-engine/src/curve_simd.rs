/**
 * =================================================================
 * APARATO: VECTORIZED JACOBIAN ENGINE (V65.1 - STABLE COMPLIANT)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: ADICIÓN EN PARALELO DE PUNTOS SECP256K1
 * =================================================================
 */

use crate::field_simd::FieldElementVector4;

/// Representa cuatro puntos en coordenadas Jacobianas para procesamiento paralelo.
#[derive(Clone, Copy)]
pub struct JacobianPointVector4 {
    pub x_coordinates: FieldElementVector4,
    pub y_coordinates: FieldElementVector4,
    pub z_coordinates: FieldElementVector4,
}

impl JacobianPointVector4 {
    /**
     * Realiza la adición de cuatro pares de puntos (P = P + Q) de forma atómica.
     */
    #[target_feature(enable = "avx2")]
    pub unsafe fn add_batch_unified(&mut self, other_points: &Self) {
        // 1. U1 = X1 * Z2^2 | U2 = X2 * Z1^2
        let z1_sq = self.z_coordinates.multiply_modular_vectorized(&self.z_coordinates);
        let z2_sq = other_points.z_coordinates.multiply_modular_vectorized(&other_points.z_coordinates);
        let u1 = self.x_coordinates.multiply_modular_vectorized(&z2_sq);
        let u2 = other_points.x_coordinates.multiply_modular_vectorized(&z1_sq);

        // 2. S1 = Y1 * Z2^3 | S2 = Y2 * Z1^3
        let s1 = self.y_coordinates.multiply_modular_vectorized(&other_points.z_coordinates).multiply_modular_vectorized(&z2_sq);
        let s2 = other_points.y_coordinates.multiply_modular_vectorized(&self.z_coordinates).multiply_modular_vectorized(&z1_sq);

        // 3. H = U2 - U1 | R = S2 - S1
        let h_diff = u2.subtract_modular_vectorized(&u1);
        let r_diff = s2.subtract_modular_vectorized(&s1);

        // 4. CÁLCULO DE COORDENADAS INTERMEDIAS
        let h_sq = h_diff.multiply_modular_vectorized(&h_diff);
        let h_cu = h_sq.multiply_modular_vectorized(&h_diff);
        let v_term = u1.multiply_modular_vectorized(&h_sq);

        // 5. RESULTADO X3 = R^2 - H^3 - 2V
        let r_sq = r_diff.multiply_modular_vectorized(&r_diff);
        let mut x3 = r_sq.subtract_modular_vectorized(&h_cu);
        let v2 = v_term.multiply_by_small_int_vectorized(2);
        x3 = x3.subtract_modular_vectorized(&v2);

        // 6. RESULTADO Y3 = R(V - X3) - S1*H^3
        let v_minus_x3 = v_term.subtract_modular_vectorized(&x3);
        let r_v_x3 = r_diff.multiply_modular_vectorized(&v_minus_x3);
        let s1_h3 = s1.multiply_modular_vectorized(&h_cu);
        let y3 = r_v_x3.subtract_modular_vectorized(&s1_h3);

        // 7. RESULTADO Z3 = H * Z1 * Z2
        let z3 = h_diff.multiply_modular_vectorized(&self.z_coordinates).multiply_modular_vectorized(&other_points.z_coordinates);

        // 8. PERSISTENCIA DE ESTADO
        self.x_coordinates = x3;
        self.y_coordinates = y3;
        self.z_coordinates = z3;
    }
}
