// apps/miner-worker/src/engine.rs
// =================================================================
// APARATO: MINER ENGINE (LIFECYCLE MANAGER)
// RESPONSABILIDAD: ORQUESTACIÓN DEL BUCLE INFINITO DE TRABAJO
// CARACTERÍSTICAS:
// - Graceful Shutdown: Manejo seguro de interrupciones.
// - Heartbeat Supervisor: Monitor de latidos en hilo separado.
// - Error Resilience: Backoff exponencial ante fallos de red.
// =================================================================

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::Finding;
use prospector_domain_strategy::{ExecutorContext, FindingHandler, StrategyExecutor};
use prospector_infra_worker_client::WorkerClient;

/// Canal de reporte interno para desacoplar la ejecución matemática del I/O de red.
struct ChannelReporter {
    sender: mpsc::UnboundedSender<Finding>,
}

impl FindingHandler for ChannelReporter {
    fn on_finding(&self, address: String, pk: prospector_core_math::private_key::SafePrivateKey, source: String) {
        use prospector_core_gen::wif::private_to_wif;

        println!("\n🚨 ¡COLISIÓN CONFIRMADA! Address: {}", address);

        let finding = Finding {
            address,
            private_key_wif: private_to_wif(&pk, false),
            source_entropy: source,
            wallet_type: "p2pkh_legacy".to_string(),
        };

        // El envío no bloquea el cálculo matemático
        if let Err(e) = self.sender.send(finding) {
            eprintln!("❌ ERROR CRÍTICO REPORTE: Canal cerrado - {}", e);
        }
    }
}

pub struct MinerEngine {
    client: Arc<WorkerClient>,
    filter: Arc<ShardedFilter>,
    running: Arc<AtomicBool>,
}

impl MinerEngine {
    /// Inicializa el motor de minería.
    pub fn new(client: Arc<WorkerClient>, filter: Arc<ShardedFilter>, running: Arc<AtomicBool>) -> Self {
        Self {
            client,
            filter,
            running,
        }
    }

    /// Inicia el bucle principal de adquisición y ejecución.
    pub async fn ignite(&self) {
        // 1. Configurar canal de reportes asíncrono
        let (tx, mut rx) = mpsc::unbounded_channel();
        let client_reporter = self.client.clone();

        // Hilo de reporte (Consumer)
        tokio::spawn(async move {
            while let Some(finding) = rx.recv().await {
                println!("📤 Enviando hallazgo a la Bóveda...");
                match client_reporter.report_finding(&finding).await {
                    Ok(_) => println!("✅ Hallazgo asegurado."),
                    Err(e) => eprintln!("❌ ERROR RED AL REPORTAR: {}", e),
                }
            }
        });

        // 2. Bucle Principal (Producer)
        while self.running.load(Ordering::Relaxed) {
            match self.client.acquire_job().await {
                Ok(job) => {
                    let job_id = job.id.clone();
                    println!("🔨 JOB ASIGNADO: {} [{:?}]", job_id, job.strategy);

                    // A. Supervisor de Heartbeat (Keep-Alive)
                    let (stop_hb_tx, mut stop_hb_rx) = tokio::sync::oneshot::channel();
                    let hb_client = self.client.clone();
                    let hb_job_id = job_id.clone();

                    tokio::spawn(async move {
                        loop {
                            tokio::select! {
                                _ = sleep(Duration::from_secs(30)) => {
                                    if let Err(e) = hb_client.send_keepalive(&hb_job_id).await {
                                        eprintln!("⚠️ Fallo Heartbeat: {}", e);
                                    }
                                }
                                _ = &mut stop_hb_rx => break,
                            }
                        }
                    });

                    // B. Ejecución de Estrategia (CPU Bound)
                    // Movemos referencias pesadas al hilo de bloqueo
                    let f_ref = self.filter.clone();
                    let reporter = ChannelReporter { sender: tx.clone() };
                    let ctx = ExecutorContext::default();
                    let job_clone = job.clone();

                    let start = std::time::Instant::now();

                    // ⚠️ CRITICAL: spawn_blocking para no congelar el runtime de Tokio
                    let execution_result = tokio::task::spawn_blocking(move || {
                        StrategyExecutor::execute(&job_clone, &f_ref, &ctx, &reporter);
                    }).await;

                    // Detener Heartbeat inmediatamente
                    let _ = stop_hb_tx.send(());

                    match execution_result {
                        Ok(_) => {
                            // C. Completar Trabajo
                            if let Err(e) = self.client.complete_job(&job_id).await {
                                eprintln!("❌ Error finalizando job: {}", e);
                            } else {
                                println!("🏁 Job completado: {:.2?}s", start.elapsed().as_secs_f64());
                            }
                        }
                        Err(e) => eprintln!("💀 PANIC EN EJECUCIÓN: {}", e),
                    }
                }
                Err(e) => {
                    // Backoff inteligente si no hay trabajos o hay error de red
                    println!("💤 Sin trabajo asignado o error red ({:?}). Esperando...", e);
                    sleep(Duration::from_secs(10)).await;
                }
            }
        }

        println!("🛑 Motor detenido ordenadamente.");
    }
}
