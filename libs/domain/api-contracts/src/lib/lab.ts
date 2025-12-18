/**
 * =================================================================
 * APARATO: LAB CONTRACT DEFINITIONS (V28.0)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (L2)
 * RESPONSABILIDAD: DEFINICIÓN DE PRUEBAS Y VERIFICACIÓN NEURAL
 * ESTADO: GOLD MASTER // SSoT
 * =================================================================
 */

import { z } from "zod";

/**
 * Estados posibles de un experimento de colisión controlada en el Ledger.
 */
export type ScenarioStatus = "idle" | "active" | "verified";

/**
 * Esquema para la creación de un nuevo Golden Ticket.
 */
export const CreateScenarioSchema = z.object({
  name: z.string().min(3).max(64).describe("Nombre de la operación designada"),
  secret_phrase: z.string().min(8).describe("Frase de entropía original"),
});

export type CreateScenarioPayload = z.infer<typeof CreateScenarioSchema>;

/**
 * Representación atómica de un Escenario de Prueba en el sistema.
 */
export interface TestScenario {
  id: string;
  name: string;
  secret_phrase: string;
  target_address: string;
  target_private_key: string;
  status: ScenarioStatus;
  created_at: string;
  verified_at?: string | null;
}

/**
 * ✅ RESOLUCIÓN Error 2305: Esquema de petición para The Interceptor.
 * Define la estructura para auditar vectores de entrada arbitrarios.
 */
export const VerifyEntropySchema = z.object({
  secret: z.string().min(1).describe("Vector de entrada (frase, hex o wif)"),
  type: z.enum(["phrase", "hex", "wif"]).default("phrase"),
});

export type VerifyEntropyPayload = z.infer<typeof VerifyEntropySchema>;

/**
 * ✅ RESOLUCIÓN Error 2305: Contrato de salida del motor forense.
 * Sincronizado con la respuesta del Orquestador en Rust.
 */
export interface EntropyResult {
  /** Dirección derivada en formato Base58Check */
  address: string;
  /** Clave privada en formato WIF (Wallet Import Format) */
  wif: string;
  /** Indica si existe una colisión con un escenario de prueba */
  is_target: boolean;
  /** Nombre del escenario coincidente, si aplica */
  matched_scenario: string | null;
}
