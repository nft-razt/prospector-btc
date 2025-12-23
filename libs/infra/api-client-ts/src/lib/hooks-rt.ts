"use client";

/**
 * =================================================================
 * APARATO: REAL-TIME NEURAL LINK HOOK (V70.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: CONSUMO Y TRANSFORMACIÓN DE SEÑALES SSE
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el receptor de telemetría de alta frecuencia.
 * Transforma los payloads binarios/JSON del Orquestador en estados
 * reactivos tipados para el Dashboard. Resuelve el error de
 * asignación de deriva de archivo (Archival Drift) mediante
 * un mapeo explícito de propiedades.
 * =================================================================
 */

import { useState, useEffect, useCallback, useMemo } from "react";
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
 * Representa la métrica de paridad entre motores de base de datos.
 */
export interface ArchivalDrift {
  /** Número de misiones certificadas pendientes de migración. */
  gap_count: number;
  /** Conteo acumulado en el ledger táctico (Motor A). */
  total_count: number;
}

/**
 * Interfaz de mando para el consumo de telemetría en la UI.
 */
export interface NeuralLinkInterface {
  /** Historial de misiones certificadas recientemente. */
  audit_history: AuditReport[];
  /** Segmentos activos en el mapa de calor de la curva. */
  heatmap_data: SwarmHeatmapSegment[];
  /** Instantáneas visuales de los nodos enjambre. */
  node_snapshots: WorkerSnapshot[];
  /** Métricas de hardware y rendimiento globales. */
  global_metrics: SystemMetrics | null;
  /** Estatus de sincronización del archivo estratégico. */
  archival_drift: ArchivalDrift;
  /** Estado de la conexión con el túnel neural. */
  is_connected: boolean;
}

/**
 * Hook Soberano de Conexión Neural.
 *
 * @returns {NeuralLinkInterface} Punto de acceso a la telemetría viva.
 */
export function useNeuralLink(): NeuralLinkInterface {
  const [audit_history, set_audit_history] = useState<AuditReport[]>([]);
  const [heatmap_data, set_heatmap_data] = useState<SwarmHeatmapSegment[]>([]);
  const [node_snapshots, set_node_snapshots] = useState<WorkerSnapshot[]>([]);
  const [global_metrics, set_global_metrics] = useState<SystemMetrics | null>(null);
  const [archival_drift, set_archival_drift] = useState<ArchivalDrift>({ gap_count: 0, total_count: 0 });
  const [is_connected, set_is_connected] = useState<boolean>(false);

  /**
   * PROCESADOR DE EVENTOS DE DOMINIO
   * ✅ RESOLUCIÓN TS2352: Mapeo explícito de propiedades soberanas.
   */
  const handle_neural_signal = useCallback((event: RealTimeEvent) => {
    switch (event.t) {
      case "sp": // SystemPulseUpdate
        set_global_metrics(event.p as SystemMetrics);
        break;
      case "ac": // MissionAuditCertified
        set_audit_history(prev => [event.p as AuditReport, ...prev].slice(0, 50));
        break;
      case "sh": // SwarmHeatmapUpdate
        set_heatmap_data(event.p as SwarmHeatmapSegment[]);
        break;
      case "ad": // ArchivalDriftUpdate
        const signal_payload = event.p as { drift_gap_count: number; total_tactical_count: number };
        set_archival_drift({
          gap_count: signal_payload.drift_gap_count,
          total_count: signal_payload.total_tactical_count
        });
        break;
    }
  }, []);

  useEffect(() => {
    if (typeof window === "undefined") return;

    const orchestrator_url = process.env.NEXT_PUBLIC_API_URL;
    const auth_token = sessionStorage.getItem("ADMIN_SESSION_TOKEN");

    if (!orchestrator_url || !auth_token) return;

    const subscription = new SSESubscription({
      url: `${orchestrator_url}/stream/metrics`,
      token: auth_token,
      onOpen: () => set_is_connected(true),
      onError: () => set_is_connected(false),
      onMessage: (raw_payload: string) => {
        const decoded_event = NeuralCodec.decodeEvent(raw_payload);
        if (decoded_event) {
          const validation = RealTimeEventSchema.safeParse(decoded_event);
          if (validation.success) handle_neural_signal(validation.data);
        }
      }
    });

    return () => subscription.close();
  }, [handle_neural_signal]);

  return {
    audit_history,
    heatmap_data,
    node_snapshots,
    global_metrics,
    archival_drift,
    is_connected
  };
}

/**
 * Hook de telemetría simplificado para componentes de hardware.
 */
export function useRealTimeTelemetry() {
  const link = useNeuralLink();
  return useMemo(() => ({
    metrics: link.global_metrics,
    isConnected: link.is_connected,
    snapshots: link.node_snapshots,
    isLoading: !link.is_connected && !link.global_metrics
  }), [link]);
}
