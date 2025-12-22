/**
 * =================================================================
 * APARATO: ARCHIVAL DATA CONTRACTS (V55.0 - INTEGRITY SYNC)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE ESTRUCTURAS PARA EL ARCHIVO ESTRATÉGICO
 * =================================================================
 */

import { z } from "zod";

/**
 * Representación de un trabajo finalizado y migrado al archivo frío.
 * Este esquema se utiliza exclusivamente para la visualización del histórico
 * en el Motor B (Supabase).
 */
export const ArchivedJobSchema = z.object({
  id: z.string().uuid(),
  workspace_id: z.string().uuid(),

  // Metadatos de búsqueda (U256 Padded Strings)
  range_start: z.string().length(78),
  range_end: z.string().length(78),
  strategy_type: z.enum(["Sequential", "Dictionary", "Kangaroo", "Forensic"]),

  // Métricas de Rendimiento consumidas desde el AuditReport original
  total_hashes: z.string().describe("Total de iteraciones realizadas"),
  duration_seconds: z.number().int().positive(),
  average_hashrate: z.number().nonnegative(),

  // Auditoría de Hallazgos
  findings_count: z.number().int().default(0),

  // Línea de Tiempo de Sincronización
  created_at: z.string().datetime(),
  archived_at: z.string().datetime(),
});

/** Tipo inferido para el archivo histórico */
export type ArchivedJob = z.infer<typeof ArchivedJobSchema>;
