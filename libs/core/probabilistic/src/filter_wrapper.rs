// libs/core/probabilistic/src/filter_wrapper.rs
// =================================================================
// APARATO: BLOOM FILTER WRAPPER (MMAP EDITION)
// RESPONSABILIDAD: GESTIÓN DE MEMORIA EFICIENTE PARA DATASETS GRANDES
// ESTADO: DOCUMENTADO & COMPLIANT
// =================================================================

use crate::errors::FilterError;
use bloomfilter::Bloom;
use memmap2::MmapOptions; // ✅ Optimización de Sistema Operativo
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// Contenedor serializable para el Filtro de Bloom.
///
/// Envuelve la implementación de bajo nivel para proveer persistencia,
/// métricas y carga optimizada por memoria virtual (mmap).
#[derive(Serialize, Deserialize)]
pub struct RichListFilter {
    inner: Bloom<String>,
    item_count: usize,
}

impl RichListFilter {
    /// Crea un nuevo filtro vacío optimizado para los parámetros dados.
    ///
    /// # Argumentos
    ///
    /// * `expected_items` - Cantidad estimada de elementos a insertar (n).
    /// * `false_positive_rate` - Tasa de error deseada (p). Ej: 0.00001.
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        let bloom = Bloom::new_for_fp_rate(expected_items, false_positive_rate);
        Self {
            inner: bloom,
            item_count: 0,
        }
    }

    /// Inserta una dirección Bitcoin en el filtro.
    ///
    /// # Argumentos
    /// * `address` - La dirección en formato string (Base58Check).
    pub fn add(&mut self, address: &str) {
        self.inner.set(&address.to_string());
        self.item_count += 1;
    }

    /// Verifica probabilísticamente si una dirección existe en el set.
    ///
    /// # Retorno
    /// * `true` - Posiblemente en el set (puede ser falso positivo).
    /// * `false` - Definitivamente NO está en el set.
    pub fn contains(&self, address: &str) -> bool {
        self.inner.check(&address.to_string())
    }

    /// Serializa el estado actual del filtro a un archivo binario en disco.
    /// Utiliza `bincode` para máxima compacidad y velocidad.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), FilterError> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, &self)?;
        Ok(())
    }

    /// Carga el filtro usando Memory Mapping (mmap).
    ///
    /// Esta es la estrategia preferida para entornos de contenedores (Docker/Colab).
    /// En lugar de leer el archivo byte a byte a un buffer en el Heap,
    /// le pedimos al Kernel que mapee el archivo a nuestro espacio de direcciones.
    ///
    /// # Safety
    /// Utiliza un bloque `unsafe` porque el mapeo de memoria puede causar comportamiento
    /// indefinido si el archivo subyacente es modificado por otro proceso mientras
    /// se lee. En nuestro contexto (Worker efímero), el archivo es un artefacto estático
    /// de solo lectura, por lo que la operación se considera segura.
    #[allow(unsafe_code)]
    pub fn load_from_file_mmap<P: AsRef<Path>>(path: P) -> Result<Self, FilterError> {
        let file = File::open(path)?;

        // SAFETY: Asumimos que 'utxo_filter.bin' es inmutable durante la ejecución del worker.
        let mmap = unsafe { MmapOptions::new().map(&file)? };

        // Deserialización directa desde la memoria mapeada (Zero-Copy para el buffer inicial)
        let filter: Self = bincode::deserialize(&mmap)?;

        Ok(filter)
    }

    /// Método Legacy (Buffered Reader).
    ///
    /// Útil como fallback si mmap falla o en sistemas de archivos exóticos
    /// donde el mapeo de memoria no está soportado.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, FilterError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let filter: Self = bincode::deserialize_from(reader)?;
        Ok(filter)
    }

    /// Retorna la cantidad de elementos insertados en el filtro.
    pub fn count(&self) -> usize {
        self.item_count
    }
}
