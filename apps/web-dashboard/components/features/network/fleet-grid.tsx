/**
 * =================================================================
 * APARATO: FLEET GRID VISUALIZER (V14.5 - GOLD MASTER)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: RENDERIZADO DE TELEMETRÍA VISUAL DEL ENJAMBRE
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la rejilla de vigilancia visual. Consume instantáneas
 * desde el Neural Link y las orquestra mediante 'IntelligentNodeFrame'.
 * Resuelve errores TS2339 al utilizar 'worker_identifier' y
 * 'operational_status' según el contrato V14.0.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import { useTranslations } from "next-intl";
import {
  Eye,
  Wifi,
  WifiOff,
  Monitor,
  Activity,
  AlertTriangle
} from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { useRealTimeTelemetry, type WorkerSnapshot } from "@prospector/api-client";
import { IntelligentNodeFrame } from "./intelligent-node-frame";
import { Skeleton } from "@/components/ui/kit/skeleton";
import { cn } from "@/lib/utils/cn";

/**
 * Organismo de visualización masiva para los nodos enjambre.
 */
export function FleetGrid(): React.ReactElement {
  const translations = useTranslations("Dashboard.fleet");
  const { snapshots, isConnected, isLoading } = useRealTimeTelemetry();

  /**
   * MÉTRICAS DERIVADAS DEL ENJAMBRE
   * Calcula el estado de salud de la flota basándose en el estatus operativo nominal.
   */
  const fleet_metrics = useMemo(() => {
    return {
      total_units: snapshots.length,
      active_units: snapshots.filter((unit) => unit.operational_status === "running").length,
      alert_units: snapshots.filter(
        (unit) => unit.operational_status === "error" || unit.operational_status === "captcha"
      ).length,
    };
  }, [snapshots]);

  // Gestión de estados iniciales de hidratación
  if (isLoading && snapshots.length === 0) return <FleetGridSkeleton />;

  if (!isLoading && snapshots.length === 0) {
    return <FleetEmptyState is_connection_lost={!isConnected} />;
  }

  return (
    <div className="space-y-6 animate-in fade-in duration-1000">
      {/* HUD DE ESTADO DE RED */}
      <header className="flex flex-col md:flex-row justify-between items-end pb-4 border-b border-zinc-800 gap-4">
        <div className="flex flex-col gap-1">
          <h3 className="text-sm font-black text-white uppercase tracking-[0.2em] flex items-center gap-3 font-mono">
            <Eye className="w-4 h-4 text-emerald-500" />
            {translations("title")}
          </h3>
          <div className="flex gap-4 text-[9px] font-mono text-zinc-500 uppercase font-bold">
            <span>Grid_Nodes: <strong className="text-white">{fleet_metrics.total_units}</strong></span>
            {fleet_metrics.alert_units > 0 && (
              <span className="text-red-500 animate-pulse">
                Critical_Alerts: <strong>{fleet_metrics.alert_units}</strong>
              </span>
            )}
          </div>
        </div>

        <div className={cn(
          "flex items-center gap-2 px-3 py-1 rounded-full text-[9px] font-black font-mono border transition-all",
          isConnected ? "border-emerald-500/30 text-emerald-500 bg-emerald-500/5" : "border-red-500/30 text-red-500 bg-red-500/5"
        )}>
          {isConnected ? <Wifi className="w-3 h-3" /> : <WifiOff className="w-3 h-3 animate-bounce" />}
          {isConnected ? "NEURAL_LINK: ACTIVE" : "NEURAL_LINK: SEVERED"}
        </div>
      </header>

      {/* REJILLA DE PROCESAMIENTO VISUAL */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-6">
        <AnimatePresence mode="popLayout">
          {snapshots.map((node_snapshot: WorkerSnapshot) => (
            <motion.div
              key={node_snapshot.worker_identifier}
              layout
              initial={{ opacity: 0, scale: 0.95 }}
              animate={{ opacity: 1, scale: 1 }}
              exit={{ opacity: 0, scale: 0.9 }}
              transition={{ duration: 0.3 }}
            >
              <IntelligentNodeFrame
                snapshot={node_snapshot}
                is_visible={true}
              />
            </motion.div>
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
}

/** ÁTOMO: ESTADO VACÍO */
function FleetEmptyState({ is_connection_lost }: { is_connection_lost: boolean }) {
  return (
    <div className="flex flex-col items-center justify-center h-80 border border-dashed border-zinc-800 rounded-2xl bg-zinc-900/10 text-zinc-500">
      <Monitor className="w-12 h-12 mb-4 opacity-10" />
      <p className="text-[10px] font-mono font-black uppercase tracking-[0.3em]">
        {is_connection_lost ? "Awaiting_Handshake..." : "No_Grid_Signals_Detected"}
      </p>
    </div>
  );
}

/** ÁTOMO: ESQUELETO DE CARGA */
function FleetGridSkeleton() {
  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
      {[...Array(4)].map((_, index) => (
        <Skeleton key={index} className="aspect-video rounded-xl bg-zinc-900/50 border border-zinc-800" />
      ))}
    </div>
  );
}
