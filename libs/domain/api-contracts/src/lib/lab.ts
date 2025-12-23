/**
 * =================================================================
 * APARATO: LAB DOMAIN CONTRACTS (V14.6 - GOLD MASTER)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE PRUEBAS Y CERTIFICACIÓN FORENSE
 * =================================================================
 */

import { z } from "zod";

/** Estatus del ciclo de vida de un escenario de prueba. */
export type ScenarioStatus = "idle" | "active" | "verified";

/**
 * Esquema de validación para la creación de Golden Tickets.
 * ✅ NIVELACIÓN SOBERANA: Nombres de campos extendidos.
 */
export const CreateScenarioSchema = z.object({
  /** Nombre táctico de la operación. */
  operation_name: z.string().min(3).max(64),
  /** Frase semilla en claro para la derivación del vector. */
  entropy_seed_phrase: z.string().min(8),
});

/** Carga útil para la creación de escenarios. */
export type CreateScenarioPayload = z.infer<typeof CreateScenarioSchema>;

/** Contrato de un Escenario de Prueba persistido. */
export interface TestScenario {
  identifier: string;
  operation_name: string;
  entropy_seed_phrase: string;
  target_bitcoin_address: string;
  target_private_key_wif: string;
  current_status: ScenarioStatus;
  crystallized_at: string;
  verified_at_timestamp?: string | null;
}

/**
 * Esquema para el Interceptor de Entropía.
 * ✅ RESOLUCIÓN TS2305: Miembro ahora exportado nominalmente.
 */
export const VerifyEntropySchema = z.object({
  entropy_vector: z.string().min(1),
  vector_type: z.enum(["phrase", "hex", "wif"]).default("phrase"),
});

export type VerifyEntropyPayload = z.infer<typeof VerifyEntropySchema>;

/**
 * Resultado del análisis forense de entropía.
 * ✅ RESOLUCIÓN TS2305: Miembro ahora exportado nominalmente.
 */
export interface EntropyResult {
  derived_bitcoin_address: string;
  derived_wallet_import_format: string;
  is_target_collision: boolean;
  matched_scenario_name: string | null;
}

/** Reporte consolidado de auditoría de red real. */
export interface VerifiedVectorAuditReport {
  vector_identifier: number;
  source_passphrase: string;
  derived_wallet_import_format: string;
  derived_bitcoin_address: string;
  mathematical_integrity_verified: boolean;
  network_reality_data?: {
    final_balance_satoshis: number;
    total_received_satoshis: number;
    confirmed_transaction_count: number;
  };
}
