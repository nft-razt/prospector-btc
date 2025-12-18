// libs/domain/models-rs/src/finding.rs
// =================================================================
// APARATO: FINDING DOMAIN MODEL (V10.0)
// RESPONSABILIDAD: DEFINICIÓN DE COLISIÓN CRIPTOGRÁFICA
// ESTADO: NO-REGRESSIONS / AUDIT-READY
// =================================================================

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// El "Santo Grial". Representa una colisión exitosa en el espacio secp256k1.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// La dirección Bitcoin (P2PKH) que contenía balance.
    pub address: String,

    /// La clave privada en formato WIF.
    /// En tránsito entre Worker y Orquestador, este campo es sensible.
    pub private_key_wif: String,

    /// Descripción de la fuente de entropía (ej: "dictionary:rockyou:line_450").
    pub source_entropy: String,

    /// Tipo de dirección: 'legacy_uncompressed', 'legacy_compressed'.
    pub wallet_type: String,

    /// --- METADATOS DE AUDITORÍA ---

    /// ID del worker que reportó el hallazgo.
    pub found_by_worker: String,

    /// ID del trabajo (rango) donde ocurrió la colisión.
    pub job_id: Option<String>,

    /// Timestamp UTC de la detección.
    pub detected_at: String,
}
