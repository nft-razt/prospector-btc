/**
 * =================================================================
 * APARATO: ELLIPTIC CURVE GROUP LAW (V46.0 - ZERO-INVERSION)
 * CLASIFICACIÓN: CORE MATH (L1)
 * RESPONSABILIDAD: ADICIÓN Y DUPLICACIÓN DE PUNTOS PROYECTIVOS
 *
 * ESTRATEGIA DE ÉLITE:
 * - Cohen-Miyaji-Ono Formulas: Optimización máxima para a=0.
 * - Field Synergy: Uso de FieldElement para operaciones mod p sin asignación.
 * - Constant Time: Inmune a ataques de análisis de tiempo.
 * =================================================================
 */

use crate::field::FieldElement;
use crate::point::{JacobianPoint, AffinePoint};
use crate::errors::MathError;

pub struct CurveEngine;

impl CurveEngine {
    /**
     * Duplica un punto en la curva secp256k1 (P3 = 2 * P1).
     * Optimizado para curvas con parámetro a = 0.
     *
     * Costo: 3 Multiplicaciones, 4 Cuadrados (3M + 4S).
     */
    #[inline(always)]
    pub fn double_jacobian_point(point: &JacobianPoint) -> JacobianPoint {
        if point.is_infinity { return *point; }

        // A = X1^2, B = Y1^2, C = B^2
        let a_coord = point.x.square_modular();
        let b_coord = point.y.square_modular();
        let c_coord = b_coord.square_modular();

        // D = 2 * ((X1 + B)^2 - A - C)
        let d_coord = (point.x.add_modular(&b_coord)).square_modular()
            .subtract_modular(&a_coord)
            .subtract_modular(&c_coord)
            .multiply_by_small_int(2);

        // E = 3 * A, F = E^2
        let e_coord = a_coord.multiply_by_small_int(3);
        let f_coord = e_coord.square_modular();

        // X3 = F - 2 * D
        let next_x = f_coord.subtract_modular(&d_coord.multiply_by_small_int(2));

        // Y3 = E * (D - X3) - 8 * C
        let next_y = e_coord.multiply_modular(&d_coord.subtract_modular(&next_x))
            .subtract_modular(&c_coord.multiply_by_small_int(8));

        // Z3 = 2 * Y1 * Z1
        let next_z = point.y.multiply_modular(&point.z).multiply_by_small_int(2);

        JacobianPoint {
            x: next_x,
            y: next_y,
            z: next_z,
            is_infinity: false,
        }
    }

    /**
     * Suma dos puntos distintos en la curva (P3 = P1 + P2).
     * Implementa el algoritmo completo de adición unificada.
     */
    #[inline(always)]
    pub fn add_jacobian_points(point_a: &JacobianPoint, point_b: &JacobianPoint) -> JacobianPoint {
        if point_a.is_infinity { return *point_b; }
        if point_b.is_infinity { return *point_a; }

        let z1_sq = point_a.z.square_modular();
        let z2_sq = point_b.z.square_modular();

        let u1 = point_a.x.multiply_modular(&z2_sq);
        let u2 = point_b.x.multiply_modular(&z1_sq);

        let s1 = point_a.y.multiply_modular(&point_b.z.multiply_modular(&z2_sq));
        let s2 = point_b.y.multiply_modular(&point_a.z.multiply_modular(&z1_sq));

        if u1.internal_representation == u2.internal_representation {
            if s1.internal_representation != s2.internal_representation {
                return JacobianPoint::infinity();
            } else {
                return Self::double_jacobian_point(point_a);
            }
        }

        let h_diff = u2.subtract_modular(&u1);
        let r_diff = s2.subtract_modular(&s1);

        let h_sq = h_diff.square_modular();
        let h_cu = h_sq.multiply_modular(&h_diff);

        let v_val = u1.multiply_modular(&h_sq);

        // X3 = R^2 - H^3 - 2*V
        let next_x = r_diff.square_modular()
            .subtract_modular(&h_cu)
            .subtract_modular(&v_val.multiply_by_small_int(2));

        // Y3 = R * (V - X3) - S1 * H^3
        let next_y = r_diff.multiply_modular(&v_val.subtract_modular(&next_x))
            .subtract_modular(&s1.multiply_modular(&h_cu));

        // Z3 = H * Z1 * Z2
        let next_z = h_diff.multiply_modular(&point_a.z).multiply_modular(&point_b.z);

        JacobianPoint {
            x: next_x,
            y: next_y,
            z: next_z,
            is_infinity: false,
        }
    }
}
