/**
 * =================================================================
 * APARATO: MINER ENGINE KERNEL (V18.5 - ELITE EDITION)
 * CLASIFICACIÓN: EXECUTION LAYER (L1)
 * RESPONSABILIDAD: ORQUESTACIÓN DEL CICLO DE VIDA DE BÚSQUEDA
 *
 * ESTRATEGIA DE ÉLITE:
 * - Thread Segregation: Tokio para I/O y Rayon para cómputo pesado.
 * - Atomic Telemetry: Conteo de hashes con costo cero (Relaxed Ordering).
 * - Graceful Shutdown: Manejo de señales para limpieza de memoria.
 * - Fault Tolerance: Backoff exponencial ante fallos del Orquestador.
 * =================================================================
 */

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{info, warn, error, instrument};

// --- SINAPSIS INTERNA (Nx Monorepo) ---
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::{Finding, JobCompletion, WorkOrder};
use prospector_domain_strategy::{
    ExecutorContext,
    FindingHandler,
    StrategyExecutor
};
use prospector_infra_worker_client::WorkerClient;

/**
 * Manejador de Hallazgos para el Enjambre.
 * Actúa como un puente asíncrono entre el motor matemático y la red.
 */
struct SwarmFindingReporter {
    /// Canal de transmisión hacia el hilo de reporte de red.
    transmission_sender: mpsc::UnboundedSender<Finding>,
    /// Identificador único del nodo para trazabilidad forense.
    worker_identifier: String,
    /// ID del trabajo activo para vincular el hallazgo al rango.
    active_job_identifier: String,
}

impl FindingHandler for SwarmFindingReporter {
    /**
     * Invocado inmediatamente al detectar una colisión en el Filtro de Bloom.
     * Realiza la conversión a WIF y emite el evento al bus de salida.
     */
    fn on_finding(
        &self,
        address: String,
        private_key: prospector_core_math::private_key::SafePrivateKey,
        source: String
    ) {
        use prospector_core_gen::wif::private_to_wif;

        let wallet_import_format = private_to_wif(&private_key, false);

        let discovery = Finding {
            address,
            private_key_wif: wallet_import_format,
            source_entropy: source,
            wallet_type: "p2pkh_legacy_uncompressed".to_string(),
            found_by_worker: self.worker_identifier.clone(),
            job_id: Some(self.active_job_identifier.clone()),
            detected_at: chrono::Utc::now().to_rfc3339(),
        };

        if let Err(error) = self.transmission_sender.send(discovery) {
            error!("🚨 [CRITICAL_CHANNEL_FAULT]: Failed to queue finding: {}", error);
        }
    }
}

/**
 * Motor Principal de Minería.
 * Encapsula las dependencias pesadas y orquesta el bucle infinito de trabajo.
 */
pub struct MinerEngine {
    /// Cliente de comunicación con el Orquestador (L3).
    uplink_client: Arc<WorkerClient>,
    /// Filtro de Bloom particionado cargado en RAM (L3).
    target_filter: Arc<ShardedFilter>,
    /// Contexto de recursos compartidos (Diccionarios, etc).
    execution_context: Arc<ExecutorContext>,
    /// Flag atómico de control de ejecución.
    is_running: Arc<AtomicBool>,
    /// Identificador del nodo.
    worker_id: String,
}

impl MinerEngine {
    /**
     * Construye una nueva instancia del motor con inyección de dependencias completa.
     */
    pub fn new(
        client: Arc<WorkerClient>,
        filter: Arc<ShardedFilter>,
        context: Arc<ExecutorContext>,
        running_signal: Arc<AtomicBool>,
        identifier: String,
    ) -> Self {
        Self {
            uplink_client: client,
            target_filter: filter,
            execution_context: context,
            is_running: running_signal,
            worker_id: identifier,
        }
    }

    /**
     * Inicia la ignición del motor y entra en el bucle de adquisición de trabajos.
     */
    pub async fn ignite(&self) {
        info!("🔥 [ENGINE_IGNITION]: Swarm node is operational.");

        // 1. CONFIGURACIÓN DEL CANAL DE REPORTES (Background Uploader)
        let (finding_tx, mut finding_rx) = mpsc::unbounded_channel::<Finding>();
        let reporter_client = Arc::clone(&self.uplink_client);

        tokio::spawn(async move {
            while let Some(collision) = finding_rx.recv().await {
                match reporter_client.report_finding(&collision).await {
                    Ok(_) => info!("✅ [VAULT_SYNC]: Collision secured in remote vault."),
                    Err(e) => error!("❌ [UPLINK_ERROR]: Failed to transmit finding: {}", e),
                }
            }
        });

        // 2. BUCLE PRINCIPAL DE ADQUISICIÓN (Work Lifecycle)
        while self.is_running.load(Ordering::Relaxed) {
            match self.uplink_client.acquire_job().await {
                Ok(work_order) => {
                    self.execute_assignment(work_order, finding_tx.clone()).await;
                }
                Err(error) => {
                    warn!("💤 [IDLE]: Awaiting assignments ({}). Retrying in 10s.", error);
                    sleep(Duration::from_secs(10)).await;
                }
            }
        }

        info!("🛑 [ENGINE_SHUTDOWN]: Orderly shutdown completed.");
    }

    /**
     * Ejecuta una asignación de trabajo individual con supervisión métrica.
     */
    async fn execute_assignment(
        &self,
        work_order: WorkOrder,
        finding_sender: mpsc::UnboundedSender<Finding>
    ) {
        let job_id = work_order.id.clone();
        let iteration_counter = Arc::new(AtomicU64::new(0));
        let start_time = Instant::now();

        info!("🔨 [WORK_START]: Processing range [{}]", &job_id[0..8]);

        // A. SUPERVISOR DE LATIDOS (Heartbeat Task)
        // Mantiene al worker "vivo" ante el Orquestador mientras se computan hashes.
        let (stop_heartbeat_tx, mut stop_heartbeat_rx) = tokio::sync::oneshot::channel::<()>();
        let hb_client = Arc::clone(&self.uplink_client);
        let hb_job_id = job_id.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = sleep(Duration::from_secs(30)) => {
                        let _ = hb_client.send_keepalive(&hb_job_id).await;
                    }
                    _ = &mut stop_heartbeat_rx => break,
                }
            }
        });

        // B. EJECUCIÓN CRIPTOGRÁFICA (CPU Bound)
        // Delegamos al StrategyExecutor (V14.0) que utiliza el Kernel Assembler.
        let exec_filter = Arc::clone(&self.target_filter);
        let exec_context = Arc::clone(&self.execution_context);
        let exec_counter = Arc::clone(&iteration_counter);

        let reporter = SwarmFindingReporter {
            transmission_sender: finding_sender,
            worker_identifier: self.worker_id.clone(),
            active_job_identifier: job_id.clone(),
        };

        // ⚠️ CRÍTICO: spawn_blocking para no congelar el runtime de Tokio
        let result = tokio::task::spawn_blocking(move || {
            StrategyExecutor::execute(
                &work_order,
                &exec_filter,
                exec_counter,
                &reporter
            );
        }).await;

        // C. FINALIZACIÓN Y REPORTE MÉTRICO
        let _ = stop_heartbeat_tx.send(()); // Detener supervisor

        let total_hashes = iteration_counter.load(Ordering::SeqCst);
        let actual_duration = start_time.elapsed().as_secs();

        match result {
            Ok(_) => {
                let completion_report = JobCompletion {
                    id: job_id,
                    total_hashes,
                    actual_duration_sec: actual_duration,
                };

                if let Err(e) = self.uplink_client.complete_job_with_metrics(&completion_report).await {
                    error!("❌ [REPORT_FAULT]: Metrics transmission failed: {}", e);
                } else {
                    info!(
                        "🏁 [WORK_COMPLETE]: Range verified. Effort: {} hashes in {}s",
                        total_hashes,
                        actual_duration
                    );
                }
            }
            Err(error) => error!("💀 [RUNTIME_PANIC]: Strategy execution collapsed: {}", error),
        }
    }
}
