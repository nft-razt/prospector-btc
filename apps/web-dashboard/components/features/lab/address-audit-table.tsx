/**
 * =================================================================
 * APARATO: ADDRESS AUDIT TABLE (V1.0 - IAN COLEMAN STYLE)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE AUDITORÍA HISTÓRICA REAL
 * =================================================================
 */

"use client";

import React from "react";
import { useQuery } from "@tanstack/react-query";
import {
  Table,
  ShieldCheck,
  History,
  Database,
  AlertTriangle,
  ExternalLink
} from "lucide-react";
import { apiClient } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { cn } from "@/lib/utils/cn";

export function AddressAuditTable() {
  /**
   * ADQUISICIÓN DE DATOS REALES (L4)
   * Consulta al endpoint del Orquestador que ejecuta la auditoría de los 33 vectores.
   */
  const { data: auditResults, isLoading } = useQuery({
    queryKey: ["brainwallet-real-audit"],
    queryFn: async () => (await apiClient.get("/lab/audit/vectors")).data,
    refetchInterval: 300000, // Cada 5 min
  });

  return (
    <Card className="bg-[#050505] border-zinc-800 shadow-2xl">
      <CardHeader className="border-b border-white/5 bg-white/2">
        <CardTitle className="text-[10px] font-black text-white uppercase tracking-[0.3em] font-mono flex items-center gap-3">
          <History className="w-4 h-4 text-primary" />
          Real-World Brainwallet Audit Ledger // 33 Vectors
        </CardTitle>
      </CardHeader>

      <CardContent className="p-0 overflow-x-auto">
        <table className="w-full text-left font-mono border-collapse">
          <thead>
            <tr className="text-[8px] font-black text-zinc-500 uppercase border-b border-zinc-800 bg-black/40">
              <th className="p-4">ID</th>
              <th className="p-4">Seed_Phrase</th>
              <th className="p-4">Derived_Address</th>
              <th className="p-4 text-center">Integrity</th>
              <th className="p-4 text-right">Live_Balance (BTC)</th>
              <th className="p-4 text-right">Historical_TX</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-white/5">
            {auditResults?.map((vector: any) => (
              <tr key={vector.id} className="hover:bg-white/2 transition-colors group">
                <td className="p-4 text-[10px] text-zinc-600">{vector.id}</td>
                <td className="p-4 text-[10px] text-zinc-300 font-bold">{vector.seed_phrase}</td>
                <td className="p-4 text-[9px] text-zinc-500">
                  <span className="flex items-center gap-2">
                    {vector.generated_address}
                    <ExternalLink className="w-2 h-2 opacity-0 group-hover:opacity-100 transition-opacity" />
                  </span>
                </td>
                <td className="p-4 text-center">
                  {vector.math_is_correct ? (
                    <ShieldCheck className="w-4 h-4 text-emerald-500 mx-auto" />
                  ) : (
                    <AlertTriangle className="w-4 h-4 text-red-500 mx-auto" />
                  )}
                </td>
                <td className="p-4 text-right text-[10px] text-emerald-400 font-black">
                  {vector.network_data ? (vector.network_data.final_balance_satoshis / 1e8).toFixed(8) : "---"}
                </td>
                <td className="p-4 text-right text-[10px] text-zinc-400">
                  {vector.network_data?.transaction_count || 0}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </CardContent>
    </Card>
  );
}
