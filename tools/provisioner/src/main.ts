import { BrowserFactory } from './lib/browser';
import { ColabController } from './lib/colab';
import { config } from './config';
import chalk from 'chalk';

async function main() {
  console.log(chalk.bold.green('⚡ PROSPECTOR PROVISIONER v3.0 (SWISS CLOCK EDITION)'));
  console.log(`🎯 Targets: ${config.WORKER_COUNT} | 🕵️ Headless: ${config.HEADLESS}`);

  const { browser, context } = await BrowserFactory.createContext();

  const deployments = [];

  // Lanzamiento escalonado para no saturar CPU local ni levantar flags en Google
  for (let i = 0; i < config.WORKER_COUNT; i++) {
    const page = await context.newPage();
    const controller = new ColabController(page, i);

    // No usamos await aquí para lanzar en paralelo, pero controlamos el batch
    const deployment = controller.deploy().catch(err => {
      console.error(chalk.red(`[Worker-${i}] Murió:`), err.message);
      page.close(); // Limpieza de recursos
    });

    deployments.push(deployment);

    // Pausa táctica entre lanzamientos (3s)
    await new Promise(r => setTimeout(r, 3000));
  }

  // Esperamos a que todos intenten desplegar
  await Promise.allSettled(deployments);

  console.log(chalk.yellow('\n⏳ MANTENIENDO SESIÓN VIVA. NO CERRAR.'));

  // Keep-alive loop para evitar cierre de proceso
  // En el futuro, aquí podemos añadir lógica de "Healthcheck" y "Respawn"
  setInterval(() => {
    process.stdout.write('.');
  }, 10000);
}

main().catch(err => {
  console.error('🔥 FATAL:', err);
  process.exit(1);
});
