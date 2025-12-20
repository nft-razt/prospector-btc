/**
 * =================================================================
 * APARATO: SHARDED PROBABILISTIC ORCHESTRATOR (V35.1 - HARDENED)
 * CLASIFICACIÓN: CORE INFRASTRUCTURE (ESTRATO L1)
 * RESPONSABILIDAD: GESTIÓN DE FILTROS DE BLOOM PARTICIONADOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el motor de fragmentación táctica del censo UTXO.
 * Utiliza un algoritmo de enrutamiento por hash determinista para
 * asegurar que el Mapa del Desierto sea consultable en O(1) de forma
 * paralela, optimizando la latencia de arranque en Google Colab.
 * =================================================================
 */

use crate::errors::FilterError;
use crate::filter_wrapper::RichListFilter;
use rayon::prelude::*;
use std::hash::{Hash, Hasher};
use std::path::Path;
// ✅ RESOLUCIÓN Warning: DefaultHasher eliminado para alcanzar Zero-Warnings

/// Estructura de mando para la gestión de fragmentos del censo.
pub struct ShardedFilter {
    /// Vector de filtros individuales que componen el censo total.
    filter_shards: Vec<RichListFilter>,
    /// Cantidad de particiones configuradas para la misión actual.
    partition_count: usize,
}

impl ShardedFilter {
    /**
     * Inicializa un nuevo sistema de filtrado particionado en memoria RAM.
     */
    pub fn new(
        partition_count: usize,
        total_expected_items: usize,
        false_positive_rate: f64,
    ) -> Self {
        let items_per_partition = (total_expected_items / partition_count) + 1;

        let mut shards_collection = Vec::with_capacity(partition_count);
        for _ in 0..partition_count {
            shards_collection.push(RichListFilter::new(items_per_partition, false_positive_rate));
        }

        Self {
            filter_shards: shards_collection,
            partition_count,
        }
    }

    /**
     * Calcula el índice de fragmento de forma determinista y soberana.
     *
     * # Nota Técnica:
     * Se utiliza SipHasher13 con llaves estáticas (0,0) para garantizar que
     * el índice sea idéntico en cualquier arquitectura o versión del compilador.
     */
    #[inline(always)]
    fn compute_deterministic_shard_index(&self, bitcoin_address: &str) -> usize {
        // Inicialización del motor SipHash estable
        let mut stable_hasher = siphasher::sip::SipHasher13::new_with_keys(0, 0);
        bitcoin_address.hash(&mut stable_hasher);
        (stable_hasher.finish() as usize) % self.partition_count
    }

    /**
     * Inserta una dirección en el fragmento correspondiente.
     */
    pub fn add(&mut self, bitcoin_address: &str) {
        let target_index = self.compute_deterministic_shard_index(bitcoin_address);
        if let Some(target_filter) = self.filter_shards.get_mut(target_index) {
            target_filter.add(bitcoin_address);
        }
    }

    /**
     * Verifica la existencia de una dirección en el mapa global en tiempo constante.
     */
    pub fn contains(&self, bitcoin_address: &str) -> bool {
        let target_index = self.compute_deterministic_shard_index(bitcoin_address);
        if let Some(target_filter) = self.filter_shards.get(target_index) {
            target_filter.contains(bitcoin_address)
        } else {
            false
        }
    }

    /**
     * Persiste los fragmentos binarios en disco de forma paralela.
     */
    pub fn save_to_directory<P: AsRef<Path>>(&self, output_directory: P) -> Result<(), FilterError> {
        let base_path = output_directory.as_ref();
        if !base_path.exists() {
            std::fs::create_dir_all(base_path)?;
        }

        self.filter_shards
            .par_iter()
            .enumerate()
            .try_for_each(|(index, filter)| {
                let file_name = format!("filter_shard_{}.bin", index);
                let full_path = base_path.join(file_name);
                filter.save_to_file(&full_path)
            })?;

        Ok(())
    }

    /**
     * Carga y ensambla el orquestador desde archivos binarios persistidos.
     */
    pub fn load_from_directory<P: AsRef<Path>>(
        source_directory: P,
        partition_count: usize,
    ) -> Result<Self, FilterError> {
        let base_path = source_directory.as_ref();

        let loaded_shards = (0..partition_count)
            .into_par_iter()
            .map(|index| {
                let file_name = format!("filter_shard_{}.bin", index);
                let full_path = base_path.join(file_name);

                RichListFilter::load_from_file_mmap(&full_path)
                    .or_else(|_| RichListFilter::load_from_file(&full_path))
            })
            .collect::<Result<Vec<RichListFilter>, FilterError>>()?;

        Ok(Self {
            filter_shards: loaded_shards,
            partition_count,
        })
    }

    /**
     * Retorna el volumen total de registros inyectados sumando todos los estratos.
     */
    pub fn get_total_indexed_count(&self) -> usize {
        self.filter_shards.iter().map(|filter| filter.count()).sum()
    }
}
