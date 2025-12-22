/**
 * =================================================================
 * APARATO: SOVEREIGN SWARM COMMANDER (V45.0 - MULTI-CLOUD READY)
 * CLASIFICACIÓN: OPS CONTROL (ESTRATO L6)
 * RESPONSABILIDAD: ORQUESTACIÓN PARALELA MULTI-VECTOR
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el General de la Red Prospector. Distribuye la carga
 * de cómputo entre proveedores cloud basándose en el ratio de
 * configuración, minimizando la huella de detección.
 * =================================================================
 */

import { BrowserFactory } from "./lib/browser";
import { ColabController } from "./lib/colab";
import { KaggleController } from "./lib/kaggle";
import { config } from "./config";
import chalk from "chalk";
import pLimit from "p-limit";

/**
 * Punto de ignición maestro para el enjambre.
 */
async function main() {
  console.log(chalk.bold.magenta("\n💠 PROSPECTOR HYDRA-IGNITION :: MULTI-VECTOR MASTER"));
  console.log(chalk.gray("--------------------------------------------------"));

  // 1. GESTIÓN DE CONCURRENCIA (Protección de Red)
  const ignition_semaphore = pLimit(3);

  try {
    // 2. INICIALIZACIÓN DEL NAVEGADOR CON MIRROR MASK
    const { browser, context, identityEmail } = await BrowserFactory.createContext();

    console.log(`${chalk.cyan("👤 IDENTITY:")} ${identityEmail || "ANONYMOUS_SESSION"}`);
    console.log(`${chalk.cyan("🌊 SWARM_TARGET:")} ${config.WORKER_COUNT} grid units`);

    // ✅ RESOLUCIÓN ERROR TS2339: Uso de la propiedad ahora validada en config.ts
    const kaggle_percentage = (config.KAGGLE_DISTRIBUTION_RATIO * 100).toFixed(0);
    console.log(`${chalk.cyan("⚖️  DISTRIBUTION:")} ${kaggle_percentage}% Kaggle / ${100 - Number(kaggle_percentage)}% Colab`);

    // 3. MAPEO DE MISIONES POR VECTOR
    const deployment_sequence = Array.from({ length: config.WORKER_COUNT }).map((_, index) => {
      return ignition_semaphore(async () => {
        const sequence_id = index + 1;
        const page = await context.newPage();

        // BALANCEADOR DE CARGA PROBABILÍSTICO
        const is_kaggle_target = Math.random() < config.KAGGLE_DISTRIBUTION_RATIO;

        if (is_kaggle_target) {
          const controller = new KaggleController(page, sequence_id, identityEmail);
          await controller.deploy_ignition();
        } else {
          const controller = new ColabController(page, sequence_id, identityEmail);
          const master_key = process.env.MASTER_VAULT_KEY || "Satoshi2009";
          await controller.deploy(master_key);
        }
      });
    });

    // 4. EJECUCIÓN MASIVA
    const results = await Promise.allSettled(deployment_sequence);

    const success_count = results.filter(r => r.status === "fulfilled").length;
    console.log(chalk.bold.green(`\n✅ IGNITION_PHASE_COMPLETE: ${success_count}/${config.WORKER_COUNT} nodes online.`));

    // Mantenimiento de la vigilancia visual
    keep_system_alive();

  } catch (error: any) {
    console.error(chalk.bgRed.white("\n🔥 FATAL_COMMAND_FAULT:"), error.message);
    process.exit(1);
  }
}

function keep_system_alive() {
  setInterval(() => {
    const memory = process.memoryUsage().rss / 1024 / 1024;
    console.log(chalk.dim(`[${new Date().toLocaleTimeString()}] SwarmCommander HUD -> RAM: ${memory.toFixed(1)} MB`));
  }, 300000);
}

main();
