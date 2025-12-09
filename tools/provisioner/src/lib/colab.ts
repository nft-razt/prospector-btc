// tools/provisioner/src/lib/colab.ts
// =================================================================
// APARATO: COLAB CONTROLLER v3.5 (SELF-HEALING & TYPESAFE)
// RESPONSABILIDAD: ORQUESTACIÓN DE NAVEGADOR, INYECCIÓN Y REPORTE DE ESTADO
// =================================================================

import { Page } from 'playwright';
import { createCursor, GhostCursor } from 'ghost-cursor-playwright';
import axios from 'axios';
import chalk from 'chalk';
import { config } from '../config';
import { generateMinerPayload } from './payload';

/**
 * Controlador autónomo para una instancia de Google Colab.
 * Maneja el ciclo de vida completo: Navegación -> Auth Check -> Inyección -> Vigilancia.
 */
export class ColabController {
  private page: Page;
  private workerId: string;
  private prefix: string;

  /** Cursor humano sintético para evasión de detección de bots */
  private cursor: GhostCursor | null = null;

  /** Intervalo de vigilancia visual */
  private surveillanceInterval: NodeJS.Timeout | null = null;

  /** Email de la identidad en uso (para reporte de fallos) */
  private identityEmail: string | null;

  /**
   * Inicializa el controlador.
   * @param page Instancia de página de Playwright.
   * @param index Índice del worker para logs.
   * @param identityEmail Email asociado a las cookies inyectadas.
   */
  constructor(page: Page, index: number, identityEmail: string | null) {
    this.page = page;
    this.identityEmail = identityEmail;
    this.workerId = `colab-node-${index}-${Date.now().toString().slice(-4)}`;
    this.prefix = chalk.cyan(`[${this.workerId}]`);
  }

  /**
   * Ejecuta la secuencia de despliegue completa.
   * Lanza excepciones si la secuencia falla para permitir reintentos o limpieza.
   */
  async deploy(): Promise<void> {
    try {
      // 1. Inicializar Cursor (Movimiento no lineal)
      this.cursor = await createCursor(this.page);

      // 2. Fase de Aproximación
      await this.navigate();

      // 3. Fase de Verificación de Identidad (Kill Switch)
      await this.checkAuth();

      // 4. Fase de Conexión de Recursos (GPU/TPU)
      await this.connectRuntime();

      // 5. Fase de Inyección de Código (Payload)
      await this.injectAndRun();

      // 6. Fase de Vigilancia (Heartbeat Visual)
      this.startSurveillance();

      console.log(`${this.prefix} ${chalk.green('✅ DESPLIEGUE COMPLETADO Y OPERATIVO.')}`);

    } catch (e: unknown) {
      const errorMsg = e instanceof Error ? e.message : String(e);
      console.error(`${this.prefix} ${chalk.red('❌ ERROR CRÍTICO DE DESPLIEGUE:')}`, errorMsg);

      // Reporte forense visual (Snapshot del error)
      await this.reportSnapshot('error');

      throw e;
    }
  }

  private async navigate(): Promise<void> {
    console.log(`${this.prefix} Navegando a zona objetivo...`);

    await this.page.goto(config.COLAB_URL, {
      waitUntil: 'domcontentloaded',
      timeout: config.NAV_TIMEOUT
    });

    // Movimiento humano aleatorio
    if (this.cursor) {
      await this.cursor.move({ x: Math.random() * 500, y: Math.random() * 500 });
    }
  }

  /**
   * Verifica si la sesión es válida.
   * Si detecta el botón de "Sign in", reporta la identidad como muerta (REVOKED).
   */
  private async checkAuth(): Promise<void> {
    try {
      // Buscamos indicadores de sesión cerrada
      const signInBtn = this.page.getByText(/^Sign in$/i);

      // Si el botón es visible, la sesión ha muerto.
      if (await signInBtn.isVisible({ timeout: 5000 })) {
        console.warn(`${this.prefix} ${chalk.yellow('⚠️ SESIÓN CADUCADA DETECTADA')}`);

        // AUTO-HEALING: Reportar muerte de identidad al Orquestador
        if (this.identityEmail) {
            await this.reportIdentityDeath(this.identityEmail);
        }

        throw new Error('COOKIES CADUCADAS: Login requerido.');
      }
    } catch (e) {
      // Si el timeout salta (no encontró el botón), es buena señal (estamos logueados).
      // Si el error es el que lanzamos nosotros, lo propagamos.
      if (e instanceof Error && e.message.includes('COOKIES CADUCADAS')) {
          throw e;
      }
    }
  }

  private async connectRuntime(): Promise<void> {
    console.log(`${this.prefix} 🔌 Conectando Runtime (GPU/TPU)...`);

    // Selectores resilientes
    const connectBtn = this.page.getByText(/^Connect$|^Reconnect$/i).first();

    if (await connectBtn.isVisible()) {
        if (this.cursor) await this.cursor.click(connectBtn);
        else await connectBtn.click();
    }

    try {
        await this.page.waitForSelector('colab-memory-usage-sparkline', { timeout: 45000 });
    } catch {
        console.warn(`${this.prefix} ⚠️ No se confirmó visualmente la conexión, procediendo a ciegas...`);
    }
  }

  private async injectAndRun(): Promise<void> {
    console.log(`${this.prefix} 💉 Inyectando Payload Minero...`);

    const editor = this.page.locator('.view-lines').first();
    await editor.waitFor({ state: 'visible' });

    if (this.cursor) await this.cursor.click(editor);
    else await editor.click();

    // Limpieza
    await this.page.keyboard.press('Control+A');
    await this.page.keyboard.press('Delete');

    // Inyección
    const payload = generateMinerPayload(this.workerId);
    await this.page.evaluate((text) => navigator.clipboard.writeText(text), payload);
    await this.page.keyboard.press('Control+V');

    await this.page.waitForTimeout(1000);
    await this.page.keyboard.press('Control+Enter');
  }

  /**
   * Notifica al Orquestador que una identidad ha dejado de funcionar.
   * Esto previene que otros workers intenten usar estas credenciales muertas.
   */
  private async reportIdentityDeath(email: string): Promise<void> {
      if (!config.ORCHESTRATOR_URL) return;

      console.log(`${this.prefix} 💀 Reportando identidad muerta: ${email}`);
      try {
          await axios.post(
              `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/revoke`,
              { email },
              { headers: { 'Authorization': `Bearer ${config.WORKER_AUTH_TOKEN}` } }
          );
      } catch (e) {
          console.error(`${this.prefix} Fallo al reportar muerte de identidad (Orchestrator down?).`);
      }
  }

  private startSurveillance(): void {
    if (this.surveillanceInterval) clearInterval(this.surveillanceInterval);

    // Snapshot cada 15s
    this.surveillanceInterval = setInterval(async () => {
      try {
        await this.reportSnapshot('running');
      } catch { /* Silent */ }
    }, 15000);
  }

  private async reportSnapshot(status: 'running' | 'error' | 'captcha'): Promise<void> {
    if (!config.ORCHESTRATOR_URL) return;

    try {
        const buffer = await this.page.screenshot({ quality: 30, type: 'jpeg' });
        const base64Image = `data:image/jpeg;base64,${buffer.toString('base64')}`;

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
                timeout: 5000
            }
        );
    } catch (e) {
        // Ignorar fallos de red en telemetría
    }
  }
}
