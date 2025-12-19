/**
 * =================================================================
 * APARATO: WORK DOMAIN MODELS (V15.0 - ESTRATO DE EJECUCIÓN)
 * CLASIFICACIÓN: DOMAIN MODELS (L2)
 * RESPONSABILIDAD: DEFINICIÓN DE ESTRUCTURAS DE TRABAJO Y AUDITORÍA
 *
 * ESTRATEGIA DE ÉLITE:
 * - BigInt Compatibility: Hashes totales representados como Strings.
 * - Discriminated Enums: Mapeo 1:1 con esquemas Zod del Dashboard.
 * - Zero-Copy Ready: Optimizado para serialización Bincode/JSON.
 * =================================================================
 */
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Representación atómica de una orden de búsqueda emitida por el Orquestador.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    /// Identificador único universal de la tarea.
    pub id: String,
    /// Configuración algorítmica de la búsqueda.
    pub strategy: SearchStrategy,
    /// Tiempo objetivo de ejecución antes del próximo Checkpoint.
    pub target_duration_sec: u64,
}

/// Informe detallado del esfuerzo computacional realizado por un nodo.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    /// ID de la orden de trabajo completada.
    pub job_id: String,
    /// Identificador del nodo que realizó el cómputo.
    pub worker_id: String,
    /// Cantidad total de claves validadas (U64 como String para evitar overflow en JS).
    pub total_hashes: String,
    /// Tiempo real consumido en la operación (Milisegundos).
    pub duration_ms: u64,
    /// Estado final del segmento: exhausted (agotado), collision_found, interrupted.
    pub exit_status: String,
    /// Última clave privada (Hex) analizada para trazabilidad forense.
    pub last_checkpoint: String,
}

/// Motores de búsqueda disponibles en el enjambre Hydra.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum SearchStrategy {
    /// Búsqueda secuencial optimizada con adición proyectiva O(1).
    Sequential {
        start_index: String,
        end_index: String,
        use_proyective_addition: bool,
    },
    /// Ataque basado en diccionarios de alta entropía.
    Dictionary { dataset_url: String, limit: usize },
    /// Resolución de rango corto mediante Pollard's Kangaroo.
    Kangaroo {
        target_pubkey: String,
        start_scalar: String,
        width: String,
    },
    /// Simulación de PRNGs vulnerables (Debian/Android).
    ForensicScan {
        target: ForensicTarget,
        range_start: String,
        range_end: String,
    },
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForensicTarget {
    /// Bug de entropía OpenSSL 2008.
    DebianOpenSSL,
    /// Bug de SecureRandom de Android 2013.
    AndroidSecureRandom,
}
