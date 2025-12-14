// apps/orchestrator/src/state.rs
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use prospector_domain_models::{WorkerHeartbeat, WorkerSnapshot};
use prospector_infra_db::TursoClient;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum SystemMode {
    Operational,
    Maintenance(String), // Razón del error (ej: "Filtro corrupto")
}

#[derive(Clone)]
pub struct AppState {
    pub db: TursoClient,
    pub workers: Arc<RwLock<HashMap<Uuid, WorkerHeartbeat>>>,
    pub snapshots: Arc<RwLock<HashMap<String, WorkerSnapshot>>>,
    // 🔥 NUEVO: Semáforo de estado
    pub system_mode: Arc<RwLock<SystemMode>>,
}

impl AppState {
    pub fn new(db_client: TursoClient) -> Self {
        Self {
            db: db_client,
            workers: Arc::new(RwLock::new(HashMap::new())),
            snapshots: Arc::new(RwLock::new(HashMap::new())),
            system_mode: Arc::new(RwLock::new(SystemMode::Operational)),
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

    pub fn update_worker(&self, heartbeat: WorkerHeartbeat) {
        if let Ok(mut map) = self.workers.write() {
            map.insert(heartbeat.worker_id, heartbeat);
        }
    }

    pub fn update_snapshot(&self, snap: WorkerSnapshot) {
        if let Ok(mut map) = self.snapshots.write() {
            map.insert(snap.worker_id.clone(), snap);
        }
    }

    pub fn get_active_workers(&self) -> Vec<WorkerHeartbeat> {
        self.workers.read().map(|m| m.values().cloned().collect()).unwrap_or_default()
    }

    pub fn get_snapshots(&self) -> Vec<WorkerSnapshot> {
        self.snapshots.read().map(|m| m.values().cloned().collect()).unwrap_or_default()
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
