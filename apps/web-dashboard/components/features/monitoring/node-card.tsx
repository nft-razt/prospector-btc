/**
 * =================================================================
 * APARATO: NODE TELEMETRY CARD
 * CLASIFICACIÓN: ATOMIC UI
 * RESPONSABILIDAD: VISUALIZACIÓN DE SALUD DE NODO INDIVIDUAL
 * ESTADO: THREAD-HEALTH AWARE
 * =================================================================
 */

import { Thermometer, Cpu, Zap } from "lucide-react";
import { cn } from "@/lib/utils/cn";

interface NodeProps {
  worker_id: string;
  mhz: number;
  cores: number;
  hashrate: number;
}

export function NodeHealthIndicator({ mhz, cores, hashrate }: NodeProps) {
  // Heurística de Throttling: Si la frecuencia es < 1.5GHz en instancias Cloud
  const isThrottled = mhz > 0 && mhz < 1500;

  return (
    <div className="bg-black/40 border border-zinc-800 rounded-lg p-3 space-y-3">
      <div className="flex justify-between items-center">
        <span className="text-[8px] font-black text-zinc-500 uppercase font-mono">Process Intelligence</span>
        {isThrottled && (
          <span className="flex items-center gap-1 text-[8px] font-black text-red-500 animate-pulse">
            <Thermometer className="w-3 h-3" /> THROTTLED
          </span>
        )}
      </div>

      <div className="grid grid-cols-3 gap-2">
        <div className="flex flex-col">
          <span className="text-[7px] text-zinc-600 uppercase">Clock Speed</span>
          <span className={cn(
            "text-[10px] font-mono font-bold",
            isThrottled ? "text-red-400" : "text-emerald-400"
          )}>
            {mhz > 0 ? `${(mhz / 1000).toFixed(1)} GHz` : "N/A"}
          </span>
        </div>

        <div className="flex flex-col">
          <span className="text-[7px] text-zinc-600 uppercase">Topolgy</span>
          <span className="text-[10px] font-mono font-bold text-zinc-300">
            {cores} Cores
          </span>
        </div>

        <div className="flex flex-col items-end">
          <span className="text-[7px] text-zinc-600 uppercase">Performance</span>
          <span className="text-[10px] font-mono font-bold text-blue-400">
            {(hashrate / 1000).toFixed(1)} KH/s
          </span>
        </div>
      </div>

      {/* Mini-Graph de carga */}
      <div className="h-1 w-full bg-zinc-900 rounded-full overflow-hidden">
        <div
          className={cn("h-full transition-all duration-1000", isThrottled ? "bg-red-500" : "bg-primary")}
          style={{ width: '100%' }}
        />
      </div>
    </div>
  );
}
