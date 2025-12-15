// libs/infra/api-client-ts/src/lib/client.ts

import axios, {
  AxiosInstance,
  AxiosError,
  InternalAxiosRequestConfig,
} from "axios";
import type {
  IdentityPayload,
  WorkerSnapshot,
  SwarmLaunchConfig,
  WorkflowRun,
} from "@prospector/api-contracts";

export interface IdentityStatusResponse {
  isActive: boolean;
  lastUpdated?: string;
  provider: string;
  nodeCount: number;
}

// Singleton mutable (Lazy)
let axiosInstance: AxiosInstance | null = null;

const getClient = (): AxiosInstance => {
  if (axiosInstance) return axiosInstance;

  // Lectura de ENV en tiempo de ejecución (Runtime), no en Build Time
  const API_URL =
    process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000/api/v1";
  const API_TOKEN = process.env.NEXT_PUBLIC_API_TOKEN || "";
  const IS_BROWSER = typeof window !== "undefined";

  axiosInstance = axios.create({
    baseURL: API_URL,
    headers: { "Content-Type": "application/json" },
    timeout: 15000,
  });

  axiosInstance.interceptors.request.use(
    (config: InternalAxiosRequestConfig) => {
      const sessionToken = IS_BROWSER
        ? sessionStorage.getItem("ADMIN_SESSION_TOKEN")
        : null;
      const activeToken = sessionToken || API_TOKEN;
      if (activeToken && config.headers) {
        config.headers.Authorization = `Bearer ${activeToken}`;
      }
      return config;
    },
    (error) => Promise.reject(error),
  );

  return axiosInstance;
};

// Facades usando getters dinámicos
export const apiClient = {
  get: <T>(url: string, conf?: any) => getClient().get<T>(url, conf),
  post: <T>(url: string, data?: any, conf?: any) =>
    getClient().post<T>(url, data, conf),
  put: <T>(url: string, data?: any, conf?: any) =>
    getClient().put<T>(url, data, conf),
  delete: <T>(url: string, conf?: any) => getClient().delete<T>(url, conf),
};

export const adminApi = {
  uploadIdentity: async (payload: IdentityPayload) =>
    getClient().post("/admin/identities", payload),
  checkIdentityStatus: async (): Promise<IdentityStatusResponse> => {
    try {
      return (
        await getClient().get<IdentityStatusResponse>(
          "/admin/identities/status",
        )
      ).data;
    } catch {
      return { isActive: false, provider: "unknown", nodeCount: 0 };
    }
  },
  getWorkerSnapshots: async () =>
    (await getClient().get<WorkerSnapshot[]>("/admin/worker-snapshots")).data,
  broadcastCommand: async (command: "shutdown" | "restart") =>
    getClient().post("/admin/command", { command }),
};

export const controlApi = {
  launchSwarm: async (config: SwarmLaunchConfig) =>
    getClient().post("/github/dispatch", config),
  getWorkflowRuns: async () =>
    (await getClient().get<WorkflowRun[]>("/github/runs")).data,
};

export const telemetryApi = {
  getSystemStatus: async () => (await getClient().get("/status")).data,
};
