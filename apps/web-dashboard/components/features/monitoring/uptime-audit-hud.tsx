/**
 * =================================================================
 * APARATO: UPTIME AUDIT TRAIL HUD (V110.0 - ELASTIC CONTROL)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE PROGRESO SATOSHI-XP Y ESCALADO
 * =================================================================
 */

"use client";

import React, { useState } from "react";
import { Activity, Server, Clock, ShieldCheck, ChevronRight, Zap } from "lucide-react";
import { useNeuralLink, controlApi } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

export function UptimeAuditTrailHUD() {
  const { audit_history, is_connected } = useNeuralLink();
  const [target_node_count, set_target_node_count] = useState(1);

  /**
   * Ejecuta el escalado dinámico del enjambre mediante el C2 Controller.
   */
  const handle_swarm_scaling = async (new_count: number) => {
    set_target_node_count(new_count);
    await controlApi.launchSwarm({
      worker_count: new_count,
      shard_count: 1, // Basado en la cuenta única de inicio
      ref: "main"
    });
  };

  return (
    <div className="space-y-6 animate-in fade-in duration-700">
      {/* PANEL DE CONTROL DINÁMICO */}
      <Card className="bg-[#0a0a0a] border-zinc-800 shadow-2xl">
        <CardHeader className="border-b border-white/5 bg-white/2">
          <div className="flex justify-between items-center">
            <CardTitle className="text-[10px] font-black text-emerald-500 uppercase tracking-[0.3em] flex items-center gap-3 font-mono">
              <Server className="w-4 h-4" />
              Swarm Elastic Controller
            </CardTitle>
            <div className="flex items-center gap-4">
               <span className="text-[9px] font-mono text-zinc-500">Target Nodes: {target_node_count}</span>
               <div className="flex gap-1">
                 <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handle_swarm_scaling(Math.max(1, target_node_count - 1))}
                    className="h-6 px-2 text-[9px]"
                 >-</Button>
                 <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handle_swarm_scaling(target_node_count + 1)}
                    className="h-6 px-2 text-[9px]"
                 >+</Button>
               </div>
            </div>
          </div>
        </CardHeader>
        <CardContent className="p-6">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="space-y-1">
              <p className="text-[8px] font-black text-zinc-600 uppercase font-mono">Current Simulation Mode</p>
              <div className="flex items-center gap-2 text-white font-bold font-mono text-xs">
                <Zap className="w-3 h-3 text-amber-500" /> SATOSHI_XP_GENESIS
              </div>
            </div>
            {/* Barra de Progreso Temporal */}
            <div className="md:col-span-2 space-y-2">
              <div className="flex justify-between text-[8px] font-mono text-zinc-500 uppercase">
                <span>Temporal Coverage Progress (Uptime)</span>
                <span className="text-emerald-500">Audit in Progress...</span>
              </div>
              <div className="h-1.5 w-full bg-zinc-900 rounded-full overflow-hidden border border-white/5">
                <div className="h-full bg-emerald-500 animate-pulse w-[15%]" />
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* LISTADO DE MISIONES CERTIFICADAS */}
      <Card className="bg-black border-zinc-900 overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full text-left font-mono">
            <thead>
              <tr className="text-[8px] font-black text-zinc-600 uppercase border-b border-zinc-800">
                <th className="p-4">Mission Identifier</th>
                <th className="p-4">Uptime Segment</th>
                <th className="p-4">Node Health</th>
                <th className="p-4">Status</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-white/5">
              {audit_history.map((report) => (
                <tr key={report.job_mission_identifier} className="hover:bg-white/2 transition-colors group">
                  <td className="p-4 text-[10px] text-blue-400 font-bold">
                    {report.job_mission_identifier.substring(0, 8)}
                  </td>
                  <td className="p-4">
                    <div className="flex items-center gap-2">
                      <Clock className="w-3 h-3 text-zinc-500" />
                      <span className="text-[10px] text-zinc-300">
                        {report.audit_footprint_checkpoint}
                      </span>
                    </div>
                  </td>
                  <td className="p-4">
                    <div className="flex items-center gap-2">
                      <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-[0_0_5px_#10b981]" />
                      <span className="text-[9px] text-zinc-500">{report.worker_node_identifier}</span>
                    </div>
                  </td>
                  <td className="p-4">
                    <span className="px-2 py-0.5 bg-zinc-900 text-zinc-400 rounded text-[8px] font-black uppercase">
                      {report.final_mission_status}
                    </td>
                  </tr>
              ))}
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  );
}
