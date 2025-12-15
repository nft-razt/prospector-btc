// apps/web-dashboard/components/features/network/fleet-grid.tsx
/**
 * =================================================================
 * APARATO: FLEET GRID VISUALIZER (v7.1 - CANONICAL STYLE)
 * RESPONSABILIDAD: RENDERIZADO DE TELEMETRÍA VISUAL
 * ESTÁNDAR: TAILWIND CSS v4 COMPLIANT
 * OPTIMIZACIÓN: React.memo() + CSS NATIVO
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import { useTranslations } from "next-intl";
import {
  Eye,
  Clock,
  Wifi,
  WifiOff,
  Monitor,
  Activity,
  AlertTriangle,
} from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

import { cn } from "@/lib/utils/cn";
import {
  useRealTimeTelemetry,
  type WorkerSnapshot,
} from "@prospector/api-client";
import { Skeleton } from "@/components/ui/kit/skeleton";

export function FleetGrid() {
  const t = useTranslations("Dashboard.fleet");
  const { snapshots, isConnected, isLoading } = useRealTimeTelemetry();

  // Métricas derivadas (Memoizadas para evitar recálculo en cada frame SSE)
  const metrics = useMemo(() => {
    return {
      total: snapshots.length,
      active: snapshots.filter((s) => s.status === "running").length,
      errors: snapshots.filter(
        (s) => s.status === "error" || s.status === "captcha",
      ).length,
    };
  }, [snapshots]);

  // Estados de carga y vacío
  if (isLoading && snapshots.length === 0) return <FleetSkeleton />;
  if (!isLoading && snapshots.length === 0)
    return <EmptyState isError={!isConnected} t={t} />;

  return (
    <div className="space-y-6">
      {/* HUD HEADER */}
      <div className="flex flex-col md:flex-row justify-between items-end pb-4 border-b border-zinc-800 gap-4">
        <div className="flex flex-col gap-1">
          <h3 className="text-sm font-bold text-zinc-100 uppercase tracking-widest flex items-center gap-2">
            <Eye className="w-4 h-4 text-emerald-500" />
            {t("title")}
          </h3>
          <div className="flex gap-3 text-[10px] font-mono text-zinc-500">
            <span>
              ACTIVE: <strong className="text-white">{metrics.total}</strong>
            </span>
            {metrics.errors > 0 && (
              <span className="text-red-500 animate-pulse">
                ALERTS: <strong>{metrics.errors}</strong>
              </span>
            )}
          </div>
        </div>

        <div
          className={cn(
            "flex items-center gap-2 px-2 py-1 rounded text-[9px] font-mono border transition-colors",
            isConnected
              ? "border-emerald-900/50 text-emerald-500"
              : "border-red-900/50 text-red-500",
          )}
        >
          {isConnected ? (
            <Wifi className="w-3 h-3" />
          ) : (
            <WifiOff className="w-3 h-3 animate-pulse" />
          )}
          {isConnected ? "NEURAL LINK" : "OFFLINE"}
        </div>
      </div>

      {/* GRID DE NODOS */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
        <AnimatePresence mode="popLayout">
          {snapshots.map((snap) => (
            <FleetNodeWrapper key={snap.worker_id} snap={snap} />
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
}

// Wrapper para animación que contiene el componente memorizado
const FleetNodeWrapper = ({ snap }: { snap: WorkerSnapshot }) => (
  <motion.div
    layout
    initial={{ opacity: 0, scale: 0.9 }}
    animate={{ opacity: 1, scale: 1 }}
    exit={{ opacity: 0, scale: 0.9 }}
    transition={{ duration: 0.2 }}
  >
    <FleetNodeCard snap={snap} />
  </motion.div>
);

// --- COMPONENTES MEMORIZADOS (OPTIMIZACIÓN) ---

const FleetNodeCard = React.memo(
  ({ snap }: { snap: WorkerSnapshot }) => {
    const isError = snap.status !== "running";

    return (
      <div
        className={cn(
          "relative group overflow-hidden rounded-lg border bg-black transition-all duration-300 hover:shadow-[0_0_20px_rgba(0,0,0,0.5)]",
          isError
            ? "border-red-900/50 hover:border-red-500/50"
            : "border-zinc-800 hover:border-emerald-500/30",
        )}
      >
        {/* Header Overlay (Fixed: bg-linear-to-b) */}
        <div className="absolute top-0 left-0 w-full bg-linear-to-b from-black/90 to-transparent p-2 flex justify-between items-start z-10 pointer-events-none">
          <span className="text-[9px] font-mono text-zinc-300 font-bold truncate w-24 drop-shadow-md bg-black/50 px-1 rounded">
            {snap.worker_id}
          </span>
          <StatusBadge status={snap.status} />
        </div>

        {/* Visual Feed */}
        <div className="aspect-video bg-zinc-900 relative group-hover:scale-105 transition-transform duration-700 ease-out">
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img
            src={snap.snapshot_base64}
            alt={`Feed ${snap.worker_id}`}
            className={cn(
              "w-full h-full object-cover transition-opacity duration-500",
              isError
                ? "opacity-50 grayscale"
                : "opacity-80 group-hover:opacity-100",
            )}
            loading="lazy" // Lazy loading nativo para performance en grids grandes
          />

          {/* Timestamp Footer (Fixed: bg-linear-to-t) */}
          <div className="absolute bottom-0 right-0 w-full bg-linear-to-t from-black/90 to-transparent p-2 flex justify-end z-10">
            <div className="flex items-center gap-1.5 text-[8px] text-zinc-400 font-mono bg-black/60 backdrop-blur px-1.5 py-0.5 rounded border border-zinc-800/50">
              <Clock className="w-2.5 h-2.5" />
              {new Date(snap.timestamp).toLocaleTimeString()}
            </div>
          </div>
        </div>

        {/* CRT Scanline Effect (Fixed: bg-size syntax) */}
        <div className="absolute inset-0 bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.25)_50%),linear-gradient(90deg,rgba(255,0,0,0.06),rgba(0,255,0,0.02),rgba(0,0,255,0.06))] z-20 bg-size-[100%_2px,3px_100%] pointer-events-none opacity-20" />
      </div>
    );
  },
  (prev, next) => {
    // Solo re-renderizar si el timestamp cambió (nueva imagen) o el estado cambió.
    return (
      prev.snap.timestamp === next.snap.timestamp &&
      prev.snap.status === next.snap.status
    );
  },
);

FleetNodeCard.displayName = "FleetNodeCard";

function StatusBadge({ status }: { status: string }) {
  const config = {
    running: {
      color: "text-emerald-500 border-emerald-500/20 bg-emerald-500/10",
      icon: Activity,
    },
    captcha: {
      color:
        "text-yellow-500 border-yellow-500/20 bg-yellow-500/10 animate-pulse",
      icon: AlertTriangle,
    },
    error: {
      color: "text-red-500 border-red-500/20 bg-red-500/10",
      icon: WifiOff,
    },
  };
  const style = config[status as keyof typeof config] || config.error;
  const Icon = style.icon;

  return (
    <div
      className={cn(
        "px-1.5 py-0.5 rounded text-[7px] font-black uppercase tracking-wider backdrop-blur-md border flex items-center gap-1",
        style.color,
      )}
    >
      <Icon className="w-2 h-2" />
      {status}
    </div>
  );
}

function EmptyState({ isError, t }: { isError: boolean; t: any }) {
  return (
    // Fixed: h-[300px] -> h-75 (Token semántico, 75 * 4px = 300px)
    <div className="flex flex-col items-center justify-center h-75 border border-dashed border-zinc-800 rounded-xl bg-zinc-900/10 text-zinc-500 animate-in fade-in zoom-in-95 duration-300">
      <div className="relative mb-4">
        <div className="absolute inset-0 bg-zinc-500/10 blur-xl rounded-full" />
        {isError ? (
          <WifiOff className="w-12 h-12 text-red-500/50 relative z-10" />
        ) : (
          <Monitor className="w-12 h-12 opacity-30 relative z-10" />
        )}
      </div>
      <p className="text-sm font-mono tracking-tight uppercase font-bold text-zinc-400">
        {isError ? t("connection_lost") : t("no_signal")}
      </p>
    </div>
  );
}

function FleetSkeleton() {
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-end pb-4 border-b border-zinc-800">
        <Skeleton className="h-6 w-48 bg-zinc-800" />
        <Skeleton className="h-6 w-24 bg-zinc-800" />
      </div>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {[...Array(4)].map((_, i) => (
          <Skeleton
            key={i}
            className="aspect-video rounded-lg bg-zinc-900/50 border border-zinc-800"
          />
        ))}
      </div>
    </div>
  );
}
