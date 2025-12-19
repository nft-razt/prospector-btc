/**
 * =================================================================
 * APARATO: PAYLOAD ENGINE (V42.1 - ZK INJECTION READY)
 * RESPONSABILIDAD: GENERACIÓN DE CÓDIGO MINERO CON SECRETO SOBERANO
 * ESTADO: COMPATIBILIDAD ES2019 // SYNCED WITH COLAB_CONTROLLER
 * =================================================================
 */

import * as fs from "fs";
import * as path from "path";
import { config } from "../config";

/**
 * Carga el template de Python y realiza la inyección de variables tácticas.
 *
 * @param workerId - Identificador único del nodo en el enjambre.
 * @param masterKey - Llave de descifrado para la Bóveda Zero-Knowledge.
 */
export function generateMinerPayload(
  workerId: string,
  masterKey: string,
): string {
  try {
    // 1. Resolución de ruta de activos estáticos
    const templatePath = path.resolve(__dirname, "../assets/miner_template.py");

    if (!fs.existsSync(templatePath)) {
      throw new Error(
        `CRITICAL_MISSING_ASSET: Template not found at ${templatePath}`,
      );
    }

    let content = fs.readFileSync(templatePath, "utf-8");

    // 2. Diccionario de inyección (Sincronizado con miner_template.py)
    const replacements: Record<string, string> = {
      "{{MINER_BINARY_URL}}": config.MINER_BINARY_URL,
      "{{ORCHESTRATOR_URL}}": config.ORCHESTRATOR_URL,
      "{{WORKER_AUTH_TOKEN}}": config.WORKER_AUTH_TOKEN,
      "{{MASTER_VAULT_KEY}}": masterKey, // ✅ RESOLUCIÓN: Inyección del secreto
      "{{WORKER_ID}}": workerId,
    };

    // 3. Reemplazo global mediante patrón de fragmentación (Universal Compatibility)
    for (const [key, value] of Object.entries(replacements)) {
      content = content.split(key).join(value);
    }

    // 4. Firma de integridad para logs de auditoría
    const signature = `PROSPECTOR-ZK-IGNITION-${Date.now().toString(16).toUpperCase()}`;
    return `# SIGNATURE: ${signature}\n${content}`;
  } catch (error: any) {
    console.error("🔥 [PAYLOAD_FAULT]:", error.message);
    throw new Error("FAILED_TO_CRYSTALLIZE_PAYLOAD");
  }
}
