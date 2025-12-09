// libs/domain/models-rs/src/work.rs
// =================================================================
// APARATO: WORK MODELS
// RESPONSABILIDAD: DEFINICIÓN DE ÓRDENES DE TRABAJO (DTOs)
// ESTADO: CLEAN (UNUSED IMPORTS REMOVED)
// =================================================================

use serde::{Serialize, Deserialize};

/// Define una unidad de trabajo asignada a un Minero.
///
/// Utilizamos `String` para los rangos (`start_index`, `end_index`) para garantizar
/// compatibilidad futura con `U256` (BigInt) y evitar desbordamientos de enteros de 64 bits
/// en la capa de transporte JSON (Javascript pierde precisión con i64).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    /// Identificador único del trabajo para trazabilidad (UUID v4 serializado como String).
    pub id: String,

    /// Estrategia criptográfica a ejecutar.
    pub strategy: SearchStrategy,

    /// Tiempo objetivo de ejecución antes de reportar (Backpressure).
    /// El worker intentará ajustar su velocidad para reportar en este intervalo.
    pub target_duration_sec: u64,
}

/// Mensaje de confirmación de ciclo de vida (Heartbeat/Completion).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCompletion {
    /// ID del trabajo que se está reportando o finalizando.
    pub id: String,
}

/// Tipos de estrategias de búsqueda soportadas (Enum Algebraico).
///
/// Serialización "Adjacently tagged" (`type`, `params`) para facilitar
/// el consumo en TypeScript/Frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum SearchStrategy {
    /// Búsqueda secuencial numérica (Fuerza Bruta Inteligente).
    Combinatoric {
        /// Prefijo estático (ej: "Satoshi").
        prefix: String,
        /// Sufijo estático (ej: "2009").
        suffix: String,
        /// Inicio del rango (Representado como String numérico para BigInt).
        start_index: String,
        /// Fin del rango (Representado como String numérico para BigInt).
        end_index: String,
    },

    /// Búsqueda basada en diccionario (Brainwallets).
    Dictionary {
        /// URL remota del dataset de palabras (ej: rockyou.txt).
        dataset_url: String,
        /// Límite de palabras a procesar (0 para todo el archivo).
        limit: usize,
    },

    /// Búsqueda de vulnerabilidades históricas (Arqueología).
    ForensicScan {
        /// Objetivo específico de la vulnerabilidad (CVE conocido).
        target: ForensicTarget,
        /// Inicio del rango de iteración (PIDs o Seeds).
        range_start: String,
        /// Fin del rango.
        range_end: String,
    },

    /// Búsqueda aleatoria pura (Monte Carlo).
    Random {
        /// Semilla inicial para el PRNG del worker.
        seed: u64,
    },
}

/// Objetivos forenses específicos conocidos en la historia de Bitcoin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForensicTarget {
    /// Debian OpenSSL Bug (2008). CVE-2008-0166.
    /// Claves generadas con entropía limitada al PID del proceso.
    DebianOpenSSL,

    /// Android Java SecureRandom Bug (2013).
    /// Colisiones de `R` en firmas ECDSA por PRNG mal inicializado.
    AndroidSecureRandom,
}
