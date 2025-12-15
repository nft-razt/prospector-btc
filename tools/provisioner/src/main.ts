// INICIO DEL ARCHIVO [tools/provisioner/src/main.ts]
import { BrowserFactory } from "./lib/browser";
import { ColabController } from "./lib/colab";
import { config } from "./config";
import chalk from "chalk";
import { z } from "zod";

// Argumentos CLI simples para sharding
const args = process.argv.slice(2);
const getArg = (name: string, def: string) => {
  const arg = args.find((a) => a.startsWith(`--${name}=`));
  return arg ? arg.split("=")[1] : def;
};

// Configuración de Lote
const SHARD_OFFSET = parseInt(getArg("offset", "0"), 10); // ID de inicio (ej: 0, 50, 100)
const WORKER_COUNT = parseInt(
  getArg("count", config.WORKER_COUNT.toString()),
  10,
);

const MAX_RETRIES = 3;
const BASE_DELAY_MS = 2000;

async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function deployWorkerWithRetry(
  context: any,
  index: number,
  identityEmail: string | null,
  attempt: number = 1,
): Promise<void> {
  // El ID del worker debe ser único globalmente en el enjambre
  const globalWorkerId = index + SHARD_OFFSET;
  const workerPrefix = `[Worker-${globalWorkerId}]`;

  try {
    const page = await context.newPage();
    const controller = new ColabController(page, globalWorkerId, identityEmail);

    console.log(
      chalk.blue(
        `${workerPrefix} Iniciando despliegue (Intento ${attempt}/${MAX_RETRIES})...`,
      ),
    );
    await controller.deploy();
  } catch (err: any) {
    console.error(chalk.red(`${workerPrefix} Fallo: ${err.message}`));

    if (attempt < MAX_RETRIES) {
      const delay = BASE_DELAY_MS * Math.pow(2, attempt - 1);
      console.log(
        chalk.yellow(`${workerPrefix} Reintentando en ${delay / 1000}s...`),
      );
      await sleep(delay);
      return deployWorkerWithRetry(context, index, identityEmail, attempt + 1);
    } else {
      console.error(
        chalk.bgRed.white(
          `${workerPrefix} 💀 ABANDONADO tras ${MAX_RETRIES} intentos.`,
        ),
      );
    }
  }
}

async function main() {
  console.log(chalk.bold.green("⚡ PROSPECTOR PROVISIONER v3.7 (SHARDED)"));
  console.log(
    `🎯 Shard Offset: ${SHARD_OFFSET} | Count: ${WORKER_COUNT} | Headless: ${config.HEADLESS}`,
  );

  try {
    const { browser, context, identityEmail } =
      await BrowserFactory.createContext();

    if (identityEmail) {
      console.log(chalk.blue(`👤 Identidad Activa: ${identityEmail}`));
    } else {
      console.log(chalk.yellow("👤 Modo: ANÓNIMO / LOCAL"));
    }

    const swarmPromises = [];

    for (let i = 0; i < WORKER_COUNT; i++) {
      const p = deployWorkerWithRetry(context, i, identityEmail);
      swarmPromises.push(p);
      // Staggering para evitar detección de ráfaga
      await sleep(3000 + Math.random() * 2000);
    }

    console.log(
      chalk.cyan(
        `\n🌊 Enjambre lanzado. ${swarmPromises.length} procesos en vuelo.\n`,
      ),
    );

    await Promise.allSettled(swarmPromises);

    console.log(
      chalk.yellow(
        "\n⏳ MANTENIENDO SESIÓN VIVA. Monitorizando nodos activos...",
      ),
    );

    // Mantener vivo hasta que el CI lo mate (timeout 6h)
    setInterval(
      () => {
        console.log(`[${new Date().toISOString()}] Heartbeat monitor...`);
      },
      1000 * 60 * 5,
    );
  } catch (err) {
    console.error("🔥 FATAL MAIN LOOP:", err);
    process.exit(1);
  } // tools/provisioner/src/main.ts
  /**
   * =================================================================
   * APARATO: PROVISIONER CLI COMMANDER
   * RESPONSABILIDAD: ORQUESTACIÓN DE FLOTA Y CICLO DE VIDA
   * =================================================================
   */

  import { BrowserFactory } from "./lib/browser";
  import { ColabController } from "./lib/colab";
  import { config } from "./config";
  import chalk from "chalk";

  // Parsing de argumentos manual (ligero, sin deps extra)
  const args = process.argv.slice(2);
  const getArg = (name: string, def: string) => {
    const arg = args.find((a) => a.startsWith(`--${name}=`));
    return arg ? arg.split("=")[1] : def;
  };

  // Configuración de Sharding (Para ejecución paralela en múltiples Runners de GH)
  const SHARD_OFFSET = parseInt(getArg("offset", "0"), 10);
  const WORKER_COUNT = parseInt(
    getArg("count", config.WORKER_COUNT.toString()),
    10,
  );

  const MAX_RETRIES = 3;
  const BASE_DELAY_MS = 5000;

  async function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  /**
   * Despliega un worker individual con lógica de reintento exponencial.
   */
  async function deployWorkerWithRetry(
    context: any,
    index: number,
    identityEmail: string | null,
    attempt: number = 1,
  ): Promise<void> {
    const globalWorkerId = index + SHARD_OFFSET;
    const workerPrefix = `[Worker-${globalWorkerId}]`;

    try {
      const page = await context.newPage();
      const controller = new ColabController(
        page,
        globalWorkerId,
        identityEmail,
      );

      console.log(
        chalk.blue(
          `${workerPrefix} Iniciando despliegue (Intento ${attempt}/${MAX_RETRIES})...`,
        ),
      );
      await controller.deploy();
    } catch (err: any) {
      const msg = err.message || String(err);
      console.error(chalk.red(`${workerPrefix} Fallo: ${msg}`));

      // Si es un error de Auth, no reintentamos con la misma identidad en bucle rápido
      if (msg === "AUTH_REQUIRED") {
        console.error(
          chalk.bgRed.white(
            `${workerPrefix} 🛑 ABORTANDO POR FALLO DE AUTENTICACIÓN.`,
          ),
        );
        return;
      }

      if (attempt < MAX_RETRIES) {
        const delay = BASE_DELAY_MS * Math.pow(2, attempt - 1); // Backoff: 5s, 10s, 20s
        console.log(
          chalk.yellow(`${workerPrefix} Reintentando en ${delay / 1000}s...`),
        );
        await sleep(delay);
        // Recursión controlada
        return deployWorkerWithRetry(
          context,
          index,
          identityEmail,
          attempt + 1,
        );
      } else {
        console.error(
          chalk.bgRed.white(
            `${workerPrefix} 💀 ABANDONADO tras ${MAX_RETRIES} intentos.`,
          ),
        );
      }
    }
  }

  async function main() {
    console.log(
      chalk.bold.green("⚡ PROSPECTOR PROVISIONER v4.5 (HYDRA SWARM)"),
    );
    console.log(
      `🎯 Shard Offset: ${SHARD_OFFSET} | Count: ${WORKER_COUNT} | Headless: ${config.HEADLESS}`,
    );

    try {
      // 1. Crear Contexto Global (Una sesión de browser para todos los tabs)
      const { browser, context, identityEmail } =
        await BrowserFactory.createContext();

      if (identityEmail) {
        console.log(chalk.blue(`👤 Identidad Activa: ${identityEmail}`));
      } else {
        console.log(
          chalk.yellow("👤 Modo: ANÓNIMO / LOCAL (Sin persistencia en Vault)"),
        );
      }

      // 2. Lanzar Enjambre (Promesas en paralelo)
      const swarmPromises = [];
      for (let i = 0; i < WORKER_COUNT; i++) {
        const p = deployWorkerWithRetry(context, i, identityEmail);
        swarmPromises.push(p);

        // Staggering: Espera aleatoria entre lanzamientos para no parecer un bot masivo
        const stagger = 3000 + Math.random() * 4000;
        await sleep(stagger);
      }

      console.log(
        chalk.cyan(
          `\n🌊 Enjambre en vuelo. ${swarmPromises.length} nodos desplegados.\n`,
        ),
      );

      // Esperamos a que todos terminen (o fallen definitivamente)
      await Promise.allSettled(swarmPromises);

      console.log(
        chalk.yellow(
          "\n⏳ CICLO DE MANTENIMIENTO ACTIVO. Monitorizando nodos...",
        ),
      );

      // Bucle infinito para mantener el proceso vivo (Docker/CI) hasta timeout externo
      // Monitoriza la salud cada 5 minutos
      setInterval(
        () => {
          const mem = process.memoryUsage();
          console.log(
            `[${new Date().toISOString()}] Heartbeat | Heap: ${(mem.heapUsed / 1024 / 1024).toFixed(2)} MB`,
          );
        },
        1000 * 60 * 5,
      );
    } catch (err) {
      console.error("🔥 FATAL MAIN LOOP ERROR:", err);
      process.exit(1);
    }
  }

  main();
}

main();
// FIN DEL ARCHIVO
