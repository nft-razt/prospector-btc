import { z } from 'zod';

// =================================================================
// CONTRATOS DE DATOS (Strict Typing)
// =================================================================

export const WorkerHeartbeatSchema = z.object({
  worker_id: z.string().uuid(),
  hostname: z.string(),
  hashrate: z.number().int().nonnegative(),
  current_job_id: z.string().uuid().nullable().optional(),
  timestamp: z.string().datetime(),
});

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
]);

export const WorkOrderSchema = z.object({
  id: z.string().uuid(),
  strategy: SearchStrategySchema,
  target_duration_sec: z.number(),
});

// Inferencias
export type WorkerHeartbeat = z.infer<typeof WorkerHeartbeatSchema>;
export type WorkOrder = z.infer<typeof WorkOrderSchema>;
export type SearchStrategy = z.infer<typeof SearchStrategySchema>;
