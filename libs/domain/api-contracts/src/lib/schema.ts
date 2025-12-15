// libs/domain/api-contracts/src/lib/schema.ts
/**
 * =================================================================
 * APARATO: DATA CONTRACTS (SINGLE SOURCE OF TRUTH)
 * RESPONSABILIDAD: DEFINICIÓN ESTRICTA DE TIPOS E INTERFACES (ZOD)
 * ALCANCE: GLOBAL (BACKEND RUST <-> FRONTEND NEXT.JS)
 * ESTADO: V6.3 (FORENSICS & VISUAL TELEMETRY ENABLED)
 * =================================================================
 */

import { z } from "zod";

// =================================================================
// 1. TELEMETRÍA DE NODO (Worker Heartbeats)
// =================================================================

export const WorkerHeartbeatSchema = z.object({
  /** Identificador único UUID v4 del nodo */
  worker_id: z.string().uuid(),
  /** Nombre del host o contenedor (ej: 'colab-runner-x89') */
  hostname: z.string(),
  /** Velocidad de hash reportada (Hashes por segundo) */
  hashrate: z.number().int().nonnegative(),
  /** ID del trabajo actual (si está minando) */
  current_job_id: z.string().uuid().nullable().optional(),
  /** Momento del reporte (ISO 8601) */
  timestamp: z.string().datetime(),
});

export type WorkerHeartbeat = z.infer<typeof WorkerHeartbeatSchema>;

// =================================================================
// 2. ESTRATEGIA DE MINERÍA (Search Logic)
// Alineado con Rust: #[serde(tag = "type", content = "params")]
// =================================================================

export const SearchStrategySchema = z.discriminatedUnion("type", [
  // A. Minería Aleatoria (Monte Carlo)
  z.object({
    type: z.literal("Random"),
    params: z.object({
      seed: z.number().describe("Semilla inicial para PRNG"),
    }),
  }),
  // B. Ataque de Diccionario (Brainwallets)
  z.object({
    type: z.literal("Dictionary"),
    params: z.object({
      dataset_url: z.string().url(),
      limit: z.number().int().nonnegative(),
    }),
  }),
  // C. Combinatoria Secuencial (Fuerza Bruta Inteligente)
  z.object({
    type: z.literal("Combinatoric"),
    params: z.object({
      prefix: z.string(),
      suffix: z.string(),
      /** Usamos String para soportar BigInt (256 bits) sin desbordamiento en JS */
      start_index: z.string(),
      end_index: z.string(),
    }),
  }),
  // D. Escaneo Forense (Arqueología de Bugs)
  z.object({
    type: z.literal("ForensicScan"),
    params: z.object({
      target: z.enum(["DebianOpenSSL", "AndroidSecureRandom"]),
      range_start: z.string(),
      range_end: z.string(),
    }),
  }),
]);

export type SearchStrategy = z.infer<typeof SearchStrategySchema>;

// =================================================================
// 3. ORDEN DE TRABAJO (Job Assignment)
// =================================================================

export const WorkOrderSchema = z.object({
  /** ID único de la asignación */
  id: z.string().uuid(),
  /** Estrategia exacta a ejecutar */
  strategy: SearchStrategySchema,
  /** Tiempo objetivo de ejecución antes de reportar (Backpressure) */
  target_duration_sec: z.number().positive(),
});

export type WorkOrder = z.infer<typeof WorkOrderSchema>;

// =================================================================
// 4. GESTIÓN DE IDENTIDAD (IAM & Cookies)
// =================================================================

export const IdentityPayloadSchema = z.object({
  platform: z.string(),
  email: z.string().email(),
  cookies: z
    .array(
      z.object({
        domain: z.string(),
        name: z.string(),
        value: z.string(),
        path: z.string(),
        secure: z.boolean().optional(),
        httpOnly: z.boolean().optional(),
        sameSite: z.string().optional(),
        expirationDate: z.number().optional(),
      }),
    )
    .nonempty("Must provide at least one cookie"),
  userAgent: z.string(),
});

export type IdentityPayload = z.infer<typeof IdentityPayloadSchema>;

// =================================================================
// 5. VIGILANCIA VISUAL (Panóptico)
// =================================================================

export const WorkerSnapshotSchema = z.object({
  worker_id: z.string(),
  status: z.enum(["running", "error", "captcha"]),
  /** Imagen en Base64 (data:image/jpeg;base64,...) */
  snapshot_base64: z.string(),
  timestamp: z.string().datetime(),
});

export type WorkerSnapshot = z.infer<typeof WorkerSnapshotSchema>;

// =================================================================
// 6. TELEMETRÍA EN TIEMPO REAL (SSE Streaming)
// Estructuras "Push" enviadas por el Orquestador
// =================================================================

/**
 * Métricas agregadas del sistema (Globales).
 * Enviadas periódicamente por el canal 'metrics'.
 */
export const SystemMetricsSchema = z.object({
  active_nodes: z.number().int().nonnegative(),
  global_hashrate: z.number().nonnegative(), // H/s
  jobs_in_flight: z.number().int().nonnegative(),
  timestamp: z.string().datetime(),
});

export type SystemMetrics = z.infer<typeof SystemMetricsSchema>;

/**
 * Eventos discretos del sistema.
 * Alineado con Rust: #[serde(tag = "event", content = "data")]
 */
export const RealTimeEventSchema = z.discriminatedUnion("event", [
  // Actualización de Métricas (Heartbeat del sistema)
  z.object({
    event: z.literal("Metrics"),
    data: SystemMetricsSchema,
  }),

  // Alerta de Colisión (Hallazgo positivo)
  z.object({
    event: z.literal("ColissionAlert"),
    data: z.object({
      address: z.string(),
      worker_id: z.string(),
    }),
  }),

  // Nuevo nodo detectado
  z.object({
    event: z.literal("NodeJoined"),
    data: z.object({
      worker_id: z.string(),
      hostname: z.string(),
    }),
  }),

  // ✅ Transmisión de Vigilancia Visual (Panóptico)
  z.object({
    event: z.literal("SnapshotReceived"),
    data: WorkerSnapshotSchema,
  }),
]);

export type RealTimeEvent = z.infer<typeof RealTimeEventSchema>;
