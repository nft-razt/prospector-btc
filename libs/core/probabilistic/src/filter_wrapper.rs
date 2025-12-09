
use bloomfilter::Bloom;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use crate::errors::FilterError;

/// Contenedor serializable para el Filtro de Bloom.
/// Usamos `TargetAddress` (String o bytes) como clave.
#[derive(Serialize, Deserialize)]
pub struct RichListFilter {
    inner: Bloom<String>, // Usamos String para compatibilidad con direcciones Base58
    item_count: usize,
}

impl RichListFilter {
    /// Crea un nuevo filtro vacío optimizado para una cantidad esperada de elementos.
    ///
    /// # Argumentos
    /// * `expected_items`: Cantidad de direcciones (ej: 50,000,000).
    /// * `false_positive_rate`: Probabilidad de error aceptable (ej: 0.0000001).
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        let bloom = Bloom::new_for_fp_rate(expected_items, false_positive_rate);
        Self {
            inner: bloom,
            item_count: 0,
        }
    }

    /// Agrega una dirección al filtro.
    pub fn add(&mut self, address: &str) {
        self.inner.set(&address.to_string());
        self.item_count += 1;
    }

    /// Verifica si una dirección *podría* estar en la lista.
    ///
    /// # Retorno
    /// * `false`: DEFINITIVAMENTE NO está (Descártala inmediatamente).
    /// * `true`: POSIBLEMENTE está (Verifica en la DB real).
    pub fn contains(&self, address: &str) -> bool {
        self.inner.check(&address.to_string())
    }

    /// Guarda el filtro en disco en formato binario comprimido.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), FilterError> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, &self)?;
        Ok(())
    }

    /// Carga el filtro desde disco (Mapeo rápido a RAM).
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, FilterError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let filter: Self = bincode::deserialize_from(reader)?;
        Ok(filter)
    }

    /// Retorna cuántos elementos se han insertado.
    pub fn count(&self) -> usize {
        self.item_count
    }
}
