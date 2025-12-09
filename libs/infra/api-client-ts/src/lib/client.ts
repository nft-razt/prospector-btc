// libs/infra/api-client-ts/src/lib/client.ts
// =================================================================
// APARATO: API CLIENT CORE
// CLASIFICACIÓN: INFRAESTRUCTURA / TRANSPORTE
// RESPONSABILIDAD: COMUNICACIÓN HTTP SEGURA CON ORCHESTRATOR
// =================================================================

import axios, { AxiosError, InternalAxiosRequestConfig, AxiosResponse } from 'axios';

// -----------------------------------------------------------------
// 1. DEFINICIONES DE TIPOS (CONTRATOS DE DATOS)
// -----------------------------------------------------------------

/**
 * Estructura del payload para la inyección de identidad (Cookies).
 * Utilizado por el sistema "Iron Vault" para aprovisionar a los workers.
 */
export interface IdentityPayload {
  /** Array de cookies en formato JSON estándar (EditThisCookie/Playwright) */
  cookies: Array<{
    domain: string;
    expirationDate?: number;
    hostOnly?: boolean;
    httpOnly?: boolean;
    name: string;
    path: string;
    sameSite?: string;
    secure?: boolean;
    session?: boolean;
    storeId?: string;
    value: string;
  }>;
  /** El User-Agent que debe imitar el worker para consistencia */
  userAgent: string;
  /** Proveedor de la identidad (actualmente solo Google Colab) */
  provider: 'google_colab';
}

/**
 * Respuesta estándar del estado de la identidad.
 */
export interface IdentityStatusResponse {
  isActive: boolean;
  lastUpdated?: string;
  provider: string;
  nodeCount: number;
}

/**
 * Estructura de la instantánea visual del worker (Panóptico).
 * Representa el estado visual y operativo de un nodo en tiempo real.
 */
export interface WorkerSnapshot {
  /** ID único del worker (puede ser UUID o nombre generado) */
  worker_id: string;
  /** Estado operativo actual */
  status: 'running' | 'error' | 'captcha';
  /** Imagen JPEG codificada en Base64 lista para src="data:..." */
  snapshot_base64: string;
  /** Marca de tiempo ISO 8601 de la captura */
  timestamp: string;
}

// -----------------------------------------------------------------
// 2. CONFIGURACIÓN DEL CLIENTE AXIOS
// -----------------------------------------------------------------

// Detección dinámica del entorno.
// En producción (Docker/Render), esta URL debe inyectarse vía variable de entorno.
const BASE_URL = process.env['NEXT_PUBLIC_API_URL'] || 'http://localhost:3000/api/v1';
const API_TOKEN = process.env['NEXT_PUBLIC_API_TOKEN'] || '';

export const apiClient = axios.create({
  baseURL: BASE_URL,
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  },
  // Timeout de 15s para evitar bloqueos en redes lentas o transmisión de imágenes pesadas
  timeout: 15000,
});

// -----------------------------------------------------------------
// 3. INTERCEPTORES (MIDDLEWARE HTTP)
// -----------------------------------------------------------------

/**
 * Interceptor de Solicitud (Request):
 * Inyecta automáticamente el Token de Autorización (Bearer) si existe.
 * Esto asegura que todas las llamadas desde el Dashboard estén autenticadas.
 */
apiClient.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    // Si hay un token definido en el entorno o la sesión, lo usamos.
    const activeToken = typeof window !== 'undefined'
      ? sessionStorage.getItem('ADMIN_SESSION_TOKEN') || API_TOKEN
      : API_TOKEN;

    if (activeToken) {
      config.headers.Authorization = `Bearer ${activeToken}`;
    }
    return config;
  },
  (error: unknown) => {
    return Promise.reject(error);
  }
);

/**
 * Interceptor de Respuesta (Response):
 * Centraliza el manejo de errores y logging (Heimdall Lite).
 * Permite que la UI reciba errores limpios o procesados.
 */
apiClient.interceptors.response.use(
  (response: AxiosResponse) => {
    return response;
  },
  (error: AxiosError) => {
    // Filtrado de ruido: No loguear cancelaciones voluntarias
    if (error.code !== 'ERR_CANCELED') {
      const errorDetails = {
        endpoint: error.config?.url,
        method: error.config?.method?.toUpperCase(),
        status: error.response?.status,
        message: (error.response?.data as any)?.message || error.message,
      };

      // En desarrollo, queremos ver todo el ruido para depuración
      if (process.env.NODE_ENV !== 'production') {
        console.error('🔥 [API_CLIENT] Error detectado:', errorDetails);
      }
    }
    return Promise.reject(error);
  }
);

// -----------------------------------------------------------------
// 4. MÓDULOS DE API (FACADES)
// -----------------------------------------------------------------

/**
 * Módulo de Administración (The Iron Vault & Panopticon).
 * Encapsula las operaciones sensibles y de vigilancia del Dashboard.
 */
export const adminApi = {
  /**
   * Sube la identidad (Cookies) a la Bóveda Segura del Orchestrator.
   * @param payload Datos de identidad crudos.
   */
  uploadIdentity: async (payload: IdentityPayload): Promise<void> => {
    await apiClient.post('/admin/identities', payload);
  },

  /**
   * Verifica el estado de la identidad actual sin revelar datos sensibles.
   * Útil para los indicadores de estado del Dashboard.
   */
  checkIdentityStatus: async (): Promise<IdentityStatusResponse> => {
    // Nota: Si el endpoint específico no existe, devuelve un mock o el estado general
    // Ajustado para apuntar a la lista si no hay endpoint de status individual
    try {
      const { data } = await apiClient.get<IdentityStatusResponse>('/admin/identities/status');
      return data;
    } catch {
      return { isActive: false, provider: 'unknown', nodeCount: 0 };
    }
  },

  /**
   * VIGILANCIA VISUAL: Obtiene las últimas capturas de pantalla de todos los workers activos.
   */
  getWorkerSnapshots: async (): Promise<WorkerSnapshot[]> => {
    const { data } = await apiClient.get<WorkerSnapshot[]>('/admin/worker-snapshots');
    return data;
  },

  /**
   * Dispara una orden de emergencia al enjambre.
   * (Ej: Detener todos los nodos, reiniciar búsqueda).
   */
  broadcastCommand: async (command: 'shutdown' | 'restart'): Promise<void> => {
    await apiClient.post('/admin/command', { command });
  }
};

/**
 * Módulo de Telemetría Pública.
 * Usado por la página principal del Dashboard.
 */
export const telemetryApi = {
  /**
   * Obtiene el estado general del enjambre (Hashrate, Nodos activos).
   */
  getSystemStatus: async () => {
    return apiClient.get('/status');
  }
};
