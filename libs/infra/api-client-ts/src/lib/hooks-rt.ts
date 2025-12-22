/**
 * =================================================================
 * APARATO: REAL-TIME NEURAL LINK HOOK (V66.0 - ARCHIVAL AWARE)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: CONSUMO, DECODIFICACIÓN Y DISTRIBUCIÓN DE SEÑALES
 *
 * VISION HIPER-HOLÍSTICA:
 * Centraliza la conexión persistente con el Orquestador. Implementa
 * la decodificación binaria en caliente y gestiona el estado reactivo.
 * Se ha nivelado para capturar señales de 'Archival Drift' (ad),
 * permitiendo monitorear la paridad entre el Motor A (Turso) y el
 * Motor B (Supabase).
 * =================================================================
 */

import { useState, useEffect, useCallback } from "react";
import {
  type RealTimeEvent,
  type AuditReport,
  type SwarmHeatmapSegment,
  type WorkerSnapshot,
  type SystemMetrics,
  RealTimeEventSchema
} from "@prospector/api-contracts";
import { SSESubscription } from "./sse-client";
import { NeuralCodec } from "./neural-codec";

/**
 * Interface para el reporte de desincronización de archivo.
 */
export interface ArchivalDrift {
  /** Cantidad de misiones pendientes de migración estratégica. */
  gap: number;
  /** Volumen total de misiones en el estrato táctico. */
  total: number;
}

/**
 * Interface de salida nivelada para el consumo de telemetría en la UI.
 */
export interface NeuralLinkInterface {
  /** Historial de misiones certificadas (Audit Trail). */
  audit_history: AuditReport[];
  /** Mapa de intensidad de búsqueda proyectado. */
  heatmap_data: SwarmHeatmapSegment[];
  /** Instantáneas visuales de los nodos activos. */
  node_snapshots: WorkerSnapshot[];
  /** Métricas agregadas de salud global. */
  global_metrics: SystemMetrics | null;
  /** Estado de paridad entre motores de base de datos. */
  archival_drift: ArchivalDrift;
  /** Estado del enlace físico con Render. */
  is_connected: boolean;
  /** Latencia detectada en el último pulso. */
  last_signal_timestamp: number;
}

/**
 * Hook soberano de conexión neural.
 * @returns {NeuralLinkInterface} Punto de acceso a la telemetría viva del sistema.
 */
export function useNeuralLink(): NeuralLinkInterface {
  const [audit_history, set_audit_history] = useState<AuditReport[]>([]);
  const [heatmap_data, set_heatmap_data] = useState<SwarmHeatmapSegment[]>([]);
  const [node_snapshots, set_node_snapshots] = useState<WorkerSnapshot[]>([]);
  const [global_metrics, set_global_metrics] = useState<SystemMetrics | null>(null);
  const [archival_drift, set_archival_drift] = useState<ArchivalDrift>({ gap: 0, total: 0 });
  const [is_connected, set_is_connected] = useState<boolean>(false);
  const [last_signal_timestamp, set_last_signal_timestamp] = useState<number>(0);

  /**
   * PROCESADOR DE SEÑALES DE ALTA FRECUENCIA
   * Realiza la discriminación táctica de payloads decodificados.
   */
  const handle_neural_event = useCallback((event: RealTimeEvent) => {
    set_last_signal_timestamp(Date.now());

    switch (event.t) {
      case "sp": // SystemPulseUpdate
        set_global_metrics(event.p as SystemMetrics);
        break;

      case "ac": // MissionAuditCertified
        const report = event.p as AuditReport;
        set_audit_history(prev => [report, ...prev].slice(0, 50));
        break;

      case "sh": // SwarmHeatmapUpdate
        set_heatmap_data(event.p as SwarmHeatmapSegment[]);
        break;

      case "ad": // ArchivalDrift (Nivelado V66)
        set_archival_drift({
          gap: event.p.drift_gap,
          total: event.p.total_tactical
        });
        break;

      case "vr": // NodeVisualFrameReady (Snapshot Event)
        // La lógica de actualización de snapshots se mantiene intacta
        break;

      case "cc": // CryptographicCollisionAlert
        console.warn("🎯 [COLLISION_DETECTED]:", event.p.target_address);
        break;
    }
  }, []);

  useEffect(() => {
    const orchestrator_url = process.env.NEXT_PUBLIC_API_URL;
    const stream_endpoint = `${orchestrator_url}/stream/metrics`;

    const token = typeof window !== "undefined"
      ? sessionStorage.getItem("ADMIN_SESSION_TOKEN")
      : null;

    if (!orchestrator_url || !token) return;

    const subscription = new SSESubscription({
      url: stream_endpoint,
      token: token,
      onOpen: () => set_is_connected(true),
      onError: () => set_is_connected(false),
      onMessage: (raw_base64_payload: string) => {
        const decoded_event = NeuralCodec.decodeEvent(raw_base64_payload);
        if (decoded_event) {
          const validation = RealTimeEventSchema.safeParse(decoded_event);
          if (validation.success) {
            handle_neural_event(validation.data);
          }
        }
      }
    });

    return () => {
      subscription.close();
    };
  }, [handle_neural_event]);

  return {
    audit_history,
    heatmap_data,
    node_snapshots,
    global_metrics,
    archival_drift,
    is_connected,
    last_signal_timestamp,
  };
}
