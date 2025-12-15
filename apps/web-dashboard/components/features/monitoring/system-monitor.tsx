// apps/web-dashboard/components/features/monitoring/system-monitor.tsx
/**
 * =================================================================
 * APARATO: SYSTEM MONITOR (Real-Time Edition)
 * RESPONSABILIDAD: VISUALIZACIÓN DE TELEMETRÍA VÍA SSE
 * CARACTERÍSTICAS: INDICADORES DE CONECTIVIDAD, DATOS EN VIVO
 * =================================================================
 */

"use client";

import { Activity, Cpu, Server, Key, Wifi, WifiOff } from "lucide-react";
import { useRealTimeTelemetry } from "@prospector/api-client"; // ✅ Nuevo Hook
import { StatCard } from "@/components/ui/kit/stat-card";
import { cn } from "@/lib/utils/cn";

// Utilidad para formatear hashrate
const formatHashrate = (hashes: number) => {
  if (hashes > 1e9) return `${(hashes / 1e9).toFixed(2)} GH/s`;
  if (hashes > 1e6) return `${(hashes / 1e6).toFixed(2)} MH/s`;
  if (hashes > 1e3) return `${(hashes / 1e3).toFixed(2)} kH/s`;
  return `${hashes} H/s`;
};

const formatCompact = (num: number) =>
  new Intl.NumberFormat("en-US", {
    notation: "compact",
    maximumFractionDigits: 1,
  }).format(num);

export function SystemMonitor() {
  const { metrics, isConnected, isLoading } = useRealTimeTelemetry();

  return (
    <div className="space-y-4">
      {/* Connection Status Indicator */}
      <div className="flex justify-end">
        <div
          className={cn(
            "flex items-center gap-2 px-2 py-1 rounded text-[10px] font-mono border transition-colors",
            isConnected
              ? "bg-emerald-950/30 border-emerald-900/50 text-emerald-500"
              : "bg-red-950/30 border-red-900/50 text-red-500",
          )}
        >
          {isConnected ? (
            <Wifi className="w-3 h-3" />
          ) : (
            <WifiOff className="w-3 h-3 animate-pulse" />
          )}
          {isConnected ? "NEURAL LINK ACTIVE" : "RECONNECTING STREAM..."}
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 animate-in fade-in slide-in-from-bottom-2 duration-500">
        {/* 1. HASHRATE GLOBAL */}
        <StatCard
          label="Global Hashrate"
          value={formatHashrate(metrics?.global_hashrate || 0)}
          subValue="Combined swarm power"
          icon={Activity}
          color="emerald"
          loading={isLoading}
        />

        {/* 2. ACTIVE NODES */}
        <StatCard
          label="Active Nodes"
          value={metrics?.active_nodes || 0}
          subValue="Reporting in last 60s"
          icon={Server}
          color="blue"
          loading={isLoading}
        />

        {/* 3. WORKLOAD */}
        <StatCard
          label="Jobs In Flight"
          value={metrics?.jobs_in_flight || 0}
          subValue="Ranges being scanned"
          icon={Cpu}
          color="amber"
          loading={isLoading}
        />

        {/* 4. SCAN VELOCITY (Proyección) */}
        <StatCard
          label="Daily Projection"
          value={formatCompact((metrics?.global_hashrate || 0) * 86400)}
          subValue="Keys / 24h"
          icon={Key}
          color="purple"
          loading={isLoading}
        />
      </div>
    </div>
  );
}
