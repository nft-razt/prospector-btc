
// =================================================================
// APARATO: ORCHESTRATOR STATE
// RESPONSABILIDAD: MEMORIA VOLÁTIL DEL ENJAMBRE
// =================================================================

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use prospector_domain_models::worker::WorkerHeartbeat;
use prospector_infra_db::TursoClient;

/// El estado global compartido por todos los hilos del servidor.
/// Se clona barato (Arc).
#[derive(Clone)]
pub struct AppState {
    /// Cliente de Base de Datos (Persistencia)
    pub db: TursoClient,

    /// Registro de Workers activos en memoria (Volátil)
    /// Key: Worker ID, Value: Último latido
    pub workers: Arc<RwLock<HashMap<Uuid, WorkerHeartbeat>>>,
}

impl AppState {
    /// Inicializa el estado.
    pub fn new(db_client: TursoClient) -> Self {
        Self {
            db: db_client,
            workers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Actualiza el estado de un minero (Thread-Safe).
    pub fn update_worker(&self, heartbeat: WorkerHeartbeat) {
        let mut map = self.workers.write().expect("RwLock envenenado");
        map.insert(heartbeat.worker_id, heartbeat);
        // Aquí podríamos limpiar workers viejos (timeout logic)
    }

    /// Obtiene una lista instantánea de los workers (para el Dashboard).
    pub fn get_active_workers(&self) -> Vec<WorkerHeartbeat> {
        let map = self.workers.read().expect("RwLock envenenado");
        map.values().cloned().collect()
    }
}
