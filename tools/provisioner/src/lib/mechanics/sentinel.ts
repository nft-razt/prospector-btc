/**
 * =================================================================
 * APARATO: SENTINEL MECHANIC (V4.5)
 * RESPONSABILIDAD: SURVEILLANCE & INCIDENT MANAGEMENT
 * ESTADO: HARDENED IDENTITY PROTECTION
 * =================================================================
 */

import { Page } from "playwright";
import axios from "axios";
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
   * Transmite el estado visual al Panóptico del Dashboard.
   */
  public async captureFrame(
    status: "running" | "error" | "captcha",
  ): Promise<void> {
    if (!config.ORCHESTRATOR_URL) return;

    try {
      const buffer = await this.page.screenshot({ type: "jpeg", quality: 20 });
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
          timeout: 5000,
        },
      );
    } catch (e: any) {
      // Fail silently for network jitter
    }
  }

  /**
   * PROTOCOLO DE EXTERMINIO DE IDENTIDAD (KILL-SWITCH).
   * Invocado ante la detección de muros de autenticación o errores 401/403.
   */
  public async triggerKillSwitch(reason: string): Promise<void> {
    if (!this.identityEmail || this.identityEmail.includes("local")) {
      console.warn(
        `${this.prefix} ⚠️ Identity is local/anonymous. Kill-switch bypassed.`,
      );
      return;
    }

    try {
      console.log(
        `${this.prefix} 💀 TRIGGERING KILL-SWITCH for ${this.identityEmail}. Reason: ${reason}`,
      );

      // Notificamos al Orquestador para que marque la identidad como 'revoked'
      await axios.post(
        `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/revoke`,
        { email: this.identityEmail },
        {
          headers: { Authorization: `Bearer ${config.WORKER_AUTH_TOKEN}` },
          timeout: 5000,
        },
      );

      console.log(`${this.prefix} ✅ Identity purged from active pool.`);

      // Captura forense del momento del baneo
      await this.captureFrame("error");
    } catch (e: any) {
      console.error(`${this.prefix} ❌ Kill-switch failure: ${e.message}`);
    }
  }

  public startHeartbeat(): void {
    this.surveillanceInterval = setInterval(
      () => this.captureFrame("running"),
      60000,
    );
  }

  public stop(): void {
    if (this.surveillanceInterval) clearInterval(this.surveillanceInterval);
  }
}
