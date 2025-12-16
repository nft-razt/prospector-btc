// libs/core/math-engine/src/kangaroo.rs
// =================================================================
// APARATO: KANGAROO MATRIX SOLVER (PARALLEL HERD)
// RESPONSABILIDAD: RESOLUCIÓN PARALELA DE ECDLP EN INTERVALOS
// ALGORITMO: POLLARD'S LAMBDA (MÉTODO DE LOS CANGUROS)
// =================================================================

use crate::errors::MathError;
use crate::public_key::SafePublicKey;
use crate::arithmetic::{add_u256_be, sub_u256_be, u128_to_u256_be};
use crate::private_key::SafePrivateKey;
use std::collections::HashMap;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

/// Configuración operativa para el algoritmo del Canguro.
///
/// Define los límites del espacio de búsqueda y la heurística de memoria
/// para la detección de colisiones (Puntos Distinguidos).
pub struct KangarooConfig {
    /// Escalar inicial del rango de búsqueda ($a$).
    /// El rango efectivo es $[a, a + w]$.
    pub start_scalar: [u8; 32],

    /// Ancho del intervalo de búsqueda ($w$).
    /// Determina la complejidad temporal esperada $O(\sqrt{w})$.
    pub width: u64,

    /// Máscara de bits para la propiedad de "Punto Distinguido" (DP).
    ///
    /// Un punto $P$ es distinguido si $H(P) \& mask == 0$.
    /// - Máscara mayor (ej: `0xFF`) -> Menos puntos -> Menor uso de RAM.
    /// - Máscara menor (ej: `0x0F`) -> Más puntos -> Detección más rápida de colisiones.
    pub dp_mask: u8,

    /// Límite máximo de trampas (huellas) almacenadas en la tabla hash.
    /// Previene el agotamiento de memoria (OOM) en búsquedas largas.
    pub max_traps: usize,
}

impl Default for KangarooConfig {
    fn default() -> Self {
        Self {
            start_scalar: [0u8; 32],
            width: 1_000_000,
            dp_mask: 0x1F,
            max_traps: 500_000,
        }
    }
}

#[derive(Clone, Copy)]
struct JumpTable {
    scalar: [u8; 32],
    distance: u128,
}

#[derive(Clone)]
struct Kangaroo {
    pub_point: SafePublicKey,
    distance: [u8; 32],
}

impl Kangaroo {
    fn new(start_point: SafePublicKey, start_dist: [u8; 32]) -> Self {
        Self {
            pub_point: start_point,
            distance: start_dist,
        }
    }

    #[inline(always)]
    fn jump(&mut self, table: &[JumpTable; 32]) -> Result<(), MathError> {
        let bytes = self.pub_point.to_bytes(true);
        // Función hash determinista simple: byte[32] % 32
        let hash_idx = (bytes[32] % 32) as usize;
        let entry = &table[hash_idx];

        // Salto en la curva: P = P + table[i].G
        self.pub_point = self.pub_point.add_scalar(&entry.scalar)?;

        // Registro de distancia recorrida: d = d + table[i].dist
        let dist_jump = u128_to_u256_be(entry.distance);
        self.distance = add_u256_be(&self.distance, &dist_jump)?;

        Ok(())
    }

    #[inline(always)]
    fn is_distinguished(&self, mask: u8) -> bool {
        let bytes = self.pub_point.to_bytes(true);
        // Verificamos si los últimos bits son cero
        (bytes[32] & mask) == 0
    }
}

/// Solucionador paralelo del Logaritmo Discreto en Curva Elíptica (ECDLP).
///
/// Implementa la variante "Parallel Pollard's Lambda" utilizando una manada
/// de canguros "Wild" (Salvajes) persiguiendo a un canguro "Tame" (Doméstico).
pub struct KangarooSolver;

impl KangarooSolver {
    /// Intenta resolver $x$ tal que $Target = x \cdot G$ dentro del rango configurado.
    ///
    /// # Retorno
    /// * `Ok(Some(key))` - Clave privada encontrada ($x$).
    /// * `Ok(None)` - Rango agotado sin éxito.
    /// * `Err(MathError)` - Fallo crítico aritmético.
    pub fn solve(target: &SafePublicKey, config: &KangarooConfig) -> Result<Option<[u8; 32]>, MathError> {
        // 1. Pre-computación de la Tabla de Saltos (Potencias de 2)
        // Esto define el comportamiento determinista del paseo aleatorio.
        let mut table_data = [JumpTable { scalar: [0;32], distance: 0 }; 32];
        for i in 0..32 {
            let val = 1u128 << (i / 2);
            let buf = u128_to_u256_be(val);
            table_data[i] = JumpTable { scalar: buf, distance: val };
        }
        let jump_table = Arc::new(table_data);

        // 2. Fase Tame (Canguro Doméstico)
        // Empieza en el final del rango y salta hacia adelante dejando trampas.
        let start_pk = SafePrivateKey::from_bytes(&config.start_scalar)?;
        let base_point = SafePublicKey::from_private(&start_pk);
        let width_bytes = u128_to_u256_be(config.width as u128);

        // Tame empieza en: Base + Width
        let tame_start = base_point.add_scalar(&width_bytes)?;
        let mut tame = Kangaroo::new(tame_start, width_bytes);

        let mut traps = HashMap::with_capacity(10000);
        // Heurística: 4 * sqrt(w) pasos promedio
        let max_steps = (config.width as f64).sqrt() as usize * 4 + 5000;

        for _ in 0..max_steps {
            tame.jump(&jump_table)?;
            if tame.is_distinguished(config.dp_mask) {
                traps.insert(tame.pub_point.to_bytes(true), tame.distance);
                if traps.len() >= config.max_traps { break; }
            }
        }

        let traps = Arc::new(traps);
        let found_key = Arc::new(RwLock::new(None));
        let finished = Arc::new(AtomicBool::new(false));

        // 3. Fase Wild (Manada Salvaje Paralela)
        // Lanza múltiples hilos, cada uno con un offset diferente desde el Target.
        let num_threads = rayon::current_num_threads();

        (0..num_threads).into_par_iter().for_each(|i| {
            if finished.load(Ordering::Relaxed) { return; }

            // Wild[i] empieza en: Target + offset[i]
            let offset_bytes = u128_to_u256_be(i as u128);
            let wild_start = match target.add_scalar(&offset_bytes) {
                Ok(p) => p,
                Err(_) => return,
            };

            let mut wild = Kangaroo::new(wild_start, offset_bytes);

            for _ in 0..max_steps {
                if finished.load(Ordering::Relaxed) { break; }
                if wild.jump(&jump_table).is_err() { break; }

                if wild.is_distinguished(config.dp_mask) {
                    let wild_key = wild.pub_point.to_bytes(true);

                    // ¿Cayó en una trampa del Tame?
                    if let Some(tame_dist) = traps.get(&wild_key) {
                        // Colisión encontrada. Resolvemos la ecuación:
                        // x = start + width + dist_tame - dist_wild - offset

                        let term1 = match add_u256_be(&width_bytes, tame_dist) {
                            Ok(v) => v,
                            Err(_) => continue
                        };
                        let term2 = wild.distance;

                        if let Ok(delta) = sub_u256_be(&term1, &term2) {
                             if let Ok(final_priv) = add_u256_be(&config.start_scalar, &delta) {
                                 let mut lock = found_key.write().unwrap();
                                 *lock = Some(final_priv);
                                 finished.store(true, Ordering::Relaxed);
                                 return;
                             }
                        }
                    }
                }
            }
        });

        let result = *found_key.read().unwrap();
        Ok(result)
    }
}
