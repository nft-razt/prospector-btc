/**
 * =================================================================
 * APARATO: WORK DOMAIN ENTITIES (V26.0 - ELITE ALIGNED)
 * CLASIFICACIÓN: DOMAIN MODELS (L2)
 * RESPONSABILIDAD: DEFINICIÓN DE ESTRUCTURAS DE MISIÓN Y AUDITORÍA
 *
 * ESTRATEGIA DE ÉLITE:
 * - Determinism: Uso de Enums etiquetados para serialización polimórfica.
 * - Documentation: Full RustDoc con cumplimiento de estándares Clippy.
 * - No-Abbreviations: Nombres de campos explícitos para rigor académico.
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Representa el objetivo específico de una auditoría forense.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ForensicTarget {
    /// Simulación del fallo de entropía de OpenSSL en Debian (2008).
    DebianOpenSslVulnerability,
    /// Simulación del fallo de SecureRandom en Android (2013).
    AndroidSecureRandomVulnerability,
}

/// Motores de búsqueda atómicos disponibles para el enjambre Hydra.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum SearchStrategy {
    /// Búsqueda secuencial optimizada con adición proyectiva.
    Sequential {
        /// Inicio del rango en formato hexadecimal (32 bytes).
        start_index_hex: String,
        /// Fin del rango en formato hexadecimal (32 bytes).
        end_index_hex: String,
    },
    /// Ataque basado en diccionarios de alta entropía.
    Dictionary {
        /// URL del dataset de frases semilla.
        dataset_url: String,
        /// Cantidad de frases por lote de procesamiento.
        batch_size: usize
    },
    /// Simulación de PRNGs vulnerables.
    ForensicScan {
        /// Objetivo forense seleccionado.
        target: ForensicTarget,
        /// Inicio del rango de semillas (u64).
        seed_range_start: String,
        /// Fin del rango de semillas (u64).
        seed_range_end: String,
    },
}

/// Confirmación de finalización de tarea enviada por el minero.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCompletion {
    /// Identificador único de la misión completada.
    pub mission_identifier: String,
    /// Volumen total de hashes procesados con éxito.
    pub total_computational_hashes: u64,
    /// Duración real de la ejecución en segundos.
    pub actual_execution_duration_seconds: u64,
}
