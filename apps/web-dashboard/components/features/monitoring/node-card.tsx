/**
 * =================================================================
 * APARATO: NODE HEALTH HUD (V110.0 - HIGH DENSITY)
 * CLASIFICACIÓN: ATOMIC UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE MÉTRICAS DE ESTRÉS DE HARDWARE
 * =================================================================
 */

import React from "react";
import { Thermometer, Cpu, Zap, Database } from "lucide-react";
import { cn } from "@/lib/utils/cn";

interface NodeHealthProps {
  mhz: number;
  temp: number;
  load: number;
  mem_bytes: number;
  worker_id: string;
}

export function NodeHealthIndicator({ mhz, temp, load, mem_bytes, worker_id }: NodeHealthProps) {
  const is_hot = temp > 80;
  const mem_gb = (mem_bytes / (1024 ** 3)).toFixed(1);

  return (
    <div className="bg-black/40 border border-zinc-800 rounded-lg p-3 space-y-4 relative overflow-hidden group">
      {/* Indicador de Alerta Térmica */}
      {is_hot && (
        <div className="absolute top-0 left-0 w-full h-0.5 bg-red-500 animate-pulse shadow-[0_0_10px_#ef4444]" />
      )}

      <div className="flex justify-between items-center">
        <span className="text-[8px] font-black text-zinc-500 uppercase font-mono tracking-widest">
          Unit: {worker_id.substring(0, 8)}
        </span>
        <div className={cn(
          "px-1.5 py-0.5 rounded text-[7px] font-bold font-mono",
          is_hot ? "bg-red-500/20 text-red-500" : "bg-emerald-500/10 text-emerald-500"
        )}>
          {temp.toFixed(1)}°C
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4 font-mono">
        <div className="space-y-1">
          <p className="text-[7px] text-zinc-600 uppercase">Clock Intelligence</p>
          <div className="flex items-center gap-2">
            <Zap className="w-3 h-3 text-amber-500" />
            <span className="text-[10px] text-zinc-200 font-bold">{(mhz / 1000).toFixed(2)} GHz</span>
          </div>
        </div>

        <div className="space-y-1">
          <p className="text-[7px] text-zinc-600 uppercase">Compute Pressure</p>
          <div className="flex items-center gap-2">
            <Cpu className="w-3 h-3 text-blue-500" />
            <span className="text-[10px] text-zinc-200 font-bold">{load.toFixed(1)}%</span>
          </div>
        </div>
      </div>

      {/* BARRA DE MEMORIA TÁCTICA */}
      <div className="space-y-1.5">
        <div className="flex justify-between text-[7px] font-bold uppercase text-zinc-700">
          <span>Memory Footprint</span>
          <span>{mem_gb} GB</span>
        </div>
        <div className="h-1 w-full bg-zinc-900 rounded-full overflow-hidden border border-white/5">
          <div
            className="h-full bg-blue-500 transition-all duration-1000"
            style={{ width: `${Math.min((parseFloat(mem_gb) / 12) * 100, 100)}%` }}
          />
        </div>
      </div>
    </div>
  );
}
