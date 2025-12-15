use crate::brainwallet::phrase_to_private_key;
use prospector_core_math::private_key::SafePrivateKey;

/// Iterador para ataques basados en listas de palabras (Brainwallets).
/// Recorre un vector de palabras pre-cargado en memoria.
pub struct DictionaryIterator<'a> {
    words: &'a [String],
    current_index: usize,
    limit: usize,
}

impl<'a> DictionaryIterator<'a> {
    /// Crea un nuevo iterador sobre un slice de palabras.
    ///
    /// # Argumentos
    /// * `words`: Referencia al dataset cargado (ej: rockyou.txt parseado).
    /// * `limit`: Límite de seguridad para detener la iteración (o 0 para todo).
    pub fn new(words: &'a [String], limit: usize) -> Self {
        let actual_limit = if limit == 0 || limit > words.len() {
            words.len()
        } else {
            limit
        };

        Self {
            words,
            current_index: 0,
            limit: actual_limit,
        }
    }
}

impl<'a> Iterator for DictionaryIterator<'a> {
    type Item = (String, SafePrivateKey);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.limit {
            return None;
        }

        let phrase = &self.words[self.current_index];
        self.current_index += 1;

        // Transformamos la palabra humana en clave privada (SHA256)
        // Delegamos a la lógica centralizada de brainwallet para consistencia.
        let pk = phrase_to_private_key(phrase);

        Some((phrase.clone(), pk))
    }
}
