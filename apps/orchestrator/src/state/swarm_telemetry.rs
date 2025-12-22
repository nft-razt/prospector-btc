/**
 * =================================================================
 * APARATO: SWARM TELEMETRY MANAGER (V165.0 - STRESS AWARE)
 * RESPONSABILIDAD: ANÁLISIS DE SALUD Y VIGILANCIA DE NODOS
 * =================================================================
 */

use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;
use prospector_domain_models::worker::{WorkerHeartbeat, WorkerSnapshot};

pub struct SwarmTelemetryManager {
    pub active_nodes_telemetry: RwLock<HashMap<Uuid, WorkerHeartbeat>>,
    pub visual_surveillance_frames: RwLock<HashMap<String, WorkerSnapshot>>,
}

impl SwarmTelemetryManager {
    pub fn new() -> Self {
        Self {
            active_nodes_telemetry: RwLock::new(HashMap::new()),
            visual_surveillance_frames: RwLock::new(HashMap::new()),
        }
    }

    /**
     * Sincroniza el latido y evalúa el riesgo térmico.
     */
    pub fn synchronize_heartbeat(&self, heartbeat: WorkerHeartbeat) {
        let mut telemetry_guard = self.active_nodes_telemetry.write().expect("Lock Poisoned");

        // Lógica de Inteligencia: Si el nodo supera los 85°C, el Dashboard debe alertar.
        if heartbeat.thermal_celsius > 85.0 {
            tracing::warn!("🔥 [THERMAL_ALERT]: Node {} is overheating: {}°C",
                heartbeat.worker_id, heartbeat.thermal_celsius);
        }

        telemetry_guard.insert(heartbeat.worker_id, heartbeat);
    }

    /**
     * Verifica si un nodo está en condiciones de recibir misiones intensivas.
     */
    pub fn is_node_healthy(&self, worker_id: &Uuid) -> bool {
        let telemetry_guard = self.active_nodes_telemetry.read().expect("Lock Poisoned");
        if let Some(hb) = telemetry_guard.get(worker_id) {
            return hb.thermal_celsius < 90.0 && hb.cpu_load_percent < 98.0;
        }
        false
    }
}
