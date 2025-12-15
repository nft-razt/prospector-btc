/**
 * =================================================================
 * APARATO: SENTINEL (MECHANIC)
 * RESPONSABILIDAD: VIGILANCIA VISUAL Y GESTIÓN DE INCIDENTES
 * INTEGRACIÓN: RUTAS SEGREGADAS (ADMIN vs INGEST)
 * =================================================================
 */

import { Page } from "playwright";
import axios from "axios";
import chalk from "chalk";
import { config } from "../../config";

export class Sentinel {
  private surveillanceInterval: NodeJS.Timeout | null = null;

  constructor(
    private page: Page,
    private workerId: string,
    private identityEmail: string | null,
    private prefix: string,
  ) {}

  /**
   * Inicia el bucle de transmisión visual al Dashboard.
   * Utiliza la ruta optimizada de ingestión para no saturar el backend.
   */
  public startSurveillance(): void {
    if (this.surveillanceInterval) clearInterval(this.surveillanceInterval);

    // Frecuencia: 45s (Balance entre visibilidad y ancho de banda)
    this.surveillanceInterval = setInterval(
      () => this.captureFrame("running"),
      45000,
    );

    // Primer frame inmediato (Boot confirmation)
    this.captureFrame("running").catch(() => {});
  }

  public stopSurveillance(): void {
    if (this.surveillanceInterval) {
      clearInterval(this.surveillanceInterval);
      this.surveillanceInterval = null;
    }
  }

  /**
   * Captura y transmite un frame del estado actual.
   */
  public async captureFrame(
    status: "running" | "error" | "captcha",
  ): Promise<void> {
    if (!config.ORCHESTRATOR_URL) return;

    try {
      // Optimización: JPEG calidad 25% reduce payload drásticamente (~50KB)
      const buffer = await this.page.screenshot({ type: "jpeg", quality: 25 });
      const base64 = `data:image/jpeg;base64,${buffer.toString("base64")}`;

      await axios.post(
        `${config.ORCHESTRATOR_URL}/api/v1/ingest/worker-snapshot`,
        {
          worker_id: this.workerId,
          status,
          snapshot_base64: base64,
          timestamp: new Date().toISOString(),
        },
        {
          headers: { Authorization: `Bearer ${config.WORKER_AUTH_TOKEN}` },
          timeout: 5000, // Fail-fast
        },
      );
    } catch (e: any) {
      if (config.DEBUG_MODE) {
        console.warn(
          `${this.prefix} Frame droppeado (Network/Backpressure): ${e.message}`,
        );
      }
    }
  }

  /**
   * KILL SWITCH: Protocolo de revocación de identidad comprometida.
   * Se ejecuta cuando se detecta un Auth Wall o fallo crítico de sesión.
   */
  public async triggerKillSwitch(): Promise<void> {
    if (!config.ORCHESTRATOR_URL || !this.identityEmail) return;

    try {
      console.log(
        `${this.prefix} 💀 KILL SWITCH ACTIVADO para: ${this.identityEmail}`,
      );

      await axios.post(
        `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/revoke`,
        { email: this.identityEmail },
        {
          headers: { Authorization: `Bearer ${config.WORKER_AUTH_TOKEN}` },
          timeout: 5000,
        },
      );

      console.log(`${this.prefix} ✅ Identidad revocada en la Bóveda Central.`);
    } catch (e: any) {
      console.error(
        `${this.prefix} ❌ Fallo al reportar revocación: ${e.message}`,
      );
    }
  }
}
