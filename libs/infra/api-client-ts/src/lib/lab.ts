/**
 * =================================================================
 * APARATO: LAB API ADAPTER (V45.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: ORQUESTACIÓN DE PRUEBAS Y CERTIFICACIÓN NEURAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el puente de comunicación hacia el estrato de laboratorio
 * del Orquestador. Centraliza la lógica de "The Interceptor" para
 * validación de entropía y el protocolo de ignición de misiones
 * de certificación (Smoke Tests) requeridas para la Tesis Doctoral.
 * =================================================================
 */

import { apiClient } from "./client";
import {
  type CreateScenarioPayload,
  type TestScenario,
  type VerifyEntropyPayload,
  type EntropyResult
} from "@prospector/api-contracts";

/**
 * Representa la respuesta táctica tras disparar una misión de certificación.
 */
export interface CertificationIgnitionResponse {
  /** Identificador único universal (UUID) de la misión generada. */
  mission_id: string;
  /** Estado de inserción en el buffer: IGNITED | QUEUED. */
  status: "IGNITED" | "QUEUED";
}

/**
 * Adaptador de infraestructura para el Laboratorio Forense y QA.
 */
export const labApi = {
  /**
   * Registra y cristaliza un nuevo Golden Ticket en el ledger táctico.
   * Transforma una frase semilla en un escenario de prueba auditable por el enjambre.
   *
   * @param payload - Atributos validados del escenario (Designación y Frase).
   * @returns Una promesa con la entidad TestScenario persistida.
   */
  createScenario: async (payload: CreateScenarioPayload): Promise<TestScenario> => {
    return await apiClient.post<TestScenario>("/lab/scenarios", payload);
  },

  /**
   * Recupera el inventario completo de experimentos criptográficos registrados.
   * Permite al operador visualizar la cobertura de pruebas activas.
   *
   * @returns Una promesa con la colección de escenarios y su estatus de verificación.
   */
  listScenarios: async (): Promise<TestScenario[]> => {
    return await apiClient.get<TestScenario[]>("/lab/scenarios");
  },

  /**
   * Ejecuta el protocolo "The Interceptor" para auditar vectores de entrada.
   * Realiza una derivación secp256k1 en tiempo real contra el censo UTXO.
   *
   * @param payload - Datos del vector a auditar (frase, hex o wif).
   * @returns Resultado del análisis forense indicando si existe una colisión.
   */
  verifyEntropy: async (payload: VerifyEntropyPayload): Promise<EntropyResult> => {
    return await apiClient.post<EntropyResult>("/lab/verify", payload);
  },

  /**
   * PROTOCOLO DE ÉLITE: Dispara la misión de certificación de integridad (Smoke Test).
   * Inyecta el Golden Vector (Bloque 1 / QPC Conocido) en el buffer de despacho
   * para validar el correcto funcionamiento del mezclador Satoshi-XP en el worker.
   *
   * @returns Promesa con los metadatos de la misión de certificación iniciada.
   */
  triggerCertificationMission: async (): Promise<CertificationIgnitionResponse> => {
    return await apiClient.post<CertificationIgnitionResponse>("/lab/certification/ignite", {});
  },

  /**
   * Consulta el estatus de una misión de certificación específica en el Ledger.
   * Permite al Dashboard verificar si el Golden Vector ha sido recuperado.
   *
   * @param mission_id - Identificador único de la misión de prueba.
   * @returns Estatus detallado de la misión y huella forense asociada.
   */
  getCertificationStatus: async (mission_id: string): Promise<any> => {
    return await apiClient.get(`/lab/certification/status/${mission_id}`);
  }
};
