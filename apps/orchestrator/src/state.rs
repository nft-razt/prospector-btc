/**
 * =================================================================
 * APARATO: SYSTEM NEURAL STATE (V125.0 - SOBERANO)
 * CLASIFICACIÓN: APPLICATION STATE (ESTRATO L1)
 * RESPONSABILIDAD: MEMORIA CENTRAL Y COORDINACIÓN OPERATIVA
 * =================================================================
 */

use crate::services::event_bus::EventBus;
use chrono::{DateTime, Utc};
use prospector_domain_models::worker::{WorkerHeartbeat, WorkerSnapshot};
use prospector_domain_models::telemetry::SystemMetrics;
use prospector_infra_db::TursoClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use uuid::Uuid;

/// Define los estados de ejecución soberanos para el enjambre Hydra.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SwarmOperationalMode {
    /// Los nodos procesan misiones y solicitan nuevas de forma ininterrumpida.
    FullExecution,
    /// Los nodos terminan su misión actual pero no reciben nuevas. Entran en hibernación.
    GracefulPause,
    /// Cese inmediato de todas las operaciones computacionales.
    EmergencyStop,
}

/// Representa el estado actual del servidor orquestador en tiempo real.
pub struct AppState {
    /// Enlace táctico a la base de datos Turso (Engine A).
    pub database_client: TursoClient,
    /// Mapa de telemetría viva de los trabajadores activos en memoria RAM.
    pub active_workers: Arc<RwLock<HashMap<Uuid, WorkerHeartbeat>>>,
    /// Almacén volátil de capturas visuales del panóptico del dashboard.
    pub visual_snapshots: Arc<RwLock<HashMap<String, WorkerSnapshot>>>,
    /// Bus de eventos para la difusión vía Neural Link (SSE).
    pub event_bus: Arc<EventBus>,
    /// Control soberano del modo de operación del sistema distribuido.
    pub operational_mode: Arc<RwLock<SwarmOperationalMode>>,
    /// Buffer de escritura diferida para optimización de transacciones SQL.
    pub heartbeat_persistence_buffer: Arc<Mutex<HashMap<Uuid, WorkerHeartbeat>>>,
}

impl AppState {
    /**
     * Inicializa una nueva instancia del estado neural con inyección de dependencias.
     */
    pub fn new(database_client: TursoClient) -> Self {
        Self {
            database_client,
            active_workers: Arc::new(RwLock::new(HashMap::new())),
            visual_snapshots: Arc::new(RwLock::new(HashMap::new())),
            event_bus: Arc::new(EventBus::new()),
            operational_mode: Arc::new(RwLock::new(SwarmOperationalMode::FullExecution)),
            heartbeat_persistence_buffer: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /**
     * Realiza una transición atómica hacia un nuevo modo operativo del enjambre.
     */
    pub fn transition_operational_mode(&self, new_mode: SwarmOperationalMode) {
        if let Ok(mut mode_guard) = self.operational_mode.write() {
            *mode_guard = new_mode;
        }
    }

    /**
     * Consulta si el orquestador tiene autorización para entregar nuevas misiones.
     */
    pub fn is_mission_acquisition_authorized(&self) -> bool {
        if let Ok(mode_guard) = self.operational_mode.read() {
            return *mode_guard == SwarmOperationalMode::FullExecution;
        }
        false
    }

    /**
     * Sincroniza la telemetría de un trabajador y encola su persistencia.
     */
    pub fn synchronize_worker_heartbeat(&self, heartbeat: WorkerHeartbeat) {
        if let Ok(mut workers_map) = self.active_workers.write() {
            workers_map.insert(heartbeat.worker_id, heartbeat.clone());
        }
        if let Ok(mut buffer_guard) = self.heartbeat_persistence_buffer.lock() {
            buffer_guard.insert(heartbeat.worker_id, heartbeat);
        }
    }

    /**
     * Genera las métricas agregadas de salud global para el Dashboard.
     */
    pub fn aggregate_system_health_metrics(&self) -> SystemMetrics {
        let workers_guard = self.active_workers.read().expect("Lock poisoned");
        let current_time = Utc::now();
        let activity_threshold = current_time - chrono::Duration::seconds(60);

        let active_nodes = workers_guard.values()
            .filter(|worker| worker.timestamp > activity_threshold)
            .collect::<Vec<_>>();

        SystemMetrics {
            active_nodes_count: active_nodes.len(),
            cumulative_global_hashrate: active_nodes.iter().map(|w| w.hashrate).sum(),
            active_missions_in_flight: active_nodes.iter().filter(|w| w.current_job_id.is_some()).count(),
            updated_at_timestamp: current_time.to_rfc3339(),
        }
    }
}
