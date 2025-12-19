/**
 * =================================================================
 * APARATO: REAL-TIME NEURAL LINK HOOK (V43.0 - STRICT TYPE)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (L4)
 * RESPONSABILIDAD: CONSUMO REACTIVO DE EVENTOS SSE
 *
 * ESTRATEGIA DE ÉLITE:
 * - ESM Default Imports: Alineación con estándares de React 19.
 * - State Generics: Eliminación de 'any' implícitos en acumuladores.
 * - Fault Tolerance: Validación Zod integrada en el flujo de entrada.
 * =================================================================
 */

import React, { useState, useEffect, useCallback } from "react";
import {
  type RealTimeEvent,
  type AuditReport,
  RealTimeEventSchema
} from "@prospector/api-contracts";
import { SSESubscription } from "./sse-client";

/**
 * Hook de conexión al Neural Link del Orquestador.
 * Provee un flujo constante de reportes de misión certificados.
 *
 * @returns Un objeto con el historial de auditoría y estado de sincronización.
 */
export function useNeuralLink() {
  /**
   * Historial de misiones certificadas.
   * ✅ RESOLUCIÓN ERROR 7006: Se define el tipo genérico explícitamente.
   */
  const [last_audit_reports, set_last_audit_reports] = useState<AuditReport[]>([]);
  const [is_link_connected, set_is_link_connected] = useState<boolean>(false);

  /**
   * Handler de eventos de alta frecuencia.
   * Realiza la discriminación táctica de payloads.
   */
  const process_incoming_event = useCallback((event: RealTimeEvent): void => {
    switch (event.event_type) {
      case "MissionAuditCertified":
        const mission_report: AuditReport = event.payload;

        set_last_audit_reports((previous_reports: AuditReport[]): AuditReport[] => {
          // Mantener solo las últimas 50 misiones (Estrategia de gestión de memoria)
          const updated_ledger = [mission_report, ...previous_reports];
          return updated_ledger.slice(0, 50);
        });
        break;

      case "CryptographicCollisionAlert":
        console.warn("🎯 COLLISION_DETECTED:", event.payload.target_address);
        break;

      default:
        // Eventos de telemetría general ignorados en este hook específico
        break;
    }
  }, []);

  useEffect(() => {
    const orchestrator_url = process.env.NEXT_PUBLIC_API_URL;
    const stream_endpoint = `${orchestrator_url}/stream/metrics`;

    // Recuperación del token de sesión administrativa
    const authentication_token = typeof window !== "undefined"
      ? sessionStorage.getItem("ADMIN_SESSION_TOKEN")
      : null;

    if (!orchestrator_url || !authentication_token) {
      return;
    }

    /**
     * Suscripción persistente vía SSE (Server-Sent Events).
     */
    const neural_subscription = new SSESubscription({
      url: stream_endpoint,
      token: authentication_token,
      onOpen: () => set_is_link_connected(true),
      onError: () => set_is_link_connected(false),
      onMessage: (raw_data: unknown): void => {
        // Validación de contrato en tiempo de ejecución (Neural Shield)
        const validation_result = RealTimeEventSchema.safeParse(raw_data);
        if (validation_result.success) {
          process_incoming_event(validation_result.data);
        }
      }
    });

    return () => {
      neural_subscription.close();
    };
  }, [process_incoming_event]);

  return {
    audit_history: last_audit_reports,
    is_connected: is_link_connected,
    is_syncing: is_link_connected && last_audit_reports.length === 0
  };
}
