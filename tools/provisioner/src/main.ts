// tools/provisioner/src/main.ts
// =================================================================
// APARATO: PROVISIONER ENTRY POINT
// RESPONSABILIDAD: ORQUESTACIÓN RESILIENTE DEL ENJAMBRE
// ESTADO: OPTIMIZADO (BACKOFF EXPONENCIAL + ERROR AISLADO)
// =================================================================

import { BrowserFactory } from './lib/browser';
import { ColabController } from './lib/colab';
import { config } from './config';
import chalk from 'chalk';

// Configuración de Resiliencia
const MAX_RETRIES = 3;
const BASE_DELAY_MS = 2000;

async function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function deployWorkerWithRetry(
  context: any,
  index: number,
  identityEmail: string | null,
  attempt: number = 1
): Promise<void> {
  const workerPrefix = `[Worker-${index}]`;

  try {
    const page = await context.newPage();
    const controller = new ColabController(page, index, identityEmail);

    console.log(chalk.blue(`${workerPrefix} Iniciando despliegue (Intento ${attempt}/${MAX_RETRIES})...`));
    await controller.deploy();

    // Si llegamos aquí, éxito. No cerramos la página para mantener vivo el worker.

  } catch (err: any) {
    console.error(chalk.red(`${workerPrefix} Fallo: ${err.message}`));

    if (attempt < MAX_RETRIES) {
      // Backoff Exponencial: 2s, 4s, 8s...
      const delay = BASE_DELAY_MS * Math.pow(2, attempt - 1);
      console.log(chalk.yellow(`${workerPrefix} Reintentando en ${delay/1000}s...`));
      await sleep(delay);
      return deployWorkerWithRetry(context, index, identityEmail, attempt + 1);
    } else {
      console.error(chalk.bgRed.white(`${workerPrefix} 💀 ABANDONADO tras ${MAX_RETRIES} intentos.`));
      // No lanzamos throw para no romper Promise.all del enjambre principal
    }
  }
}

async function main() {
  console.log(chalk.bold.green('⚡ PROSPECTOR PROVISIONER v3.6 (RESILIENT SWARM)'));
  console.log(`🎯 Targets: ${config.WORKER_COUNT} | 🕵️ Headless: ${config.HEADLESS}`);

  try {
    // 1. Contexto Global (Browser)
    const { browser, context, identityEmail } = await BrowserFactory.createContext();

    if (identityEmail) {
        console.log(chalk.blue(`👤 Identidad Activa: ${identityEmail}`));
    } else {
        console.log(chalk.yellow('👤 Modo: ANÓNIMO / LOCAL'));
    }

    // 2. Lanzamiento Paralelo con Control de Concurrencia
    // Usamos un array de promesas, pero las disparamos con un pequeño delay inicial
    // para no saturar la CPU/Red al abrir muchas pestañas a la vez.
    const swarmPromises = [];

    for (let i = 0; i < config.WORKER_COUNT; i++) {
      // "Fire and Forget" gestionado. Cada worker maneja sus propios reintentos.
      const p = deployWorkerWithRetry(context, i, identityEmail);
      swarmPromises.push(p);

      // Staggering (Escalonamiento) de 2 segundos entre inicios
      await sleep(2000);
    }

    // 3. Monitorización
    console.log(chalk.cyan(`\n🌊 Enjambre lanzado. ${swarmPromises.length} procesos en vuelo.\n`));

    // Esperamos a que todos terminen sus intentos de despliegue (éxito o fallo final)
    await Promise.allSettled(swarmPromises);

    console.log(chalk.yellow('\n⏳ MANTENIENDO SESIÓN VIVA. Monitorizando nodos activos...'));

    // Keep-alive loop para Docker
    setInterval(() => {}, 1000 * 60 * 60);

  } catch (err) {
    console.error('🔥 FATAL MAIN LOOP:', err);
    process.exit(1);
  }
}

main();
