// libs/infra/api-client-ts/src/lib/client.ts
// =================================================================
// APARATO: API CLIENT CORE (TRANSPORT LAYER)
// DEPENDENCIA: @prospector/api-contracts
// =================================================================

import axios, { AxiosError, InternalAxiosRequestConfig, AxiosResponse } from 'axios';
// USO DE 'import type' PARA EVITAR CONFLICTOS DE VALOR EN TIEMPO DE COMPILACIÓN
import type { IdentityPayload, WorkerSnapshot } from '@prospector/api-contracts';

// --- DEFINICIONES DE RESPUESTA DE RED ---
export interface IdentityStatusResponse {
  isActive: boolean;
  lastUpdated?: string;
  provider: string;
  nodeCount: number;
}

// --- CONFIGURACIÓN DE ENTORNO SEGURA ---
// TypeScript ahora reconocerá 'process' gracias al ajuste en tsconfig.lib.json
const ENV_API_URL = process.env['NEXT_PUBLIC_API_URL'];
const ENV_API_TOKEN = process.env['NEXT_PUBLIC_API_TOKEN'];

const BASE_URL = ENV_API_URL || 'http://localhost:3000/api/v1';
const API_TOKEN = ENV_API_TOKEN || '';

// --- INSTANCIA AXIOS ---
export const apiClient = axios.create({
  baseURL: BASE_URL,
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  },
  timeout: 15000,
});

// --- INTERCEPTORES ---
apiClient.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    // Detección de entorno (Browser vs Node) para sessionStorage
    const isBrowser = typeof window !== 'undefined';

    const activeToken = isBrowser
      ? sessionStorage.getItem('ADMIN_SESSION_TOKEN') || API_TOKEN
      : API_TOKEN;

    if (activeToken) {
      config.headers.Authorization = `Bearer ${activeToken}`;
    }
    return config;
  },
  (error: unknown) => Promise.reject(error)
);

apiClient.interceptors.response.use(
  (response: AxiosResponse) => response,
  (error: AxiosError) => {
    // Logging silencioso en producción, detallado en desarrollo
    if (error.code !== 'ERR_CANCELED' && process.env.NODE_ENV !== 'production') {
      console.error('🔥 [API_CLIENT] Error:', {
        url: error.config?.url,
        status: error.response?.status,
        message: error.message
      });
    }
    return Promise.reject(error);
  }
);

// --- FACADES (MÉTODOS PÚBLICOS) ---
export const adminApi = {
  uploadIdentity: async (payload: IdentityPayload): Promise<void> => {
    await apiClient.post('/admin/identities', payload);
  },
  checkIdentityStatus: async (): Promise<IdentityStatusResponse> => {
    try {
      const { data } = await apiClient.get<IdentityStatusResponse>('/admin/identities/status');
      return data;
    } catch {
      return { isActive: false, provider: 'unknown', nodeCount: 0 };
    }
  },
  getWorkerSnapshots: async (): Promise<WorkerSnapshot[]> => {
    const { data } = await apiClient.get<WorkerSnapshot[]>('/admin/worker-snapshots');
    return data;
  },
  broadcastCommand: async (command: 'shutdown' | 'restart'): Promise<void> => {
    await apiClient.post('/admin/command', { command });
  }
};

export const telemetryApi = {
  getSystemStatus: async () => {
    return apiClient.get('/status');
  }
};
