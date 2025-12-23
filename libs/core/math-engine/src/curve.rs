/**
 * =================================================================
 * APARATO: STANDARD JACOBIAN CURVE ENGINE (V120.5 - ELITE EDITION)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: LEYES DE GRUPO UNIFICADAS PARA SECP256K1
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa las leyes de adición en el espacio proyectivo Jacobiano.
 * Esta versión está específicamente diseñada para maximizar el hashrate
 * eliminando la división modular (inverso) del bucle caliente.
 *
 * # Mathematical Proof:
 * Utiliza las fórmulas de Cohen-Miyaji-Ono para la suma de puntos.
 * La relación entre coordenadas proyectivas y afines es:
 * x = X / Z^2
 * y = Y / Z^3
 *
 * Performance:
 * La adición mixta (Jacobiano + Afín) reduce el costo de 12M a 8M
 * (donde M es una multiplicación modular).
 * =================================================================
 */

use crate::prelude::*;

pub struct UnifiedCurveEngine;

impl UnifiedCurveEngine {
    /**
     * Realiza la adición de un punto en coordenadas Jacobianas y un punto en Afines.
     * P_resultante = Punto_Jacobiano + Punto_Afin(x, y, Z=1)
     *
     * # Arguments:
     * * `point_alpha_jacobian` - El punto acumulador actual en la curva.
     * * `point_beta_affine_x` - Coordenada X del punto a sumar (Z es implícitamente 1).
     * * `point_beta_affine_y` - Coordenada Y del punto a sumar.
     *
     * # Errors:
     * Gestiona casos excepcionales como el punto al infinito o duplicación técnica.
     */
    #[inline(always)]
    pub fn add_mixed_deterministic(
        point_alpha_jacobian: &JacobianPoint,
        point_beta_affine_x: &FieldElement,
        point_beta_affine_y: &FieldElement,
    ) -> JacobianPoint {
        // 1. GESTIÓN DE ELEMENTO NEUTRO (Punto al Infinito)
        if point_alpha_jacobian.is_infinity {
            return JacobianPoint::from_affine(
                point_beta_affine_x.internal_words,
                point_beta_affine_y.internal_words
            );
        }

        // 2. DERIVACIÓN DE COMPONENTES DE CAMPO
        // U2 = x2 * Z1^2
        let coordinate_z1_squared = point_alpha_jacobian.z.square_modular();
        let term_u2 = point_beta_affine_x.multiply_modular(&coordinate_z1_squared);

        // S2 = y2 * Z1^3
        let coordinate_z1_cubed = point_alpha_jacobian.z.multiply_modular(&coordinate_z1_squared);
        let term_s2 = point_beta_affine_y.multiply_modular(&coordinate_z1_cubed);

        // 3. CÁLCULO DE DIFERENCIAS (H y R)
        // H = U2 - X1
        let term_h = term_u2.subtract_modular(&point_alpha_jacobian.x);
        // R = S2 - Y1
        let term_r = term_s2.subtract_modular(&point_alpha_jacobian.y);

        // 4. VALIDACIÓN DE CASOS EXCEPCIONALES
        if term_h.is_zero() {
            if term_r.is_zero() {
                // Los puntos son idénticos en el plano afín, se procede a duplicación.
                return Self::double_point_jacobian(point_alpha_jacobian);
            } else {
                // Los puntos son inversos, el resultado es el elemento neutro.
                return JacobianPoint::infinity();
            }
        }

        // 5. CÁLCULO DE COORDENADAS RESULTANTES (X3, Y3, Z3)
        let term_h_squared = term_h.square_modular();
        let term_h_cubed = term_h.multiply_modular(&term_h_squared);
        // V = X1 * H^2
        let term_v = point_alpha_jacobian.x.multiply_modular(&term_h_squared);

        // Z3 = Z1 * H
        let coordinate_z_3 = point_alpha_jacobian.z.multiply_modular(&term_h);

        // X3 = R^2 - H^3 - 2V
        let term_r_squared = term_r.square_modular();
        let term_two_v = term_v.add_modular(&term_v);
        let coordinate_x_3 = term_r_squared
            .subtract_modular(&term_h_cubed)
            .subtract_modular(&term_two_v);

        // Y3 = R * (V - X3) - Y1 * H^3
        let term_v_minus_x3 = term_v.subtract_modular(&coordinate_x_3);
        let term_r_v_x3 = term_r.multiply_modular(&term_v_minus_x3);
        let term_y1_h3 = point_alpha_jacobian.y.multiply_modular(&term_h_cubed);
        let coordinate_y_3 = term_r_v_x3.subtract_modular(&term_y1_h3);

        JacobianPoint {
            x: coordinate_x_3,
            y: coordinate_y_3,
            z: coordinate_z_3,
            is_infinity: false,
        }
    }

    /**
     * Implementa la duplicación de un punto en coordenadas Jacobianas (P = 2P).
     * Optimizado para curvas de Weierstrass con a = 0 (secp256k1).
     *
     * Costo: 3 Multiplicaciones (M) + 4 Cuadrados (S).
     */
    #[inline(always)]
    pub fn double_point_jacobian(point: &JacobianPoint) -> JacobianPoint {
        if point.is_infinity || point.y.is_zero() {
            return JacobianPoint::infinity();
        }

        // term_m = 3 * X^2
        let coordinate_x_squared = point.x.square_modular();
        let term_m = coordinate_x_squared
            .add_modular(&coordinate_x_squared)
            .add_modular(&coordinate_x_squared);

        // term_s = 4 * X * Y^2
        let coordinate_y_squared = point.y.square_modular();
        let term_x_y2 = point.x.multiply_modular(&coordinate_y_squared);
        let term_s = term_x_y2
            .add_modular(&term_x_y2)
            .add_modular(&term_x_y2)
            .add_modular(&term_x_y2);

        // X3 = M^2 - 2*S
        let term_m_squared = term_m.square_modular();
        let term_two_s = term_s.add_modular(&term_s);
        let coordinate_x_3 = term_m_squared.subtract_modular(&term_two_s);

        // Z3 = 2 * Y * Z
        let term_y_z = point.y.multiply_modular(&point.z);
        let coordinate_z_3 = term_y_z.add_modular(&term_y_z);

        // Y3 = M * (S - X3) - 8 * Y^4
        let coordinate_y_fourth = coordinate_y_squared.square_modular();
        let term_eight_y4 = coordinate_y_fourth.multiply_by_u64(8);
        let term_s_minus_x3 = term_s.subtract_modular(&coordinate_x_3);
        let coordinate_y_3 = term_m
            .multiply_modular(&term_s_minus_x3)
            .subtract_modular(&term_eight_y4);

        JacobianPoint {
            x: coordinate_x_3,
            y: coordinate_y_3,
            z: coordinate_z_3,
            is_infinity: false,
        }
    }
}
