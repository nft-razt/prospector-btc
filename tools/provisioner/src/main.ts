// tools/provisioner/src/main.ts
// =================================================================
// APARATO: PROVISIONER ENTRY POINT
// RESPONSABILIDAD: INICIO DEL ENJAMBRE Y CONTROL DE CONCURRENCIA
// ESTADO: OPTIMIZADO (IDENTITY AWARE)
// =================================================================

import { BrowserFactory } from './lib/browser';
import { ColabController } from './lib/colab';
import { config } from './config';
import chalk from 'chalk';

async function main() {
  console.log(chalk.bold.green('⚡ PROSPECTOR PROVISIONER v3.5 (HYDRA-ZERO)'));
  console.log(`🎯 Targets: ${config.WORKER_COUNT} | 🕵️ Headless: ${config.HEADLESS}`);

  // 1. Crear contexto con Identidad inyectada
  const { browser, context, identityEmail } = await BrowserFactory.createContext();

  if (identityEmail) {
      console.log(chalk.blue(`👤 Operando bajo identidad: ${identityEmail}`));
  } else {
      console.log(chalk.yellow('👤 Operando en modo ANÓNIMO / LOCAL'));
  }

  const deployments = [];

  // 2. Lanzamiento escalonado
  for (let i = 0; i < config.WORKER_COUNT; i++) {
    const page = await context.newPage();

    // Inyectamos el email de identidad para el "Kill Switch"
    const controller = new ColabController(page, i, identityEmail);

    const deployment = controller.deploy().catch(err => {
      console.error(chalk.red(`[Worker-${i}] Fallo crítico:`), err.message);
      // No cerramos la página inmediatamente para permitir depuración visual si no es headless
      if (config.HEADLESS) page.close();
    });

    deployments.push(deployment);

    // Pausa táctica entre lanzamientos (3s) para evitar rate-limiting de creación de instancias
    await new Promise(r => setTimeout(r, 3000));
  }

  // 3. Espera de despliegue
  await Promise.allSettled(deployments);

  console.log(chalk.yellow('\n⏳ MANTENIENDO SESIÓN VIVA. PRESIONA CTRL+C PARA DETENER.'));

  // 4. Keep-alive Loop (Evita que el proceso de Node muera)
  setInterval(() => {
    // Heartbeat local en terminal
    // process.stdout.write('.');
  }, 10000);
}

main().catch(err => {
  console.error('🔥 FATAL MAIN LOOP:', err);
  process.exit(1);
});
