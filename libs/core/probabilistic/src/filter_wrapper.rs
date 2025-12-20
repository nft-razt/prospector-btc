/**
 * =================================================================
 * APARATO: PROBABILISTIC FILTER WRAPPER (V22.0 - HARDENED)
 * CLASIFICACIÓN: CORE INFRASTRUCTURE (L1)
 * RESPONSABILIDAD: ABSTRACCIÓN DE FILTROS DE BLOOM CON MMAP Y FALLBACK
 * =================================================================
 */

use crate::errors::FilterError;
use bloomfilter::Bloom;
use memmap2::MmapOptions;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter}; // ✅ RESOLUCIÓN: BufReader ahora se utiliza
use std::path::Path;

/// Contenedor de alta densidad para la lista de carteras con saldo (UTXO).
#[derive(Serialize, Deserialize)]
pub struct RichListFilter {
    inner_bloom_structure: Bloom<String>,
    indexed_item_count: usize,
}

impl RichListFilter {
    /**
     * Inicializa un nuevo filtro vacío con una tasa de error específica.
     */
    #[must_use]
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        let bloom = Bloom::new_for_fp_rate(expected_items, false_positive_rate);
        Self {
            inner_bloom_structure: bloom,
            indexed_item_count: 0,
        }
    }

    /**
     * Inserta una dirección en el filtro e incrementa el contador de auditoría.
     *
     * @param bitcoin_address Dirección en formato Base58Check.
     */
    pub fn add(&mut self, bitcoin_address: &str) {
        self.inner_bloom_structure.set(&bitcoin_address.to_string());
        self.indexed_item_count += 1;
    }

    /**
     * Verifica la existencia de una dirección en el censo.
     */
    #[must_use]
    pub fn contains(&self, bitcoin_address: &str) -> bool {
        self.inner_bloom_structure.check(&bitcoin_address.to_string())
    }

    /**
     * Retorna la cantidad de elementos procesados en este filtro.
     */
    pub fn count(&self) -> usize {
        self.indexed_item_count
    }

    /**
     * Persiste el filtro en un artefacto binario.
     */
    pub fn save_to_file<P: AsRef<Path>>(&self, storage_path: P) -> Result<(), FilterError> {
        let file = File::create(storage_path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, &self).map_err(FilterError::SerializationError)
    }

    /**
     * Carga el filtro utilizando mapeo de memoria (mmap) para eficiencia extrema.
     */
    #[allow(unsafe_code)]
    pub fn load_from_file_mmap<P: AsRef<Path>>(storage_path: P) -> Result<Self, FilterError> {
        let file = File::open(storage_path)?;
        let memory_map = unsafe { MmapOptions::new().map(&file)? };
        let filter: Self = bincode::deserialize(&memory_map)?;
        Ok(filter)
    }

    /**
     * Carga el filtro mediante I/O estándar (Fallback).
     * ✅ RESOLUCIÓN: Método inyectado para satisfacer el contrato de ShardedFilter.
     */
    pub fn load_from_file<P: AsRef<Path>>(storage_path: P) -> Result<Self, FilterError> {
        let file = File::open(storage_path)?;
        let reader = BufReader::new(file);
        bincode::deserialize_from(reader).map_err(FilterError::SerializationError)
    }
}
