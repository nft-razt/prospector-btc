/**
 * =================================================================
 * APARATO: KAGGLE VECTOR CONTROLLER (V10.0 - SOBERANO)
 * CLASIFICACIÓN: OPS MECHANICS (ESTRATO L6)
 * RESPONSABILIDAD: ORQUESTACIÓN DE IGNICIÓN EN KAGGLE KERNELS
 * =================================================================
 */

import { Page } from "playwright";
import { createCursor, GhostCursor } from "ghost-cursor-playwright";
import chalk from "chalk";
import { Sentinel } from "./mechanics/sentinel";
import { generate_mission_payload } from "./payload";

export class KaggleController {
  private worker_id: string;
  private prefix: string;
  private sentinel: Sentinel;
  private cursor: GhostCursor | null = null;

  constructor(private page: Page, index: number, email: string | null) {
    this.worker_id = `hydra-kaggle-${index}`;
    this.prefix = chalk.blue(`[${this.worker_id}]`);
    this.sentinel = new Sentinel(page, this.worker_id, email, this.prefix);
  }

  /**
   * Ejecuta la secuencia de despliegue en Kaggle.
   */
  public async deploy_ignition(): Promise<void> {
    try {
      this.cursor = await createCursor(this.page);

      console.log(`${this.prefix} 🛰️ Aproximando a Vector Kaggle...`);
      await this.page.goto("https://www.kaggle.com/code/new", {
        timeout: 60000,
        waitUntil: "networkidle"
      });

      // 1. Verificación de Seguridad
      if (this.page.url().includes("login")) {
        throw new Error("KAGGLE_AUTH_FAILURE: Session expired or invalid cookies.");
      }

      // 2. Inyección de Payload (Silent Stealth Mode)
      const editor_selector = ".cm-content";
      await this.page.waitForSelector(editor_selector);

      const payload = generate_mission_payload(this.worker_id);

      await this.page.click(editor_selector);
      await this.page.keyboard.press("Control+A");
      await this.page.keyboard.press("Backspace");

      // Inyección vía Portapapeles (Evasión de plataforma)
      await this.page.evaluate((text) => navigator.clipboard.writeText(text), payload);
      await this.page.keyboard.press("Control+V");
      await this.page.waitForTimeout(1000);

      // 3. Ignición
      console.log(`${this.prefix} 🔥 Disparando Kernel...`);
      await this.page.keyboard.press("Control+Enter");

      this.sentinel.startHeartbeat();
      console.log(`${this.prefix} 🟢 VECTOR_KAGGLE_OPERATIONAL.`);

    } catch (error: any) {
      console.error(`${this.prefix} 🔴 FALLO DE VECTOR: ${error.message}`);
      await this.sentinel.captureFrame("error");
      throw error;
    }
  }
}
