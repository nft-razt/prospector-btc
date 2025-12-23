/**
 * =================================================================
 * APARATO: INTELLIGENT NODE FRAME (V14.5 - PANOPTICON)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE VÍDEO CON HUD TÉRMICO SUPERPUESTO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el 'Panóptico Térmico'. Integra la telemetría de hardware
 * (cpu_temp, load) directamente sobre el flujo visual del worker.
 * Resuelve errores TS2339 y TS2305 al sincronizar con 'NodeHardwareMetrics'.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import {
  Thermometer,
  Cpu,
  Zap,
  RefreshCw,
  ShieldCheck,
  Activity,
  AlertCircle
} from "lucide-react";
import { type WorkerSnapshot, type NodeHardwareMetrics } from "@prospector/api-client";
import { cn } from "@/lib/utils/cn";

interface IntelligentNodeFrameProperties {
  /** Instantánea certificada del nodo en el enjambre. */
  snapshot: WorkerSnapshot;
  /** Control de renderizado para optimización de scroll. */
  is_visible: boolean;
}

export function IntelligentNodeFrame({
  snapshot,
  is_visible
}: IntelligentNodeFrameProperties): React.ReactElement {
  // EXTRACCIÓN SOBERANA (Resolución TS2339)
  const {
    worker_identifier,
    operational_status,
    snapshot_base64_data,
    captured_at_timestamp,
    hardware_metrics
  } = snapshot;

  /**
   * FALLBACK DE TELEMETRÍA (L5 Resilience)
   * Garantiza que el frame renderice incluso si el heartbeat está pendiente.
   */
  const telemetry_state: NodeHardwareMetrics = useMemo(() => hardware_metrics || {
    cpu_frequency_megahertz: 0,
    cpu_load_percentage: 0,
    cpu_temperature_celsius: 0,
    ram_usage_megabytes: 0,
    is_thermal_throttling_active: false
  }, [hardware_metrics]);

  const is_thermal_critical = telemetry_state.cpu_temperature_celsius > 85;

  return (
    <div className={cn(
      "relative aspect-video rounded-xl overflow-hidden border bg-zinc-950 transition-all duration-500 group",
      is_thermal_critical ? "border-red-500 shadow-[0_0_20px_#ef444433]" : "border-white/5 hover:border-emerald-500/40"
    )}>

      {/* 1. LIENZO VISUAL (GHOST RUN) */}
      <div className="absolute inset-0 z-0 bg-zinc-900/30 flex items-center justify-center">
        {is_visible && snapshot_base64_data ? (
          <img
            src={snapshot_base64_data}
            alt={`Node_${worker_identifier}`}
            className={cn(
              "w-full h-full object-cover transition-all duration-1000",
              operational_status !== "running" ? "grayscale opacity-30 blur-[2px]" : "opacity-60 group-hover:opacity-100"
            )}
            loading="lazy"
          />
        ) : (
          <RefreshCw className="w-5 h-5 text-zinc-800 animate-spin" />
        )}
      </div>

      {/* 2. ESTRATO HUD: CENTRO DE MANDO SUPERPUESTO */}
      <div className="absolute inset-0 z-10 pointer-events-none p-3 flex flex-col justify-between select-none">

        {/* Cabecera: ID y Reloj de Captura */}
        <div className="flex justify-between items-start">
          <div className="bg-black/70 backdrop-blur-md px-2 py-1 rounded border border-white/10 flex items-center gap-2">
            <div className={cn(
              "w-1.5 h-1.5 rounded-full",
              operational_status === "running" ? "bg-emerald-500 animate-pulse" : "bg-red-500"
            )} />
            <span className="text-[9px] font-black text-white font-mono uppercase tracking-tighter">
              UNIDAD_{worker_identifier.substring(0, 8).toUpperCase()}
            </span>
          </div>

          <div className="bg-black/40 backdrop-blur-sm px-1.5 py-0.5 rounded text-[7px] font-mono text-zinc-500">
            {new Date(captured_at_timestamp).toLocaleTimeString([], { hour12: false })}
          </div>
        </div>

        {/* Sensores de Hardware */}
        <div className="space-y-2">
          <div className="grid grid-cols-3 gap-2">
            <HardwareSensorBadge
              icon={<Thermometer className="w-2.5 h-2.5" />}
              value={`${telemetry_state.cpu_temperature_celsius.toFixed(0)}°C`}
              is_alert={is_thermal_critical}
            />
            <HardwareSensorBadge
              icon={<Cpu className="w-2.5 h-2.5" />}
              value={`${telemetry_state.cpu_load_percentage.toFixed(0)}%`}
              is_alert={telemetry_state.cpu_load_percentage > 90}
            />
            <HardwareSensorBadge
              icon={<ShieldCheck className="w-2.5 h-2.5" />}
              value={operational_status.toUpperCase()}
              is_alert={operational_status !== "running"}
            />
          </div>

          {/* Barra de Presión de Memoria */}
          <div className="bg-black/60 backdrop-blur-md p-1.5 rounded border border-white/5 space-y-1">
            <div className="flex justify-between text-[7px] font-black font-mono uppercase text-zinc-500">
              <span>Memory_Pressure</span>
              <span className="text-zinc-300">{(telemetry_state.ram_usage_megabytes / 1024).toFixed(1)} GB</span>
            </div>
            <div className="h-0.5 w-full bg-zinc-900 rounded-full overflow-hidden">
              <div
                className="h-full bg-blue-500 transition-all duration-1000 shadow-[0_0_5px_#3b82f6]"
                style={{ width: `${Math.min((telemetry_state.ram_usage_megabytes / 12000) * 100, 100)}%` }}
              />
            </div>
          </div>
        </div>
      </div>

      {/* 3. CAPA DE AMBIENTACIÓN TÉCNICA (CRT FX) */}
      <div className="absolute inset-0 z-20 pointer-events-none opacity-20">
        <div className="absolute inset-0 bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.1)_50%),linear-gradient(90deg,rgba(255,0,0,0.02),rgba(0,255,0,0.01),rgba(0,0,255,0.02))] bg-size-[100%_2px,3px_100%]" />
      </div>
    </div>
  );
}

/** ÁTOMO: BADGE DE SENSOR */
function HardwareSensorBadge({ icon, value, is_alert }: { icon: React.ReactNode, value: string, is_alert: boolean }) {
  return (
    <div className="bg-black/80 backdrop-blur-md p-1.5 rounded border border-white/5 flex flex-col gap-0.5">
      <div className="flex items-center gap-1 opacity-40 text-zinc-100">
        {icon}
      </div>
      <span className={cn("text-[9px] font-mono font-black tracking-tighter", is_alert ? "text-red-500" : "text-emerald-500")}>
        {value}
      </span>
    </div>
  );
}
