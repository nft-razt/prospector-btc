import { useQuery } from '@tanstack/react-query';
import { apiClient } from './client';
import { WorkerHeartbeatSchema, type WorkerHeartbeat } from './schemas';
import { z } from 'zod';

// Schema de respuesta (Array de Workers)
const SystemStatusSchema = z.array(WorkerHeartbeatSchema);

async function fetchSystemStatus(): Promise<WorkerHeartbeat[]> {
  const { data } = await apiClient.get('/status');
  // Validación en Runtime: Si la API devuelve basura, esto lanza error
  return SystemStatusSchema.parse(data);
}

export function useSystemStatus() {
  return useQuery({
    queryKey: ['system-status'],
    queryFn: fetchSystemStatus,
    refetchInterval: 2000, // Polling cada 2s (Tiempo Real)
    retry: 3,
  });
}
