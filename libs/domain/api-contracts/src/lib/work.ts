/**
 * =================================================================
 * APARATO: WORK DOMAIN CONTRACTS (V125.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN CONTRACT (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE MISIONES Y ESTRATEGIAS DE BÚSQUEDA
 *
 * VISION HIPER-HOLÍSTICA:
 * Sincroniza los esquemas de búsqueda con el motor matemático L1.
 * Implementa la unión discriminada para estrategias secuenciales,
 * forenses (Satoshi-XP) y de diccionario.
 * =================================================================
 */

import { z } from "zod";

/**
 * Clasificación cronológica del set UTXO.
 */
export const TargetStrataSchema = z.enum([
  "SatoshiEra",
  "VulnerableLegacy",
  "StandardLegacy",
  "FullTacticalSet",
]);

export type TargetStrata = z.infer<typeof TargetStrataSchema>;

/**
 * Esquema para la Estrategia de Búsqueda.
 * Sincronizado con #[serde(tag = "strategy_type", content = "parameters")] de Rust.
 */
export const SearchStrategySchema = z.discriminatedUnion("strategy_type", [
  z.object({
    strategy_type: z.literal("Sequential"),
    parameters: z.object({
      start_index_hexadecimal: z.string(),
      end_index_hexadecimal: z.string(),
    }),
  }),
  z.object({
    strategy_type: z.literal("SatoshiWindowsXpForensic"),
    parameters: z.object({
      scenario_template_identifier: z.string(),
      uptime_seconds_start: z.number(),
      uptime_seconds_end: z.number(),
      hardware_clock_frequency: z.number(),
    }),
  }),
  z.object({
    strategy_type: z.literal("Dictionary"),
    parameters: z.object({
      dataset_resource_locator: z.string(),
      processing_batch_size: z.number(),
    }),
  }),
]);

export type SearchStrategy = z.infer<typeof SearchStrategySchema>;

/**
 * Contrato de Orden de Trabajo (Misión Táctica).
 */
export const WorkOrderSchema = z.object({
  job_mission_identifier: z.string().uuid(),
  lease_duration_seconds: z.number(),
  strategy: SearchStrategySchema,
  required_strata: TargetStrataSchema,
});

export type WorkOrder = z.infer<typeof WorkOrderSchema>;
