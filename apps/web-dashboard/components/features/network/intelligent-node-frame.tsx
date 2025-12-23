/**
 * =================================================================
 * APARATO: INTELLIGENT NODE FRAME (V15.0 - THERMAL STRESS HUD)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE VÍDEO Y SALUD DEL SILICIO
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import { Thermometer, Cpu, Zap, Activity, AlertTriangle, ShieldCheck } from "lucide-react";
import { type WorkerSnapshot, type NodeHardwareMetrics } from "@prospector/api-client";
import { cn } from "@/lib/utils/cn";

interface IntelligentNodeFrameProperties {
  snapshot: WorkerSnapshot;
  is_visible: boolean;
}

export function IntelligentNodeFrame({ snapshot, is_visible }: IntelligentNodeFrameProperties): React.ReactElement {
  const { worker_identifier, operational_status, snapshot_base64_data, hardware_metrics } = snapshot;

  const telemetry: NodeHardwareMetrics = useMemo(() => hardware_metrics || {
    cpu_frequency_megahertz: 0,
    cpu_load_percentage: 0,
    cpu_temperature_celsius: 0,
    ram_usage_megabytes: 0,
    is_thermal_throttling_active: false
  }, [hardware_metrics]);

  const is_overheated = telemetry.cpu_temperature_celsius > 80;
  const is_throttling = telemetry.is_thermal_throttling_active;

  return (
    <div className={cn(
      "relative aspect-video rounded-xl overflow-hidden border transition-all duration-500 bg-black group",
      is_overheated ? "border-red-500 shadow-[0_0_25px_rgba(239,68,68,0.3)]" : "border-white/10 hover:border-emerald-500/50"
    )}>
      {/* 1. FLUJO VISUAL */}
      <div className="absolute inset-0 z-0">
        {is_visible && snapshot_base64_data ? (
          <img
            src={snapshot_base64_data}
            alt={worker_identifier}
            className={cn("w-full h-full object-cover transition-opacity duration-1000", is_throttling ? "opacity-30" : "opacity-50 group-hover:opacity-80")}
          />
        ) : (
          <div className="w-full h-full flex items-center justify-center bg-zinc-900">
            <Activity className="w-6 h-6 text-zinc-800 animate-pulse" />
          </div>
        )}
      </div>

      {/* 2. OVERLAY DE ESTRÉS (CRITICAL HUD) */}
      <div className="absolute inset-0 z-10 p-3 flex flex-col justify-between pointer-events-none">
        <div className="flex justify-between items-start">
          <div className="px-2 py-1 bg-black/80 backdrop-blur-md border border-white/10 rounded flex items-center gap-2">
             <div className={cn("w-1.5 h-1.5 rounded-full", is_throttling ? "bg-red-500 animate-ping" : "bg-emerald-500")} />
             <span className="text-[9px] font-black text-white font-mono uppercase tracking-tighter">
               NODE_{worker_identifier.substring(0, 8)}
             </span>
          </div>

          {is_throttling && (
            <div className="bg-red-600 text-white text-[8px] font-black px-2 py-0.5 rounded animate-pulse font-mono">
              THERMAL_THROTTLING_ACTIVE
            </div>
          )}
        </div>

        <div className="space-y-2">
          {/* MATRIZ DE SENSORES */}
          <div className="grid grid-cols-3 gap-2">
            <div className="bg-black/80 backdrop-blur-md p-1.5 rounded border border-white/5">
              <p className="text-[6px] text-zinc-500 uppercase font-bold mb-1">Temp</p>
              <div className="flex items-center gap-1.5">
                <Thermometer className={cn("w-3 h-3", is_overheated ? "text-red-500" : "text-emerald-500")} />
                <span className={cn("text-xs font-black font-mono", is_overheated ? "text-red-400" : "text-zinc-200")}>
                  {telemetry.cpu_temperature_celsius.toFixed(0)}°C
                </span>
              </div>
            </div>

            <div className="bg-black/80 backdrop-blur-md p-1.5 rounded border border-white/5">
              <p className="text-[6px] text-zinc-500 uppercase font-bold mb-1">Load</p>
              <div className="flex items-center gap-1.5">
                <Cpu className="w-3 h-3 text-blue-500" />
                <span className="text-xs font-black font-mono text-zinc-200">
                  {telemetry.cpu_load_percentage.toFixed(0)}%
                </span>
              </div>
            </div>

            <div className="bg-black/80 backdrop-blur-md p-1.5 rounded border border-white/5">
              <p className="text-[6px] text-zinc-500 uppercase font-bold mb-1">Clock</p>
              <div className="flex items-center gap-1.5">
                <Zap className="w-3 h-3 text-amber-500" />
                <span className="text-xs font-black font-mono text-zinc-200">
                  {(telemetry.cpu_frequency_megahertz / 1000).toFixed(1)}G
                </span>
              </div>
            </div>
          </div>

          {/* BARRA DE MEMORIA */}
          <div className="bg-black/60 backdrop-blur-sm p-1 rounded border border-white/5">
            <div className="h-1 w-full bg-zinc-800 rounded-full overflow-hidden">
              <div
                className="h-full bg-emerald-500 transition-all duration-1000"
                style={{ width: `${Math.min((telemetry.ram_usage_megabytes / 12000) * 100, 100)}%` }}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
