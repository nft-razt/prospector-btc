import { useQuery } from '@tanstack/react-query';
import { apiClient } from './client';
import { WorkerHeartbeatSchema, type WorkerHeartbeat } from '@prospector/api-contracts';
import { z } from 'zod';

// Validamos que la API devuelva un array de latidos correcto
const SystemStatusSchema = z.array(WorkerHeartbeatSchema);

async function fetchSystemStatus(): Promise<WorkerHeartbeat[]> {
  const { data } = await apiClient.get('/status');
  return SystemStatusSchema.parse(data);
}

export function useSystemStatus() {
  return useQuery({
    queryKey: ['system-status'],
    queryFn: fetchSystemStatus,
    refetchInterval: 2000,
    retry: 3,
  });
}
