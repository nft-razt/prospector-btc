/**
 * =================================================================
 * APARATO: SYSTEM TELEMETRY HOOKS (V55.0 - SOBERANO SYNC)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L4)
 * RESPONSABILIDAD: CONSUMO REACTIVO Y AGREGACIÓN DE MÉTRICAS
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el escrutinio de salud del enjambre. Procesa los latidos
 * recibidos desde Turso y calcula el rendimiento global del sistema.
 * Resuelve el error TS2339 al sincronizarse con la nomenclatura
 * académica definida en el estrato L2.
 * =================================================================
 */

import { useQuery } from "@tanstack/react-query";
import { apiClient } from "./client";
import {
  type WorkerHeartbeat,
  WorkerHeartbeatSchema
} from "@prospector/api-contracts";
import { z } from "zod";

/**
 * Interface para el resumen de rendimiento global.
 */
export interface TelemetryMetricsSummary {
  active_nodes_count: number;
  total_nodes_registered: number;
  global_hashrate_per_second: number;
  projected_keys_per_day: bigint;
}

/**
 * Hook Soberano de Telemetría.
 * Realiza el análisis de vitalidad de los nodos y agregación de potencia.
 */
export function useSystemTelemetry() {
  return useQuery({
    queryKey: ["system-telemetry-v16.5"],
    queryFn: async () => {
      // 1. ADQUISICIÓN DESDE ESTRATO TÁCTICO
      const network_response = await apiClient.get<WorkerHeartbeat[]>("/swarm/status");

      // 2. ESCUDO DE VALIDACIÓN (ZOD)
      return z.array(WorkerHeartbeatSchema).parse(network_response);
    },
    refetchInterval: 5000,
    select: (workers_collection: WorkerHeartbeat[]) => {
      const activity_threshold_ms = Date.now() - 60000;

      /**
       * ✅ RESOLUCIÓN TS2339:
       * Mapeo nominal: 'timestamp' -> 'timestamp_utc'.
       */
      const active_workers = workers_collection.filter(
        (heartbeat) => new Date(heartbeat.timestamp_utc).getTime() > activity_threshold_ms
      );

      /**
       * ✅ RESOLUCIÓN TS2339:
       * Mapeo nominal: 'hashrate' -> 'current_hashrate'.
       */
      const total_global_hashrate = active_workers.reduce(
        (accumulator, heartbeat) => accumulator + (heartbeat.current_hashrate || 0),
        0
      );

      return {
        raw_data: workers_collection,
        aggregated_metrics: {
          active_nodes_count: active_workers.length,
          total_nodes_registered: workers_collection.length,
          global_hashrate_per_second: total_global_hashrate,
          projected_keys_per_day: BigInt(total_global_hashrate) * BigInt(86400),
        } as TelemetryMetricsSummary,
      };
    },
  });
}
