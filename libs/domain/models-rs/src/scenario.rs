/**
 * =================================================================
 * APARATO: SCENARIO REGISTRY MODELS (V105.0)
 * CLASIFICACIÓN: DOMAIN MODELS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE PLANTILLAS MAESTRAS DE ENTROPÍA
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Representa el registro oficial de una configuración de sistema operativo histórica.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTemplateRegistry {
    /// Identificador único (ej: "XP_SP3_EN_LITE").
    pub template_identifier: String,
    /// Nombre descriptivo para el Dashboard.
    pub display_name: String,
    /// Hash SHA-256 del binario original para verificar integridad.
    pub binary_integrity_hash: String,
    /// Tamaño exacto del buffer en bytes.
    pub buffer_size_bytes: u32,
    /// Clasificación del entorno (Desktop, Server, VirtualMachine).
    pub environment_category: String,
    /// Fecha en que se capturó este snapshot de memoria.
    pub captured_at_timestamp: String,
}
