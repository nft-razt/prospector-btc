// libs/domain/api-contracts/src/lib/schema.ts
// =================================================================
// APARATO: DATA CONTRACTS (SSoT)
// ESTADO: ATOMIZADO (PURE TYPESCRIPT/ZOD)
// =================================================================

import { z } from 'zod';

// --- TELEMETRÍA ---

export const WorkerHeartbeatSchema = z.object({
  worker_id: z.string().uuid(),
  hostname: z.string(),
  hashrate: z.number().int().nonnegative(),
  current_job_id: z.string().uuid().nullable().optional(),
  timestamp: z.string().datetime(),
});

export type WorkerHeartbeat = z.infer<typeof WorkerHeartbeatSchema>;

// --- ESTRATEGIA DE MINERÍA ---

// Alineación con Rust #[serde(tag = "type", content = "params")]
export const SearchStrategySchema = z.discriminatedUnion('type', [
  z.object({
    type: z.literal('Random'),
    params: z.object({ seed: z.number() })
  }),
  z.object({
    type: z.literal('Dictionary'),
    params: z.object({ dataset_url: z.string(), limit: z.number() })
  }),
  z.object({
    type: z.literal('Combinatoric'),
    params: z.object({
      prefix: z.string(),
      suffix: z.string(),
      start_index: z.number(),
      end_index: z.number()
    })
  }),
  // Nuevo: Soporte para escaneo forense
  z.object({
    type: z.literal('ForensicScan'),
    params: z.object({
        target: z.enum(['DebianOpenSSL', 'AndroidSecureRandom']),
        range_start: z.string(),
        range_end: z.string()
    })
  })
]);

export type SearchStrategy = z.infer<typeof SearchStrategySchema>;

// --- ORDEN DE TRABAJO ---

export const WorkOrderSchema = z.object({
  id: z.string().uuid(),
  strategy: SearchStrategySchema,
  target_duration_sec: z.number(),
});

export type WorkOrder = z.infer<typeof WorkOrderSchema>;

// --- IDENTIDAD Y SEGURIDAD ---

export const IdentityPayloadSchema = z.object({
  platform: z.string(),
  email: z.string().email(),
  cookies: z.array(z.object({
    domain: z.string(),
    name: z.string(),
    value: z.string(),
    path: z.string(),
    secure: z.boolean().optional(),
    httpOnly: z.boolean().optional(),
    sameSite: z.string().optional(),
    expirationDate: z.number().optional(),
  })),
  userAgent: z.string(),
});

export type IdentityPayload = z.infer<typeof IdentityPayloadSchema>;

// --- VIGILANCIA (PANÓPTICO) ---

export const WorkerSnapshotSchema = z.object({
  worker_id: z.string(),
  status: z.enum(['running', 'error', 'captcha']),
  snapshot_base64: z.string(),
  timestamp: z.string().datetime(),
});

export type WorkerSnapshot = z.infer<typeof WorkerSnapshotSchema>;
