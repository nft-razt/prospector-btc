use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Define una unidad de trabajo asignada a un Minero.
/// Utilizamos String para los rangos para garantizar compatibilidad futura con U256 (BigInt)
/// y evitar desbordamientos de enteros en la capa de transporte JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    /// Identificador único del trabajo para trazabilidad (UUID v4)
    pub id: String,

    /// Estrategia criptográfica a ejecutar
    pub strategy: SearchStrategy,

    /// Tiempo objetivo de ejecución antes de reportar (Backpressure)
    pub target_duration_sec: u64,
}

/// Mensaje de confirmación de ciclo de vida (Heartbeat/Completion).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCompletion {
    /// ID del trabajo que se está reportando
    pub id: String,
}

/// Tipos de estrategias de búsqueda soportadas (Enum Algebraico).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum SearchStrategy {
    /// Búsqueda secuencial numérica (Fuerza Bruta Inteligente).
    Combinatoric {
        /// Prefijo estático (ej: "Satoshi")
        prefix: String,
        /// Sufijo estático (ej: "2009")
        suffix: String,
        /// Inicio del rango (Representado como String numérico)
        start_index: String,
        /// Fin del rango (Representado como String numérico)
        end_index: String,
    },

    /// Búsqueda basada en diccionario (Brainwallets).
    Dictionary {
        dataset_url: String,
        limit: usize,
    },

    /// Búsqueda de vulnerabilidades históricas (Arqueología).
    ForensicScan {
        target: ForensicTarget,
        range_start: String,
        range_end: String,
    },

    /// Búsqueda aleatoria pura (Monte Carlo).
    Random {
        seed: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForensicTarget {
    DebianOpenSSL,
    AndroidSecureRandom,
}
