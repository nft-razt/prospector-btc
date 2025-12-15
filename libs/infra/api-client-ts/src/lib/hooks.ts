/**
 * =================================================================
 * APARATO: TELEMETRY HOOKS
 * RESPONSABILIDAD: CONSUMO DE DATOS VIVOS Y TRANSFORMACIÓN
 * ESTRATEGIA: SMART POLLING (INTERVALOS ADAPTATIVOS)
 * =================================================================
 */

import { useQuery } from "@tanstack/react-query";
import { apiClient } from "./client";
// 🔥 CORRECCIÓN: Importar tipo desde schemas.ts, no client.ts
import { type WorkerHeartbeat } from "./schemas";
import { z } from "zod";

// Esquema de validación para la respuesta del endpoint /status
const SystemStatusSchema = z.array(
  z.object({
    worker_id: z.string(),
    hostname: z.string(),
    hashrate: z.number(),
    timestamp: z.string(),
    // Campos opcionales que podrían venir del backend
    current_job_id: z.string().nullable().optional(),
  }),
);

/**
 * Hook maestro para el estado del sistema.
 * Realiza un polling cada 2 segundos.
 */
export function useSystemTelemetry() {
  return useQuery({
    queryKey: ["system-telemetry"],
    queryFn: async () => {
      const { data } = await apiClient.get("/status");
      // Validación Zod para evitar crashes en UI por datos corruptos
      return SystemStatusSchema.parse(data);
    },
    // Frecuencia de actualización agresiva para sensación "Real-Time"
    refetchInterval: 2000,
    // Si el usuario cambia de tab, pausamos para ahorrar ancho de banda
    refetchOnWindowFocus: true,
    // Cálculo de métricas derivadas (Memoización automática por React Query)
    select: (workers) => {
      const activeThreshold = Date.now() - 60000; // 1 minuto

      const activeWorkers = workers.filter(
        (w: any) => new Date(w.timestamp).getTime() > activeThreshold,
      );
      const totalHashrate = activeWorkers.reduce(
        (acc: number, w: any) => acc + w.hashrate,
        0,
      );

      return {
        raw: workers,
        metrics: {
          activeNodes: activeWorkers.length,
          totalNodes: workers.length,
          globalHashrate: totalHashrate, // Hashes/sec
          // Estimación de claves por día (Hashrate * 60 * 60 * 24)
          keysPerDay: totalHashrate * 86400,
        },
      };
    },
  });
}
