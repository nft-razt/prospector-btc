// libs/domain/api-contracts/src/lib/control.ts
/**
 * =================================================================
 * APARATO: SWARM CONTROL CONTRACTS
 * RESPONSABILIDAD: DEFINICIÓN DE ESTRUCTURAS DE MANDO C2
 * =================================================================
 */

import { z } from 'zod';

/**
 * Esquema de configuración para el lanzamiento de un enjambre.
 * Alineado con los inputs del workflow de GitHub Actions.
 */
export const SwarmLaunchSchema = z.object({
  /** Número de workers (navegadores) por contenedor/runner */
  worker_count: z.number().min(1).max(50).default(30),

  /** Número de contenedores paralelos (Shards) */
  shard_count: z.number().min(1).max(20).default(5),

  /** Rama del repositorio a utilizar (por defecto: main) */
  ref: z.string().default('main'),
});

export type SwarmLaunchConfig = z.infer<typeof SwarmLaunchSchema>;

/**
 * Representación del estado de una ejecución de GitHub Workflow.
 */
export interface WorkflowRun {
  id: number;
  name: string;
  status: 'queued' | 'in_progress' | 'completed' | 'failure' | 'cancelled';
  conclusion: string | null;
  created_at: string;
  html_url: string;
  run_number: number;
}
