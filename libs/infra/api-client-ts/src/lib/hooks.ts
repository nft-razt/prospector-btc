/**
 * =================================================================
 * APARATO: TELEMETRY HOOKS (V54.0 - ELITE SYNC)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (L4)
 * RESPONSABILIDAD: CONSUMO REACTIVO DE MÉTRICAS DEL ENJAMBRE
 *
 * ESTRATEGIA DE ÉLITE:
 * - Deterministic Typing: Uso estricto de WorkerHeartbeat de L2.
 * - Runtime Validation: Escudo Zod contra datos de red corruptos.
 * - Projective Aware: Preparado para métricas de adición proyectiva.
 * =================================================================
 */

import { useQuery } from "@tanstack/react-query";
import { apiClient } from "./client";
import {
  type WorkerHeartbeat,
  WorkerHeartbeatSchema
} from "@prospector/api-contracts"; // ✅ RESOLUCIÓN TS2307
import { z } from "zod";

/**
 * Hook de Telemetría Maestra.
 * Realiza el escrutinio de la salud de los nodos y el Hashrate global.
 */
export function useSystemTelemetry() {
  return useQuery({
    queryKey: ["system-telemetry"],
    queryFn: async () => {
      // Adquisición de datos desde el estrato táctico (Turso)
      const response = await apiClient.get<WorkerHeartbeat[]>("/swarm/status");

      // Validación soberana: Si el backend cambia el esquema, la UI falla preventivamente
      return z.array(WorkerHeartbeatSchema).parse(response);
    },
    refetchInterval: 2000,
    select: (workers) => {
      const activeThreshold = Date.now() - 60000;

      const activeWorkers = workers.filter(
        (w) => new Date(w.timestamp).getTime() > activeThreshold
      );

      const totalHashrate = activeWorkers.reduce(
        (acc, w) => acc + (w.hashrate || 0),
        0
      );

      return {
        raw: workers,
        metrics: {
          activeNodes: activeWorkers.length,
          totalNodes: workers.length,
          globalHashrate: totalHashrate, // Hashes/seg
          keysPerDay: BigInt(totalHashrate) * BigInt(86400),
        },
      };
    },
  });
}
