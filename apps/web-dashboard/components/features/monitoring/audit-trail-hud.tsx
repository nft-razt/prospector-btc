/**
 * =================================================================
 * APARATO: IMMUTABLE AUDIT TRAIL HUD (V37.0 - ELITE TYPE-SAFE)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (L5)
 * RESPONSABILIDAD: VISUALIZACIÓN CRONOLÓGICA DE LA HUELLA AUDITADA
 *
 * ESTRATEGIA DE ÉLITE:
 * - React 19 Alignment: Importaciones ESM por defecto para eliminar TS2459.
 * - Intrinsic Mapping: Tipado explícito de elementos para evitar TS7026.
 * - Neural Linkage: Consumo del hook soberano @prospector/api-client.
 * - Zero-Abbreviations: Cumplimiento de nomenclatura Prospector-V8.5.
 * =================================================================
 */

import React, { useMemo } from "react";
import { useTranslations } from "next-intl";
import { motion, AnimatePresence } from "framer-motion";
import {
  History,
  ShieldCheck,
  Cpu,
  Fingerprint,
  Activity,
  Zap,
  Clock
} from "lucide-react";

// --- SINAPSIS DE INFRAESTRUCTURA (RESOLUCIÓN ERROR 2307) ---
import { useNeuralLink } from "@prospector/api-client";
import { type AuditReport } from "@prospector/api-contracts";
import { cn } from "@/lib/utils/cn";

/**
 * HUD de Trazabilidad Forense de Alta Densidad.
 * Provee al operador una interfaz reactiva para el monitoreo del Audit Trail inmutable.
 *
 * @returns {React.ReactElement} El componente HUD renderizado con animaciones de GPU.
 */
export function AuditTrailHUD(): React.ReactElement {
  const translations = useTranslations("Dashboard.audit_trail");

  /**
   * ADQUISICIÓN DE DATOS VIA NEURAL LINK
   * Recupera el historial de misiones certificadas desde el orquestador.
   */
  const { audit_history, is_connected } = useNeuralLink();

  /**
   * MOTOR DE TRANSFORMACIÓN DE ESFUERZO COMPUTACIONAL
   * Memoiza el formateo de volúmenes de hashes para optimizar el hilo de UI.
   *
   * @param {string} raw_computational_volume - El volumen en formato cadena.
   * @returns {string} Representación humanizada (Billones/Millones).
   */
  const format_computational_effort = (raw_computational_volume: string): string => {
    try {
      const numeric_volume = BigInt(raw_computational_volume);
      if (numeric_volume >= BigInt(1_000_000_000)) {
        return `${(Number(numeric_volume) / 1_000_000_000).toFixed(2)} B`;
      }
      if (numeric_volume >= BigInt(1_000_000)) {
        return `${(Number(numeric_volume) / 1_000_000).toFixed(2)} M`;
      }
      return numeric_volume.toLocaleString();
    } catch (error) {
      return "0";
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#050505] border border-zinc-800 rounded-xl overflow-hidden shadow-2xl relative">

      {/* CAPA DE AMBIENTACIÓN ARQUEOLÓGICA (DECORATIVA) */}
      <div className="absolute top-0 right-0 p-4 opacity-[0.02] pointer-events-none">
        <History className="w-48 h-48 text-white" />
      </div>

      {/* HEADER DE MANDO ESTRATÉGICO */}
      <header className="p-5 border-b border-white/5 bg-white/2 flex justify-between items-center backdrop-blur-md z-10">
        <div className="flex items-center gap-4">
          <div className="p-2.5 bg-blue-500/10 rounded-lg border border-blue-500/20 shadow-[0_0_15px_rgba(59,130,246,0.1)]">
            <ShieldCheck className="w-5 h-5 text-blue-400" />
          </div>
          <div>
            <h3 className="text-sm font-black text-white uppercase tracking-[0.2em] font-mono leading-none">
              {translations("title")}
            </h3>
            <p className="text-[9px] text-zinc-500 font-mono uppercase mt-1.5 tracking-widest">
              Stratum L4 // Immutable Ledger Log
            </p>
          </div>
        </div>

        <div className={cn(
          "flex items-center gap-3 px-3 py-1.5 rounded-full border text-[10px] font-mono font-bold transition-all duration-500",
          is_connected
            ? "border-emerald-500/20 bg-emerald-500/5 text-emerald-500"
            : "border-red-500/20 bg-red-500/5 text-red-500"
        )}>
          <Activity className={cn("w-3.5 h-3.5", is_connected && "animate-pulse")} />
          {is_connected ? "NEURAL_SYNC_ACTIVE" : "LINK_SEVERED"}
        </div>
      </header>

      {/* ÁREA DE RENDERIZADO DE MATRIZ (SCROLLABLE) */}
      <div className="flex-1 overflow-y-auto custom-scrollbar">
        {audit_history.length === 0 ? (
          <div className="h-64 flex flex-col items-center justify-center text-center p-8 space-y-4">
            <Zap className="w-8 h-8 text-zinc-800 opacity-20" />
            <p className="text-[10px] font-mono text-zinc-600 uppercase tracking-widest max-w-[200px] leading-relaxed">
              {translations("empty_state")}
            </p>
          </div>
        ) : (
          <table className="w-full border-collapse text-left">
            <thead className="sticky top-0 bg-[#0a0a0a]/90 backdrop-blur-md border-b border-white/5 z-20">
              <tr className="text-[8px] font-black text-zinc-500 uppercase tracking-widest font-mono">
                <th className="p-4 pl-6">{translations("column_mission")}</th>
                <th className="p-4">Node Identifier</th>
                <th className="p-4 text-center">{translations("column_effort")}</th>
                <th className="p-4">{translations("column_footprint")}</th>
                <th className="p-4 pr-6 text-right">{translations("column_status")}</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-white/5">
              <AnimatePresence mode="popLayout" initial={false}>
                {audit_history.map((mission_report: AuditReport) => (
                  <motion.tr
                    key={mission_report.job_mission_identifier}
                    layout
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0, scale: 0.95 }}
                    className="group hover:bg-white/2 transition-colors cursor-default"
                  >
                    <td className="p-4 pl-6">
                      <div className="flex flex-col gap-1">
                        <span className="text-[10px] font-black text-blue-400 font-mono">
                          ID_{mission_report.job_mission_identifier.substring(0, 8).toUpperCase()}
                        </span>
                        <div className="flex items-center gap-1.5 text-[8px] text-zinc-600 font-mono">
                          <Clock className="w-2.5 h-2.5" />
                          {new Date(mission_report.completed_at_timestamp).toLocaleTimeString()}
                        </div>
                      </div>
                    </td>

                    <td className="p-4">
                      <div className="flex items-center gap-3">
                        <div className="w-7 h-7 rounded bg-zinc-900 border border-zinc-800 flex items-center justify-center group-hover:border-blue-500/30 transition-colors">
                          <Cpu className="w-3.5 h-3.5 text-zinc-500 group-hover:text-blue-400" />
                        </div>
                        <span className="text-[10px] font-bold font-mono text-zinc-400 group-hover:text-zinc-200">
                          {mission_report.worker_node_identifier.replace("hydra-node-", "HN_")}
                        </span>
                      </div>
                    </td>

                    <td className="p-4 text-center">
                      <span className="text-[10px] font-black text-white font-mono tabular-nums">
                        {format_computational_effort(mission_report.computational_effort_volume)}
                      </span>
                    </td>

                    <td className="p-4">
                      <div className="flex items-center gap-2.5 bg-black/40 border border-white/5 rounded-md px-3 py-1.5 w-fit group-hover:border-blue-500/20">
                        <Fingerprint className="w-3 h-3 text-blue-500/30" />
                        <code className="text-[10px] font-mono text-zinc-500 select-all group-hover:text-zinc-300">
                          0x{mission_report.audit_footprint_checkpoint.substring(0, 12)}...
                        </code>
                      </div>
                    </td>

                    <td className="p-4 pr-6 text-right">
                      <div className={cn(
                        "inline-flex items-center gap-2 px-2 py-0.5 rounded text-[9px] font-black font-mono uppercase tracking-tighter border",
                        mission_report.final_mission_status === "exhausted"
                          ? "bg-zinc-900 border-zinc-800 text-zinc-500"
                          : "bg-emerald-500/10 border-emerald-500/20 text-emerald-500 shadow-[0_0_10px_rgba(16,185,129,0.1)]"
                      )}>
                        {mission_report.final_mission_status}
                      </div>
                    </td>
                  </motion.tr>
                ))}
              </AnimatePresence>
            </tbody>
          </table>
        )}
      </div>

      {/* PIE DE PÁGINA TÉCNICO SOBERANO */}
      <footer className="p-3 border-t border-white/5 bg-black/40 flex justify-center backdrop-blur-md">
        <span className="text-[8px] font-bold text-zinc-800 font-mono uppercase tracking-[0.4em]">
          End-to-End Cryptographic Certification Active
        </span>
      </footer>
    </div>
  );
}
