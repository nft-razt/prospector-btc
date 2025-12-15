// libs/infra/api-client-ts/src/lib/hooks-rt.ts
// =================================================================
// APARATO: REAL-TIME TELEMETRY HOOK (v7.0 - MEMORY SAFE)
// RESPONSABILIDAD: GESTIÓN DE ESTADO REACTIVO VIA SSE
// CARACTERÍSTICAS: AUTO-PRUNING (GARBAGE COLLECTION)
// =================================================================

import { useState, useEffect, useRef, useCallback } from "react";
import { SSESubscription } from "./sse-client";
import {
  type SystemMetrics,
  type RealTimeEvent,
  type WorkerSnapshot,
  RealTimeEventSchema,
} from "@prospector/api-contracts";

const PRUNE_INTERVAL_MS = 30000; // Limpiar cada 30s
const STALE_THRESHOLD_MS = 120000; // Eliminar nodos mudos por >2 min

export function useRealTimeTelemetry() {
  const [metrics, setMetrics] = useState<SystemMetrics | null>(null);
  const [snapshots, setSnapshots] = useState<Record<string, WorkerSnapshot>>(
    {},
  );
  const [isConnected, setIsConnected] = useState(false);

  const subscriptionRef = useRef<SSESubscription | null>(null);
  const cleanerRef = useRef<NodeJS.Timeout | null>(null);

  const handleEvent = useCallback((event: RealTimeEvent) => {
    switch (event.event) {
      case "Metrics":
        setMetrics(event.data);
        break;
      case "SnapshotReceived":
        setSnapshots((prev) => ({
          ...prev,
          [event.data.worker_id]: event.data,
        }));
        break;
      case "ColissionAlert":
        console.warn(`🚨 COLISIÓN DETECTADA en worker ${event.data.worker_id}`);
        break;
    }
  }, []);

  // --- GARBAGE COLLECTOR (GC) ---
  useEffect(() => {
    cleanerRef.current = setInterval(() => {
      const now = Date.now();
      setSnapshots((prev) => {
        let hasChanges = false;
        const next = { ...prev };

        Object.keys(next).forEach((key) => {
          const ts = new Date(next[key].timestamp).getTime();
          if (now - ts > STALE_THRESHOLD_MS) {
            delete next[key];
            hasChanges = true;
          }
        });

        return hasChanges ? next : prev;
      });
    }, PRUNE_INTERVAL_MS);

    return () => {
      if (cleanerRef.current) clearInterval(cleanerRef.current);
    };
  }, []);

  useEffect(() => {
    const API_URL =
      process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000/api/v1";
    const STREAM_URL = `${API_URL}/stream/metrics`;

    // Soporte híbrido: Token de sesión (Browser) o ENV (Build/Server)
    const token =
      typeof window !== "undefined"
        ? sessionStorage.getItem("ADMIN_SESSION_TOKEN") ||
          process.env.NEXT_PUBLIC_API_TOKEN
        : undefined;

    if (!token) return;

    subscriptionRef.current = new SSESubscription({
      url: STREAM_URL,
      token,
      onOpen: () => setIsConnected(true),
      onError: () => setIsConnected(false),
      onMessage: (rawEvent) => {
        const result = RealTimeEventSchema.safeParse(rawEvent);
        if (result.success) handleEvent(result.data);
      },
    });

    return () => {
      subscriptionRef.current?.close();
    };
  }, [handleEvent]);

  return {
    metrics,
    // Ordenamos por timestamp descendente para ver lo más reciente primero
    snapshots: Object.values(snapshots).sort(
      (a, b) =>
        new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime(),
    ),
    isConnected,
    isLoading: !metrics && !isConnected,
  };
}
