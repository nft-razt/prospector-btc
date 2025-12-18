// libs/domain/models-rs/src/work.rs
/**
 * =================================================================
 * APARATO: WORK DOMAIN MODELS (V12.0 - ANALYTICS)
 * RESPONSABILIDAD: DEFINICIÓN DE ÓRDENES Y REPORTES DE TRABAJO
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    pub id: String,
    pub strategy: SearchStrategy,
    pub target_duration_sec: u64,
}

/// Reporte de éxito enviado por el Worker al agotar un rango.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCompletion {
    pub id: String,
    /// Cantidad total de llaves procesadas en este rango.
    pub total_hashes: u64,
    /// Duración real de la operación en segundos.
    pub actual_duration_sec: u64,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum SearchStrategy {
    Combinatoric {
        prefix: String,
        suffix: String,
        start_index: String,
        end_index: String,
    },
    Dictionary {
        dataset_url: String,
        limit: usize,
    },
    Kangaroo {
        target_pubkey: String,
        start_scalar: String,
        width: u64,
    },
    ForensicScan {
        target: ForensicTarget,
        range_start: String,
        range_end: String,
    },
    Random {
        seed: u64,
    },
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForensicTarget {
    DebianOpenSSL,
    AndroidSecureRandom,
}
