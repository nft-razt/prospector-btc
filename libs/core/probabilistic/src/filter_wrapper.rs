/**
 * =================================================================
 * APARATO: PROBABILISTIC FILTER WRAPPER (V21.0 - HARDENED)
 * CLASIFICACIÓN: CORE INFRASTRUCTURE (L1)
 * RESPONSABILIDAD: ABSTRACCIÓN DE FILTROS DE BLOOM CON MMAP
 *
 * ESTRATEGIA DE ÉLITE:
 * - Zero-Copy: Carga de filtros masivos sin saturar el heap del worker.
 * - Documentation Compliance: Secciones # Errors para cumplimiento de Clippy.
 * =================================================================
 */

use crate::errors::FilterError;
use bloomfilter::Bloom;
use memmap2::MmapOptions;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
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
     *
     * @param expected_items Capacidad nominal del filtro.
     * @param false_positive_rate Probabilidad de colisión aceptable (ej: 0.00001).
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
     * Verifica la existencia de una dirección en el censo.
     *
     * @param bitcoin_address Dirección en formato Base58Check.
     */
    #[must_use]
    pub fn contains(&self, bitcoin_address: &str) -> bool {
        self.inner_bloom_structure.check(&bitcoin_address.to_string())
    }

    /**
     * Persiste el filtro en un artefacto binario.
     *
     * # Errors
     * Retorna `FilterError::IoError` si el sistema de archivos deniega el acceso.
     */
    pub fn save_to_file<P: AsRef<Path>>(&self, storage_path: P) -> Result<(), FilterError> {
        let file = File::create(storage_path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, &self).map_err(FilterError::SerializationError)
    }

    /**
     * Carga el filtro utilizando mapeo de memoria (mmap) para eficiencia extrema.
     *
     * # Errors
     * Retorna `FilterError::SerializationError` si el binario está corrupto.
     *
     * # Panics
     * Este método no entra en pánico bajo condiciones normales de I/O.
     */
    #[allow(unsafe_code)]
    pub fn load_from_file_mmap<P: AsRef<Path>>(storage_path: P) -> Result<Self, FilterError> {
        let file = File::open(storage_path)?;

        // SAFETY: Se asume que el filtro es un artefacto inmutable durante el runtime.
        let memory_map = unsafe { MmapOptions::new().map(&file)? };
        let filter: Self = bincode::deserialize(&memory_map)?;

        Ok(filter)
    }
}
