// libs/core/math-engine/src/curve.rs
/**
 * =================================================================
 * APARATO: STANDARD JACOBIAN CURVE ENGINE (V114.0 - FIELD SYNC)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: ADICIÓN DE PUNTOS SECP256K1 (P + G)
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la fórmula de adición proyectiva estándar (Chudnovsky).
 * Actualizado para consumir el FieldElement V112.0 con aritmética u128.
 * =================================================================
 */

use crate::point::JacobianPoint;

pub struct UnifiedCurveEngine;

impl UnifiedCurveEngine {
    /// Realiza la adición de dos puntos en coordenadas Jacobianas.
    /// P3 = P1 + P2
    /// Costo: 12 Multiplicaciones + 4 Cuadrados.
    #[inline(always)]
    pub fn add_points_unified(
        p1: &JacobianPoint,
        p2: &JacobianPoint,
    ) -> JacobianPoint {
        // 1. MANEJO DE ELEMENTO NEUTRO (Identidad aditiva)
        if p1.is_infinity { return *p2; }
        if p2.is_infinity { return *p1; }

        let x1 = &p1.x;
        let y1 = &p1.y;
        let z1 = &p1.z;

        let x2 = &p2.x;
        let y2 = &p2.y;
        let z2 = &p2.z;

        // Formula: Cohen/Chudnovsky (a=0 for secp256k1)
        // U1 = X1 * Z2^2
        let z2_sq = z2.square_modular();
        let u1 = x1.multiply_modular(&z2_sq);

        // U2 = X2 * Z1^2
        let z1_sq = z1.square_modular();
        let u2 = x2.multiply_modular(&z1_sq);

        // S1 = Y1 * Z2^3
        let z2_cu = z2.multiply_modular(&z2_sq);
        let s1 = y1.multiply_modular(&z2_cu);

        // S2 = Y2 * Z1^3
        let z1_cu = z1.multiply_modular(&z1_sq);
        let s2 = y2.multiply_modular(&z1_cu);

        // H = U2 - U1
        let h = u2.subtract_modular(&u1);

        // R = S2 - S1
        let r = s2.subtract_modular(&s1);

        // 2. DETECCIÓN DE PUNTOS IDÉNTICOS (P1 == P2)
        // Si H == 0 y R == 0, los puntos son iguales -> Usar duplicación.
        // Si H == 0 y R != 0, los puntos son opuestos -> Retornar Infinito.
        if h.is_zero() {
            if r.is_zero() {
                return Self::double_point(p1);
            } else {
                return JacobianPoint::infinity();
            }
        }

        // 3. CÁLCULO FINAL DE COORDENADAS
        // Z3 = Z1 * Z2 * H
        let z1_z2 = z1.multiply_modular(z2);
        let z3 = z1_z2.multiply_modular(&h);

        // X3 = R^2 - H^3 - 2*U1*H^2
        let r_sq = r.square_modular();
        let h_sq = h.square_modular();
        let h_cu = h.multiply_modular(&h_sq);
        let u1_h2 = u1.multiply_modular(&h_sq);

        let two_u1_h2 = u1_h2.add_modular(&u1_h2); // x2 es más barato que mul(2)

        // T = R^2 - H^3
        let t = r_sq.subtract_modular(&h_cu);
        let x3 = t.subtract_modular(&two_u1_h2);

        // Y3 = R * (U1*H^2 - X3) - S1*H^3
        let term_diff = u1_h2.subtract_modular(&x3);
        let term_r = r.multiply_modular(&term_diff);
        let s1_h3 = s1.multiply_modular(&h_cu);

        let y3 = term_r.subtract_modular(&s1_h3);

        JacobianPoint {
            x: x3,
            y: y3,
            z: z3,
            is_infinity: false,
        }
    }

    /// Duplicación optimizada para y^2 = x^3 + 7.
    /// P3 = 2 * P1
    #[inline(always)]
    pub fn double_point(p: &JacobianPoint) -> JacobianPoint {
        if p.is_infinity { return *p; }

        let x = &p.x;
        let y = &p.y;
        let z = &p.z;

        // S = 4 * X * Y^2
        let y_sq = y.square_modular();
        let x_y2 = x.multiply_modular(&y_sq);
        let s_2 = x_y2.add_modular(&x_y2);
        let s = s_2.add_modular(&s_2);

        // M = 3 * X^2 (para a=0)
        let x_sq = x.square_modular();
        let m_2 = x_sq.add_modular(&x_sq);
        let m = m_2.add_modular(&x_sq);

        // X3 = M^2 - 2*S
        let m_sq = m.square_modular();
        let s_2_final = s.add_modular(&s);
        let x3 = m_sq.subtract_modular(&s_2_final);

        // Z3 = 2 * Y * Z
        let yz = y.multiply_modular(z);
        let z3 = yz.add_modular(&yz);

        // Y3 = M * (S - X3) - 8 * Y^4
        let y4 = y_sq.square_modular(); // Y^4
        let y4_2 = y4.add_modular(&y4);
        let y4_4 = y4_2.add_modular(&y4_2);
        let y4_8 = y4_4.add_modular(&y4_4); // 8 * Y^4

        let diff_s_x3 = s.subtract_modular(&x3);
        let term_m = m.multiply_modular(&diff_s_x3);
        let y3 = term_m.subtract_modular(&y4_8);

        JacobianPoint {
            x: x3,
            y: y3,
            z: z3,
            is_infinity: false,
        }
    }
}
