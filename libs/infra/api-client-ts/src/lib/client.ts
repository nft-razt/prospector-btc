/**
 * =================================================================
 * APARATO: RESILIENT API CLIENT (V18.5 - FULL SYNC)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (L4)
 * RESPONSABILIDAD: GESTIÓN DE COMUNICACIÓN ASÍNCRONA Y REINTENTOS
 * ESTADO: PRODUCTION READY // NO ABBREVIATIONS
 * =================================================================
 */

import axios, {
  AxiosInstance,
  AxiosError,
  InternalAxiosRequestConfig,
  AxiosRequestConfig,
} from "axios";
import { type Finding } from "@prospector/api-contracts"; // ✅ RESOLUCIÓN: Importación exitosa

/**
 * Configuración de la política de reintentos para hallazgos críticos.
 */
const CRITICAL_RETRY_POLICY = {
  max_retries: 10,
  base_delay_milliseconds: 1000,
  exponential_factor: 2,
};

class ResilientApiClient {
  private internal_axios_instance: AxiosInstance;

  constructor() {
    this.internal_axios_instance = axios.create({
      baseURL:
        process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000/api/v1",
      timeout: 15000,
      headers: { "Content-Type": "application/json" },
    });

    this.initialize_interceptors();
  }

  private initialize_interceptors(): void {
    this.internal_axios_instance.interceptors.request.use(
      (config: InternalAxiosRequestConfig) => {
        const authorization_token =
          typeof window !== "undefined"
            ? sessionStorage.getItem("ADMIN_SESSION_TOKEN")
            : process.env.API_TOKEN;

        if (authorization_token && config.headers) {
          config.headers.Authorization = `Bearer ${authorization_token}`;
        }
        return config;
      },
    );

    this.internal_axios_instance.interceptors.response.use(
      (response) => response,
      (error: AxiosError) => {
        if (error.response?.status === 503) {
          console.error(
            "⛔ SYSTEM_IN_MAINTENANCE: Service temporarily unavailable.",
          );
        }
        return Promise.reject(error);
      },
    );
  }

  /**
   * Ejecuta una petición GET con tipado genérico.
   */
  public async get<T>(
    endpoint_url: string,
    request_configuration?: AxiosRequestConfig,
  ): Promise<T> {
    const response = await this.internal_axios_instance.get<T>(
      endpoint_url,
      request_configuration,
    );
    return response.data;
  }

  /**
   * Ejecuta una petición POST con tipado genérico.
   */
  public async post<T>(
    endpoint_url: string,
    payload?: unknown,
    request_configuration?: AxiosRequestConfig,
  ): Promise<T> {
    const response = await this.internal_axios_instance.post<T>(
      endpoint_url,
      payload,
      request_configuration,
    );
    return response.data;
  }

  /**
   * PROTOCOLO DE REPORTE DE COLISIÓN (MISIÓN CRÍTICA).
   * Implementa una persistencia agresiva en el cliente para asegurar que
   * ningún hallazgo se pierda debido a inestabilidades de red.
   *
   * @param collision_data - Los detalles completos del hallazgo criptográfico.
   */
  public async report_cryptographic_finding(
    collision_data: Finding,
  ): Promise<void> {
    let current_attempt = 0;

    const execute_synchronization = async (): Promise<void> => {
      try {
        await this.internal_axios_instance.post(
          "/swarm/finding",
          collision_data,
        );
        console.log(
          `✅ VAULT_SYNC: Collision for [${collision_data.address}] secured.`,
        );
      } catch (error) {
        current_attempt++;
        const retry_delay =
          CRITICAL_RETRY_POLICY.base_delay_milliseconds *
          Math.pow(CRITICAL_RETRY_POLICY.exponential_factor, current_attempt);

        console.error(
          `🚨 SYNC_FAULT: Collision report failed. Attempt ${current_attempt}. Retrying in ${retry_delay}ms...`,
        );

        await new Promise((resolve) => setTimeout(resolve, retry_delay));
        return execute_synchronization();
      }
    };

    return execute_synchronization();
  }
}

export const apiClient = new ResilientApiClient();
