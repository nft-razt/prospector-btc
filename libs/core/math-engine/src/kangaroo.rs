// libs/core/math-engine/src/kangaroo.rs
// =================================================================
// APARATO: POLLARD'S KANGAROO (LAMBDA ALGORITHM)
// RESPONSABILIDAD: RESOLUCIÓN DE LOGARITMO DISCRETO (ECDLP)
// ALCANCE: BÚSQUEDA DE RANGO CORTO (Short Range Search)
// COMPLEJIDAD: O(sqrt(w)) - Ultrarrápido para rangos < 2^50
// =================================================================

use crate::errors::MathError;
use crate::public_key::SafePublicKey;
use crate::arithmetic::add_u128_to_u256_be; // ✅ Uso del Átomo Aritmético
use std::collections::HashMap;

/// Configuración operativa del Canguro.
pub struct KangarooConfig {
    /// Límite inferior del rango de búsqueda (Private Key start).
    /// Representado como bytes BigEndian (32 bytes).
    pub start_scalar: [u8; 32],

    /// Ancho del intervalo de búsqueda (Width).
    /// Define el esfuerzo máximo: O(sqrt(width)).
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
            dp_mask: 0x1F, // Default conservador (Balanceado)
        }
    }
}

/// Estado de un Canguro individual (Tame o Wild).
#[derive(Clone)]
struct Kangaroo {
    /// Clave Pública actual (Posición en la curva).
    pub_point: SafePublicKey,
    /// Distancia acumulada recorrida (Escalar).
    distance: u128,
}

impl Kangaroo {
    /// Inicializa un canguro en un punto conocido.
    fn new(start_point: SafePublicKey, start_distance: u128) -> Self {
        Self {
            pub_point: start_point,
            distance: start_distance,
        }
    }

    /// Función de Paso Determinista (The Jump).
    /// Decide el tamaño del salto basado en la entropía del punto actual.
    /// $f(P) = 2^{P.x \mod k}$
    ///
    /// Esto asegura que si dos canguros caen en el mismo punto, seguirán el mismo camino.
    #[inline]
    fn jump(&mut self, jump_table: &[u128; 32], scalar_table: &[[u8; 32]; 32]) -> Result<(), MathError> {
        // Usamos el byte menos significativo de la representación comprimida para velocidad.
        // Esto actúa como una función hash rápida y determinista.
        let bytes = self.pub_point.to_bytes(true);
        // Usamos el byte en índice 32 (el último de 33 bytes) como fuente de entropía
        let hash_idx = (bytes[32] % 32) as usize;

        // Actualizamos la distancia escalar
        let jump_dist = jump_table[hash_idx];
        self.distance = self.distance.wrapping_add(jump_dist);

        // Actualizamos el punto en la curva (P' = P + jump * G)
        self.pub_point = self.pub_point.add_scalar(&scalar_table[hash_idx])?;

        Ok(())
    }

    /// Determina si el punto actual es un "Punto Distinguido" usando la máscara.
    /// Un punto es distinguido si sus N bits bajos son cero.
    #[inline]
    fn is_distinguished(&self, mask: u8) -> bool {
        let bytes = self.pub_point.to_bytes(true);
        (bytes[32] & mask) == 0
    }
}

/// Motor de Resolución Canguro.
pub struct KangarooSolver;

impl KangarooSolver {
    /// Intenta resolver la clave privada de `target` sabiendo que está cerca de `base`.
    ///
    /// # Retorno
    /// * `Ok(Some(private_key_bytes))` - Clave encontrada.
    /// * `Ok(None)` - Clave no encontrada en el rango especificado (Agotado).
    /// * `Err` - Error matemático crítico.
    pub fn solve(target: &SafePublicKey, config: &KangarooConfig) -> Result<Option<[u8; 32]>, MathError> {
        // 1. PRE-COMPUTACIÓN DE TABLAS (Optimización CPU)
        // Generamos potencias de 2 para los saltos: 2^0, 2^1 ... 2^31
        // Esto permite saltos variados para evitar ciclos cortos.
        let mut jump_table = [0u128; 32];
        let mut scalar_table = [[0u8; 32]; 32];

        for i in 0..32 {
            let val = 1u128 << (i / 2); // Saltos: 1, 1, 2, 2, 4, 4... para convergencia
            jump_table[i] = val;

            // Convertir u128 a [u8; 32] BigEndian para la suma de puntos
            let mut buf = [0u8; 32];
            buf[16..32].copy_from_slice(&val.to_be_bytes());
            scalar_table[i] = buf;
        }

        // 2. CONFIGURACIÓN DEL CANGURO DOMESTICADO (TAME)
        // Empieza al final del rango conocido.
        // $P_{tame} = P_{base} + width * G$
        // Sabemos la clave privada relativa (distance = width).
        let start_pk = crate::private_key::SafePrivateKey::from_bytes(&config.start_scalar)?;
        let base_point = SafePublicKey::from_private(&start_pk);

        let width_u128 = config.width as u128;

        // Calculamos el offset escalar para el punto de inicio Tame
        let tame_offset_scalar = add_u128_to_u256_be(&[0u8; 32], width_u128)?;
        let tame_start_point = base_point.add_scalar(&tame_offset_scalar)?;

        let mut tame = Kangaroo::new(tame_start_point, width_u128);

        // 3. CONFIGURACIÓN DEL CANGURO SALVAJE (WILD)
        // Empieza en el punto objetivo desconocido.
        // $P_{wild} = Target$
        // Su distancia relativa inicial es 0 (offset desconocido x).
        let mut wild = Kangaroo::new(*target, 0);

        // Map para guardar las trampas del Tame: (CompressedPubKey -> Distance)
        // Capacidad inicial ajustada heurísticamente.
        let mut traps: HashMap<Vec<u8>, u128> = HashMap::with_capacity(10000);

        // Límite de iteraciones de seguridad (aprox 4 * sqrt(width))
        let max_steps = (config.width as f64).sqrt() as usize * 4 + 2000;

        // 4. FASE DE COLOCACIÓN DE TRAMPAS (TAME RUN)
        for _ in 0..max_steps {
            tame.jump(&jump_table, &scalar_table)?;

            if tame.is_distinguished(config.dp_mask) {
                traps.insert(tame.pub_point.to_bytes(true), tame.distance);
            }

            // Si salta demasiado lejos del rango, paramos.
            if tame.distance > width_u128 + config.width as u128 {
                break;
            }
        }

        // 5. FASE DE CAZA (WILD RUN)
        for _ in 0..max_steps {
            wild.jump(&jump_table, &scalar_table)?;

            if wild.is_distinguished(config.dp_mask) {
                let wild_key = wild.pub_point.to_bytes(true);

                // VERIFICAR COLISIÓN
                if let Some(tame_dist) = traps.get(&wild_key) {
                    // ¡COLISIÓN ENCONTRADA!
                    // Ecuación: Target + d_wild * G = Base + d_tame * G
                    // Target = Base + (d_tame - d_wild) * G
                    // x = start + (d_tame - d_wild)

                    if *tame_dist > wild.distance {
                        let diff = tame_dist - wild.distance;

                        // Reconstruir la clave privada final usando el Átomo Aritmético
                        // priv = start_scalar + diff
                        let recovered_key = add_u128_to_u256_be(&config.start_scalar, diff)?;
                        return Ok(Some(recovered_key));
                    }
                }
            }

            // Si el Wild salta más que el Tame máximo posible, falló.
            if wild.distance > width_u128 + config.width as u128 {
                break;
            }
        }

        Ok(None) // No se encontró en este rango
    }
}
