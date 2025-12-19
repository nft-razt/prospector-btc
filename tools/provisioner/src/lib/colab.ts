/**
 * =================================================================
 * APARATO: COLAB CONTROLLER (V38.1 - HANDSHAKE ALIGNED)
 * CLASIFICACIÓN: COMPOSITE CONTROLLER (L6)
 * RESPONSABILIDAD: ORQUESTACIÓN DE DESPLIEGUE EN GOOGLE COLAB
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

  constructor(
    private page: Page,
    index: number,
    identityEmail: string | null,
  ) {
    this.workerId = `hydra-node-${index}`;
    this.prefix = chalk.cyan(`[${this.workerId}]`);
    this.sentinel = new Sentinel(
      page,
      this.workerId,
      identityEmail,
      this.prefix,
    );
  }

  /**
   * Ejecuta la secuencia de despliegue inyectando material criptográfico.
   *
   * @param masterKey Llave maestra para el motor de descifrado del worker.
   */
  public async deploy(masterKey: string): Promise<void> {
    try {
      this.cursor = await createCursor(this.page);
      this.navigator = new ColabNavigator(this.page, this.cursor, this.prefix);

      console.log(`${this.prefix} 🛰️ Iniciando aproximación a Colab...`);
      await this.navigator.approachTarget();

      // 1. Verificación de Muro de Autenticación
      const isAuthWallVisible = await this.navigator.detectAuthWall();
      if (isAuthWallVisible) {
        await this.sentinel.triggerKillSwitch("AUTH_WALL_DETECTED");
        throw new Error("RECOIL: Authentication required for this identity.");
      }

      // 2. Adquisición de Recursos de Computación
      await this.navigator.acquireRuntime();

      // 3. Inyección y Ejecución (Fase de Ignición)
      await this.injectAndRun(masterKey);

      // 4. Activación de Vigilancia Activa
      this.sentinel.startHeartbeat();
      console.log(`${this.prefix} 🟢 IGNICIÓN EXITOSA: Nodo operando en red.`);
    } catch (e: any) {
      console.error(`${this.prefix} 🔴 FALLO DE DESPLIEGUE: ${e.message}`);
      await this.sentinel.captureFrame("error");
      this.sentinel.stop();
      throw e;
    }
  }

  /**
   * Realiza la inyección del código Python en el editor Monaco.
   */
  private async injectAndRun(masterKey: string): Promise<void> {
    const editor = this.page.locator(SELECTORS.EDITOR.LINE).first();
    await editor.waitFor({ state: "visible", timeout: 15000 });

    // Enfoque y limpieza de celda
    if (this.cursor) {
      await this.cursor.click(editor);
    } else {
      await editor.click();
    }

    await this.page.keyboard.press("Control+A");
    await this.page.keyboard.press("Backspace");

    // ✅ RESOLUCIÓN: Ahora pasamos 2 argumentos, alineado con el Payload Engine V42.1
    const payload = generateMinerPayload(this.workerId, masterKey);

    // Inyección vía Portapapeles (Estrategia de evasión anti-bot)
    await this.page.evaluate(
      (text) => window.navigator.clipboard.writeText(text),
      payload,
    );

    await this.page.keyboard.press("Control+V");
    await this.page.waitForTimeout(500);

    // Ejecución de la celda (Ignición)
    await this.page.keyboard.press("Control+Enter");
  }
}
