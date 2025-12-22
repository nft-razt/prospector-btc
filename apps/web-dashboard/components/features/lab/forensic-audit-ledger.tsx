/**
 * =================================================================
 * APARATO: FORENSIC AUDIT LEDGER (V1.0 - IAN COLEMAN STYLE)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE AUDITORÍA REAL DE RED
 * =================================================================
 */

"use client";

import React from "react";
import { useQuery } from "@tanstack/react-query";
import {
  History,
  ShieldCheck,
  AlertTriangle,
  ExternalLink,
  Search,
  Database
} from "lucide-react";
import { apiClient } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { cn } from "@/lib/utils/cn";

export function ForensicAuditLedger() {
  const { data: results, isLoading } = useQuery({
    queryKey: ["forensic-brainwallet-dataset"],
    queryFn: async () => (await apiClient.get("/lab/audit/brainwallet-dataset")).data,
    staleTime: 600000,
  });

  return (
    <div className="space-y-6 animate-in fade-in duration-700">
      <Card className="bg-black border-zinc-800 shadow-2xl overflow-hidden">
        <CardHeader className="border-b border-white/5 bg-zinc-900/30 p-6">
          <CardTitle className="text-xs font-black text-white uppercase tracking-[0.4em] font-mono flex items-center gap-3">
            <History className="w-4 h-4 text-emerald-500" />
            Strategic Brainwallet Auditor // Live Blockchain Synchronization
          </CardTitle>
        </CardHeader>

        <CardContent className="p-0 overflow-x-auto custom-scrollbar">
          <table className="w-full text-left font-mono border-collapse">
            <thead>
              <tr className="text-[8px] font-black text-zinc-500 uppercase border-b border-zinc-800 bg-black/60">
                <th className="p-4 border-r border-zinc-900">Vector_ID</th>
                <th className="p-4 border-r border-zinc-900">Input_Seed</th>
                <th className="p-4 border-r border-zinc-900">Derived_Address</th>
                <th className="p-4 text-center border-r border-zinc-900">Math_Symmetry</th>
                <th className="p-4 text-right border-r border-zinc-900 text-emerald-500">Live_Balance_BTC</th>
                <th className="p-4 text-right">Historical_TX</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-white/5">
              {results?.map((vector: any) => (
                <tr key={vector.vector_id} className="hover:bg-white/2 transition-all group">
                  <td className="p-4 text-[10px] text-zinc-600 border-r border-zinc-900/50">
                    {vector.vector_id.toString().padStart(2, '0')}
                  </td>
                  <td className="p-4 text-[10px] text-zinc-200 font-bold border-r border-zinc-900/50">
                    "{vector.source_passphrase}"
                  </td>
                  <td className="p-4 border-r border-zinc-900/50">
                    <div className="flex flex-col gap-1">
                      <span className="text-[9px] text-zinc-500 select-all">{vector.derived_bitcoin_address}</span>
                      <span className="text-[7px] text-zinc-700 uppercase font-black">WIF: {vector.derived_wif_compressed.substring(0, 15)}...</span>
                    </div>
                  </td>
                  <td className="p-4 text-center border-r border-zinc-900/50">
                    {vector.mathematical_integrity_verified ? (
                      <div className="flex items-center justify-center gap-1.5 text-emerald-500">
                        <ShieldCheck className="w-3.5 h-3.5" />
                        <span className="text-[8px] font-black uppercase">Certified</span>
                      </div>
                    ) : (
                      <AlertTriangle className="w-3.5 h-3.5 text-red-500 mx-auto" />
                    )}
                  </td>
                  <td className="p-4 text-right border-r border-zinc-900/50">
                    <span className={cn(
                      "text-[10px] font-black",
                      vector.network_reality_data?.final_balance_satoshis > 0 ? "text-emerald-400 animate-pulse" : "text-zinc-700"
                    )}>
                      {vector.network_reality_data
                        ? (vector.network_reality_data.final_balance_satoshis / 1e8).toFixed(8)
                        : "OFFLINE"}
                    </span>
                  </td>
                  <td className="p-4 text-right text-[10px] text-zinc-500">
                    {vector.network_reality_data?.confirmed_transaction_count || 0} TXs
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </CardContent>
      </Card>

      <div className="flex justify-between items-center px-4">
        <div className="flex items-center gap-2 text-[9px] text-zinc-600 font-mono uppercase font-black">
          <Database className="w-3 h-3" />
          Data Source: Blockchain.info RPC Proxy
        </div>
        <div className="text-[9px] text-zinc-800 font-mono uppercase">
          Tesis Stratum L5 // Forensic Verification Protocol
        </div>
      </div>
    </div>
  );
}
