/**
 * =================================================================
 * APARATO: LAB DOMAIN CONTRACTS (V13.0 - GOLD MASTER)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE PRUEBAS Y CERTIFICACIÓN FORENSE
 * =================================================================
 */

import { z } from "zod";

/**
 * Estados del ciclo de vida de un escenario de prueba en el Ledger.
 */
export type ScenarioStatus = "idle" | "active" | "verified";

/**
 * Esquema de validación para la creación de Golden Tickets.
 */
export const CreateScenarioSchema = z.object({
  /** Nombre táctico de la operación (ej: SATOSHI-BLOCK-1). */
  operation_name: z.string().min(3).max(64),
  /** Frase semilla en claro para la derivación del vector. */
  entropy_seed_phrase: z.string().min(8),
});

/** Carga útil para la creación de escenarios. */
export type CreateScenarioPayload = z.infer<typeof CreateScenarioSchema>;

/**
 * Contrato de un Escenario de Prueba persistido.
 */
export interface TestScenario {
  /** Identificador único universal (UUID). */
  identifier: string;
  /** Nombre designado para el experimento. */
  operation_name: string;
  /** Frase de entropía original (Custodiada en el Ledger de Laboratorio). */
  entropy_seed_phrase: string;
  /** Dirección Bitcoin Legacy (P2PKH) resultante. */
  target_bitcoin_address: string;
  /** Clave privada en formato WIF comprimido. */
  target_private_key_wif: string;
  /** Estado actual del experimento. */
  current_status: ScenarioStatus;
  /** Marca de tiempo de cristalización. */
  crystallized_at: string;
  /** Marca de tiempo de la verificación exitosa (opcional). */
  verified_at_timestamp?: string | null;
}

/**
 * Esquema de validación para el Interceptor de Entropía Manual.
 * ✅ RESOLUCIÓN TS2305: Miembro ahora definido y exportado nominalmente.
 */
export const VerifyEntropySchema = z.object({
  /** Vector de entrada (Frase, Hexadecimal o WIF). */
  entropy_vector: z.string().min(1),
  /** Tipo de codificación del vector. */
  vector_type: z.enum(["phrase", "hex", "wif"]).default("phrase"),
});

/** Carga útil para la verificación de entropía. */
export type VerifyEntropyPayload = z.infer<typeof VerifyEntropySchema>;

/**
 * Resultado forense del análisis de un vector de entropía.
 * ✅ RESOLUCIÓN TS2305: Miembro ahora definido y exportado nominalmente.
 */
export interface EntropyResult {
  /** Dirección pública derivada en formato Base58Check. */
  derived_bitcoin_address: string;
  /** Clave privada resultante en formato WIF. */
  derived_wallet_import_format: string;
  /** Indica si el vector colisiona con un objetivo del censo. */
  is_target_collision: boolean;
  /** Nombre del escenario coincidente, si existe. */
  matched_scenario_name: string | null;
}

/**
 * Reporte consolidado de auditoría de red real.
 */
export interface VerifiedVectorAuditReport {
  /** ID del vector en el dataset soberano. */
  vector_identifier: number;
  /** Frase de entropía fuente. */
  source_passphrase: string;
  /** Clave WIF generada. */
  derived_wallet_import_format: string;
  /** Dirección Bitcoin generada. */
  derived_bitcoin_address: string;
  /** Certificación de integridad matemática local. */
  mathematical_integrity_verified: boolean;
  /** Información de saldo y actividad recuperada de la red Bitcoin. */
  network_reality_data?: {
    final_balance_satoshis: number;
    total_received_satoshis: number;
    confirmed_transaction_count: number;
  };
}
