/**
 * =================================================================
 * APARATO: DOMAIN SCHEMA LEVELING (V52.0 - FULL ALIGNMENT)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (L2)
 * RESPONSABILIDAD: SINCRONIZACIÓN DE TIPOS PARA EL NEURAL LINK
 *
 * ESTRATEGIA DE ÉLITE:
 * - Zero-Abbreviation: Sincronía 1:1 con el modelo Rust AuditReport.
 * - BigInt Safe: computational_effort_volume manejado como string determinista.
 * =================================================================
 */

import { z } from "zod";

// --- ESTRATO DE AUDITORÍA (CERTIFICACIÓN) ---

/**
 * Esquema de validación para los reportes de misión completados.
 * Recibido vía SSE (RealTimeEvent) o API REST.
 */
export const AuditReportSchema = z.object({
  /** Identificador único de la misión asignada */
  job_mission_identifier: z.string().uuid(),

  /** Identificador del nodo que realizó el cómputo */
  worker_node_identifier: z.string(),

  /** Volumen total de claves validadas (Representación String de U64) */
  computational_effort_volume: z.string().regex(/^\d+$/),

  /** Tiempo real consumido en milisegundos */
  execution_duration_ms: z.number().int().nonnegative(),

  /** Estado final de la misión (exhausted | completed | interrupted | error) */
  final_mission_status: z.string(),

  /** El último punto de control auditado (Hexadecimal o Índice) */
  audit_footprint_checkpoint: z.string(),

  /** Marca de tiempo ISO-8601 de finalización */
  completed_at_timestamp: z.string().datetime(),
});

export type AuditReport = z.infer<typeof AuditReportSchema>;

// --- ACTUALIZACIÓN DEL BUS DE EVENTOS ---

export const RealTimeEventSchema = z.discriminatedUnion("event_type", [
  z.object({
    event_type: z.literal("SystemPulseUpdate"),
    payload: z.any() // SystemMetricsSchema
  }),
  z.object({
    event_type: z.literal("CryptographicCollisionAlert"),
    payload: z.object({ target_address: z.string(), discovery_node: z.string() })
  }),
  /** ✅ NIVELACIÓN CRÍTICA: Notificación de Auditoría Certificada */
  z.object({
    event_type: z.literal("MissionAuditCertified"),
    payload: AuditReportSchema
  }),
  z.object({
    event_type: z.literal("NodeVisualFeedUpdate"),
    payload: z.any() // WorkerSnapshotSchema
  }),
]);

export type RealTimeEvent = z.infer<typeof RealTimeEventSchema>;
