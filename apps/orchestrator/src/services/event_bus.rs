// apps/orchestrator/src/services/event_bus.rs
// =================================================================
// APARATO: EVENT BUS SERVICE (v6.2 - HIGH THROUGHPUT)
// RESPONSABILIDAD: DISPATCHER ASÍNCRONO DE MENSAJES (MPSC/BROADCAST)
// CAPACIDAD: 2048 EVENTOS EN COLA (Prevención de Lag)
// =================================================================

use prospector_domain_models::{RealTimeEvent, SystemMetrics, WorkerSnapshot};
use tokio::sync::broadcast;
use tracing::{debug, info, warn};

/// Capacidad del buffer del canal Broadcast.
/// Si se llena (consumidores lentos), los mensajes antiguos se descartan (Lag).
/// Aumentado a 2048 para soportar ráfagas de 300 workers.
const EVENT_BUFFER_CAPACITY: usize = 2048;

#[derive(Debug, Clone)]
pub struct EventBus {
    tx: broadcast::Sender<RealTimeEvent>,
}

impl EventBus {
    /// Inicializa el bus de eventos global.
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(EVENT_BUFFER_CAPACITY);
        debug!(
            "⚡ EventBus inicializado con capacidad para {} eventos.",
            EVENT_BUFFER_CAPACITY
        );
        Self { tx }
    }

    /// Crea una suscripción al canal.
    /// Cada llamada crea un nuevo Receiver independiente.
    pub fn subscribe(&self) -> broadcast::Receiver<RealTimeEvent> {
        self.tx.subscribe()
    }

    /// Publica un evento a todos los suscriptores activos.
    /// Retorna el número de receptores activos.
    fn publish(&self, event: RealTimeEvent) -> usize {
        // En broadcast, send() falla solo si no hay receptores, lo cual no es un error crítico aquí.
        match self.tx.send(event) {
            Ok(receivers) => receivers,
            Err(_) => 0, // 0 receptores activos
        }
    }

    // --- API SEMÁNTICA (FACADES) ---

    /// Notifica métricas globales agregadas (Heartbeat del sistema).
    pub fn notify_metrics(&self, metrics: SystemMetrics) {
        // Nivel Trace para no inundar logs en producción, Debug en desarrollo
        // tracing::trace!("Bus: Métricas emitidas");
        self.publish(RealTimeEvent::Metrics(metrics));
    }

    /// Notifica una colisión crítica (Hallazgo de clave privada).
    pub fn notify_collision(&self, worker_id: String, address: String) {
        info!(
            "🚨 BUS: ALERTA DE COLISIÓN DE DIFUSIÓN [Worker: {}] -> {}",
            worker_id, address
        );
        self.publish(RealTimeEvent::ColissionAlert { worker_id, address });
    }

    /// Notifica que un nuevo nodo se ha unido al enjambre.
    pub fn notify_node_joined(&self, worker_id: String, hostname: String) {
        debug!("✨ BUS: Nuevo nodo detectado: {} ({})", worker_id, hostname);
        self.publish(RealTimeEvent::NodeJoined {
            worker_id,
            hostname,
        });
    }

    /// Retransmite una captura de pantalla (Vigilancia Visual).
    /// Datos pesados (Base64).
    pub fn notify_snapshot(&self, snapshot: WorkerSnapshot) {
        let size_kb = snapshot.snapshot_base64.len() / 1024;
        if size_kb > 500 {
            warn!(
                "⚠️ BUS: Snapshot grande detectado ({} KB). Puede causar lag en clientes lentos.",
                size_kb
            );
        }
        self.publish(RealTimeEvent::SnapshotReceived(snapshot));
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
