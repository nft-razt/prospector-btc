/**
 * =================================================================
 * APARATO: FINDINGS INVENTORY HUD (V26.1 - TAILWIND V4 OPTIMIZED)
 * CLASIFICACIÓN: FEATURE UI (L5)
 * RESPONSABILIDAD: VISUALIZACIÓN SEGURA DE COLISIONES CRIPTOGRÁFICAS
 *
 * ESTRATEGIA DE INTEGRIDAD:
 * - Style: Aplicación de clases canónicas (max-h-150, opacity-2).
 * - Security: Ofuscación persistente de material sensible (WIF).
 * - Performance: Sincronización táctica optimizada mediante TanStack Query.
 * =================================================================
 */

"use client";

import React, { useState } from "react";
import {
  ShieldCheck,
  Lock,
  Eye,
  EyeOff,
  Terminal,
  Database,
  Fingerprint,
  Activity
} from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import {
  apiClient,
  type Finding
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
 * Componente principal para la gestión visual del inventario de hallazgos tácticos.
 * Implementa una interfaz de "Bóveda" para la inspección de claves recuperadas.
 */
export function FindingsInventory() {
  /**
   * Estado de visibilidad de claves privadas.
   * Mapea la dirección Bitcoin a un booleano para revelado individual.
   */
  const [visibleKeys, setVisibleKeys] = useState<Record<string, boolean>>({});

  /**
   * ADQUISICIÓN DE DATOS TÁCTICOS (L3)
   * Resolución de Error TS2339 integrada: Acceso directo a la respuesta de Axios.
   */
  const { data: findings, isLoading, isError } = useQuery<Finding[]>({
    queryKey: ["tactical-findings"],
    queryFn: () => apiClient.get<Finding[]>("/swarm/findings"),
    refetchInterval: 5000,
  });

  /**
   * Alterna el estado de desenfoque de una clave privada.
   * @param address Identificador único (Dirección Bitcoin).
   */
  const toggleKeyVisibility = (address: string): void => {
    setVisibleKeys((prev) => ({ ...prev, [address]: !prev[address] }));
  };

  if (isError) {
    return (
      <div className="p-8 border border-red-900/30 bg-red-950/5 rounded-xl text-center animate-in fade-in duration-500">
        <p className="text-[10px] font-mono text-red-500 uppercase tracking-widest">
          Vault Link Failure // Signal Interrupted
        </p>
      </div>
    );
  }

  return (
    <Card className="bg-[#050505] border-emerald-900/30 shadow-[0_0_50px_rgba(16,185,129,0.05)] relative overflow-hidden group">
      {/* Visual Ambiance Icon (Background Decoration) */}
      <div className="absolute top-0 right-0 p-4 opacity-[0.03] group-hover:opacity-[0.08] transition-opacity duration-1000 pointer-events-none">
        <Database className="w-32 h-32 text-emerald-500" />
      </div>

      <CardHeader className="border-b border-emerald-900/20 bg-emerald-950/5 p-4 z-10">
        <CardTitle className="text-[10px] font-black text-emerald-500 uppercase tracking-[0.4em] flex items-center gap-3 font-mono">
          <ShieldCheck className="w-4 h-4" />
          The Vault // Tactical Findings
        </CardTitle>
      </CardHeader>

      {/* ✅ CANONICAL CLASS: max-h-150 (Reemplaza a max-h-[600px]) */}
      <CardContent className="p-0 max-h-150 overflow-y-auto custom-scrollbar z-10">
        {isLoading ? (
          <div className="p-6 space-y-4">
            {[1, 2].map((iterator) => (
              <Skeleton key={`finding-skeleton-${iterator}`} className="h-32 w-full bg-emerald-950/10" />
            ))}
          </div>
        ) : (
          <div className="divide-y divide-emerald-900/10">
            {findings?.length === 0 && (
              <div className="p-20 text-center flex flex-col items-center gap-4">
                <Activity className="w-6 h-6 text-zinc-800 animate-pulse" />
                <p className="opacity-20 font-mono text-[9px] tracking-[0.3em] uppercase">
                  Awaiting Swarm Collision Signal...
                </p>
              </div>
            )}

            {/* ✅ TYPING RESOLVED: finding: Finding (Error TS7006 Fixed) */}
            {findings?.map((finding: Finding) => (
              <div
                key={finding.address}
                /* ✅ CANONICAL CLASS: hover:bg-emerald-500/2 (Reemplaza a hover:bg-emerald-500/[0.02]) */
                className="p-6 hover:bg-emerald-500/2 transition-colors group/item"
              >
                <div className="flex justify-between items-start mb-5">
                  <div className="space-y-1.5">
                    <div className="text-[8px] text-zinc-500 font-mono uppercase tracking-tighter flex items-center gap-1.5">
                      <Fingerprint className="w-2.5 h-2.5" /> Target Identity
                    </div>
                    <div className="text-xs font-black text-white font-mono select-all tracking-tight">
                      {finding.address}
                    </div>
                  </div>
                  <span className="px-2 py-0.5 bg-emerald-500/10 border border-emerald-500/20 rounded text-[8px] font-black text-emerald-500 uppercase font-mono shadow-[0_0_10px_rgba(16,185,129,0.1)]">
                    {finding.wallet_type.replace('_', ' ')}
                  </span>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {/* SENSITIVE DATA MODULE (REVEAL LOGIC) */}
                  <div className="bg-black/50 border border-zinc-800 rounded-lg p-3 relative overflow-hidden group/wif">
                    <div className="flex justify-between items-center mb-2">
                      <span className="text-[7px] text-zinc-600 uppercase font-black flex items-center gap-1.5 font-mono">
                        <Lock className="w-2.5 h-2.5" /> Private Key (WIF)
                      </span>
                      <button
                        onClick={() => toggleKeyVisibility(finding.address)}
                        className="text-zinc-500 hover:text-emerald-400 transition-colors focus:outline-none"
                        aria-label="Toggle sensitive data visibility"
                      >
                        {visibleKeys[finding.address] ? <EyeOff className="w-3.5 h-3.5" /> : <Eye className="w-3.5 h-3.5" />}
                      </button>
                    </div>

                    <div className={cn(
                      "text-[10px] font-mono transition-all duration-500 select-all break-all",
                      visibleKeys[finding.address]
                        ? "text-amber-400 blur-0"
                        : "text-zinc-800 blur-md opacity-40"
                    )}>
                      {finding.private_key_wif}
                    </div>
                  </div>

                  {/* FORENSIC CONTEXT MODULE */}
                  <div className="bg-black/30 border border-zinc-900 rounded-lg p-3">
                    <span className="text-[7px] text-zinc-600 uppercase font-black flex items-center gap-1.5 mb-2 font-mono">
                      <Terminal className="w-2.5 h-2.5" /> Entropy Lineage
                    </span>
                    <div className="text-[9px] text-zinc-500 font-mono italic leading-relaxed">
                      ↳ {finding.source_entropy}
                    </div>
                    <div className="mt-2 text-[7px] text-zinc-700 font-mono uppercase">
                      Audit Timestamp: {new Date(finding.detected_at).toLocaleString()}
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </CardContent>

      <div className="p-3 border-t border-emerald-900/20 bg-emerald-950/5 flex justify-between px-6 z-10">
        <span className="text-[7px] font-black text-emerald-900/60 font-mono uppercase tracking-[0.2em]">
          Strategic Ledger Isolation // Secure Link
        </span>
        <div className="flex items-center gap-2">
          <div className="h-1 w-1 bg-emerald-500 rounded-full animate-pulse shadow-[0_0_5px_#10b981]" />
          <span className="text-[7px] font-black text-zinc-600 font-mono uppercase">
            Neural Sync Online
          </span>
        </div>
      </div>
    </Card>
  );
}
