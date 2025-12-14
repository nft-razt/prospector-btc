/**
 * =================================================================
 * APARATO: API CLIENT CORE (TRANSPORT LAYER)
 * CLASIFICACIÓN: INFRAESTRUCTURA DE NIVEL 4 (HYDRA NETWORK)
 * RESPONSABILIDAD: ORQUESTACIÓN DE PETICIONES HTTP, AUTH & ERRORES
 * =================================================================
 */

import axios, {
  AxiosInstance,
  AxiosError,
  InternalAxiosRequestConfig,
  AxiosResponse
} from 'axios';

// Importación estricta de contratos de dominio (Single Source of Truth)
import type {
  IdentityPayload,
  WorkerSnapshot,
  SwarmLaunchConfig,
  WorkflowRun
} from '@prospector/api-contracts';

// -----------------------------------------------------------------
// 1. DEFINICIONES DE TIPOS DE RESPUESTA (DTOs LOCALES)
// -----------------------------------------------------------------

/**
 * Estructura de respuesta para el estado de salud de las identidades.
 * No es un contrato de dominio compartido, sino un DTO específico de vista.
 */
export interface IdentityStatusResponse {
  /** Indica si hay al menos una identidad operativa */
  isActive: boolean;
  /** Timestamp ISO de la última inyección exitosa */
  lastUpdated?: string;
  /** Nombre del proveedor (ej: google_colab) */
  provider: string;
  /** Cantidad de identidades disponibles en la bóveda */
  nodeCount: number;
}

// -----------------------------------------------------------------
// 2. CONFIGURACIÓN DE ENTORNO BLINDADA
// -----------------------------------------------------------------

const ENV_CONFIG = {
  API_URL: process.env['NEXT_PUBLIC_API_URL'] || 'http://localhost:3000/api/v1',
  API_TOKEN: process.env['NEXT_PUBLIC_API_TOKEN'] || '',
  IS_BROWSER: typeof window !== 'undefined',
  IS_PROD: process.env.NODE_ENV === 'production',
};

// -----------------------------------------------------------------
// 3. FÁBRICA DE CLIENTE HTTP (AXIOS FACTORY)
// -----------------------------------------------------------------

/**
 * Crea una instancia configurada de Axios con interceptores de seguridad y logging.
 * Implementa el patrón Singleton implícito al exportar la instancia constante.
 */
const createApiClient = (): AxiosInstance => {
  const instance = axios.create({
    baseURL: ENV_CONFIG.API_URL,
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    },
    // Timeout agresivo para fail-fast en redes inestables
    timeout: 15000,
  });

  // --- INTERCEPTOR DE PETICIÓN (INYECCIÓN DE AUTH) ---
  instance.interceptors.request.use(
    (config: InternalAxiosRequestConfig) => {
      // Prioridad:
      // 1. Session Storage (Admin logueado en Dashboard)
      // 2. Variable de Entorno (Server-Side Fetching / Build Time)
      const sessionToken = ENV_CONFIG.IS_BROWSER
        ? sessionStorage.getItem('ADMIN_SESSION_TOKEN')
        : null;

      const activeToken = sessionToken || ENV_CONFIG.API_TOKEN;

      if (activeToken && config.headers) {
        config.headers.Authorization = `Bearer ${activeToken}`;
      }

      return config;
    },
    (error: unknown) => Promise.reject(error)
  );

  // --- INTERCEPTOR DE RESPUESTA (OBSERVABILIDAD & ERROR NORMALIZATION) ---
  instance.interceptors.response.use(
    (response: AxiosResponse) => response,
    (error: AxiosError) => {
      // Ignorar cancelaciones intencionales (React Query pre-fetching)
      if (error.code === 'ERR_CANCELED') {
        return Promise.reject(error);
      }

      // Logging estructurado en desarrollo para depuración rápida
      if (!ENV_CONFIG.IS_PROD) {
        console.error('🔥 [API_CLIENT] Fallo de Transporte:', {
          endpoint: error.config?.url,
          method: error.config?.method?.toUpperCase(),
          status: error.response?.status,
          message: error.message,
          data: error.response?.data
        });
      }

      // Aquí se podría implementar lógica de refresh token si fuera necesario
      return Promise.reject(error);
    }
  );

  return instance;
};

/** Instancia única compartida del cliente HTTP */
export const apiClient = createApiClient();

// -----------------------------------------------------------------
// 4. SERVICIOS DE DOMINIO (FACADES)
// -----------------------------------------------------------------

/**
 * Servicio Administrativo.
 * Maneja operaciones de alto privilegio: Gestión de Identidad y Vigilancia.
 */
export const adminApi = {
  /**
   * Sube nuevas credenciales (Cookies) a la Bóveda del Orquestador.
   * @param payload - Datos de la identidad y cookies crudas.
   */
  uploadIdentity: async (payload: IdentityPayload): Promise<void> => {
    await apiClient.post('/admin/identities', payload);
  },

  /**
   * Verifica el estado de salud del pool de identidades.
   * Utilizado por los guards de autenticación y widgets de estado.
   */
  checkIdentityStatus: async (): Promise<IdentityStatusResponse> => {
    try {
      // Nota: Este endpoint debe existir en el backend o simularse
      const { data } = await apiClient.get<IdentityStatusResponse>('/admin/identities/status');
      return data;
    } catch (error) {
      // Fail-safe: Si falla, asumimos estado inactivo en lugar de romper la UI
      return { isActive: false, provider: 'unknown', nodeCount: 0 };
    }
  },

  /**
   * Obtiene la telemetría visual (Screenshots) de todos los nodos activos.
   * Alimenta el "Fleet Grid" del Dashboard.
   */
  getWorkerSnapshots: async (): Promise<WorkerSnapshot[]> => {
    const { data } = await apiClient.get<WorkerSnapshot[]>('/admin/worker-snapshots');
    return data;
  },

  /**
   * Envía comandos de transmisión global al enjambre.
   * @param command - Instrucción crítica ('shutdown' | 'restart').
   */
  broadcastCommand: async (command: 'shutdown' | 'restart'): Promise<void> => {
    await apiClient.post('/admin/command', { command });
  }
};

/**
 * Servicio de Control y Orquestación (C2).
 * Maneja la interacción con la infraestructura de despliegue (GitHub Actions).
 */
export const controlApi = {
  /**
   * Dispara el workflow de aprovisionamiento en GitHub Actions.
   * Proxificado a través de la API interna de Next.js para proteger el GITHUB_PAT.
   * @param config - Configuración de granularidad del despliegue.
   */
  launchSwarm: async (config: SwarmLaunchConfig): Promise<void> => {
    // POST a ruta interna de Next.js (/app/api/github/dispatch)
    await apiClient.post('/github/dispatch', config);
  },

  /**
   * Consulta el historial de ejecuciones de CI/CD.
   * @returns Lista de runs recientes con su estado (success/failure/running).
   */
  getWorkflowRuns: async (): Promise<WorkflowRun[]> => {
    // GET a ruta interna de Next.js (/app/api/github/runs)
    const { data } = await apiClient.get<WorkflowRun[]>('/github/runs');
    return data;
  }
};

/**
 * Servicio de Telemetría Pública.
 * Operaciones de lectura de baja latencia para monitores en tiempo real.
 */
export const telemetryApi = {
  /**
   * Obtiene el estado consolidado del sistema (Hashrate, Nodos, Hallazgos).
   * @returns Objeto crudo de respuesta (Tipado en los hooks de consumo).
   */
  getSystemStatus: async (): Promise<unknown> => {
    const { data } = await apiClient.get('/status');
    return data;
  }
};
