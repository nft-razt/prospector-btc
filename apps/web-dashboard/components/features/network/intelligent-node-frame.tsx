/**
 * =================================================================
 * APARATO: INTELLIGENT NODE FRAME (V60.0 - THERMAL AWARENESS)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE VÍDEO CON HUD DE ESTRÉS DE HARDWARE
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el 'Panóptico Térmico'. Superpone una capa de telemetría
 * decodificada directamente sobre el frame visual del worker.
 * Permite al operador identificar cuellos de botella de hardware
 * (CPU/RAM/Temperatura) en tiempo real sin abandonar la vista
 * de cuadrícula, optimizando la gestión de nodos efímeros.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import {
  Thermometer,
  Cpu,
  Zap,
  AlertCircle,
  RefreshCw,
  ShieldCheck,
  Activity
} from "lucide-react";
import { type WorkerSnapshot } from "@prospector/api-client";
import { cn } from "@/lib/utils/cn";

/**
 * Propiedades del marco inteligente del nodo.
 */
interface IntelligentNodeFrameProps {
  /** Instantánea con telemetría y frame base64. */
  snapshot: WorkerSnapshot;
  /** Estado de visibilidad para optimización de renderizado. */
  is_visible: boolean;
}

/**
 * Organismo de visualización para nodos individuales del enjambre.
 *
 * @param {IntelligentNodeFrameProps} props - Datos del snapshot y visibilidad.
 * @returns {React.ReactElement} El frame con HUD superpuesto.
 */
export function IntelligentNodeFrame({
  snapshot,
  is_visible
}: IntelligentNodeFrameProps): React.ReactElement {
  const { metrics, worker_id, status, snapshot_base64, timestamp } = snapshot;

  /**
   * CÁLCULO DE NIVEL DE RIESGO TÉRMICO
   * UX Perceptiva: Determina el color y animaciones según la temperatura.
   */
  const thermal_risk_level = useMemo(() => {
    // Los workers en Colab suelen operar entre 60°C y 85°C.
    if (metrics.cpu_temp > 85) return "critical";
    if (metrics.cpu_temp > 75) return "warning";
    return "stable";
  }, [metrics.cpu_temp]);

  /**
   * CÁLCULO DE PRESIÓN DE MEMORIA
   * Identifica si el worker está cerca del límite de 12GB de Colab Free.
   */
  const memory_pressure_percent = useMemo(() => {
    // Estimación basada en el límite nominal de 12.7GB (13000 MB)
    return Math.min((metrics.ram_usage_mb / 13000) * 100, 100);
  }, [metrics.ram_usage_mb]);

  return (
    <div className={cn(
      "relative aspect-video rounded-xl overflow-hidden border bg-zinc-950 transition-all duration-700 group",
      thermal_risk_level === "critical"
        ? "border-red-500 shadow-[0_0_30px_rgba(239,68,68,0.2)]"
        : "border-white/5 hover:border-emerald-500/30"
    )}>

      {/* 1. VISUAL FEED (THE GHOST RUN) */}
      <div className="absolute inset-0 z-0 bg-zinc-900/50 flex items-center justify-center">
        {is_visible && snapshot_base64 ? (
          <img
            src={snapshot_base64}
            alt={`Node ${worker_id} Feed`}
            className={cn(
              "w-full h-full object-cover transition-all duration-1000",
              status !== "running" ? "grayscale opacity-40 blur-sm" : "opacity-70 group-hover:opacity-100"
            )}
            loading="lazy"
          />
        ) : (
          <div className="flex flex-col items-center gap-2">
            <RefreshCw className="w-5 h-5 text-zinc-700 animate-spin" />
            <span className="text-[7px] font-mono text-zinc-600 uppercase tracking-widest">Awaiting Pulse</span>
          </div>
        )}
      </div>

      {/* 2. ESTRATO HUD: TELEMETRÍA SUPERPUESTA (INTERFAZ DE MANDO) */}
      <div className="absolute inset-0 z-10 pointer-events-none p-3 flex flex-col justify-between">

        {/* Cabecera del HUD: Identidad y Status */}
        <div className="flex justify-between items-start">
          <div className="bg-black/60 backdrop-blur-md px-2 py-1 rounded border border-white/10 flex items-center gap-2">
            <div className={cn(
              "w-1.5 h-1.5 rounded-full",
              status === "running" ? "bg-emerald-500 animate-pulse" : "bg-red-500"
            )} />
            <span className="text-[9px] font-black text-white font-mono uppercase tracking-tighter">
              {worker_id.replace("hydra-node-", "UNIDAD_")}
            </span>
          </div>

          <div className="flex flex-col items-end gap-1">
            {metrics.is_throttling && (
              <div className="bg-red-600 text-white px-2 py-0.5 rounded text-[7px] font-black animate-pulse shadow-lg">
                THROTTLING_ACTIVE
              </div>
            )}
            <div className="bg-black/40 backdrop-blur-sm px-1.5 py-0.5 rounded text-[7px] font-mono text-zinc-400">
              {new Date(timestamp).toLocaleTimeString([], { hour12: false })}
            </div>
          </div>
        </div>

        {/* Panel Inferior: Sensores de Hardware */}
        <div className="space-y-2">
          {/* Indicadores de Sensores Atómicos */}
          <div className="grid grid-cols-3 gap-2">
            <SensorBadge
              icon={<Thermometer className="w-2.5 h-2.5" />}
              label="TEMP"
              value={`${metrics.cpu_temp.toFixed(1)}°C`}
              status={thermal_risk_level}
            />
            <SensorBadge
              icon={<Cpu className="w-2.5 h-2.5" />}
              label="CARGA"
              value={`${metrics.cpu_load.toFixed(0)}%`}
              status={metrics.cpu_load > 90 ? "warning" : "stable"}
            />
            <SensorBadge
              icon={<ShieldCheck className="w-2.5 h-2.5" />}
              label="SALUD"
              value={metrics.is_throttling ? "ESTRÉS" : "ÓPTIMA"}
              status={metrics.is_throttling ? "critical" : "stable"}
            />
          </div>

          {/* Barra de Consumo de RAM (Colab Free Limit Shield) */}
          <div className="bg-black/60 backdrop-blur-md p-1.5 rounded border border-white/10 space-y-1">
            <div className="flex justify-between items-center text-[7px] font-black uppercase font-mono">
              <span className="text-zinc-500 flex items-center gap-1">
                <Activity className="w-2 h-2" /> Memoria Volátil
              </span>
              <span className={cn(
                memory_pressure_percent > 85 ? "text-red-400" : "text-zinc-300"
              )}>
                {(metrics.ram_usage_mb / 1024).toFixed(1)} GB / 12.7
              </span>
            </div>
            <div className="h-0.5 w-full bg-zinc-800 rounded-full overflow-hidden">
              <div
                className={cn(
                  "h-full transition-all duration-1000",
                  memory_pressure_percent > 85 ? "bg-red-500" : "bg-blue-500"
                )}
                style={{ width: `${memory_pressure_percent}%` }}
              />
            </div>
          </div>
        </div>
      </div>

      {/* 3. CAPA DE EFECTOS CRT (ESTÉTICA DE AUDITORÍA) */}
      <div className="absolute inset-0 z-20 pointer-events-none opacity-30">
        <div className="absolute inset-0 bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.1)_50%),linear-gradient(90deg,rgba(255,0,0,0.015),rgba(0,255,0,0.01),rgba(0,0,255,0.015))] bg-size-[100%_2px,3px_100%]" />
      </div>

    </div>
  );
}

/**
 * ÁTOMO: SENSOR BADGE
 * Componente interno para visualizar métricas individuales con coherencia visual.
 */
function SensorBadge({
  icon,
  label,
  value,
  status
}: {
  icon: React.ReactNode,
  label: string,
  value: string,
  status: "stable" | "warning" | "critical"
}) {
  const status_color_map = {
    stable: "text-emerald-500",
    warning: "text-amber-500",
    critical: "text-red-500",
  };

  return (
    <div className="bg-black/70 backdrop-blur-md p-1.5 rounded border border-white/5 flex flex-col gap-0.5">
      <div className="flex items-center gap-1 opacity-40">
        {icon}
        <span className="text-[6px] font-black uppercase tracking-widest font-mono">{label}</span>
      </div>
      <span className={cn("text-[9px] font-mono font-bold leading-none", status_color_map[status])}>
        {value}
      </span>
    </div>
  );
}
