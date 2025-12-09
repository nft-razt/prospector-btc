// =================================================================
// APARATO: ORCHESTRATOR STATE
// RESPONSABILIDAD: MEMORIA VOLÁTIL DEL ENJAMBRE (DATOS + VISUAL)
// =================================================================

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use prospector_domain_models::{WorkerHeartbeat, WorkerSnapshot};
use prospector_infra_db::TursoClient;

/// El estado global compartido por todos los hilos del servidor.
/// Se clona barato (Arc).
#[derive(Clone)]
pub struct AppState {
    /// Cliente de Base de Datos (Persistencia)
    pub db: TursoClient,

    /// Registro de Workers activos (Latidos numéricos)
    /// Key: Worker ID (Uuid)
    pub workers: Arc<RwLock<HashMap<Uuid, WorkerHeartbeat>>>,

    /// Registro de Vigilancia Visual (Panóptico)
    /// Key: Worker ID (String)
    /// Almacenamos solo la ÚLTIMA foto de cada worker para ahorrar RAM.
    pub snapshots: Arc<RwLock<HashMap<String, WorkerSnapshot>>>,
}

impl AppState {
    /// Inicializa el estado.
    pub fn new(db_client: TursoClient) -> Self {
        Self {
            db: db_client,
            workers: Arc::new(RwLock::new(HashMap::new())),
            snapshots: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Actualiza el estado de un minero (Thread-Safe).
    pub fn update_worker(&self, heartbeat: WorkerHeartbeat) {
        let mut map = self.workers.write().expect("RwLock workers envenenado");
        map.insert(heartbeat.worker_id, heartbeat);
        // TODO: Implementar limpieza de workers inactivos (TTL)
    }

    /// Actualiza la instantánea visual de un worker.
    pub fn update_snapshot(&self, snap: WorkerSnapshot) {
        let mut map = self.snapshots.write().expect("RwLock snapshots envenenado");
        map.insert(snap.worker_id.clone(), snap);
    }

    /// Obtiene una lista instantánea de los workers (numérico).
    pub fn get_active_workers(&self) -> Vec<WorkerHeartbeat> {
        let map = self.workers.read().expect("RwLock workers envenenado");
        map.values().cloned().collect()
    }

    /// Obtiene todas las capturas de pantalla actuales (visual).
    pub fn get_snapshots(&self) -> Vec<WorkerSnapshot> {
        let map = self.snapshots.read().expect("RwLock snapshots envenenado");
        map.values().cloned().collect()
    }
}
