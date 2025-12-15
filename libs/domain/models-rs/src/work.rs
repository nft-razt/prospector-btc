// libs/domain/models-rs/src/work.rs
// =================================================================
// APARATO: WORK MODELS (V7.0 - KANGAROO ENABLED)
// RESPONSABILIDAD: DEFINICIÓN DE ÓRDENES DE TRABAJO
// CAMBIO: INCLUSIÓN DE ESTRATEGIA 'KANGAROO'
// =================================================================

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Define una unidad de trabajo asignada a un Minero.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    pub id: String,
    pub strategy: SearchStrategy,
    pub target_duration_sec: u64,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCompletion {
    pub id: String,
}

/// Tipos de estrategias de búsqueda soportadas.
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
    ForensicScan {
        target: ForensicTarget,
        range_start: String,
        range_end: String,
    },
    /// ✅ NUEVA ESTRATEGIA: CANGURO
    /// Diseñada para rangos donde sabemos que la clave está "cerca" de un punto base.
    Kangaroo {
        /// Clave pública objetivo (Compressed Hex String).
        /// El minero intentará encontrar su clave privada.
        target_pubkey: String,

        /// Escalar de inicio del rango de búsqueda (Hex String de 32 bytes).
        /// Representa el límite inferior del intervalo.
        start_scalar: String,

        /// Ancho del intervalo de búsqueda (u64).
        /// El rango efectivo es [start, start + width].
        width: u64,
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
