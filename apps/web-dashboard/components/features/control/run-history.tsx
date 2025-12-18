/**
 * =================================================================
 * APARATO: STRATEGIC RUN HISTORY HUD (V25.0)
 * CLASIFICACIÓN: FEATURE UI (L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DEL CICLO DE VIDA DE TRABAJOS (L4)
 *
 * ESTRATEGIA DE INTEGRIDAD:
 * - Data Source: Engine B (Supabase / Estratégico).
 * - Type Safety: Vinculación estricta con ArchivedJob Contract.
 * - UX: Estados de carga (Skeleton) y Empty States unificados.
 * =================================================================
 */

"use client";

import React from "react";
import { useQuery } from "@tanstack/react-query";
import { History, HardDrive, Clock, Activity, ShieldCheck } from "lucide-react";
import {
  strategicArchive,
  type ArchivedJob
} from "@prospector/api-client";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent
} from "@/components/ui/kit/card";
import { Skeleton } from "@/components/ui/kit/skeleton";
import { cn } from "@/lib/utils/cn";

/**
 * HUD de Historial de Ejecuciones.
 * Provee una ventana al archivo frío de la base de datos estratégica.
 */
export function RunHistory() {
  // ADQUISICIÓN DE DATOS ESTRATÉGICOS (L4)
  const { data: archives, isLoading } = useQuery<ArchivedJob[]>({
    queryKey: ["run-history-archive"],
    queryFn: () => strategicArchive.getHistory(10),
    staleTime: 60000, // Los datos archivados son inmutables (1 min refresh)
  });

  return (
    <Card className="bg-[#050505] border-slate-800 flex flex-col h-full shadow-2xl relative overflow-hidden group">
      {/* CAPA DE AMBIENTACIÓN TÉCNICA */}
      <div className="absolute top-0 right-0 p-2 opacity-5 pointer-events-none group-hover:opacity-10 transition-opacity duration-1000">
        <HardDrive className="w-24 h-24 text-white" />
      </div>

      <CardHeader className="border-b border-white/5 bg-white/2 p-4">
        <CardTitle className="text-[10px] font-black text-zinc-400 uppercase tracking-[0.3em] flex items-center gap-3 font-mono">
          <History className="w-3.5 h-3.5 text-blue-400" />
          Strategic Archives // Cold Storage
        </CardTitle>
      </CardHeader>

      <CardContent className="p-0 flex-1 overflow-y-auto custom-scrollbar">
        {isLoading ? (
          <div className="p-4 space-y-3">
            {[1, 2, 3, 4].map((index) => (
              <Skeleton key={`skeleton-job-${index}`} className="h-14 w-full bg-zinc-900/50 rounded-lg" />
            ))}
          </div>
        ) : (
          <div className="divide-y divide-white/5">
            {archives?.length === 0 && (
              <div className="p-16 text-center">
                <Activity className="w-8 h-8 text-zinc-800 mx-auto mb-4 opacity-20" />
                <p className="text-[9px] font-mono text-zinc-600 uppercase tracking-widest">
                  No Archived Data Detected
                </p>
              </div>
            )}

            {/* ✅ RESOLUCIÓN Error 7006: Tipado explícito del parámetro de iteración */}
            {archives?.map((job: ArchivedJob) => (
              <div
                key={job.id}
                className="p-4 hover:bg-white/2 transition-all duration-300 group/item cursor-default border-l-2 border-transparent hover:border-blue-500/40"
              >
                <div className="flex justify-between items-start mb-2">
                  <div className="flex flex-col gap-1">
                    <span className="text-[9px] font-black text-zinc-300 font-mono uppercase tracking-tight">
                      Range: <span className="text-blue-400">{job.range_start.substring(0, 12)}...</span>
                    </span>
                    <div className="flex items-center gap-2 text-[8px] text-zinc-500 font-mono">
                      <Clock className="w-2.5 h-2.5" />
                      {new Date(job.created_at).toLocaleDateString()} // {job.strategy_type}
                    </div>
                  </div>

                  <div className="text-right">
                    <div className={cn(
                      "text-[10px] font-black font-mono",
                      job.findings_count > 0 ? "text-emerald-500 animate-pulse" : "text-zinc-600"
                    )}>
                      {job.findings_count > 0 ? `🎯 ${job.findings_count} COLLISION(S)` : "CLEAN"}
                    </div>
                    <div className="text-[7px] text-zinc-700 font-mono uppercase tracking-tighter mt-1">
                      Avg Hash: {(job.average_hashrate / 1000).toFixed(2)} kH/s
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </CardContent>

      {/* FOOTER TÉCNICO DE CONTEXTO */}
      <div className="p-3 border-t border-white/5 bg-black/40 flex justify-between items-center px-5">
        <div className="flex items-center gap-2">
          <ShieldCheck className="w-3 h-3 text-zinc-600" />
          <span className="text-[7px] font-bold text-zinc-700 font-mono uppercase tracking-widest">
            Engine B: Supabase PostgreSQL
          </span>
        </div>
        <div className="flex items-center gap-2">
          <div className="h-1 w-1 bg-blue-500 rounded-full animate-pulse shadow-[0_0_5px_#3b82f6]" />
          <span className="text-[7px] font-black text-zinc-500 font-mono uppercase">
            Sync Active
          </span>
        </div>
      </div>
    </Card>
  );
}
