/**
 * =================================================================
 * APARATO: STRATUM MANIFEST MODEL (V100.0)
 * CLASIFICACIÓN: DOMAIN MODELS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DEL CONTRATO DE INTEGRIDAD DE DATOS
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratumManifest {
    /// Token único de auditoría que identifica esta versión del censo.
    pub audit_token: String,
    /// Mapa de estratos y sus respectivos hashes de integridad.
    /// Key: satoshi_era, vulnerable_legacy, etc.
    pub strata_integrity_map: HashMap<String, String>,
    /// Marca de tiempo de la cristalización.
    pub crystallized_at: String,
}

impl StratumManifest {
    pub fn new() -> Self {
        Self {
            audit_token: String::new(),
            strata_integrity_map: HashMap::new(),
            crystallized_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /**
     * Añade un estrato y recalcula el Audit Token global.
     */
    pub fn add_strata(&mut self, identifier: String, hash: String) {
        self.strata_integrity_map.insert(identifier, hash);
        self.recalculate_audit_token();
    }

    fn recalculate_audit_token(&mut self) {
        let mut hasher = Sha256::new();
        // Ordenamos las claves para que el hash sea determinista
        let mut keys: Vec<_> = self.strata_integrity_map.keys().collect();
        keys.sort();

        for key in keys {
            hasher.update(key.as_bytes());
            hasher.update(self.strata_integrity_map.get(key).unwrap().as_bytes());
        }

        self.audit_token = format!("{:x}", hasher.finalize());
    }
}

impl Default for StratumManifest {
    fn default() -> Self {
        Self::new()
    }
}
