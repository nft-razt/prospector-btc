/**
 * =================================================================
 * APARATO: OPERATIONAL NEXUS MANAGER (V180.0 - SOBERANO)
 * CLASIFICACIÓN: APPLICATION STATE ATOM (ESTRATO L1-APP)
 * RESPONSABILIDAD: MANDO OPERATIVO Y ESTADO DE CERTIFICACIÓN
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la caja negra de comando del sistema. Sincroniza el
 * estado de integridad con el modo de ejecución del enjambre,
 * asegurando la coherencia atómica en entornos concurrentes.
 * =================================================================
 */

use std::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SwarmOperationalMode {
    FullExecution,
    GracefulPause,
    EmergencyStop,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SystemIntegrityStatus {
    AwaitingCertification,
    CertificationInProgress,
    CertifiedOperational,
    IntegrityCompromised,
}

pub struct OperationalNexusManager {
    /// Modo de ejecución global protegido para acceso concurrente.
    current_mode: RwLock<SwarmOperationalMode>,
    /// Nivel de validación criptográfica del censo.
    integrity_status: RwLock<SystemIntegrityStatus>,
}

impl OperationalNexusManager {
    /**
     * Inicializa el nexo operativo en estado de espera y ejecución plena.
     */
    pub fn new() -> Self {
        Self {
            current_mode: RwLock::new(SwarmOperationalMode::FullExecution),
            integrity_status: RwLock::new(SystemIntegrityStatus::AwaitingCertification),
        }
    }

    /// Retorna el modo operativo actual (Run/Pause/Stop).
    pub fn get_current_mode(&self) -> SwarmOperationalMode {
        *self.current_mode.read().expect("FATAL: Operational Lock Poisoned")
    }

    /// Retorna el nivel de integridad certificado por la autoridad.
    pub fn get_integrity_status(&self) -> SystemIntegrityStatus {
        *self.integrity_status.read().expect("FATAL: Integrity Lock Poisoned")
    }

    /**
     * Actualiza el estatus de integridad tras un hallazgo de certificación.
     */
    pub fn update_integrity(&self, new_status: SystemIntegrityStatus) {
        let mut status_guard = self.integrity_status.write().expect("FATAL: Integrity Lock Poisoned");
        *status_guard = new_status;
    }

    /**
     * Transiciona el modo operativo del enjambre.
     * Consumido por el administrative_control_stratum.
     */
    pub fn transition_mode(&self, new_mode: SwarmOperationalMode) {
        let mut mode_guard = self.current_mode.write().expect("FATAL: Operational Lock Poisoned");
        *mode_guard = new_mode;
    }
}

impl Default for OperationalNexusManager {
    fn default() -> Self {
        Self::new()
    }
}
