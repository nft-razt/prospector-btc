/**
 * =================================================================
 * APARATO: MISSION CONTROL MANAGER (V150.0)
 * RESPONSABILIDAD: GESTIÓN DE BUFFER DE ÓRDENES DE TRABAJO
 * =================================================================
 */

use std::collections::VecDeque;
use std::sync::Mutex;
use prospector_domain_models::work::WorkOrder;

pub struct MissionControlManager {
    /// Cola FIFO de misiones listas para ser consumidas por el enjambre.
    /// El acceso está protegido por un Mutex para garantizar la atomicidad del 'Pull'.
    active_dispatch_queue: Mutex<VecDeque<WorkOrder>>,
}

impl MissionControlManager {
    pub fn new() -> Self {
        Self {
            active_dispatch_queue: Mutex::new(VecDeque::with_capacity(1000)),
        }
    }

    /**
     * Extrae una misión de la cola de despacho.
     * Operación O(1). No requiere transacciones de base de datos.
     */
    pub fn pull_assignment(&self) -> Option<WorkOrder> {
        let mut queue_guard = self.active_dispatch_queue.lock().expect("Mission Queue Poisoned");
        queue_guard.pop_front()
    }

    /**
     * Inyecta un lote de misiones pre-asignadas en la cola.
     */
    pub fn hydrate_queue(&self, batch: Vec<WorkOrder>) {
        let mut queue_guard = self.active_dispatch_queue.lock().expect("Mission Queue Poisoned");
        queue_guard.extend(batch);
    }

    /**
     * Retorna la cantidad de misiones remanentes en el buffer de memoria.
     */
    pub fn get_available_buffer_size(&self) -> usize {
        let queue_guard = self.active_dispatch_queue.lock().expect("Mission Queue Poisoned");
        queue_guard.len()
    }
}

impl Default for MissionControlManager {
    fn default() -> Self {
        Self::new()
    }
}
