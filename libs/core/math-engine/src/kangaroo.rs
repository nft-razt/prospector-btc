// libs/core/math-engine/src/kangaroo.rs
use crate::arithmetic::{add_u256_be, sub_u256_be, u128_to_u256_be};
/**
 * =================================================================
 * APARATO: KANGAROO MATRIX SOLVER (V16.2)
 * RESPONSABILIDAD: RESOLUCIÓN PARALELA DE ECDLP
 * ESTRATEGIA: POLLARD'S LAMBDA + ASM ARITHMETIC
 * =================================================================
 */
use crate::errors::MathError;
use crate::private_key::SafePrivateKey;
use crate::public_key::SafePublicKey;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

pub struct KangarooConfig {
    pub start_scalar: [u8; 32],
    pub width: u64,
    pub dp_mask: u8,
    pub max_traps: usize,
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
        let hash_idx = (bytes[32] % 32) as usize;
        let entry = &table[hash_idx];

        self.pub_point = self.pub_point.add_scalar(&entry.scalar)?;
        let dist_jump = u128_to_u256_be(entry.distance);
        self.distance = add_u256_be(&self.distance, &dist_jump)?;

        Ok(())
    }

    #[inline(always)]
    fn is_distinguished(&self, mask: u8) -> bool {
        let bytes = self.pub_point.to_bytes(true);
        (bytes[32] & mask) == 0
    }
}

pub struct KangarooSolver;

impl KangarooSolver {
    pub fn solve(
        target: &SafePublicKey,
        config: &KangarooConfig,
    ) -> Result<Option<[u8; 32]>, MathError> {
        let mut table_data = [JumpTable {
            scalar: [0; 32],
            distance: 0,
        }; 32];
        for i in 0..32 {
            let val = 1u128 << (i / 2);
            table_data[i] = JumpTable {
                scalar: u128_to_u256_be(val),
                distance: val,
            };
        }
        let jump_table = Arc::new(table_data);

        let start_pk = SafePrivateKey::from_bytes(&config.start_scalar)?;
        let base_point = SafePublicKey::from_private(&start_pk);
        let width_bytes = u128_to_u256_be(config.width as u128);

        let tame_start = base_point.add_scalar(&width_bytes)?;
        let mut tame = Kangaroo::new(tame_start, width_bytes);
        let mut traps = HashMap::with_capacity(10000);
        let max_steps = (config.width as f64).sqrt() as usize * 4 + 5000;

        for _ in 0..max_steps {
            let _ = tame.jump(&jump_table);
            if tame.is_distinguished(config.dp_mask) {
                traps.insert(tame.pub_point.to_bytes(true), tame.distance);
                if traps.len() >= config.max_traps {
                    break;
                }
            }
        }

        let traps_arc = Arc::new(traps);
        let found_key = Arc::new(RwLock::new(None));
        let finished = Arc::new(AtomicBool::new(false));

        (0..rayon::current_num_threads())
            .into_par_iter()
            .for_each(|i| {
                if finished.load(Ordering::Relaxed) {
                    return;
                }
                let offset_bytes = u128_to_u256_be(i as u128);
                let wild_start = match target.add_scalar(&offset_bytes) {
                    Ok(p) => p,
                    Err(_) => return,
                };

                let mut wild = Kangaroo::new(wild_start, offset_bytes);
                for _ in 0..max_steps {
                    if finished.load(Ordering::Relaxed) {
                        break;
                    }
                    if wild.jump(&jump_table).is_err() {
                        break;
                    }

                    if wild.is_distinguished(config.dp_mask) {
                        if let Some(tame_dist) = traps_arc.get(&wild.pub_point.to_bytes(true)) {
                            if let Ok(term1) = add_u256_be(&width_bytes, tame_dist) {
                                if let Ok(delta) = sub_u256_be(&term1, &wild.distance) {
                                    if let Ok(final_priv) =
                                        add_u256_be(&config.start_scalar, &delta)
                                    {
                                        let mut lock = found_key.write().unwrap();
                                        *lock = Some(final_priv);
                                        finished.store(true, Ordering::Relaxed);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            });

        let res = *found_key.read().unwrap();
        Ok(res)
    }
}
