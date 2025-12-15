/**
 * =================================================================
 * APARATO: COLAB CONTROLLER (ORCHESTRATOR)
 * CLASIFICACIÓN: COMPOSITE CONTROLLER
 * RESPONSABILIDAD: COORDINACIÓN DE MECÁNICAS DE DESPLIEGUE
 * ESTADO: ATOMIZADO & OPTIMIZADO (SCOPE FIX)
 * =================================================================
 */

import { Page } from "playwright";
import { createCursor, GhostCursor } from "ghost-cursor-playwright";
import chalk from "chalk";

import { SELECTORS } from "./selectors";
import { generateMinerPayload } from "./payload";
import { ColabNavigator } from "./mechanics/navigator";
import { Sentinel } from "./mechanics/sentinel";

/**
 * Controlador soberano de una instancia de Google Colab.
 * Coordina navegación, inyección y vigilancia delegando en mecánicas especializadas.
 */
export class ColabController {
  private workerId: string;
  private prefix: string;
  private cursor: GhostCursor | null = null;

  // Mecánicas Delegadas
  private navigator: ColabNavigator | null = null;
  private sentinel: Sentinel;

  constructor(
    private page: Page,
    index: number,
    identityEmail: string | null,
  ) {
    // Generación de ID único global para trazabilidad
    this.workerId = `hydra-node-${index}-${Date.now().toString().slice(-5)}`;
    this.prefix = chalk.cyan(`[${this.workerId}]`);

    // Inicialización del Centinela (siempre activo para logs)
    this.sentinel = new Sentinel(
      page,
      this.workerId,
      identityEmail,
      this.prefix,
    );
  }

  /**
   * Ejecuta la secuencia de despliegue completa (Pipeline).
   */
  async deploy(): Promise<void> {
    try {
      console.log(`${this.prefix} Iniciando secuencia de despliegue...`);

      // 1. Inicialización de Motor Humano (Ghost Cursor)
      this.cursor = await createCursor(this.page);

      // 2. Inicialización del Navegador con Cursor
      this.navigator = new ColabNavigator(this.page, this.cursor, this.prefix);

      // 3. Fase de Aproximación
      await this.navigator.approachTarget();

      // 4. Inspección de Seguridad (Auth Wall Check)
      const authBlocked = await this.navigator.detectAuthWall();
      if (authBlocked) {
        console.warn(
          `${this.prefix} ${chalk.bgRed.white.bold(" 🛡️ AUTH WALL DETECTADO ")}`,
        );
        await this.sentinel.triggerKillSwitch();
        throw new Error("AUTH_REQUIRED"); // Abortar flujo
      }

      // 5. Adquisición de Recursos (GPU/TPU)
      await this.navigator.acquireRuntime();

      // 6. Inyección de Payload (Polimórfico)
      await this.injectAndRun();

      // 7. Activación del Panóptico (Vigilancia Visual)
      this.sentinel.startSurveillance();

      console.log(
        `${this.prefix} ${chalk.green("✅ NODO OPERATIVO Y MINANDO.")}`,
      );
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : String(e);

      // Si es un error de Auth, lo propagamos para que el Main Loop decida (no reintentar rápido)
      if (msg === "AUTH_REQUIRED") {
        throw e;
      }

      console.error(
        `${this.prefix} ${chalk.red("❌ FALLO DE DESPLIEGUE:")} ${msg}`,
      );

      // Captura forense del error antes de morir
      await this.sentinel.captureFrame("error");
      this.sentinel.stopSurveillance();

      throw e;
    }
  }

  /**
   * Inyecta el código Python y ejecuta la celda.
   * Utiliza el portapapeles para evitar detección de tipeo sintético.
   * (Esta lógica se mantiene aquí por su simplicidad y acoplamiento directo al flujo principal).
   */
  private async injectAndRun(): Promise<void> {
    console.log(`${this.prefix} 💉 Inyectando vector minero...`);

    // 1. Enfocar Editor
    const editor = this.page.locator(SELECTORS.EDITOR.LINE).first();
    await editor.waitFor({ state: "visible", timeout: 15000 });

    if (this.cursor) await this.cursor.click(editor);
    else await editor.click();

    // 2. Limpieza de celda (Ctrl+A -> Del)
    await this.page.keyboard.press("Control+A");
    await this.page.keyboard.press("Backspace");

    // 3. Generación del Payload
    const payload = generateMinerPayload(this.workerId);

    // 4. Inyección vía Clipboard (Evasión de heurística de tipeo)
    // ✅ CORRECCIÓN: Uso de window.navigator para evitar conflicto con this.navigator
    await this.page.evaluate(
      (text) => window.navigator.clipboard.writeText(text),
      payload,
    );
    await this.page.keyboard.press("Control+V");

    // 5. Pausa Humana (Thinking Time)
    await this.page.waitForTimeout(1000 + Math.random() * 500);

    // 6. Ejecución (Ctrl+Enter)
    await this.page.keyboard.press("Control+Enter");
  }
}
