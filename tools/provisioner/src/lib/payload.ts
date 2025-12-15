// tools/provisioner/src/lib/payload.ts
// =================================================================
// APARATO: PAYLOAD ENGINE (TEMPLATE BASED)
// RESPONSABILIDAD: GENERACIÓN DE CÓDIGO MINERO ESTABLE
// ESTADO: COMPATIBILIDAD UNIVERSAL (ES2019 SAFE)
// =================================================================

import * as fs from "fs";
import * as path from "path";
import { config } from "../config";

/**
 * Carga el template de Python y reemplaza las variables de configuración.
 * Utiliza una estrategia de inyección segura agnóstica de la versión de ES.
 *
 * @param workerId - Identificador único del nodo.
 * @returns Código Python listo para inyección.
 */
export function generateMinerPayload(workerId: string): string {
  try {
    // 1. Resolver ruta del template (Assets estáticos)
    // Se asume que el archivo miner_template.py existe en ../assets/
    const templatePath = path.resolve(__dirname, "../assets/miner_template.py");

    if (!fs.existsSync(templatePath)) {
      throw new Error(`Template no encontrado en: ${templatePath}`);
    }

    // 2. Leer contenido crudo del template
    let content = fs.readFileSync(templatePath, "utf-8");

    // 3. Definición de Variables de Inyección
    const replacements: Record<string, string> = {
      "{{MINER_BINARY_URL}}": config.MINER_BINARY_URL,
      "{{ORCHESTRATOR_URL}}": config.ORCHESTRATOR_URL,
      "{{WORKER_AUTH_TOKEN}}": config.WORKER_AUTH_TOKEN,
      "{{WORKER_ID}}": workerId,
    };

    // 4. Ejecución del Reemplazo (Patrón Universal)
    for (const [key, value] of Object.entries(replacements)) {
      // CORRECCIÓN CRÍTICA: Usamos split().join() en lugar de replaceAll()
      // Esto evita errores de compilación TS2550 en targets antiguos (ES2019/ES2020)
      // y garantiza el reemplazo global de todas las ocurrencias.
      content = content.split(key).join(value);
    }

    // 5. Firma de Integridad (Header)
    const signature = `PROSPECTOR-GEN-${Date.now().toString(16).toUpperCase()}`;
    return `# SIGNATURE: ${signature}\n${content}`;
  } catch (error: any) {
    console.error("❌ Error generando payload:", error.message);
    // Propagamos el error para detener el despliegue del worker defectuoso
    throw new Error("Payload Generation Failed");
  }
}
