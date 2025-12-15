// libs/core/probabilistic/src/sharded.rs
// =================================================================
// APARATO: SHARDED BLOOM FILTER MANAGER
// RESPONSABILIDAD: ORQUESTACIÓN DE MÚLTIPLES FILTROS (PARTICIONAMIENTO)
// ESTRATEGIA: DETERMINISTIC ROUTING (Address -> Shard Index)
// =================================================================

use crate::errors::FilterError;
use crate::filter_wrapper::RichListFilter;
use rayon::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path; // Paralelismo para carga/guardado

/// Contenedor de filtros particionados.
/// Permite manejar un dataset gigante fragmentado en archivos más pequeños.
pub struct ShardedFilter {
    shards: Vec<RichListFilter>,
    shard_count: usize,
}

impl ShardedFilter {
    /// Inicializa un nuevo sistema particionado en memoria.
    ///
    /// # Argumentos
    /// * `shard_count`: Número de particiones (Recomendado: 4 u 8 para paralelismo de descarga).
    /// * `total_expected_items`: Total de direcciones estimadas.
    /// * `fp_rate`: Tasa de falsos positivos deseada globalmente.
    pub fn new(shard_count: usize, total_expected_items: usize, fp_rate: f64) -> Self {
        let items_per_shard = (total_expected_items / shard_count) + 1;

        // Inicializamos N filtros vacíos
        let mut shards = Vec::with_capacity(shard_count);
        for _ in 0..shard_count {
            shards.push(RichListFilter::new(items_per_shard, fp_rate));
        }

        Self {
            shards,
            shard_count,
        }
    }

    /// Determina el índice del shard para una dirección dada.
    /// Usa un hash rápido no criptográfico para distribución uniforme.
    #[inline(always)]
    fn get_shard_index(&self, address: &str) -> usize {
        let mut s = DefaultHasher::new();
        address.hash(&mut s);
        (s.finish() as usize) % self.shard_count
    }

    /// Inserta una dirección en el shard correspondiente.
    pub fn add(&mut self, address: &str) {
        let idx = self.get_shard_index(address);
        // SAFETY: get_shard_index garantiza módulo shard_count
        if let Some(filter) = self.shards.get_mut(idx) {
            filter.add(address);
        }
    }

    /// Verifica si una dirección existe (enrutando al shard correcto).
    /// O(1) - Solo verifica 1 filtro, no todos.
    pub fn contains(&self, address: &str) -> bool {
        let idx = self.get_shard_index(address);
        if let Some(filter) = self.shards.get(idx) {
            filter.contains(address)
        } else {
            false
        }
    }

    /// Guarda los shards en disco usando un patrón de nombres predecible.
    /// Ej: output_dir/filter_shard_0.bin, filter_shard_1.bin...
    pub fn save_to_dir<P: AsRef<Path>>(&self, dir: P) -> Result<(), FilterError> {
        let dir = dir.as_ref();
        if !dir.exists() {
            std::fs::create_dir_all(dir)?;
        }

        // Escritura paralela usando Rayon para velocidad extrema en SSDs
        self.shards
            .par_iter()
            .enumerate()
            .try_for_each(|(idx, filter)| {
                let filename = format!("filter_shard_{}.bin", idx);
                let path = dir.join(filename);
                filter.save_to_file(&path)
            })?;

        Ok(())
    }

    /// Carga shards desde un directorio.
    ///
    /// # Estrategia Híbrida
    /// Intenta usar `mmap` para carga instantánea. Si falla, usa fallback de buffer.
    pub fn load_from_dir<P: AsRef<Path>>(dir: P, count: usize) -> Result<Self, FilterError> {
        let dir = dir.as_ref();

        // Carga paralela
        let shards = (0..count)
            .into_par_iter()
            .map(|idx| {
                let filename = format!("filter_shard_{}.bin", idx);
                let path = dir.join(filename);

                // Intento prioritario: Memory Map
                RichListFilter::load_from_file_mmap(&path)
                    .or_else(|_| RichListFilter::load_from_file(&path))
            })
            .collect::<Result<Vec<RichListFilter>, FilterError>>()?;

        Ok(Self {
            shards,
            shard_count: count,
        })
    }

    /// Retorna el conteo total de elementos indexados sumando todos los shards.
    pub fn total_count(&self) -> usize {
        self.shards.iter().map(|f| f.count()).sum()
    }
}
