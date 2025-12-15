// apps/orchestrator/src/state.rs
// =================================================================
// APARATO: APP STATE (BUFFERED EDITION)
// RESPONSABILIDAD: MEMORIA COMPARTIDA & BUFFER DE ESCRITURA
// MEJORA: WRITE-BEHIND PATTERN PARA DB
// =================================================================

use crate::services::event_bus::EventBus;
use chrono::{DateTime, Utc};
use prospector_domain_models::{WorkerHeartbeat, WorkerSnapshot};
use prospector_infra_db::TursoClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock}; // Mutex para el buffer de escritura (más simple que RwLock para write-heavy)
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum SystemMode {
    Operational,
    Maintenance(String),
}

#[derive(Clone)]
pub struct AppState {
    pub db: TursoClient,

    // Estado de Lectura (Para Dashboard y API) - RwLock para alta concurrencia de lectura
    pub workers: Arc<RwLock<HashMap<Uuid, WorkerHeartbeat>>>,
    pub snapshots: Arc<RwLock<HashMap<String, WorkerSnapshot>>>,
    pub system_mode: Arc<RwLock<SystemMode>>,

    // ✅ BUFFER DE ESCRITURA (CIRCUIT BREAKER)
    // Usamos Mutex porque solo el FlushService y el Handler acceden brevemente.
    // Clave: Uuid del worker (para deduplicación automática).
    pub heartbeat_buffer: Arc<Mutex<HashMap<Uuid, WorkerHeartbeat>>>,

    pub events: Arc<EventBus>,
}

impl AppState {
    pub fn new(db_client: TursoClient) -> Self {
        Self {
            db: db_client,
            workers: Arc::new(RwLock::new(HashMap::new())),
            snapshots: Arc::new(RwLock::new(HashMap::new())),
            system_mode: Arc::new(RwLock::new(SystemMode::Operational)),
            // Buffer inicia vacío
            heartbeat_buffer: Arc::new(Mutex::new(HashMap::new())),
            events: Arc::new(EventBus::new()),
        }
    }

    pub fn set_mode(&self, mode: SystemMode) {
        if let Ok(mut w) = self.system_mode.write() {
            *w = mode;
        }
    }

    pub fn is_operational(&self) -> Result<(), String> {
        if let Ok(r) = self.system_mode.read() {
            match &*r {
                SystemMode::Operational => Ok(()),
                SystemMode::Maintenance(reason) => Err(reason.clone()),
            }
        } else {
            Err("System Lock Poisoned".to_string())
        }
    }

    /// Actualiza el estado de un worker.
    /// 1. Actualiza RAM (Lectura inmediata).
    /// 2. Encola en Buffer (Persistencia diferida).
    pub fn update_worker(&self, heartbeat: WorkerHeartbeat) {
        // A. Actualización en RAM (Hot Path)
        if let Ok(mut map) = self.workers.write() {
            if !map.contains_key(&heartbeat.worker_id) {
                self.events.notify_node_joined(
                    heartbeat.worker_id.to_string(),
                    heartbeat.hostname.clone(),
                );
            }
            map.insert(heartbeat.worker_id, heartbeat.clone());
        }

        // B. Encolado en Buffer (Cold Path)
        if let Ok(mut buffer) = self.heartbeat_buffer.lock() {
            // Insertar reemplaza cualquier heartbeat previo del mismo worker que no se haya guardado aún.
            // Esto es "Write Coalescing".
            buffer.insert(heartbeat.worker_id, heartbeat);
        }
    }

    pub fn update_snapshot(&self, snap: WorkerSnapshot) {
        if let Ok(mut map) = self.snapshots.write() {
            map.insert(snap.worker_id.clone(), snap);
        }
    }

    pub fn get_active_workers(&self) -> Vec<WorkerHeartbeat> {
        self.workers
            .read()
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default()
    }

    pub fn get_snapshots(&self) -> Vec<WorkerSnapshot> {
        self.snapshots
            .read()
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default()
    }

    pub fn prune_stale_snapshots(&self, seconds: i64) -> usize {
        if let Ok(mut map) = self.snapshots.write() {
            let initial = map.len();
            let threshold = Utc::now() - chrono::Duration::seconds(seconds);
            map.retain(|_, snap| {
                DateTime::parse_from_rfc3339(&snap.timestamp)
                    .map(|ts| ts.with_timezone(&Utc) > threshold)
                    .unwrap_or(false)
            });
            initial - map.len()
        } else {
            0
        }
    }
}
