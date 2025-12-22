/**
 * =================================================================
 * APARATO: ARCHIVAL STATUS HUD (V110.0 - ELITE LEVELLED)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE PARIDAD DE MOTORES GEMELOS
 *
 * ESTRATEGIA DE ÉLITE:
 * - Reactive Sourcing: Consume 'archival_drift' desde el Neural Link.
 * - Zero Regressions: Mantiene la lógica de alerta visual por desincronización.
 * - Full Internationalization: Todas las cadenas delegadas a los diccionarios.
 * =================================================================
 */

"use client";

import React from "react";
import { ShieldCheck, ShieldAlert, Database, CloudSync } from "lucide-react";
import { useTranslations } from "next-intl";
import { useNeuralLink } from "@prospector/api-client";
import { cn } from "@/lib/utils/cn";

/**
 * Organismo visual para el monitoreo de integridad del archivo estratégico.
 * Alerta al operador si existe una deriva (drift) entre el Motor A y el Motor B.
 *
 * @returns {React.ReactElement} La tarjeta de estado de archivo nivelada.
 */
export function ArchivalStatusCard(): React.ReactElement {
  const t = useTranslations("Dashboard.archival_status");
  const { archival_drift, is_connected } = useNeuralLink();

  // Cálculo de estado degradado: Si hay una brecha mayor a 0 misiones
  const is_degraded = archival_drift.gap > 0;

  return (
    <div className={cn(
      "p-5 rounded-2xl border transition-all duration-1000 relative overflow-hidden group",
      is_degraded
        ? "bg-red-950/10 border-red-500/30 shadow-[0_0_40px_rgba(239,68,68,0.1)]"
        : "bg-emerald-950/5 border-emerald-500/20"
    )}>
      {/* Visual Glitch/Ambient Effect */}
      <div className="absolute top-0 right-0 p-4 opacity-5 group-hover:opacity-10 transition-opacity">
        <Database className={cn("w-16 h-16", is_degraded ? "text-red-500" : "text-emerald-500")} />
      </div>

      <div className="flex justify-between items-start mb-8 relative z-10">
        <div className="flex items-center gap-4">
          <div className={cn(
            "p-2 rounded-lg border",
            is_degraded ? "bg-red-500/10 border-red-500/20" : "bg-emerald-500/10 border-emerald-500/20"
          )}>
            <CloudSync className={cn("w-5 h-5", is_degraded ? "text-red-500" : "text-emerald-500")} />
          </div>
          <div>
            <h4 className="text-[11px] font-black text-white uppercase tracking-[0.2em] font-mono">
              {t("engine_b_parity")}
            </h4>
            <p className="text-[8px] text-zinc-500 font-mono uppercase tracking-widest">
              {t("strategic_vault_link")}
            </p>
          </div>
        </div>

        {is_connected ? (
          is_degraded ? (
            <ShieldAlert className="w-5 h-5 text-red-500 animate-pulse" />
          ) : (
            <ShieldCheck className="w-5 h-5 text-emerald-500" />
          )
        ) : (
          <div className="w-2 h-2 rounded-full bg-zinc-800 animate-pulse" />
        )}
      </div>

      <div className="space-y-5 relative z-10">
        <div className="flex justify-between items-end">
          <span className="text-[9px] text-zinc-600 font-bold uppercase font-mono tracking-tighter">
            {t("archival_integrity")}
          </span>
          <span className={cn(
            "text-2xl font-black font-mono tracking-tighter",
            is_degraded ? "text-red-400" : "text-emerald-400"
          )}>
            {is_degraded ? "DEGRADED" : "100.00%"}
          </span>
        </div>

        {/* ALERTA TÁCTICA: Solo visible si hay deriva de datos */}
        {is_degraded && (
          <div className="bg-red-500/10 border border-red-500/20 p-3 rounded-xl flex items-center gap-3 animate-in slide-in-from-bottom-2 duration-500">
            <ShieldAlert className="w-4 h-4 text-red-500 shrink-0" />
            <p className="text-[9px] text-red-200 font-mono leading-tight uppercase font-bold">
              {t("sync_drift_detected", { count: archival_drift.gap })}
            </p>
          </div>
        )}

        <div className="pt-4 border-t border-white/5 flex justify-between items-center">
            <p className="text-[7px] text-zinc-700 font-mono uppercase font-bold tracking-widest">
              {t("total_archived_missions")}: {archival_drift.total - archival_drift.gap}
            </p>
            <div className="flex gap-1">
              <div className={cn("h-1 w-3 rounded-full", is_degraded ? "bg-red-500" : "bg-emerald-500")} />
              <div className={cn("h-1 w-1 rounded-full", is_connected ? "bg-primary animate-pulse" : "bg-zinc-800")} />
            </div>
        </div>
      </div>
    </div>
  );
}
