/**
 * =================================================================
 * APARATO: ESTRUCTURAS SOBERANAS DE TRABAJO (V105.0)
 * RESPONSABILIDAD: DEFINICIÓN DE CONTRATOS DE MISIÓN Y AUDITORÍA
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ForensicArchaeologyTarget {
    DebianOpenSslEntropyVulnerability,
    AndroidSecureRandomVulnerability,
    SatoshiWindowsXpEnvironment,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum SearchStrategy {
    Sequential {
        start_index_hexadecimal: String,
        end_index_hexadecimal: String,
    },
    Dictionary {
        dataset_resource_locator: String,
        processing_batch_size: usize,
    },
    /// PROTOCOLO ELITE: Reconstrucción determinista del entorno Satoshi-XP.
    SatoshiWindowsXpForensic {
        scenario_template_identifier: String,
        uptime_seconds_start: u64,
        uptime_seconds_end: u64,
        hardware_clock_frequency: u64,
    },
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    pub job_mission_identifier: String,
    pub lease_duration_seconds: u64,
    pub strategy: SearchStrategy,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub job_mission_identifier: String,
    pub worker_node_identifier: String,
    pub computational_effort_volume: String,
    pub execution_duration_milliseconds: u64,
    pub final_mission_status: String,
    pub audit_footprint_checkpoint: String,
    pub completed_at_timestamp: String,
}
