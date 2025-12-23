/**
 * =================================================================
 * APARATO: COMMAND & CONTROL API ADAPTER (V1.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: ORQUESTACIÓN DE DESPLIEGUE REMOTO (C2)
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el puente de comunicación hacia los API Routes del
 * Dashboard que actúan como proxy hacia GitHub Actions.
 * Garantiza que el 'Swarm Launcher' pueda disparar misiones
 * de auditoría sin exponer tokens en el cliente.
 * =================================================================
 */

import { apiClient } from "./client";
import {
  type SwarmLaunchConfig,
  type WorkflowRun
} from "@prospector/api-contracts";

/**
 * Adaptador de mando táctico para la gestión de infraestructura remota.
 */
export const controlApi = {
  /**
   * Recupera el historial de ejecuciones del provisioner desde la nube.
   * Utilizado para visualizar el estado de salud de los runners de GitHub.
   *
   * @returns {Promise<WorkflowRun[]>} Lista de estados de ejecución.
   */
  getWorkflowRuns: async (): Promise<WorkflowRun[]> => {
    return await apiClient.get<WorkflowRun[]>("/github/runs");
  },

  /**
   * Dispara una secuencia de ignición del enjambre.
   * Envía la configuración de workers y shards al centro de mando.
   *
   * @param configuration - Parámetros de misión validados por Zod.
   * @returns {Promise<void>} Confirmación de despacho aceptada.
   */
  launchSwarm: async (configuration: SwarmLaunchConfig): Promise<void> => {
    return await apiClient.post("/github/dispatch", configuration);
  }
};
