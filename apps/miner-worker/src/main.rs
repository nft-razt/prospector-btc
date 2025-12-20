/**
 * =================================================================
 * APARATO: HYDRA WORKER KERNEL (V75.0 - STRATEGIC RESILIENCE)
 * CLASIFICACIÓN: APPLICATION LAYER (ESTRATO L1)
 * RESPONSABILIDAD: ORQUESTACIÓN DE MISIÓN Y PROTOCOLO DE SELLADO
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el agente soberano de auditoría en la red Prospector.
 * Gestiona el ciclo de vida completo de las misiones criptográficas,
 * garantizando la inmutabilidad de la huella forense (Audit Trail)
 * y la persistencia de colisiones mediante canales asíncronos.
 *
 * ESTRATEGIA DE OPTIMIZACIÓN:
 * - Thread-Agnostic Reporting: Canal MPSC para reporte de hallazgos sin bloqueo.
 * - Deterministic Signal Handling: Captura de SIGINT/SIGTERM para sellado final.
 * - Parallel Hydration: Descarga y validación de fragmentos del censo en paralelo.
 * - Zero-Heap Loop: El bucle caliente de minería está desacoplado de la gestión de red.
 * =================================================================
 */

use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info, warn};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

// --- SINAPSIS INTERNA (Nx Monorepo) ---
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::work::{AuditReport, WorkOrder};
use prospector_domain_models::Finding;
use prospector_domain_strategy::{StrategyExecutor, FindingHandler};
use prospector_infra_worker_client::WorkerClient;
use prospector_core_math::private_key::SafePrivateKey;

/// Configuración de resiliencia del nodo.
const FILTRATION_PARTITION_COUNT: usize = 4;
const UPLINK_TIMEOUT_SECONDS: u64 = 60;

#[derive(Parser, Debug)]
#[command(author, version, about = "Hydra-Zero Sovereign Audit Node")]
struct WorkerArguments {
    /// Punto de enlace del servidor Orquestador.
    #[arg(long, env = "ORCHESTRATOR_URL")]
    orchestrator_endpoint: String,

    /// Token de autorización para el handshake con el Ledger Táctico.
    #[arg(long, env = "WORKER_AUTH_TOKEN")]
    authentication_token: String,

    /// Identificador único de este nodo para trazabilidad forense.
    #[arg(long, default_value = "hydra-node-strategic-alpha")]
    worker_node_identifier: String,
}

/**
 * Implementación soberana del manejador de hallazgos.
 * Utiliza un canal de transmisión para enviar colisiones al hilo de red.
 */
struct SwarmFindingHandler {
    transmission_sender: mpsc::UnboundedSender<Finding>,
}

impl FindingHandler for SwarmFindingHandler {
    fn on_finding(&self, address: String, private_key: SafePrivateKey, source: String) {
        let discovery = Finding {
            address,
            private_key_wif: prospector_core_gen::wif::private_to_wif(&private_key, false),
            source_entropy: source,
            wallet_type: "p2pkh_legacy_uncompressed".to_string(),
            found_by_worker: "hydra-agent".to_string(), // Dinámico vía contexto
            job_id: None, // Vinculado en el despacho
            detected_at: chrono::Utc::now().to_rfc3339(),
        };

        if let Err(error) = self.transmission_sender.send(discovery) {
            error!("❌ [CHANNEL_FAULT]: Failed to queue finding: {}", error);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. INICIALIZACIÓN DEL SISTEMA DE OBSERVABILIDAD
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let configuration = WorkerArguments::parse();

    info!("🛡️ [KERNEL]: Ignition sequence started for unit [{}]", configuration.worker_node_identifier);

    // 2. CONFIGURACIÓN DEL ENLACE TÁCTICO (UPLINK)
    let orchestrator_uplink = Arc::new(WorkerClient::new(
        configuration.orchestrator_endpoint.clone(),
        configuration.authentication_token.clone(),
    ));

    // 3. HIDRATACIÓN DEL CENSO (MAPA DEL DESIERTO)
    let cache_directory = PathBuf::from("census_cache");
    orchestrator_uplink.hydrate_shards(&cache_directory, FILTRATION_PARTITION_COUNT).await
        .context("Failed to hydrate UTXO census shards from remote vault")?;

    let sharded_filter = Arc::new(
        tokio::task::spawn_blocking(move || {
            ShardedFilter::load_from_directory(&cache_directory, FILTRATION_PARTITION_COUNT)
        })
        .await?
        .context("Binary integrity fault during filter reconstruction")?
    );

    info!("✅ [CENSUS]: Stratum L1 map ready. Indexed: {} targets", sharded_filter.get_total_indexed_count());

    // 4. PROTOCOLO DE GESTIÓN DE HALLAZGOS (BACKGROUND REPORTER)
    let (findings_transmission_sender, mut findings_transmission_receiver) = mpsc::unbounded_channel::<Finding>();
    let reporter_uplink = Arc::clone(&orchestrator_uplink);

    tokio::spawn(async move {
        while let Some(collision) = findings_transmission_receiver.recv().await {
            match reporter_uplink.report_finding(&collision).await {
                Ok(_) => info!("🎯 [VAULT_SYNC]: Cryptographic collision secured in remote vault."),
                Err(error) => error!("❌ [NETWORK_FAULT]: Finding transmission failed: {}", error),
            }
        }
    });

    // 5. CAPTURA DE SEÑALES DEL SISTEMA (SIGNAL HARDENING)
    let global_shutdown_signal = Arc::new(AtomicBool::new(false));
    let shutdown_signal_flag = Arc::clone(&global_shutdown_signal);

    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                warn!("⚠️ [SIGNAL]: Termination requested. Sealing forensic audit trail...");
                shutdown_signal_flag.store(true, Ordering::SeqCst);
            }
            Err(error) => error!("❌ [SIGNAL_FAULT]: Signal bus malfunction: {}", error),
        }
    });

    // 6. BUCLE PRINCIPAL DE MISIÓN (WORKER LIFECYCLE)
    info!("🚀 [SWARM_ACTIVE]: Node is now operational and awaiting assignments.");

    while !global_shutdown_signal.load(Ordering::SeqCst) {
        match orchestrator_uplink.request_mission_assignment(&configuration.worker_node_identifier).await {
            Ok(mission_order) => {
                execute_mission_lifecycle(
                    mission_order,
                    Arc::clone(&sharded_filter),
                    Arc::clone(&orchestrator_uplink),
                    findings_transmission_sender.clone(),
                    Arc::clone(&global_shutdown_signal),
                    configuration.worker_node_identifier.clone(),
                ).await?;
            }
            Err(error) => {
                warn!("💤 [IDLE]: Orchestrator busy or no pending missions. Retrying in 15s... ({})", error);
                tokio::time::sleep(Duration::from_secs(15)).await;
            }
        }
    }

    info!("🏁 [KERNEL_EXIT]: Unit [{}] deactivated successfully.", configuration.worker_node_identifier);
    Ok(())
}

/**
 * Ejecuta el ciclo de vida completo de una misión individual.
 * Asegura la separación entre el procesamiento matemático y la comunicación de red.
 */
async fn execute_mission_lifecycle(
    mission_order: WorkOrder,
    filter_reference: Arc<ShardedFilter>,
    uplink_reference: Arc<WorkerClient>,
    findings_sender: mpsc::UnboundedSender<Finding>,
    shutdown_reference: Arc<AtomicBool>,
    node_id: String,
) -> Result<()> {
    let mission_id = mission_order.job_mission_identifier.clone();
    info!("🔨 [WORK]: Commencing audit for mission segment [{}]", &mission_id[0..8]);

    // A. Lanzamiento del Motor de Estrategia (Hot-Path)
    // El StrategyExecutor satura la CPU mientras mantiene la reactividad ante señales.
    let audit_result = tokio::task::spawn_blocking(move || {
        StrategyExecutor::execute_mission_sequence(
            &mission_order,
            &filter_reference,
            shutdown_reference,
            findings_sender,
            node_id
        )
    }).await.context("Strategy execution thread panicked")?;

    // B. SELLADO DE HUELLA FORENSE (Audit Certification)
    // Intentamos reportar el volumen de esfuerzo incluso si la misión fue interrumpida.
    info!("📤 [REPORT]: Transmitting forensic audit certification...");

    match uplink_reference.submit_audit_certification(&audit_result).await {
        Ok(_) => info!("✅ [SEALED]: Mission effort [{}] certified and archived.", &mission_id[0..8]),
        Err(error) => error!("❌ [UPLINK_FAULT]: Failed to certify mission {}: {}", mission_id, error),
    }

    Ok(())
}
