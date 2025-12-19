/**
 * =================================================================
 * APARATO: DOMAIN DATA CONTRACTS (V51.0 - STRATEGIC HARMONIZATION)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (L2)
 * RESPONSABILIDAD: FUENTE ÚNICA DE VERDAD (SSoT) BACKEND/FRONTEND
 *
 * ESTRATEGIA DE ÉLITE:
 * - Projective Optimization: Tipos para el motor Rust O(1).
 * - BigInt Safe: Hashes y rangos representados como Strings deterministas.
 * - Explicit Exports: Resuelve errores de miembros no encontrados en L4.
 * =================================================================
 */

import { z } from "zod";

// --- TIPOS AUXILIARES ---
const HexString = z
  .string()
  .regex(/^[0-9a-fA-F]+$/, "Must be a valid hexadecimal string");

const BigIntString = z
  .string()
  .regex(/^\d+$/, "Must be a valid numeric string for BigInt representation");

// =================================================================
// 1. ESTRATO DE IDENTIDAD Y SEGURIDAD (IAM)
// =================================================================

export const IdentityStatusSchema = z.enum([
  "active", "ratelimited", "expired", "revoked"
]);
export type IdentityStatus = z.infer<typeof IdentityStatusSchema>;

export const EncryptedIdentityPayloadSchema = z.object({
  cipher_text_base64: z.string().describe("Contenido cifrado en Base64"),
  initialization_vector_base64: z.string().describe("IV único de la operación"),
  salt_base64: z.string().describe("Sal de derivación PBKDF2"),
});
export type EncryptedIdentityPayload = z.infer<typeof EncryptedIdentityPayloadSchema>;

export const IdentitySchema = z.object({
  id: z.string().uuid(),
  platform: z.string(),
  email: z.string().email(),
  credentials_json: z.string().describe("JSON serializado de EncryptedIdentityPayload"),
  user_agent: z.string(),
  usage_count: z.number().int().nonnegative(),
  last_used_at: z.string().datetime().nullable(),
  created_at: z.string().datetime(),
  status: IdentityStatusSchema,
});
export type Identity = z.infer<typeof IdentitySchema>;

export const IdentityPayloadSchema = z.object({
  platform: z.string(),
  email: z.string().email(),
  cookies: z.union([z.any(), EncryptedIdentityPayloadSchema]),
  userAgent: z.string(),
});
export type IdentityPayload = z.infer<typeof IdentityPayloadSchema>;

// =================================================================
// 2. ESTRATO DE EJECUCIÓN Y ESTRATEGIA (MINING ENGINE)
// =================================================================

export const SearchStrategySchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("Sequential"),
    params: z.object({
      start_index: BigIntString,
      end_index: BigIntString,
      use_proyective_addition: z.boolean().default(true)
    })
  }),
  z.object({
    type: z.literal("Dictionary"),
    params: z.object({
      dataset_url: z.string().url(),
      limit: z.number().int()
    })
  }),
  z.object({
    type: z.literal("Kangaroo"),
    params: z.object({
      target_pubkey: HexString.min(66),
      start_scalar: HexString.length(64),
      width: BigIntString
    })
  }),
  z.object({
    type: z.literal("ForensicScan"),
    params: z.object({
      target: z.enum(["DebianOpenSSL", "AndroidSecureRandom"]),
      range_start: BigIntString,
      range_end: BigIntString
    })
  }),
  z.object({
    type: z.literal("Random"),
    params: z.object({ seed: z.number() })
  }),
]);
export type SearchStrategy = z.infer<typeof SearchStrategySchema>;

export const WorkOrderSchema = z.object({
  id: z.string().uuid(),
  strategy: SearchStrategySchema,
  target_duration_sec: z.number().positive(),
});
export type WorkOrder = z.infer<typeof WorkOrderSchema>;

export const AuditReportSchema = z.object({
  id: z.string().uuid(),
  total_hashes: BigIntString,
  actual_duration_sec: z.number().int().nonnegative(),
  last_checkpoint: HexString.optional(),
  exit_status: z.enum(["exhausted", "collision_found", "interrupted"]),
});
export type AuditReport = z.infer<typeof AuditReportSchema>;

// =================================================================
// 3. ESTRATO DE TELEMETRÍA (REAL-TIME)
// =================================================================

export const WorkerHeartbeatSchema = z.object({
  worker_id: z.string().uuid(),
  hostname: z.string(),
  hashrate: z.number().int().nonnegative(),
  current_job_id: z.string().uuid().nullable().optional(),
  timestamp: z.string().datetime(),
  cpu_frequency_mhz: z.number().int().nonnegative(),
  cpu_load_percent: z.number().min(0).max(100),
  core_count: z.number().int().positive(),
});
export type WorkerHeartbeat = z.infer<typeof WorkerHeartbeatSchema>;

export const SystemMetricsSchema = z.object({
  active_nodes: z.number().int().nonnegative(),
  global_hashrate: z.number().nonnegative(),
  jobs_in_flight: z.number().int().nonnegative(),
  timestamp: z.string().datetime(),
});
export type SystemMetrics = z.infer<typeof SystemMetricsSchema>;

export const WorkerSnapshotSchema = z.object({
  worker_id: z.string(),
  status: z.enum(["running", "error", "captcha", "idle"]),
  snapshot_base64: z.string(),
  timestamp: z.string().datetime(),
});
export type WorkerSnapshot = z.infer<typeof WorkerSnapshotSchema>;

export const RealTimeEventSchema = z.discriminatedUnion("event", [
  z.object({ event: z.literal("Metrics"), data: SystemMetricsSchema }),
  z.object({ event: z.literal("ColissionAlert"), data: z.object({ address: z.string(), worker_id: z.string() }) }),
  z.object({ event: z.literal("NodeJoined"), data: z.object({ worker_id: z.string(), hostname: z.string() }) }),
  z.object({ event: z.literal("SnapshotReceived"), data: WorkerSnapshotSchema }),
]);
export type RealTimeEvent = z.infer<typeof RealTimeEventSchema>;
