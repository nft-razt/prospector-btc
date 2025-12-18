/**
 * =================================================================
 * APARATO: ARCHIVAL DATA CONTRACTS
 * CLASIFICACIÓN: DOMAIN CONTRACTS (L2)
 * RESPONSABILIDAD: DEFINICIÓN DE ESTRUCTURAS PARA EL ARCHIVO ESTRATÉGICO
 * ESTADO: ELITE TYPING (SSoT)
 * =================================================================
 */

import { z } from "zod";

/**
 * Representación de un trabajo finalizado y migrado al archivo frío.
 * Utilizado para el histórico de auditoría y reportes de tesis.
 */
export const ArchivedJobSchema = z.object({
  id: z.string().uuid(),
  workspace_id: z.string().uuid(),

  // Metadatos de búsqueda (U256 Padded Strings)
  range_start: z.string().length(78),
  range_end: z.string().length(78),
  strategy_type: z.enum(["Combinatoric", "Dictionary", "Kangaroo", "Forensic"]),

  // Métricas de Rendimiento
  total_hashes: z.string().describe("Total de iteraciones realizadas"),
  duration_seconds: z.number().int().positive(),
  average_hashrate: z.number().nonnegative(),

  // Auditoría de Hallazgos
  findings_count: z.number().int().default(0),

  // Línea de Tiempo
  created_at: z.string().datetime(),
  archived_at: z.string().datetime(),
});

export type ArchivedJob = z.infer<typeof ArchivedJobSchema>;
