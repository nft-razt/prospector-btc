/**
 * =================================================================
 * APARATO: ADMIN API ADAPTER (V36.0)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (L4)
 * RESPONSABILIDAD: GESTIÓN DE IDENTIDADES Y ESTADO DE BÓVEDA
 * =================================================================
 */

import { apiClient } from "./client";
import { type Identity, type IdentityPayload } from "@prospector/api-contracts";

/**
 * Interfaz de administración para el operador Hydra.
 */
export const adminApi = {
  /**
   * Sube una identidad cifrada (Zero-Knowledge) a la base de datos táctica.
   * @param payload Datos de identidad incluyendo el Ciphertext AES-GCM.
   */
  uploadIdentity: async (payload: IdentityPayload): Promise<void> => {
    return await apiClient.post("/admin/identities", payload);
  },

  /**
   * Recupera el inventario de identidades activas.
   */
  listIdentities: async (): Promise<Identity[]> => {
    return await apiClient.get<Identity[]>("/admin/identities");
  },

  /**
   * Verifica la capacidad y salud de la Bóveda de Identidad.
   * Utilizado por el Pre-Flight Modal.
   */
  checkIdentityStatus: async (): Promise<{
    nodeCount: number;
    activeLeases: number;
  }> => {
    const identities = await adminApi.listIdentities();
    return {
      nodeCount: identities.length,
      activeLeases: identities.filter((id) => id.usage_count > 0).length,
    };
  },
};
