/**
 * =================================================================
 * APARATO: PROVISIONER SWARM COMMANDER (V38.0 - HYPER-SPEED)
 * CLASIFICACIÓN: OPS CONTROL (L6)
 * RESPONSABILIDAD: IGNICIÓN PARALELA Y GESTIÓN DE IDENTIDAD ZK
 * ESTADO: GOLD MASTER // NO ABBREVIATIONS // TS-ERRORS FIXED
 * =================================================================
 */

import { BrowserFactory } from "./lib/browser";
import { ColabController } from "./lib/colab";
import { config } from "./config";
import chalk from "chalk";
// ✅ RESOLUCIÓN Error 2307: Importación de semáforo de concurrencia
import pLimit from "p-limit";

/**
 * Orquesta la ignición masiva del enjambre de minería.
 * Implementa un modelo de hilos paralelos para maximizar la velocidad de despliegue.
 */
async function main() {
  console.log(chalk.bold.green("\n🚀 PROSPECTOR HYDRA-IGNITION SEQUENCE :: V38.0"));
  // ✅ RESOLUCIÓN Error 2339: Reemplazo de 'zinc' por 'gray' (Canónico en Chalk 4)
  console.log(chalk.gray("--------------------------------------------------"));

  // 1. GESTIÓN DE CONCURRENCIA (Élite: 5 flujos simultáneos para evitar rate-limits)
  const ignitionSemaphore = pLimit(5);

  // 2. RECUPERACIÓN DE SECRETO MAESTRO
  const masterKey = process.env.MASTER_VAULT_KEY || "Netflix69";

  try {
    // Inicialización del motor de navegación con Fingerprinting Único
    const { browser, context, identityEmail } = await BrowserFactory.createContext();

    logInfo(`👤 OPERATOR_IDENTITY: ${identityEmail || "ANONYMOUS_SESSION"}`);
    logInfo(`🌊 TARGET_SWARM_SIZE: ${config.WORKER_COUNT} units`);

    // 3. MAPEO DE TAREAS PARALELIZADAS
    const deploymentSequence = Array.from({ length: config.WORKER_COUNT }).map((_, index) => {
      return ignitionSemaphore(async () => {
        const sequenceId = index + 1;
        const page = await context.newPage();
        const controller = new ColabController(page, sequenceId, identityEmail);

        try {
          // ✅ RESOLUCIÓN Error 2554: El contrato ahora exige y recibe la Master Key
          await controller.deploy(masterKey);
        } catch (error: any) {
          // El error se captura individualmente para no detener el resto del enjambre
