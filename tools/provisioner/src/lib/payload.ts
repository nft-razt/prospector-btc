/**
 * =================================================================
 * APARATO: HYDRA PAYLOAD CRYSTALLIZER (V45.0)
 * CLASIFICACIÓN: OPS CONTROL (L6)
 * RESPONSABILIDAD: GENERACIÓN DE CÓDIGO PYTHON CON INYECCIÓN SOBERANA
 *
 * ESTRATEGIA DE ÉLITE:
 * - Mapping de 7 puntos: Sincronización total con el template miner_template.py.
 * - Zero-Regression: Mantiene la firma de integridad para el Panóptico.
 * - String Fragmentation: Reemplazo global determinista.
 * =================================================================
 */

import * as fs from "fs";
import * as path from "path";
import { config } from "../config";

/**
 * Transforma el template estático de Python en un script de misión activo.
 *
 * @param worker_node_identifier Identificador único del nodo en el enjambre.
 * @returns El código fuente Python listo para ser pegado en la celda de Colab.
 */
export function generate_mission_payload(worker_node_identifier: string): string {
  try {
    // 1. Resolución de la ruta del activo estático (Template V10.8)
    const template_file_path = path.resolve(__dirname, "../assets/miner_template.py");

    if (!fs.existsSync(template_file_path)) {
      throw new Error(`CRITICAL_MISSING_ASSET: Template not found at ${template_file_path}`);
    }

    let python_content = fs.readFileSync(template_file_path, "utf-8");

    // 2. Diccionario de Inyección Estratégica (Mapping V10.8)
    const injection_map: Record<string, string> = {
      "{{MINER_BINARY_URL}}": config.MINER_BINARY_URL,
      "{{ORCHESTRATOR_URL}}": config.ORCHESTRATOR_URL,
      "{{WORKER_AUTH_TOKEN}}": config.WORKER_AUTH_TOKEN,
      "{{MASTER_VAULT_KEY}}": config.MASTER_VAULT_KEY,
      "{{FILTER_BASE_URL}}": config.FILTER_BASE_URL,
      "{{FILTER_SHARDS}}": config.FILTER_SHARDS.toString(),
      "{{WORKER_ID}}": worker_node_identifier,
    };

    // 3. Ejecución del Reemplazo Determinista
    for (const [placeholder, actual_value] of Object.entries(injection_map)) {
      // Utilizamos split/join para reemplazo global compatible con todos los entornos
      python_content = python_content.split(placeholder).join(actual_value);
    }

    // 4. Inyección de Metadatos de Auditoría
    const mission_signature = `HYDRA-ZK-IGNITION-ID-${Date.now().toString(16).toUpperCase()}`;

    return [
      `# =================================================`,
      `# MISSION_SIGNATURE: ${mission_signature}`,
      `# NODE_ID: ${worker_node_identifier}`,
      `# =================================================`,
      python_content
    ].join("\n");

  } catch (error: any) {
    console.error("🔥 [PAYLOAD_FAULT]: Failed to crystallize mission payload:", error.message);
    throw new Error("MAPPING_FAILURE: Strategic variables could not be injected.");
  }
}
