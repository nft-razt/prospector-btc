/**
 * =================================================================
 * APARATO: DOMAIN UNIFIED SCHEMAS (V72.0 - GOLD MASTER)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE SEÑALES DE ALTA FIDELIDAD
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la Fuente Única de Verdad (SSoT) para la comunicación
 * entre el enjambre y el dashboard. Implementa validación estricta
 * para telemetría de hardware, mapas de calor y auditoría.
 * =================================================================
 */

import { z } from "zod";

/**
 * Métricas de hardware capturadas directamente del silicio del worker.
 */
export const NodeHardwareMetricsSchema = z.object({
  cpu_frequency_megahertz: z.number().nonnegative(),
  cpu_load_percentage: z.number().min(0).max(100),
  cpu_temperature_celsius: z.number(),
  ram_usage_megabytes: z.number().nonnegative(),
  is_thermal_throttling_active: z.boolean(),
});

export type NodeHardwareMetrics = z.infer<typeof NodeHardwareMetricsSchema>;

/**
 * Reporte de Auditoría Inmutable generado tras completar una misión.
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
 * Instantánea visual del navegador del worker para vigilancia visual.
 */
export const WorkerSnapshotSchema = z.object({
  worker_identifier: z.string(),
  operational_status: z.enum(["running", "captcha", "error", "idle"]),
  snapshot_base64_data: z.string(),
  captured_at_timestamp: z.string().datetime(),
  hardware_metrics: NodeHardwareMetricsSchema.optional(),
});

export type WorkerSnapshot = z.infer<typeof WorkerSnapshotSchema>;

/**
 * Latido de vida (Heartbeat) de alta frecuencia.
 */
export const WorkerHeartbeatSchema = z.object({
  worker_identifier: z.string().uuid(),
  hostname_identity: z.string(),
  current_hashrate: z.number().int().nonnegative(),
  active_job_identifier: z.string().uuid().nullable(),
  timestamp_utc: z.string().datetime(),
  hardware_metrics: NodeHardwareMetricsSchema,
});

export type WorkerHeartbeat = z.infer<typeof WorkerHeartbeatSchema>;

/**
 * Segmento de ocupación para el Mapa de Calor de la Curva.
 */
export const SwarmHeatmapSegmentSchema = z.object({
  normalized_start_position: z.number().min(0).max(1),
  intensity_weight: z.number().min(0).max(1),
  mission_identifier: z.string().uuid(),
});

export type SwarmHeatmapSegment = z.infer<typeof SwarmHeatmapSegmentSchema>;

/**
 * Métricas agregadas del pulso global del sistema.
 */
export const SystemMetricsSchema = z.object({
  active_nodes_count: z.number().int().nonnegative(),
  cumulative_global_hashrate: z.number().nonnegative(),
  active_missions_in_flight: z.number().int().nonnegative(),
  system_timestamp_milliseconds: z.number().positive(),
});

export type SystemMetrics = z.infer<typeof SystemMetricsSchema>;

/**
 * Evento de Tiempo Real discriminado para transmisión binaria.
 */
export const RealTimeEventSchema = z.discriminatedUnion("t", [
  z.object({ t: z.literal("sp"), p: SystemMetricsSchema }),
  z.object({ t: z.literal("ac"), p: AuditReportSchema }),
  z.object({ t: z.literal("sh"), p: z.array(SwarmHeatmapSegmentSchema) }),
  z.object({
    t: z.literal("cc"),
    p: z.object({
      target_bitcoin_address: z.string(),
      discovery_node_identifier: z.string()
    })
  }),
  z.object({
    t: z.literal("ad"),
    p: z.object({
      drift_gap_count: z.number(),
      total_tactical_count: z.number()
    })
  }),
]);

export type RealTimeEvent = z.infer<typeof RealTimeEventSchema>;
