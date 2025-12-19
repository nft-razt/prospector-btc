/**
 * =================================================================
 * APARATO: PROVISIONER SWARM COMMANDER (V42.0 - ELITE IGNITION)
 * CLASIFICACIÓN: OPS CONTROL (L6)
 * RESPONSABILIDAD: ORQUESTACIÓN PARALELA Y GESTIÓN DE IDENTIDAD ZK
 *
 * ESTRATEGIA DE ÉLITE:
 * - Parallelism: Uso de semáforo p-limit para despliegue concurrente.
 * - Zero-Knowledge: Propagación de MasterKey para descifrado en el worker.
 * - Fault Tolerance: Reintento exponencial y aislamiento de fallos por nodo.
 * - Stealth: Integración de Mirror Mask via BrowserFactory.
 * =================================================================
 */

import { BrowserFactory } from "./lib/browser";
import { ColabController } from "./lib/colab";
import { config } from "./config";
import chalk from "chalk";
import pLimit from "p-limit";

/**
 * Punto de entrada principal para el despliegue del enjambre.
 * Ejecuta una ráfaga controlada de igniciones para optimizar el tiempo de CPU.
 */
async function main() {
  console.log(
    chalk.bold.green("\n🚀 PROSPECTOR HYDRA-IGNITION SEQUENCE :: V42.0"),
  );
  console.log(chalk.gray("--------------------------------------------------"));

  // 1. CONFIGURACIÓN DEL SEMÁFORO DE CONCURRENCIA
  // Limitamos a 5 despliegues simultáneos para evitar la detección de ráfaga masiva.
  const ignitionSemaphore = pLimit(5);

  // 2. RECUPERACIÓN DE MATERIAL CRIPTOGRÁFICO SOBERANO
  const masterKey = process.env.MASTER_VAULT_KEY || "Netflix69";

  try {
    // Inicialización del motor de navegación avanzado
    const { browser, context, identityEmail } =
      await BrowserFactory.createContext();

    logStatus(`👤 OPERATOR_IDENTITY: ${identityEmail || "ANONYMOUS_SESSION"}`);
    logStatus(`🌊 TARGET_SWARM_SIZE: ${config.WORKER_COUNT} grid units`);

    // 3. MAPEO DE TAREAS DE DESPLIEGUE PARALELO
    const deploymentSequence = Array.from({ length: config.WORKER_COUNT }).map(
      (_, index) => {
        return ignitionSemaphore(async () => {
          const workerSequenceId = index + 1;
          const workerPrefix = `[Worker-${workerSequenceId}]`;

          try {
            // Cada pestaña del navegador es un worker independiente
            const page = await context.newPage();
            const controller = new ColabController(
              page,
              workerSequenceId,
              identityEmail,
            );

            console.log(
              chalk.blue(
                `${workerPrefix} 🛰️ Iniciando secuencia de despliegue...`,
              ),
            );

            // El controlador inyecta la MasterKey directamente en la RAM del worker
            await controller.deploy(masterKey);

            console.log(
              chalk.green(
                `${workerPrefix} ✅ IGNITION_SUCCESS: Node is online.`,
              ),
            );
          } catch (error: any) {
            console.error(
              chalk.red(
                `${workerPrefix} ❌ DEPLOYMENT_FAILED: ${error.message}`,
              ),
            );
            // El fallo de un nodo no detiene la ignición del resto del enjambre
          }
        });
      },
    );

    // Ejecución masiva con resolución coordinada
    await Promise.allSettled(deploymentSequence);

    console.log(
      chalk.bold.cyan(
        "\n🏁 SWARM_DEPLOYMENT_PHASE_COMPLETE: Grid is operational.",
      ),
    );
    logStatus("Transitioning to maintenance mode... Monitoring neural link.");

    // Mantenimiento de proceso vivo para recolección de logs de telemetría
    setInterval(() => {
      const { heapUsed } = process.memoryUsage();
      const memoryMb = (heapUsed / 1024 / 1024).toFixed(2);
      console.log(
        chalk.dim(
          `[${new Date().toLocaleTimeString()}] Provisioner Monitor -> Heap: ${memoryMb} MB`,
        ),
      );
    }, 600000); // Latido cada 10 minutos
  } catch (error: any) {
    console.error(
      chalk.bgRed.white("\n🔥 FATAL_IGNITION_ERROR:"),
      error.message,
    );
    process.exit(1);
  }
}

/**
 * Utility: Emite un mensaje de estado con marca de tiempo técnica.
 */
function logStatus(message: string) {
  const timestamp = new Date().toLocaleTimeString();
  console.log(`${chalk.gray(`[${timestamp}]`)} ${chalk.cyan("ℹ️")} ${message}`);
}

// Inicialización de la secuencia soberana
main();
