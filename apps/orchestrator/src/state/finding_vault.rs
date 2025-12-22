/**
 * =================================================================
 * APARATO: FINDING VAULT MANAGER (V160.0 - ATOMIC BUFFER)
 * CLASIFICACIÓN: APPLICATION STATE ATOM (ESTRATO L1-APP)
 * RESPONSABILIDAD: CUSTODIA TEMPORAL DE COLISIONES DETECTADAS
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa una zona de tránsito segura para los hallazgos del enjambre.
 * Utiliza un buffer de memoria protegido para absorber ráfagas de
 * detecciones, permitiendo que la API responda al minero en microsegundos
 * mientras la persistencia ocurre de forma asíncrona.
 * =================================================================
 */

use std::sync::Mutex;
use prospector_domain_models::finding::Finding;

pub struct FindingVaultManager {
    /// Acumulador de hallazgos pendientes de persistencia física.
    /// Protegido por Mutex para garantizar la integridad en el push concurrente.
    pending_findings_collection: Mutex<Vec<Finding>>,
}

impl FindingVaultManager {
    /**
     * Inicializa la bóveda volátil con capacidad de reserva.
     */
    pub fn new() -> Self {
        Self {
            pending_findings_collection: Mutex::new(Vec::with_capacity(500)),
        }
    }

    /**
     * Inyecta un nuevo hallazgo en el buffer de tránsito.
     * Operación de memoria pura, sin latencia de red ni disco.
     */
    pub fn deposit_finding(&self, discovery: Finding) {
        let mut collection_guard = self.pending_findings_collection
            .lock()
            .expect("Finding Vault Lock Poisoned");
        collection_guard.push(discovery);
    }

    /**
     * Drena todos los hallazgos del buffer para su procesamiento masivo.
     * Deja el buffer original vacío y listo para nuevas ráfagas.
     */
    pub fn drain_vault_for_flush(&self) -> Vec<Finding> {
        let mut collection_guard = self.pending_findings_collection
            .lock()
            .expect("Finding Vault Lock Poisoned");

        // Transferencia atómica de propiedad de los datos
        std::mem::take(&mut *collection_guard)
    }

    /**
     * Retorna el volumen actual de hallazgos en espera.
     */
    pub fn get_pending_count(&self) -> usize {
        let collection_guard = self.pending_findings_collection
            .lock()
            .expect("Finding Vault Lock Poisoned");
        collection_guard.len()
    }
}

impl Default for FindingVaultManager {
    fn default() -> Self {
        Self::new()
    }
}
