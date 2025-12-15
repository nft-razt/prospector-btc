// libs/domain/api-contracts/src/lib/lab.ts
/**
 * =================================================================
 * APARATO: LAB CONTRACTS (SSoT)
 * RESPONSABILIDAD: DEFINICIONES DE TIPOS PARA EL LABORATORIO DE PRUEBAS
 * ALCANCE: GESTIÓN DE ESCENARIOS Y VERIFICACIÓN MANUAL (INTERCEPTOR)
 * ESTADO: PRODUCTION READY (FULL SCHEMA)
 * =================================================================
 */

import { z } from "zod";

// =================================================================
// 1. ENTIDAD DE DOMINIO: TEST SCENARIO
// Refleja la estructura de la tabla 'test_scenarios' en Turso.
// =================================================================

export const TestScenarioSchema = z.object({
  /** ID único del escenario (UUID v4) */
  id: z.string().uuid(),

  /** Nombre descriptivo para identificación humana (ej: "Alpha Test") */
  name: z.string(),

  /** La frase semilla o secreto original (Input) */
  secret_phrase: z.string(),

  /** La dirección Bitcoin esperada (Output Derivado) */
  target_address: z.string(),

  /** Estado del ciclo de vida del escenario */
  status: z.enum(["idle", "active", "verified"]),

  /** Fecha de creación (ISO 8601) */
  created_at: z.string().datetime(),

  /** Fecha de verificación por un worker (si aplica) */
  verified_at: z.string().datetime().nullable().optional(),
});

export type TestScenario = z.infer<typeof TestScenarioSchema>;

// =================================================================
// 2. DTO: CREACIÓN DE ESCENARIO
// Payload enviado por 'ScenarioCreator' al Backend.
// =================================================================

export const CreateScenarioSchema = z.object({
  name: z
    .string()
    .min(3, "Name is too short (min 3 chars)")
    .max(50, "Name is too long"),

  secret_phrase: z
    .string()
    .min(5, "Phrase is too weak (min 5 chars) for meaningful testing"),
});

export type CreateScenarioPayload = z.infer<typeof CreateScenarioSchema>;

// =================================================================
// 3. DTO: VERIFICACIÓN DE ENTROPÍA (THE INTERCEPTOR)
// Payload enviado por 'ManualVerifier' para probar el motor matemático.
// =================================================================

export const VerifyEntropySchema = z.object({
  /**
   * El secreto a probar. Puede ser una frase (brainwallet),
   * una clave privada en Hex, o WIF.
   */
  secret: z.string().min(1, "Input required"),

  /**
   * Hint para el parser del backend sobre cómo interpretar el secreto.
   * Por defecto 'phrase' para brainwallets simples.
   */
  type: z.enum(["phrase", "private_key_hex", "wif"]).default("phrase"),
});

export type VerifyEntropyPayload = z.infer<typeof VerifyEntropySchema>;

// =================================================================
// 4. DTO: RESULTADO DE VERIFICACIÓN
// Respuesta del Orquestador tras la simulación de hallazgo.
// =================================================================

export const EntropyResultSchema = z.object({
  /** Dirección derivada matemáticamente por el backend (P2PKH Legacy) */
  address: z.string(),

  /** Clave privada en formato WIF comprimido/descomprimido */
  wif: z.string(),

  /**
   * Verdadero si esta dirección existe en la tabla 'test_scenarios'
   * o en la lista de objetivos reales (si se implementa).
   */
  is_target: z.boolean(),

  /**
   * Nombre del escenario coincidente, si 'is_target' es true.
   * Útil para feedback visual: "Linked to Scenario: Alpha Test"
   */
  matched_scenario: z.string().nullable().optional(),
});

export type EntropyResult = z.infer<typeof EntropyResultSchema>;
