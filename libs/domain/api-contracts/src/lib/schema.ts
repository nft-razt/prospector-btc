/**
 * =================================================================
 * APARATO: DOMAIN UNIFIED SCHEMAS (V70.1 - RECOVERY SYNC)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE ESQUEMAS DE TELEMETRÍA Y AUDITORÍA
 * =================================================================
 */

import { z } from "zod";

/**
 * Esquema de Reporte de Auditoría Inmutable.
 */
export const AuditReportSchema = z.object({
  job_mission_identifier: z.string().uuid(),
  worker_node_identifier: z.string(),
  computational_effort_volume: z.string(),
  execution_duration_milliseconds: z.number().nonnegative(),
  final_mission_status: z.string(),
  audit_footprint_checkpoint: z.string(),
  completed_at_timestamp: z.string().datetime(),
});

export type AuditReport = z.infer<typeof AuditReportSchema>;

/**
 * Esquema de segmentos para el Mapa de Calor.
 */
export const SwarmHeatmapSegmentSchema = z.object({
  normalized_start: z.number().min(0).max(1),
  intensity: z.number().min(0).max(1),
  mission_id: z.string().uuid(),
});

export type SwarmHeatmapSegment = z.infer<typeof SwarmHeatmapSegmentSchema>;

/**
 * Esquema de Métricas de Hardware Globales.
 */
export const SystemMetricsSchema = z.object({
  active_nodes_count: z.number().int().nonnegative(),
  cumulative_global_hashrate: z.number().nonnegative(),
  active_missions_in_flight: z.number().int().nonnegative(),
  timestamp_ms: z.number().positive(),
});

export type SystemMetrics = z.infer<typeof SystemMetricsSchema>;

/**
 * ✅ RESOLUCIÓN TS2305: Esquemas de Nodo (Snapshot y Heartbeat)
 */
export const WorkerSnapshotSchema = z.object({
  worker_id: z.string(),
  status: z.string(),
  snapshot_base64: z.string(),
  timestamp: z.string(),
});

export type WorkerSnapshot = z.infer<typeof WorkerSnapshotSchema>;

export const WorkerHeartbeatSchema = z.object({
  worker_id: z.string().uuid(),
  hostname: z.string(),
  hashrate: z.number().int().nonnegative(),
  current_job_id: z.string().uuid().nullable(),
  timestamp: z.string().datetime(),
  cpu_frequency_mhz: z.number(),
  cpu_load_percent: z.number(),
  thermal_celsius: z.number(),
  memory_used_bytes: z.number(),
  core_count: z.number(),
});

export type WorkerHeartbeat = z.infer<typeof WorkerHeartbeatSchema>;

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
