/**
 * =================================================================
 * APARATO: FORENSIC AUDIT LEDGER (V11.4 - ELITE LEVELLED)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE AUDITORÍA DE RED EN TIEMPO REAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el libro mayor de auditoría forense para los vectores
 * de la Tesis. Resuelve el error TS2571 mediante la inyección del
 * contrato 'VerifiedVectorAuditReport'.
 *
 * # Performance:
 * Utiliza 'staleTime' extendido para reducir ráfagas innecesarias
 * hacia el proxy de la Blockchain. Renderizado acelerado por GPU.
 * =================================================================
 */

"use client";

import React from "react";
import { useQuery } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import {
  History,
  ShieldCheck,
  AlertTriangle,
  Database,
  Activity,
  ArrowUpRight
} from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

// --- SINAPSIS DE INFRAESTRUCTURA (L2 & L4) ---
import { labApi, type VerifiedVectorAuditReport } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { cn } from "@/lib/utils/cn";

/**
 * Organismo para la visualización detallada del estatus de los vectores de control.
 */
export function ForensicAuditLedger(): React.ReactElement {
  const translations = useTranslations("Dashboard.lab");

  /**
   * ADQUISICIÓN DE INTELIGENCIA ESTRATÉGICA
   * ✅ RESOLUCIÓN TS2571: El genérico garantiza que 'audit_reports'
   * posea las propiedades del contrato de laboratorio.
   */
  const { data: audit_reports, isLoading, isError } = useQuery<VerifiedVectorAuditReport[]>({
    queryKey: ["forensic-brainwallet-dataset-ledger"],
    queryFn: () => labApi.listForensicAuditReports(),
    staleTime: 600000, // 10 Minutos de persistencia en caché
  });

  return (
    <div className="space-y-6 animate-in fade-in duration-1000">
      <Card className="bg-black border-zinc-800 shadow-2xl overflow-hidden relative group">
        {/* Efecto visual de fondo: Red de Escaneo */}
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.02] pointer-events-none" />

        <CardHeader className="border-b border-white/5 bg-zinc-900/30 p-6 relative z-10">
          <div className="flex justify-between items-center">
            <div className="space-y-1.5">
              <CardTitle className="text-xs font-black text-white uppercase tracking-[0.4em] font-mono flex items-center gap-3">
                <History className="w-4 h-4 text-emerald-500" />
                {translations("audit_ledger_title")}
              </CardTitle>
              <p className="text-[8px] text-zinc-500 font-mono uppercase tracking-widest">
                Stratum L4 // Live Blockchain Synchronization Active
              </p>
            </div>

            <div className="flex items-center gap-3">
               <div className="h-1.5 w-1.5 rounded-full bg-primary animate-pulse shadow-[0_0_8px_#10b981]" />
               <span className="text-[9px] font-black text-zinc-400 font-mono uppercase">Neural_Uplink: Online</span>
            </div>
          </div>
        </CardHeader>

        <CardContent className="p-0 overflow-x-auto custom-scrollbar relative z-10">
          <table className="w-full text-left font-mono border-collapse">
            <thead>
              <tr className="text-[8px] font-black text-zinc-600 uppercase border-b border-zinc-800 bg-black/60">
                <th className="p-4 border-r border-zinc-900">Vector_ID</th>
                <th className="p-4 border-r border-zinc-900">Input_Source</th>
                <th className="p-4 border-r border-zinc-900">Cryptographic_Identity</th>
                <th className="p-4 text-center border-r border-zinc-900">Integrity</th>
                <th className="p-4 text-right border-r border-zinc-900 text-emerald-500">Live_Balance_BTC</th>
                <th className="p-4 text-right">Activity</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-white/5">
              <AnimatePresence mode="popLayout">
                {isLoading ? (
                  <ForensicSkeletonRows />
                ) : (
                  audit_reports?.map((report) => (
                    <motion.tr
                      key={report.vector_identifier}
                      initial={{ opacity: 0, y: 10 }}
                      animate={{ opacity: 1, y: 0 }}
                      className="hover:bg-emerald-500/5 transition-all group"
                    >
                      {/* ID del Vector con Padding */}
                      <td className="p-4 text-[10px] text-zinc-600 border-r border-zinc-900/50">
                        {report.vector_identifier.toString().padStart(2, "0")}
                      </td>

                      {/* Fuente de Entropía (Passphrase) */}
                      <td className="p-4 text-[10px] text-zinc-200 font-bold border-r border-zinc-900/50 italic">
                        "{report.source_passphrase}"
                      </td>

                      {/* Identidad Criptográfica (Dirección + WIF Preview) */}
                      <td className="p-4 border-r border-zinc-900/50">
                        <div className="flex flex-col gap-1">
                          <span className="text-[10px] text-zinc-400 select-all tracking-tight">
                            {report.derived_bitcoin_address}
                          </span>
                          <span className="text-[7px] text-zinc-700 uppercase font-black tracking-tighter">
                            WIF_SEC: {report.derived_wallet_import_format.substring(0, 12)}...
                          </span>
                        </div>
                      </td>

                      {/* Validación de Integridad Matemática */}
                      <td className="p-4 text-center border-r border-zinc-900/50">
                        {report.mathematical_integrity_verified ? (
                          <div className="flex items-center justify-center gap-1.5 text-emerald-500">
                            <ShieldCheck className="w-3.5 h-3.5" />
                            <span className="text-[7px] font-black uppercase tracking-tighter">Verified</span>
                          </div>
                        ) : (
                          <AlertTriangle className="w-3.5 h-3.5 text-red-500 mx-auto" />
                        )}
                      </td>

                      {/* Saldo Real en la Blockchain */}
                      <td className="p-4 text-right border-r border-zinc-900/50">
                        <span className={cn(
                          "text-[11px] font-black tabular-nums",
                          (report.network_reality_data?.final_balance_satoshis ?? 0) > 0
                            ? "text-emerald-400 animate-pulse"
                            : "text-zinc-700"
                        )}>
                          {report.network_reality_data
                            ? (report.network_reality_data.final_balance_satoshis / 1e8).toFixed(8)
                            : "UPLINK_OFFLINE"}
                        </span>
                      </td>

                      {/* Conteo de Transacciones Históricas */}
                      <td className="p-4 text-right">
                        <div className="flex items-center justify-end gap-2 text-[10px] text-zinc-500">
                          {report.network_reality_data?.confirmed_transaction_count ?? 0}
                          <span className="text-[8px] uppercase opacity-40">Txs</span>
                          <ArrowUpRight className="w-2.5 h-2.5 opacity-0 group-hover:opacity-100 transition-opacity" />
                        </div>
                      </td>
                    </motion.tr>
                  ))
                )}
              </AnimatePresence>
            </tbody>
          </table>
        </CardContent>
      </Card>

      {/* FOOTER TÉCNICO DE CAPA L5 */}
      <footer className="flex justify-between items-center px-4">
        <div className="flex items-center gap-3">
          <Database className="w-3.5 h-3.5 text-zinc-700" />
          <span className="text-[9px] text-zinc-600 font-mono uppercase font-black tracking-widest">
            Source: Blockchain.info / Multi-Proxy Stratum
          </span>
        </div>
        <div className="text-[9px] text-zinc-800 font-mono uppercase font-bold">
          Tesis Stratum L5 // Forensic Verification Sequence V11.4
        </div>
      </footer>

      {/* Alerta de Error de Sincronización */}
      {isError && (
        <motion.div
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          className="p-4 bg-red-950/20 border border-red-900/30 rounded-xl flex items-center gap-4"
        >
          <AlertTriangle className="w-5 h-5 text-red-500" />
          <p className="text-[10px] text-red-400 font-mono uppercase font-black tracking-wider">
            CRITICAL_SYNC_FAULT: The laboratory gateway is unreachable. Audit data may be stale.
          </p>
        </motion.div>
      )}
    </div>
  );
}

/**
 * ÁTOMO: FILAS SKELETON (LOADING STATE)
 * Mantiene la coherencia visual durante la hidratación de datos.
 */
function ForensicSkeletonRows(): React.ReactElement {
  return (
    <>
      {[...Array(6)].map((_, index) => (
        <tr key={`skeleton-row-${index}`} className="animate-pulse border-b border-white/5">
          <td colSpan={6} className="p-4">
            <div className="h-6 bg-zinc-900/50 rounded-lg w-full" />
          </td>
        </tr>
      ))}
    </>
  );
}
