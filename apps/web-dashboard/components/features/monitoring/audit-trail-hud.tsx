/**
 * =================================================================
 * APARATO: AUDIT TRAIL STRATEGIC HUD (V50.0 - REACTIVE)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DEL LEDGER INMUTABLE DE MISIONES
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la interfaz de certificación de la Tesis.
 * 1. Consume misiones certificadas desde el Neural Link (SSE).
 * 2. Visualiza la huella forense (Checkpoint Hex).
 * 3. Muestra el estatus de integridad del reporte.
 * 4. Animación de ráfagas para feedback táctico instantáneo.
 * =================================================================
 */

"use client";

import React from "react";
import {
  ShieldCheck,
  Fingerprint,
  Cpu,
  Clock,
  History,
  Activity,
  AlertCircle
} from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";
import { useNeuralLink } from "@prospector/api-client";
import { formatComputationalEffort, formatExecutionTime } from "@/lib/utils/telemetry";
import { cn } from "@/lib/utils/cn";

export function AuditTrailHUD() {
  // ADQUISICIÓN NEURAL: Obtenemos el historial de misiones del flujo binario
  const { audit_history, is_connected } = useNeuralLink();

  return (
    <div className="bg-black/60 backdrop-blur-xl border border-zinc-800 rounded-2xl overflow-hidden shadow-2xl flex flex-col h-full group">
      {/* HUD HEADER: Estatus del Enlace de Auditoría */}
      <header className="p-5 border-b border-white/5 bg-white/2 flex justify-between items-center">
        <div className="flex items-center gap-4">
          <div className="p-2 bg-blue-500/10 rounded-lg border border-blue-500/20">
            <History className="w-4 h-4 text-blue-400" />
          </div>
          <div>
            <h2 className="text-xs font-black text-white uppercase tracking-[0.3em] font-mono">
              Immutable Audit Ledger
            </h2>
            <p className="text-[8px] text-zinc-500 font-mono uppercase tracking-widest">
              Stratum L4 // Continuity Chain Verified
            </p>
          </div>
        </div>

        <div className={cn(
          "flex items-center gap-2 px-3 py-1 rounded-full border text-[9px] font-bold font-mono transition-all",
          is_connected ? "bg-emerald-500/10 border-emerald-500/20 text-emerald-500" : "bg-red-500/10 border-red-500/20 text-red-500"
        )}>
          <div className={cn("w-1.5 h-1.5 rounded-full", is_connected ? "bg-emerald-500 animate-pulse" : "bg-red-500")} />
          {is_connected ? "LINK_ACTIVE" : "LINK_SEVERED"}
        </div>
      </header>

      {/* TACTICAL TABLE BODY */}
      <div className="flex-1 overflow-y-auto custom-scrollbar">
        <table className="w-full text-left border-collapse font-mono">
          <thead className="sticky top-0 bg-[#050505] z-20">
            <tr className="text-[8px] font-black text-zinc-600 uppercase border-b border-zinc-800">
              <th className="p-4">Mission_ID</th>
              <th className="p-4 text-center">Volume_Effort</th>
              <th className="p-4">Forensic_Checkpoint</th>
              <th className="p-4 text-right">Status</th>
            </tr>
          </thead>
          <tbody className="relative">
            <AnimatePresence initial={false}>
              {audit_history.length === 0 ? (
                <tr>
                  <td colSpan={4} className="p-20 text-center">
                    <div className="flex flex-col items-center gap-3 opacity-20">
                      <Activity className="w-8 h-8 text-zinc-500 animate-pulse" />
                      <span className="text-[10px] uppercase font-black tracking-widest">Awaiting Mission Certification</span>
                    </div>
                  </td>
                </tr>
              ) : (
                audit_history.map((report) => (
                  <motion.tr
                    key={report.job_mission_identifier}
                    initial={{ opacity: 0, x: -20, backgroundColor: "rgba(16, 185, 129, 0.1)" }}
                    animate={{ opacity: 1, x: 0, backgroundColor: "transparent" }}
                    transition={{ duration: 0.5 }}
                    className="hover:bg-white/2 transition-colors border-b border-white/5 group/row"
                  >
                    {/* Mission ID con Identidad de Nodo */}
                    <td className="p-4">
                      <div className="flex flex-col gap-0.5">
                        <span className="text-[10px] text-blue-400 font-black">
                          {report.job_mission_identifier.substring(0, 8).toUpperCase()}
                        </span>
                        <span className="text-[7px] text-zinc-600 uppercase font-bold flex items-center gap-1">
                          <Cpu className="w-2 h-2" /> {report.worker_node_identifier.substring(0, 12)}
                        </span>
                      </div>
                    </td>

                    {/* Volumen de Esfuerzo (MH/GH/TH) */}
                    <td className="p-4 text-center">
                      <div className="flex flex-col gap-0.5">
                        <span className="text-[10px] text-zinc-200 font-black tracking-tighter">
                          {formatComputationalEffort(report.computational_effort_volume)}
                        </span>
                        <span className="text-[7px] text-zinc-600 uppercase">
                          in {formatExecutionTime(report.execution_duration_milliseconds)}
                        </span>
                      </div>
                    </td>

                    {/* Huella Forense (Checkpoint Hex) */}
                    <td className="p-4">
                      <div className="flex items-center gap-2 bg-black/40 border border-white/5 px-2 py-1.5 rounded-md group-hover/row:border-blue-500/30 transition-colors">
                        <Fingerprint className="w-3 h-3 text-zinc-700" />
                        <span className="text-[9px] text-zinc-500 truncate max-w-[140px] select-all">
                          0x{report.audit_footprint_checkpoint}
                        </span>
                      </div>
                    </td>

                    {/* Estatus Final con Sello de Integridad */}
                    <td className="p-4 text-right">
                      <div className="flex flex-col items-end gap-1.5">
                        <div className={cn(
                          "px-2 py-0.5 rounded text-[8px] font-black uppercase tracking-tighter",
                          report.final_mission_status === "completed"
                            ? "bg-emerald-500/10 text-emerald-500 border border-emerald-500/20"
                            : "bg-amber-500/10 text-amber-500 border border-amber-500/20"
                        )}>
                          {report.final_mission_status}
                        </div>
                        <div className="flex items-center gap-1 opacity-40 group-hover/row:opacity-100 transition-opacity">
                          <ShieldCheck className="w-2.5 h-2.5 text-emerald-500" />
                          <span className="text-[6px] text-zinc-500 font-bold uppercase tracking-widest">Certified</span>
                        </div>
                      </div>
                    </td>
                  </motion.tr>
                ))
              )}
            </AnimatePresence>
          </tbody>
        </table>
      </div>

      {/* HUD FOOTER: Resumen de Capa */}
      <footer className="p-3 bg-black/40 border-t border-white/5 flex justify-between items-center px-6">
        <div className="flex items-center gap-2">
          <AlertCircle className="w-3 h-3 text-zinc-600" />
          <span className="text-[7px] font-black text-zinc-600 uppercase tracking-[0.2em]">
            Integrity Checksum: SHA-256 Linkage Active
          </span>
        </div>
        <span className="text-[7px] font-bold text-zinc-800 font-mono uppercase">
          Tesis Stratum L5 // V10.8 Master
        </span>
      </footer>
    </div>
  );
}
