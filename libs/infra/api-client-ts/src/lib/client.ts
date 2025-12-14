// libs/infra/api-client-ts/src/lib/client.ts
// =================================================================
// APARATO: API CLIENT CORE (LAZY LOADING)
// MEJORA: COMPATIBILIDAD CON NEXT.JS SERVER COMPONENTS
// =================================================================

import axios, { AxiosInstance, AxiosError } from 'axios';
import type { IdentityPayload, WorkerSnapshot, SwarmLaunchConfig, WorkflowRun } from '@prospector/api-contracts';

// Singleton mutable para cachear la instancia
let axiosInstance: AxiosInstance | null = null;

// Obtención Lazy de configuración
const getClient = (): AxiosInstance => {
  if (axiosInstance) return axiosInstance;

  // Leemos las variables EN EL MOMENTO DE USO, no en importación
  const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api/v1';
  const API_TOKEN = process.env.NEXT_PUBLIC_API_TOKEN || '';
  const IS_BROWSER = typeof window !== 'undefined';

  axiosInstance = axios.create({
    baseURL: API_URL,
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    },
    timeout: 15000,
  });

  axiosInstance.interceptors.request.use(
    (config) => {
      const sessionToken = IS_BROWSER ? sessionStorage.getItem('ADMIN_SESSION_TOKEN') : null;
      const activeToken = sessionToken || API_TOKEN;
      if (activeToken && config.headers) {
        config.headers.Authorization = `Bearer ${activeToken}`;
      }
      return config;
    },
    (error) => Promise.reject(error)
  );

  return axiosInstance;
};

// --- WRAPPERS DE ACCESO ---
// Usamos getters para asegurar que getClient() se llame en runtime

export const adminApi = {
  uploadIdentity: async (payload: IdentityPayload) => getClient().post('/admin/identities', payload),
  checkIdentityStatus: async () => {
      try {
          const { data } = await getClient().get('/admin/identities/status');
          return data;
      } catch { return { isActive: false, provider: 'unknown', nodeCount: 0 }; }
  },
  getWorkerSnapshots: async () => (await getClient().get<WorkerSnapshot[]>('/admin/worker-snapshots')).data,
};

export const controlApi = {
  launchSwarm: async (config: SwarmLaunchConfig) => getClient().post('/github/dispatch', config),
  getWorkflowRuns: async () => (await getClient().get<WorkflowRun[]>('/github/runs')).data,
};

export const telemetryApi = {
  getSystemStatus: async () => (await getClient().get('/status')).data,
};

// Exportamos una referencia genérica para casos de borde
export const apiClient = {
    get: <T>(url: string, conf?: any) => getClient().get<T>(url, conf),
    post: <T>(url: string, data?: any, conf?: any) => getClient().post<T>(url, data, conf),
};
