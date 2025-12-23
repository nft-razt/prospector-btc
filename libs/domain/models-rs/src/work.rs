/**
 * =================================================================
 * APARATO: WORK DOMAIN MODELS (V150.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN MODELS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE CONTRATOS DE MISIÓN Y AUDITORÍA
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la Fuente Única de Verdad (SSoT) para las órdenes de
 * trabajo y los reportes de certificación. Implementa el soporte
 * para estratificación de datos (TargetStrata) y la telemetría
 * de hardware necesaria para el despacho inteligente.
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Clasificación cronológica del set UTXO de Bitcoin.
/// Determina qué fragmentos binarios del censo debe cargar el trabajador.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetStrata {
    /// Bloques 0 a 100,000. Prioridad máxima para Tesis Satoshi-XP.
    SatoshiEra,
    /// Direcciones vulnerables de la era 2011-2014 (Debian/Android Bugs).
    VulnerableLegacy,
    /// El resto de direcciones P2PKH activas con balance > 0.001 BTC.
    StandardLegacy,
    /// Carga total del universo auditado (Requiere >16GB RAM).
    FullTacticalSet,
}

/// Unión discriminada de las estrategias de búsqueda criptográfica soberanas.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "strategy_type", content = "parameters")]
pub enum SearchStrategy {
    /// Barrido secuencial de un rango escalar U256 utilizando aritmética Jacobiana.
    Sequential {
        start_index_hexadecimal: String,
        end_index_hexadecimal: String,
    },
    /// PROTOCOLO SOBERANO: Reconstrucción de entropía de Windows XP (2009).
    SatoshiWindowsXpForensic {
        /// Identificador de la plantilla binaria PERF_DATA_BLOCK (ej: "XP_SP3_MASTER").
        scenario_template_identifier: String,
        /// Segundo de inicio desde el arranque simulado.
        uptime_seconds_start: u64,
        /// Segundo de fin del segmento de búsqueda.
        uptime_seconds_end: u64,
        /// Frecuencia del cristal de la placa base (Hz).
        hardware_clock_frequency: u64,
    },
    /// Auditoría de vulnerabilidad en el PRNG de Android (CVE-2013-7372).
    AndroidLcgForensic {
        seed_range_start: u64,
        seed_range_end: u64,
    },
    /// Resolución de logaritmo discreto mediante Pollard's Kangaroo.
    KangarooLambda {
        target_public_key_hexadecimal: String,
        range_width_max: u64,
    },
    /// Auditoría basada en listas de palabras y frases semilla (Brainwallets).
    Dictionary {
        dataset_resource_locator: String,
        processing_batch_size: usize,
    },
}

/// Contrato de Orden de Trabajo (Misión Táctica).
/// Emitido por el Orquestador hacia los nodos del enjambre.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    /// Identificador único universal de la misión asignada.
    pub job_mission_identifier: String,
    /// Tiempo máximo permitido (segundos) para reportar progreso antes de revocación.
    pub lease_duration_seconds: u64,
    /// Estrategia algorítmica inyectada con sus parámetros técnicos.
    pub strategy: SearchStrategy,
    /// El estrato del censo requerido para esta misión específica.
    pub required_strata: TargetStrata,
}

/// Reporte de Certificación Forense.
/// Emitido por el trabajador al finalizar un segmento de auditoría.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    /// Vinculación con la orden de trabajo original.
    pub job_mission_identifier: String,
    /// Identificador de la unidad física que realizó el cómputo.
    pub worker_node_identifier: String,
    /// Volumen total de wallets consultadas (representación string para evitar overflow).
    pub total_wallets_audited: String,
    /// Tiempo real consumido en el cálculo neto (milisegundos).
    pub execution_duration_milliseconds: u64,
    /// Estatus final de la misión (completed, interrupted, hardware_fault).
    pub final_mission_status: String,
    /// Último punto auditado (Huella forense para reanudación inmutable).
    pub audit_footprint_checkpoint: String,
    /// Marca de tiempo UTC de la finalización.
    pub completed_at_timestamp: String,
    /// MÉTRICA DE ÉLITE: Eficiencia media de procesamiento (Hashes / Milisegundo).
    pub average_computational_efficiency: f64,
}

/// DTO de Handshake: Solicitud de trabajo enriquecida con métricas de hardware.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionRequestPayload {
    pub worker_id: String,
    pub hardware_capacity: NodeHardwareCapacity,
}

/// Metadatos de hardware para el motor de despacho inteligente (L3).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHardwareCapacity {
    /// Memoria RAM libre reportada en Megabytes.
    pub ram_available_mb: u64,
    /// Conteo de hilos lógicos de ejecución.
    pub cpu_cores: u32,
    /// Disponibilidad de instrucciones vectoriales AVX2.
    pub supports_avx2: bool,
}
