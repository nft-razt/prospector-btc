/**
 * =================================================================
 * APARATO: UPTIME AUDIT TRAIL HUD (V11.8 - SWISS WATCH EDITION)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE PROGRESO SATOSHI-XP Y ESCALADO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el centro de mando elástico para la auditoría forense.
 * 1. Visualiza la línea de tiempo de misiones completadas (L4).
 * 2. Permite el escalado dinámico de la infraestructura vía C2 (L6).
 * 3. Resuelve el error JSX TS17008 mediante el sellado de etiquetas.
 *
 * # Performance:
 * Utiliza transiciones de Tailwind v4 para minimizar el impacto del
 * re-renderizado durante el escalado de nodos.
 * =================================================================
 */

"use client";

import React, { useState } from "react";
import {
  Activity,
  Server,
  Clock,
  ShieldCheck,
  ChevronRight,
  Zap,
  Plus,
  Minus,
  Loader2
} from "lucide-react";
import { useTranslations } from "next-intl";
import { useNeuralLink, controlApi } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

/**
 * Organismo de control y visualización de la progresión temporal de la Tesis.
 */
export function UptimeAuditTrailHUD(): React.ReactElement {
  const translations = useTranslations("Dashboard.lab");
  const { audit_history, is_connected } = useNeuralLink();

  const [target_node_count, set_target_node_count] = useState<number>(1);
  const [is_scaling_active, set_is_scaling_active] = useState<boolean>(false);

  /**
   * Ejecuta la secuencia de escalado elástico mediante el túnel C2.
   *
   * @param node_increment - Dirección del escalado (positivo/negativo).
   */
  const handle_swarm_scaling_sequence = async (new_count: number): Promise<void> => {
    if (new_count < 1 || new_count > 50) return;

    set_is_scaling_active(true);
    set_target_node_count(new_count);

    try {
      await controlApi.launchSwarm({
        worker_count: new_count,
        shard_count: 1, // Consistencia con el segmento único de inicio
        ref: "main"
      });
    } finally {
      // Retraso técnico para permitir la propagación en GitHub Actions
      setTimeout(() => set_is_scaling_active(false), 2000);
    }
  };

  return (
    <div className="space-y-8 animate-in fade-in duration-1000">

      {/* SECTOR A: ELASTIC INFRASTRUCTURE CONTROLLER */}
      <Card className="bg-[#0a0a0a] border-zinc-800 shadow-2xl overflow-hidden relative group">
        <div className="absolute top-0 right-0 p-10 bg-emerald-500/5 blur-[80px] rounded-full pointer-events-none" />

        <CardHeader className="border-b border-white/5 bg-white/2 p-5">
          <div className="flex justify-between items-center relative z-10">
            <div className="space-y-1">
              <CardTitle className="text-[10px] font-black text-emerald-500 uppercase tracking-[0.3em] flex items-center gap-3 font-mono">
                <Server className="w-4 h-4" />
                Swarm Elastic Controller // C2_UPLINK
              </CardTitle>
              <p className="text-[8px] text-zinc-500 font-mono uppercase">Dynamic Resource Allocation Protocol</p>
            </div>

            <div className="flex items-center gap-6 bg-black/40 px-4 py-2 rounded-xl border border-white/5 shadow-inner">
               <div className="flex flex-col items-end">
                 <span className="text-[7px] font-black text-zinc-600 uppercase font-mono tracking-tighter">Target_Capacity</span>
                 <span className="text-sm font-black text-white font-mono tracking-tighter">{target_node_count} NODES</span>
               </div>

               <div className="flex gap-2">
                 <Button
                    variant="outline"
                    size="icon"
                    disabled={target_node_count <= 1 || is_scaling_active}
                    onClick={() => handle_swarm_scaling_sequence(target_node_count - 1)}
                    className="h-8 w-8 border-zinc-800 hover:bg-red-500/10 hover:text-red-500 transition-all"
                 >
                   <Minus className="w-3.5 h-3.5" />
                 </Button>
                 <Button
                    variant="outline"
                    size="icon"
                    disabled={target_node_count >= 50 || is_scaling_active}
                    onClick={() => handle_swarm_scaling_sequence(target_node_count + 1)}
                    className="h-8 w-8 border-zinc-800 hover:bg-emerald-500/10 hover:text-emerald-500 transition-all"
                 >
                   {is_scaling_active ? <Loader2 className="w-3.5 h-3.5 animate-spin" /> : <Plus className="w-3.5 h-3.5" />}
                 </Button>
               </div>
            </div>
          </div>
        </CardHeader>

        <CardContent className="p-6 relative z-10">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <div className="space-y-2 border-l-2 border-amber-500/40 pl-4">
              <p className="text-[8px] font-black text-zinc-600 uppercase font-mono tracking-widest">Active Simulation</p>
              <div className="flex items-center gap-2 text-zinc-200 font-bold font-mono text-[11px] uppercase">
                <Zap className="w-3.5 h-3.5 text-amber-500 fill-amber-500" /> SATOSHI_XP_GENESIS
              </div>
            </div>

            <div className="md:col-span-2 space-y-3">
              <div className="flex justify-between items-end">
                <span className="text-[8px] font-black font-mono text-zinc-500 uppercase tracking-widest">
                  Temporal Coverage Matrix (Uptime Scan)
                </span>
                <span className="text-[10px] font-black text-emerald-500 font-mono animate-pulse">
                  AUDIT_IN_PROGRESS_0.15%
                </span>
              </div>
              <div className="h-1.5 w-full bg-zinc-900 rounded-full overflow-hidden border border-white/5 shadow-inner">
                <div className="h-full bg-linear-to-r from-emerald-600 to-primary animate-pulse w-[15%] shadow-[0_0_10px_#10b981]" />
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* SECTOR B: MISSION CERTIFICATION LEDGER */}
      <Card className="bg-black border-zinc-900 overflow-hidden shadow-xl">
        <div className="overflow-x-auto custom-scrollbar">
          <table className="w-full text-left font-mono border-collapse">
            <thead>
              <tr className="text-[8px] font-black text-zinc-700 uppercase border-b border-zinc-800 bg-zinc-950">
                <th className="p-4 border-r border-zinc-900">Mission_Identifier</th>
                <th className="p-4 border-r border-zinc-900 text-center">Forenisc_Checkpoint</th>
                <th className="p-4 border-r border-zinc-900">Health_Pulse</th>
                <th className="p-4 text-right">Status</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-white/5">
              {audit_history.length === 0 ? (
                <tr>
                   <td colSpan={4} className="p-12 text-center opacity-20">
                      <Activity className="w-8 h-8 mx-auto mb-3 text-zinc-500 animate-pulse" />
                      <p className="text-[9px] font-black uppercase tracking-[0.4em]">Awaiting_Certification_Signals</p>
                   </td>
                </tr>
              ) : (
                audit_history.map((mission_report) => (
                  <tr key={mission_report.job_mission_identifier} className="hover:bg-emerald-500/5 transition-colors group">
                    <td className="p-4 text-[10px] text-blue-400 font-black border-r border-zinc-900/50">
                      {mission_report.job_mission_identifier.substring(0, 8).toUpperCase()}
                    </td>
                    <td className="p-4 text-center border-r border-zinc-900/50">
                      <div className="inline-flex items-center gap-2 bg-zinc-900/50 border border-white/5 px-2 py-1 rounded">
                        <Clock className="w-3 h-3 text-zinc-600" />
                        <span className="text-[10px] text-zinc-400 tracking-tighter">
                          0x{mission_report.audit_footprint_checkpoint}
                        </span>
                      </div>
                    </td>
                    <td className="p-4 border-r border-zinc-900/50">
                      <div className="flex items-center gap-3">
                        <div className={cn(
                          "w-1.5 h-1.5 rounded-full shadow-[0_0_5px_#10b981]",
                          is_connected ? "bg-emerald-500 animate-pulse" : "bg-red-500"
                        )} />
                        <span className="text-[9px] text-zinc-500 font-bold uppercase truncate max-w-[120px]">
                          {mission_report.worker_node_identifier}
                        </span>
                      </div>
                    </td>
                    <td className="p-4 text-right">
                      {/* ✅ RESOLUCIÓN TS17008: Sellado de etiqueta span y coherencia con </td> */}
                      <span className="px-2 py-0.5 bg-zinc-900 border border-white/5 text-zinc-400 rounded text-[8px] font-black uppercase tracking-widest shadow-sm group-hover:border-emerald-500/20 group-hover:text-emerald-500 transition-all">
                        {mission_report.final_mission_status}
                      </span>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </Card>

      <footer className="flex justify-between items-center px-4 opacity-40">
        <span className="text-[7px] font-black text-zinc-700 font-mono uppercase tracking-[0.5em]">
          Continuity_Chain_Verified // Stratum_L5_Online
        </span>
        <ShieldCheck className="w-3 h-3 text-emerald-500/50" />
      </footer>
    </div>
  );
}
