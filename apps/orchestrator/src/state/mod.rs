/**
 * =================================================================
 * APARATO: SOVEREIGN STATE ORCHESTRATOR (V210.1 - TYPE SYNCED)
 * CLASIFICACIÓN: APPLICATION STATE (ESTRATO L1-APP)
 * RESPONSABILIDAD: COORDINACIÓN DE MEMORIA VOLÁTIL Y AUTORIDAD OPERATIVA
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el núcleo central de datos y control del Orquestador.
 * Esta versión resuelve el error E0599 asegurando que el método
 * 'workers()' sea público y retorne una referencia segura al
 * SwarmTelemetryManager, permitiendo la purga de nodos inactivos.
 * =================================================================
 */

pub mod mission_control;
pub mod swarm_telemetry;
pub mod operational_nexus;
pub mod finding_vault;

use std::sync::{Arc, RwLock, Mutex};
use std::collections::HashMap;
use prospector_infra_db::TursoClient;
use crate::services::event_bus::EventBus;
use prospector_domain_models::worker::WorkerHeartbeat;

/**
 * Define los modos operativos críticos para la estabilidad del protocolo.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum SystemMode {
    /// El sistema está plenamente operativo y aceptando misiones.
    Operational,
    /// El sistema está restringido por mantenimiento o falta de artefactos.
    Maintenance(String),
}

/**
 * Orquestador del Estado Neural de la Aplicación.
 * Diseñado para ser clonado de forma segura entre hilos de ejecución (Axum State).
 */
#[derive(Clone)]
pub struct AppState {
    /// Cliente de enlace táctico con el Motor A (Turso/libSQL).
    pub database_client: TursoClient,
    /// Bus de eventos de alta frecuencia para el Neural Link (SSE).
    pub event_bus: Arc<EventBus>,
    /// Manager del inventario de misiones listas para despacho O(1).
    pub mission_control: Arc<mission_control::MissionControlManager>,
    /// Manager de telemetría de hardware y vigilancia visual del enjambre.
    pub swarm_telemetry: Arc<swarm_telemetry::SwarmTelemetryManager>,
    /// Nexo de autoridad y estatus de certificación de la Tesis.
    pub operational_nexus: Arc<operational_nexus::OperationalNexusManager>,
    /// Bóveda atómica para el tránsito de hallazgos criptográficos.
    pub finding_vault: Arc<finding_vault::FindingVaultManager>,

    // --- ESTRATOS DE CONTROL VITAL ---
    /// Estado actual de disponibilidad del servicio (Liveness).
    pub current_system_mode: Arc<RwLock<SystemMode>>,
    /// Buffer de persistencia diferida para latidos de nodos.
    pub heartbeat_buffer: Arc<Mutex<HashMap<String, WorkerHeartbeat>>>,
}

impl AppState {
    /**
     * Realiza la ignición del estado soberano inyectando la conexión táctica.
     *
     * @param database_client Cliente de base de datos pre-configurado.
     * @returns Una instancia completa y atomizada del AppState.
     */
    pub fn new(database_client: TursoClient) -> Self {
        Self {
            database_client: database_client.clone(),
            event_bus: Arc::new(EventBus::new()),
            mission_control: Arc::new(mission_control::MissionControlManager::new()),
            swarm_telemetry: Arc::new(swarm_telemetry::SwarmTelemetryManager::new()),
            operational_nexus: Arc::new(operational_nexus::OperationalNexusManager::new()),
            finding_vault: Arc::new(finding_vault::FindingVaultManager::new()),
            current_system_mode: Arc::new(RwLock::new(SystemMode::Operational)),
            heartbeat_buffer: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // --- ACCESORES DE CONVENIENCIA (RESOLUCIÓN E0599) ---

    /**
     * Provee una referencia clonable al cliente de base de datos.
     */
    pub fn db(&self) -> TursoClient {
        self.database_client.clone()
    }

    /**
     * Retorna una referencia al SwarmTelemetryManager.
     * Requerido por servicios de mantenimiento como 'The Reaper'.
     *
     * # Performance
     * Retorna una referencia al Arc para evitar incrementos atómicos innecesarios.
     */
    pub fn workers(&self) -> &Arc<swarm_telemetry::SwarmTelemetryManager> {
        &self.swarm_telemetry
    }

    // --- MÉTODOS DE CONTROL OPERATIVO ---

    /**
     * Actualiza el modo operativo global durante la secuencia de bootstrap.
     */
    pub fn set_mode(&self, new_mode: SystemMode) {
        let mut mode_guard = self.current_system_mode.write()
            .expect("FATAL: System Mode Lock Poisoned");
        *mode_guard = new_mode;
    }

    /**
     * Evalúa si el sistema es apto para procesar misiones en vuelo.
     */
    pub fn is_operational(&self) -> Result<(), String> {
        let mode_guard = self.current_system_mode.read()
            .expect("FATAL: System Mode Lock Poisoned");
        match &*mode_guard {
            SystemMode::Operational => Ok(()),
            SystemMode::Maintenance(reason) => Err(reason.clone()),
        }
    }

    /**
     * Valida si el enjambre tiene autorización para adquirir nuevas misiones.
     */
    pub fn is_mission_acquisition_authorized(&self) -> bool {
        let integrity_status = self.operational_nexus.get_integrity_status();
        integrity_status == crate::state::operational_nexus::SystemIntegrityStatus::CertifiedOperational ||
        integrity_status == crate::state::operational_nexus::SystemIntegrityStatus::AwaitingCertification
    }

    // --- PROTOCOLO DE HIGIENE ---

    /**
     * Purga los frames visuales obsoletos de la memoria RAM del servidor.
     *
     * @param timeout_seconds Tiempo de vida máximo del snapshot en segundos.
     * @returns Cantidad de instantáneas visuales eliminadas.
     */
    pub fn prune_stale_snapshots(&self, timeout_seconds: i64) -> usize {
        let mut frames_guard = self.swarm_telemetry.visual_surveillance_frames.write()
            .expect("FATAL: Visual Frames Lock Poisoned");

        let initial_count = frames_guard.len();
        let expiration_threshold = chrono::Utc::now() - chrono::Duration::seconds(timeout_seconds);

        frames_guard.retain(|_, snapshot| {
            if let Ok(parsed_time) = chrono::DateTime::parse_from_rfc3339(&snapshot.timestamp) {
                parsed_time.with_timezone(&chrono::Utc) > expiration_threshold
            } else {
                false
            }
        });

        initial_count - frames_guard.len()
    }
}
