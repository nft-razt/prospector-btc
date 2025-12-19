/**
 * =================================================================
 * APARATO: HYDRA WORKER KERNEL (V65.0 - SIGNAL HARDENED)
 * CLASIFICACIÓN: APPLICATION LAYER (L1)
 * RESPONSABILIDAD: GESTIÓN DE MISIÓN Y PROTOCOLO DE SELLADO
 *
 * ESTRATEGIA DE ÉLITE:
 * - Deterministic Shutdown: Captura señales de SO para evitar pérdida de huella.
 * - Async-Blocking Synergy: Tokio gestiona red mientras hilos bloqueantes saturan CPU.
 * - Forensic Sealing: Garantiza el envío del AuditReport antes del pánico del proceso.
 * =================================================================
 */

mod cpu_manager;

use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info, warn};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;

// --- SINAPSIS INTERNA (Nx Monorepo) ---
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::work::{AuditReport, WorkOrder};
use prospector_domain_models::Finding;
use prospector_domain_strategy::{StrategyExecutor, FindingHandler};
use prospector_infra_worker_client::WorkerClient;

/// Configuración de Resiliencia
const FILTRATION_SHARDS: usize = 4;
const UPLINK_TIMEOUT_SECONDS: u64 = 45;

#[derive(Parser, Debug)]
#[command(author, version, about = "Hydra-Zero Sovereign Node")]
struct WorkerArguments {
    #[arg(long, env = "ORCHESTRATOR_URL")]
    orchestrator_endpoint: String,

    #[arg(long, env = "WORKER_AUTH_TOKEN")]
    authentication_token: String,

    #[arg(long, default_value = "hydra-node-mit-alpha")]
    worker_node_identifier: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let configuration = WorkerArguments::parse();

    info!("🛡️ [KERNEL]: Iniciando protocolo de auditoría en unit [{}]", configuration.worker_node_identifier);

    // 1. HIDRATACIÓN DEL ENTORNO (MAPA DEL DESIERTO)
    let uplink_client = Arc::new(WorkerClient::new(
        configuration.orchestrator_endpoint.clone(),
        configuration.authentication_token.clone(),
    ));

    let cache_path = PathBuf::from("census_cache");
    uplink_client.hydrate_shards(&cache_path, FILTRATION_SHARDS).await?;

    let filter = Arc::new(
        tokio::task::spawn_blocking(move || {
            ShardedFilter::load_from_dir(&cache_path, FILTRATION_SHARDS)
        })
        .await??
    );

    // 2. CONFIGURACIÓN DEL SISTEMA DE INTERRUPCIÓN (SIGNAL HANDLER)
    // Este flag notificará a la "hormiguita" que debe dejar de correr y escribir su diario.
    let global_shutdown_signal = Arc::new(AtomicBool::new(false));
    let signal_listener_flag = Arc::clone(&global_shutdown_signal);

    tokio::spawn(async move {
        // Escuchamos interrupción del usuario (Ctrl+C) o del sistema (Kill/Colab shutdown)
        match signal::ctrl_c().await {
            Ok(()) => {
                warn!("⚠️ [SIGNAL]: Interrupción detectada. Sellando huella forense...");
                signal_listener_flag.store(true, Ordering::SeqCst);
            }
            Err(err) => error!("❌ [SIGNAL_FAULT]: Error en el bus de señales: {}", err),
        }
    });

    // 3. BUCLE DE MISIÓN SOBERANA
    info!("🔥 [IGNITION]: Enjambre activo. Awaiting assignments...");

    while !global_shutdown_signal.load(Ordering::SeqCst) {
        match uplink_client.request_mission_assignment(&configuration.worker_node_identifier).await {
            Ok(mission_order) => {
                execute_mission_lifecycle(
                    mission_order,
                    Arc::clone(&filter),
                    Arc::clone(&uplink_client),
                    Arc::clone(&global_shutdown_signal),
                ).await?;
            }
            Err(err) => {
                warn!("💤 [IDLE]: Servidor ocupado o sin misiones. Re-sincronizando en 10s... ({})", err);
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    }

    info!("🏁 [KERNEL_EXIT]: Unit [{}] desactivada con éxito.", configuration.worker_node_identifier);
    Ok(())
}

/**
 * Gestiona el ciclo de vida completo de una misión individual.
 * Garantiza que incluso ante un shutdown, se intente reportar el progreso.
 */
async fn execute_mission_lifecycle(
    order: WorkOrder,
    filter: Arc<ShardedFilter>,
    client: Arc<WorkerClient>,
    shutdown_flag: Arc<AtomicBool>,
) -> Result<()> {
    let mission_id = order.job_mission_identifier.clone();
    info!("🔨 [WORK]: Iniciando auditoría de bloque [{}]", &mission_id[0..8]);

    // A. Lanzar motor matemático en hilo dedicado (L2 Executor)
    // Pasamos el shutdown_flag para que el bucle interno de adición Jacobiana pueda detenerse.
    let thread_filter = Arc::clone(&filter);
    let thread_shutdown = Arc::clone(&shutdown_flag);

    let audit_result = tokio::task::spawn_blocking(move || {
        // El StrategyExecutor es ahora consciente del tiempo y las señales
        StrategyExecutor::execute_mission_sequence(
            &order,
            &thread_filter,
            thread_shutdown,
            &EmptyFindingHandler // Mock por ahora, reportado via canal en V11
        )
    }).await?;

    // B. SELLADO ESTRATÉGICO (Misión Crítica)
    // Intentamos reportar el resultado (huella y volumen de hashes) al Orquestador.
    info!("📤 [REPORT]: Transmitiendo huella forense a la Bóveda Táctica...");

    match client.submit_audit_certification(&audit_result).await {
        Ok(_) => info!("✅ [SEALED]: Misión [{}] certificada e inmutable.", &mission_id[0..8]),
        Err(e) => error!("❌ [UPLINK_FAULT]: Fallo al certificar misión {}: {}", mission_id, e),
    }

    Ok(())
}

/// Handler temporal para cumplir el contrato de tipos (Será nivelado en L3)
struct EmptyFindingHandler;
impl FindingHandler for EmptyFindingHandler {
    fn on_finding(&self, _addr: String, _pk: SafePrivateKey, _src: String) {}
}
