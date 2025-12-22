/**
 * =================================================================
 * APARATO: DOMAIN UNIFIED SCHEMAS (V70.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE ESQUEMAS DE TELEMETRÍA Y AUDITORÍA
 * =================================================================
 */

import { z } from "zod";

/**
 * Esquema de Reporte de Auditoría Inmutable.
 * Representa la certificación final de una misión de búsqueda.
 */
export const AuditReportSchema = z.object({
  job_mission_identifier: z.string().uuid(),
  worker_node_identifier: z.string(),
  computational_effort_volume: z.string().describe("Volumen de hashes en representación string"),
  execution_duration_milliseconds: z.number().nonnegative(),
  final_mission_status: z.string(),
  audit_footprint_checkpoint: z.string().describe("Último escalar procesado en hex"),
  completed_at_timestamp: z.string().datetime(),
});

/** Tipo inferido del reporte de auditoría */
export type AuditReport = z.infer<typeof AuditReportSchema>;

/**
 * Esquema de segmentos para el Mapa de Calor (Heatmap).
 */
export const SwarmHeatmapSegmentSchema = z.object({
  normalized_start: z.number().min(0).max(1),
  intensity: z.number().min(0).max(1),
  mission_id: z.string().uuid(),
});

/** Tipo inferido del segmento de calor */
export type SwarmHeatmapSegment = z.infer<typeof SwarmHeatmapSegmentSchema>;

/**
 * Esquema de Métricas de Hardware de Alta Frecuencia.
 */
export const SystemMetricsSchema = z.object({
  active_nodes_count: z.number().int().nonnegative(),
  cumulative_global_hashrate: z.number().nonnegative(),
  active_missions_in_flight: z.number().int().nonnegative(),
  timestamp_ms: z.number().positive(),
});

/** Tipo inferido de métricas globales */
export type SystemMetrics = z.infer<typeof SystemMetricsSchema>;

/**
 * CONTRATO MAESTRO DE EVENTOS (RealTimeEvent)
 */
export const RealTimeEventSchema = z.discriminatedUnion("t", [
  z.object({ t: z.literal("sp"), p: SystemMetricsSchema }),
  z.object({ t: z.literal("ac"), p: AuditReportSchema }),
  z.object({ t: z.literal("sh"), p: z.array(SwarmHeatmapSegmentSchema) }),
  z.object({ t: z.literal("cc"), p: z.object({ target_address: z.string(), discovery_node: z.string() }) }),
  z.object({ t: z.literal("ad"), p: z.object({ drift_gap: z.number(), total_tactical: z.number() }) }),
]);

export type RealTimeEvent = z.infer<typeof RealTimeEventSchema>;
