/**
 * =================================================================
 * APARATO: LABORATORY INFRASTRUCTURE ADAPTER (V47.0 - GOLD MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: ORQUESTACIÓN DE PRUEBAS Y CERTIFICACIÓN NEURAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el puente de comunicación hacia el estrato de laboratorio
 * del Orquestador Rust. Garantiza el tipado absoluto en las ráfagas
 * de validación de entropía y misiones de humo.
 * =================================================================
 */

import { apiClient } from "./client";
import {
  type CreateScenarioPayload,
  type TestScenario,
  type VerifyEntropyPayload,
  type EntropyResult,
  type VerifiedVectorAuditReport,
} from "@prospector/api-contracts"; // ✅ RESOLUCIÓN TS2305

/**
 * Respuesta del orquestador tras la ignición de una misión de certificación.
 */
export interface CertificationIgnitionResponse {
  /** UUID de la misión inyectada en el despacho. */
  mission_identifier: string;
  /** Estado operativo: IGNITED | QUEUED. */
  execution_status: "IGNITED" | "QUEUED";
}

/**
 * Interfaz de Servicio del Laboratorio Forense.
 */
export const labApi = {
  /**
   * Registra y cristaliza un nuevo escenario de prueba en el Ledger.
   *
   * @param payload Atributos del Golden Ticket (Nombre y Frase).
   */
  createScenario: async (payload: CreateScenarioPayload): Promise<TestScenario> => {
    return await apiClient.post<TestScenario>("/lab/scenarios", payload);
  },

  /**
   * Recupera el inventario íntegro de experimentos registrados.
   */
  listScenarios: async (): Promise<TestScenario[]> => {
    return await apiClient.get<TestScenario[]>("/lab/scenarios");
  },

  /**
   * Ejecuta el protocolo 'The Interceptor' para auditar vectores de entropía.
   * Realiza una derivación secp256k1 en tiempo real en el Orquestador.
   *
   * @param payload Vector de entrada y su tipo de codificación.
   * @returns Resultado del análisis forense y posibles colisiones.
   */
  verifyEntropy: async (payload: VerifyEntropyPayload): Promise<EntropyResult> => {
    return await apiClient.post<EntropyResult>("/lab/verify", payload);
  },

  /**
   * Dispara la misión de certificación de integridad Satoshi-XP.
   * Valida el funcionamiento del mezclador inyectando el Bloque 1.
   */
  triggerCertificationMission: async (): Promise<CertificationIgnitionResponse> => {
    return await apiClient.post<CertificationIgnitionResponse>("/lab/certification/ignite", {});
  },

  /**
   * Recupera los reportes de auditoría de red real para los 33 vectores.
   * Certifica que el sistema posee 'Verdad de Red'.
   */
  listForensicAuditReports: async (): Promise<VerifiedVectorAuditReport[]> => {
    return await apiClient.get<VerifiedVectorAuditReport[]>("/lab/audit/brainwallet-dataset");
  },
};
