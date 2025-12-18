/**
 * =================================================================
 * APARATO: COLAB CONTROLLER (V38.0 - SECURE HANDSHAKE)
 * CLASIFICACIÓN: COMPOSITE CONTROLLER (L6)
 * RESPONSABILIDAD: ORQUESTACIÓN DE DESPLIEGUE CON AUTODEFENSA ZK
 * ESTADO: GOLD MASTER // NO ABBREVIATIONS
 * =================================================================
 */

import { Page } from "playwright";
import { createCursor, GhostCursor } from "ghost-cursor-playwright";
import chalk from "chalk";

import { SELECTORS } from "./selectors";
import { ColabNavigator } from "./mechanics/navigator";
import { Sentinel } from "./mechanics/sentinel";
import { generateMinerPayload } from "./payload";

export class ColabController {
  private workerId: string;
  private prefix: string;
  private sentinel: Sentinel;
  private navigator: ColabNavigator | null = null;
  private cursor: GhostCursor | null = null;

  constructor(private page: Page, index: number, identityEmail: string | null) {
    this.workerId = `hydra-node-${index}`;
    this.prefix = chalk.cyan(`[${this.workerId}]`);
    this.sentinel = new Sentinel(page, this.workerId, identityEmail, this.prefix);
  }

  /**
   * Ejecuta la secuencia de despliegue inyectando material criptográfico.
   *
   * @param masterKey Llave maestra para que el worker descifre su identidad in-memory.
   */
  public async deploy(masterKey: string): Promise<void> {
    try {
      this.cursor = await createCursor(this.page);
      this.navigator = new ColabNavigator(this.page, this.cursor, this.prefix);

      console.log(`${this.prefix} 🛰️  Navegando a Runtime de Colab...`);
      await this.navigator.approachTarget();

      // Validación de Muro de Autenticación
      const isAuthWallVisible = await this.navigator.detectAuthWall();
      if (isAuthWallVisible) {
        await this.sentinel.triggerKillSwitch("SESSION_EXPIRED_DETECTED");
        throw new Error("AUTH_REQUIRED_RECOIL");
      }

      // Adquisición de VM
      await this.navigator.acquireRuntime();

      // Inyección de Payload con MasterKey
      await this.injectAndRun(masterKey);

      this.sentinel.startHeartbeat();
      console.log(`${this.prefix} 🟢 IGNICIÓN EXITOSA: Nodo en escucha de tareas.`);

    } catch (e: any) {
      console.error(`${this.prefix} 🔴 FALLO CRÍTICO: ${e.message}`);
      await this.sentinel.captureFrame("error");
      this.sentinel.stop();
      throw e;
    }
  }

  /**
   * Realiza la inyección del payload Python incluyendo el secreto de descifrado.
   */
  private async injectAndRun(masterKey: string): Promise<void> {
    const editor = this.page.locator(SELECTORS.EDITOR.LINE).first();
    await editor.waitFor({ state: "visible", timeout: 15000 });

    if (this.cursor) {
        await this.cursor.click(editor);
    } else {
        await editor.click();
    }

    await this.page.keyboard.press("Control+A");
    await this.page.keyboard.press("Backspace");

    // ✅ NIVELACIÓN: El payload ahora incluye la llave maestra para el motor Rust
    const payload = generateMinerPayload(this.workerId, masterKey);

    await this.page.evaluate(
      (text) => window.navigator.clipboard.writeText(text),
      payload
    );

    await this.page.keyboard.press("Control+V");
    await this.page.waitForTimeout(500);
    await this.page.keyboard.press("Control+Enter");
  }
}
