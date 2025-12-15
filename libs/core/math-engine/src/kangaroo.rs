// libs/core/math-engine/src/kangaroo.rs
// =================================================================
// APARATO: POLLARD'S KANGAROO (LAMBDA ALGORITHM)
// RESPONSABILIDAD: RESOLUCIÓN DE LOGARITMO DISCRETO (ECDLP)
// ALCANCE: BÚSQUEDA DE RANGO CORTO (Short Range Search)
// COMPLEJIDAD: O(sqrt(w))
// =================================================================

use crate::errors::MathError;
use crate::public_key::SafePublicKey;
use crate::arithmetic::add_u128_to_u256_be; // ✅ Importación del nuevo átomo
use std::collections::HashMap;

/// Configuración operativa del Canguro.
pub struct KangarooConfig {
    /// Límite inferior del rango de búsqueda (Private Key start).
    /// Representado como bytes BigEndian (32 bytes).
    pub start_scalar: [u8; 32],

    /// Ancho del intervalo de búsqueda (Width).
    pub width: u64,

    /// Máscara para Puntos Distinguidos.
    /// Determina la frecuencia de almacenamiento en RAM.
    /// Ej: 0x1F (binario 11111) -> 1 de cada 32 puntos se guarda.
    /// Mayor máscara = Menos RAM, más CPU (pasos extra).
    pub dp_mask: u8,
}

impl Default for KangarooConfig {
    fn default() -> Self {
        Self {
            start_scalar: [0u8; 32],
            width: 1_000_000,
            dp_mask: 0x1F, // Default conservador
        }
    }
}

/// Estado de un Canguro individual.
#[derive(Clone)]
struct Kangaroo {
    pub_point: SafePublicKey,
    distance: u128,
}

impl Kangaroo {
    fn new(start_point: SafePublicKey, start_distance: u128) -> Self {
        Self {
            pub_point: start_point,
            distance: start_distance,
        }
    }

    /// Función de Paso Determinista (The Jump).
    #[inline]
    fn jump(&mut self, jump_table: &[u128; 32], scalar_table: &[[u8; 32]; 32]) -> Result<(), MathError> {
        // Hash rápido: Byte LSB de la clave comprimida
        let bytes = self.pub_point.to_bytes(true);
        // Usamos el byte en índice 32 (el último de 33 bytes)
        let hash_idx = (bytes[32] % 32) as usize;

        let jump_dist = jump_table[hash_idx];
        self.distance = self.distance.wrapping_add(jump_dist);

        // Aritmética de Curva: P' = P + jump * G
        self.pub_point = self.pub_point.add_scalar(&scalar_table[hash_idx])?;

        Ok(())
    }

    /// Determina si es un Punto Distinguido usando la máscara configurada.
    #[inline]
    fn is_distinguished(&self, mask: u8) -> bool {
        let bytes = self.pub_point.to_bytes(true);
        (bytes[32] & mask) == 0
    }
}

pub struct KangarooSolver;

impl KangarooSolver {
    pub fn solve(target: &SafePublicKey, config: &KangarooConfig) -> Result<Option<[u8; 32]>, MathError> {
        // 1. PRE-COMPUTACIÓN (Tabla de Potencias de 2)
        // Usamos potencias de 2 para maximizar la entropía de los saltos
        // sin requerir números primos complejos.
        let mut jump_table = [0u128; 32];
        let mut scalar_table = [[0u8; 32]; 32];

        for i in 0..32 {
            let val = 1u128 << (i / 2); // Saltos: 1, 1, 2, 2, 4, 4... para convergencia suave
            jump_table[i] = val;

            // Convertir u128 a [u8; 32] para la suma de puntos
            // Esto usa nuestro nuevo motor aritmético indirectamente (via to_be_bytes)
            let mut buf = [0u8; 32];
            buf[16..32].copy_from_slice(&val.to_be_bytes());
            scalar_table[i] = buf;
        }

        // 2. SETUP TAME (Canguro Domesticado)
        let start_pk = crate::private_key::SafePrivateKey::from_bytes(&config.start_scalar)?;
        let base_point = SafePublicKey::from_private(&start_pk);

        let width_u128 = config.width as u128;

        // Punto inicial del Tame = Base + Width
        let tame_offset_scalar = add_u128_to_u256_be(&[0u8; 32], width_u128)?;
        let tame_start_point = base_point.add_scalar(&tame_offset_scalar)?;

        let mut tame = Kangaroo::new(tame_start_point, width_u128);

        // 3. SETUP WILD (Canguro Salvaje)
        let mut wild = Kangaroo::new(*target, 0);

        // Mapa de Trampas: CompressedPubKey -> Distance
        let mut traps: HashMap<Vec<u8>, u128> = HashMap::with_capacity(10000);

        // Heurística de Límite: 4 * sqrt(width) es el promedio estadístico
        let max_steps = (config.width as f64).sqrt() as usize * 4 + 2000;

        // 4. FASE DE COLOCACIÓN DE TRAMPAS
        for _ in 0..max_steps {
            tame.jump(&jump_table, &scalar_table)?;

            if tame.is_distinguished(config.dp_mask) {
                traps.insert(tame.pub_point.to_bytes(true), tame.distance);
            }

            // Abortar si se aleja demasiado
            if tame.distance > width_u128 + config.width as u128 {
                break;
            }
        }

        // 5. FASE DE CAZA
        for _ in 0..max_steps {
            wild.jump(&jump_table, &scalar_table)?;

            if wild.is_distinguished(config.dp_mask) {
                let wild_key = wild.pub_point.to_bytes(true);

                if let Some(tame_dist) = traps.get(&wild_key) {
                    // COLISIÓN CONFIRMADA
                    if *tame_dist > wild.distance {
                        let diff = tame_dist - wild.distance;
                        // Recuperación: key = start + (tame_dist - wild_dist)
                        return Ok(Some(add_u128_to_u256_be(&config.start_scalar, diff)?));
                    }
                }
            }

            if wild.distance > width_u128 + config.width as u128 {
                break;
            }
        }

        Ok(None)
    }
}
