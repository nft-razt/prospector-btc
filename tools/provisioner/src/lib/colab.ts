// tools/provisioner/src/lib/colab.ts
import { Page } from 'playwright';
import { createCursor, GhostCursor } from 'ghost-cursor-playwright';
import axios from 'axios';
import chalk from 'chalk';
import { config } from '../config';
import { generateMinerPayload } from './payload';

// ATOMIZACIÓN: Diccionario de Selectores (Fácil de actualizar si Google cambia el UI)
const SELECTORS = {
    SIGN_IN_BTN: ['text=Sign in', 'a[href*="accounts.google.com"]'],
    CONNECT_BTN: ['#connect', 'text=Connect', 'text=Reconnect', 'colab-connect-button'],
    RUNTIME_MENU: 'colab-runtime-menu-button',
    CHANGE_RUNTIME: 'text=Change runtime type',
    GPU_RADIO: 'paper-radio-button[name="accelerator"][label="T4 GPU"]',
    EDITOR_LINE: '.view-lines',
    // Indicadores de estado
    RAM_DISK_BAR: 'colab-memory-usage-sparkline',
    BUSY_INDICATOR: 'colab-status-bar[status="busy"]'
};

export class ColabController {
  private page: Page;
  private workerId: string;
  private prefix: string;
  private cursor: GhostCursor | null = null;
  private surveillanceInterval: NodeJS.Timeout | null = null;
  private identityEmail: string | null;

  constructor(page: Page, index: number, identityEmail: string | null) {
    this.page = page;
    this.identityEmail = identityEmail;
    this.workerId = `hydra-node-${index}-${Date.now().toString().slice(-4)}`;
    this.prefix = chalk.cyan(`[${this.workerId}]`);
  }

  async deploy(): Promise<void> {
    try {
      this.cursor = await createCursor(this.page);

      await this.navigate();
      await this.checkAuth();
      // await this.configureRuntime(); // Opcional: Descomentar si necesitamos forzar GPU
      await this.connectRuntime();
      await this.injectAndRun();
      this.startSurveillance();

      console.log(`${this.prefix} ${chalk.green('✅ OPERATIVO.')}`);

    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : String(e);
      console.error(`${this.prefix} ${chalk.red('❌ FALLO:')} ${msg}`);
      await this.reportSnapshot('error');
      throw e;
    }
  }

  private async navigate(): Promise<void> {
    // console.log(`${this.prefix} Navigating...`);
    await this.page.goto(config.COLAB_URL, { timeout: config.NAV_TIMEOUT, waitUntil: 'domcontentloaded' });
    if (this.cursor) await this.cursor.move({ x: Math.random() * 500, y: Math.random() * 500 });
  }

  private async checkAuth(): Promise<void> {
    // Comprobación rápida: ¿Existe alguno de los botones de login?
    const loginSelector = SELECTORS.SIGN_IN_BTN.join(',');
    if (await this.page.isVisible(loginSelector, { timeout: 3000 })) {
        console.warn(`${this.prefix} ${chalk.yellow('⚠️ LOGIN REQUERIDO')}`);
        if (this.identityEmail) await this.reportIdentityDeath(this.identityEmail);
        throw new Error('AUTH_REQUIRED');
    }
  }

  private async connectRuntime(): Promise<void> {
    console.log(`${this.prefix} 🔌 Conectando...`);

    // Intentar encontrar cualquier botón de conexión
    for (const sel of SELECTORS.CONNECT_BTN) {
        try {
            const btn = this.page.locator(sel).first();
            if (await btn.isVisible({ timeout: 500 })) {
                if (this.cursor) await this.cursor.click(btn);
                else await btn.click();
                break; // Clickeado, salir del loop
            }
        } catch {}
    }

    // Esperar a que aparezca la barra de RAM/Disco (Confirmación de Runtime activo)
    try {
        await this.page.waitForSelector(SELECTORS.RAM_DISK_BAR, { timeout: 30000 });
    } catch {
        console.warn(`${this.prefix} ⚠️ No se confirmó conexión visual, continuando...`);
    }
  }

  private async injectAndRun(): Promise<void> {
    console.log(`${this.prefix} 💉 Inyectando...`);

    const editor = this.page.locator(SELECTORS.EDITOR_LINE).first();
    await editor.waitFor({ state: 'visible' });
    if (this.cursor) await this.cursor.click(editor);
    else await editor.click();

    // Limpieza agresiva del editor
    await this.page.keyboard.press('Control+A');
    await this.page.keyboard.press('Delete');

    // Inyección vía Clipboard (Más rápido y fiable que type)
    const payload = generateMinerPayload(this.workerId);
    await this.page.evaluate((text) => navigator.clipboard.writeText(text), payload);
    await this.page.keyboard.press('Control+V');

    await this.page.waitForTimeout(500);
    await this.page.keyboard.press('Control+Enter'); // Ejecutar celda
  }

  // ... (reportIdentityDeath, startSurveillance y reportSnapshot se mantienen igual que en el snapshot original, son correctos)

  private async reportIdentityDeath(email: string): Promise<void> {
      if (!config.ORCHESTRATOR_URL) return;
      try {
          await axios.post(
              `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/revoke`,
              { email },
              { headers: { 'Authorization': `Bearer ${config.WORKER_AUTH_TOKEN}` } }
          );
      } catch {}
  }

  private startSurveillance(): void {
    if (this.surveillanceInterval) clearInterval(this.surveillanceInterval);
    this.surveillanceInterval = setInterval(() => this.reportSnapshot('running'), 15000);
  }

  private async reportSnapshot(status: 'running' | 'error' | 'captcha'): Promise<void> {
    if (!config.ORCHESTRATOR_URL) return;
    try {
        const buffer = await this.page.screenshot({ quality: 20, type: 'jpeg' }); // Calidad baja para velocidad
        const base64 = `data:image/jpeg;base64,${buffer.toString('base64')}`;
        await axios.post(
            `${config.ORCHESTRATOR_URL}/api/v1/admin/worker-snapshot`,
            { worker_id: this.workerId, status, snapshot_base64: base64, timestamp: new Date().toISOString() },
            { headers: { 'Authorization': `Bearer ${config.WORKER_AUTH_TOKEN}` }, timeout: 3000 }
        );
    } catch {}
  }
}
