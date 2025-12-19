/**
 * =================================================================
 * APARATO: LAB API ADAPTER (V28.0)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (L4)
 * RESPONSABILIDAD: COMUNICACIÓN CON EL MÓDULO DE LABORATORIO
 * ESTADO: FIXED // ZERO ABBREVIATIONS
 * =================================================================
 */

import { apiClient } from "./client";
import {
  type CreateScenarioPayload,
  type TestScenario,
  type VerifyEntropyPayload,
  type EntropyResult,
} from "@prospector/api-contracts";

/**
 * Adaptador para operaciones de laboratorio y verificación de entropía.
 * Provee la interfaz lógica para interactuar con "The Interceptor".
 */
export const labApi = {
  /**
   * Registra un nuevo Golden Ticket en el ledger táctico.
   *
   * @param payload Atributos del escenario a cristalizar.
   */
  createScenario: async (
    payload: CreateScenarioPayload,
  ): Promise<TestScenario> => {
    return await apiClient.post<TestScenario>("/lab/scenarios", payload);
  },

  /**
   * Recupera el inventario completo de experimentos activos.
   */
  listScenarios: async (): Promise<TestScenario[]> => {
    return await apiClient.get<TestScenario[]>("/lab/scenarios");
  },

  /**
   * Ejecuta un escaneo de verificación sobre un vector de entropía.
   *
   * @param payload Datos del vector a auditar (frase, hex o wif).
   * @returns Resultado del análisis criptográfico y su estatus de colisión.
   */
  verifyEntropy: async (
    payload: VerifyEntropyPayload,
  ): Promise<EntropyResult> => {
    // RESOLUCIÓN Error 2305: Ahora los tipos son reconocidos globalmente
    return await apiClient.post<EntropyResult>("/lab/verify", payload);
  },
};
