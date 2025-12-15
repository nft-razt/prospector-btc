/**
 * =================================================================
 * APARATO: COLAB NAVIGATOR (MECHANIC)
 * RESPONSABILIDAD: NAVEGACIÓN TÁCTICA Y ADQUISICIÓN DE RUNTIME
 * PATRÓN: HEURÍSTICA DE REINTENTOS
 * =================================================================
 */

import { Page } from "playwright";
import { GhostCursor } from "ghost-cursor-playwright";
import chalk from "chalk";
import { SELECTORS } from "../selectors";
import { config } from "../../config";

export class ColabNavigator {
  constructor(
    private page: Page,
    private cursor: GhostCursor | null,
    private prefix: string,
  ) {}

  /**
   * Ejecuta la secuencia de aproximación al objetivo.
   * Incluye movimiento entrópico del cursor para calentar la sesión.
   */
  public async approachTarget(): Promise<void> {
    await this.page.goto(config.COLAB_URL, {
      timeout: config.NAV_TIMEOUT,
      waitUntil: "domcontentloaded",
    });

    // Simulación de "duda humana" con el mouse
    if (this.cursor) {
      await this.cursor.move({
        x: Math.random() * 500 + 100,
        y: Math.random() * 500 + 100,
      });
    }
  }

  /**
   * Verifica si existe un muro de autenticación (Login Wall).
   * @returns true si se detecta bloqueo de login.
   */
  public async detectAuthWall(): Promise<boolean> {
    // Promise.race es eficiente, pero Promise.any es semánticamente mejor aquí
    // para buscar la *primera* coincidencia positiva.
    try {
      await Promise.any(
        SELECTORS.AUTH.SIGN_IN_BTN.map((selector) =>
          this.page.waitForSelector(selector, {
            timeout: 2000,
            state: "visible",
          }),
        ),
      );
      return true;
    } catch {
      return false; // Ningún selector de login fue encontrado
    }
  }

  /**
   * Intenta negociar la asignación de una VM (Connect).
   * Utiliza una estrategia de fuerza bruta sobre múltiples selectores.
   */
  public async acquireRuntime(): Promise<void> {
    console.log(`${this.prefix} 🔌 Negociando asignación de VM...`);

    let connected = false;

    // 1. Intento de Clic en botón de conexión
    for (const selector of SELECTORS.RUNTIME.CONNECT_BTN) {
      try {
        const btn = this.page.locator(selector).first();
        if (await btn.isVisible({ timeout: 1000 })) {
          console.log(`${this.prefix} Interactuando con: ${selector}`);
          if (this.cursor) {
            await this.cursor.click(btn);
          } else {
            await btn.click();
          }
          connected = true;
          break; // Click exitoso
        }
      } catch {
        continue; // Probar siguiente selector
      }
    }

    if (!connected) {
      console.log(
        `${this.prefix} ℹ️ Botón 'Connect' no visible. Asumiendo auto-conexión o estado conectado.`,
      );
    }

    // 2. Espera activa de recursos (Confirmación de Éxito)
    try {
      await this.page.waitForSelector(SELECTORS.RUNTIME.RESOURCE_MONITOR, {
        timeout: 45000,
      });
      console.log(`${this.prefix} ⚡ Recursos asignados. Runtime activo.`);
    } catch (e) {
      console.warn(
        `${this.prefix} ⚠️ Timeout esperando confirmación visual de RAM. Procediendo bajo riesgo.`,
      );
    }
  }
}
