import { Page, FrameLocator } from 'playwright';
import { config } from '../config';
import { generateMinerPayload } from './payload';
import chalk from 'chalk';

export class ColabController {
  private page: Page;
  private workerId: string;
  private prefix: string;

  constructor(page: Page, index: number) {
    this.page = page;
    this.workerId = `colab-node-${index}-${Date.now().toString().slice(-4)}`;
    this.prefix = chalk.cyan(`[${this.workerId}]`);
  }

  async deploy() {
    try {
      await this.navigate();
      await this.checkAuth();
      await this.connectRuntime();
      await this.injectAndRun();
      console.log(`${this.prefix} ${chalk.green('✅ DESPLIEGUE EXITOSO. MINERO ACTIVO.')}`);
    } catch (e: any) {
      console.error(`${this.prefix} ${chalk.red('❌ FALLO DE DESPLIEGUE:')}`, e.message);
      if (config.DEBUG_MODE) {
        await this.page.screenshot({ path: `logs/error-${this.workerId}.png` });
      }
      throw e;
    }
  }

  private async navigate() {
    console.log(`${this.prefix} Navegando a Colab...`);
    await this.page.goto(config.COLAB_URL, {
      waitUntil: 'domcontentloaded',
      timeout: config.NAV_TIMEOUT
    });
  }

  private async checkAuth() {
    // Buscamos el avatar del usuario. Si no está, probablemente no estamos logueados.
    try {
      await this.page.locator('# header-user-avatar-button').or(this.page.locator('img[src*="googleusercontent.com"]')).first().waitFor({ state: 'visible', timeout: 15000 });
    } catch {
      throw new Error('AUTH_ERROR: No se detectó sesión activa.');
    }
  }

  private async connectRuntime() {
    console.log(`${this.prefix} 🔌 Solicitando Runtime...`);

    // Selectores resilientes basados en texto y roles, no en clases ofuscadas
    const connectBtn = this.page.getByText(/^Connect$|^Reconnect$/i).first();

    if (await connectBtn.isVisible()) {
      await connectBtn.click();
    }

    // Esperamos a que aparezca el indicador de recursos (RAM/Disk)
    // Esto confirma que Google nos dio una VM.
    await this.page.waitForSelector('colab-memory-usage-sparkline', { timeout: 45000 })
      .catch(() => { console.warn(`${this.prefix} ⚠️ Timeout esperando recursos UI, pero continuamos...`); });
  }

  private async injectAndRun() {
    console.log(`${this.prefix} 💉 Inyectando Payload...`);

    // Enfocamos el editor de código
    const editor = this.page.locator('.view-lines').first();
    await editor.click({ force: true });

    // Limpiamos celda
    await this.page.keyboard.press('Control+A');
    await this.page.keyboard.press('Delete');

    // Pegamos el código (más rápido y seguro que typear)
    const payload = generateMinerPayload(this.workerId);
    await this.page.evaluate((text) => navigator.clipboard.writeText(text), payload);
    await this.page.keyboard.press('Control+V');

    // Ejecutamos (Ctrl+Enter)
    await this.page.keyboard.press('Control+Enter');

    // Validación visual de inicio
    // Buscamos dentro del iframe de salida
    const outputFrame = this.page.frameLocator('iframe').last();
    await outputFrame.getByText('PROSPECTOR PAYLOAD').first().waitFor({ state: 'visible', timeout: 20000 });
  }
}
