/**
 * =================================================================
 * APARATO: IDENTITY INVENTORY HUD (V14.8 - TYPE SECURED)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN Y MONITOREO DE LA BÓVEDA DE ACCESO
 *
 * VISION HIPER-HOLÍSTICA:
 * Provee al operador visibilidad sobre el pool de identidades.
 * Resuelve el error TS2305 al importar 'Identity' desde el barril
 * consolidado de infraestructura (L4).
 * =================================================================
 */

"use client";

import React from "react";
import { useQuery } from "@tanstack/react-query";
import { formatDistanceToNow } from "date-fns";
import {
  Server,
  Clock,
  ShieldCheck,
  Trash2,
  Activity,
  RefreshCw
} from "lucide-react";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { adminApi, type Identity } from "@prospector/api-client";
import { Skeleton } from "@/components/ui/kit/skeleton";
import { cn } from "@/lib/utils/cn";

export function IdentityInventory(): React.ReactElement {
  /**
   * ADQUISICIÓN DE IDENTIDADES (L4)
   * RESOLUCIÓN: El tipo 'Identity' ahora fluye correctamente desde api-client.
   */
  const { data: identities_collection, isLoading, isError } = useQuery<Identity[]>({
    queryKey: ["identities-vault-inventory-v14.8"],
    queryFn: () => adminApi.listIdentities(),
    refetchInterval: 15000,
  });

  if (isLoading) return <InventorySkeleton />;

  return (
    <div className="flex flex-col h-full bg-[#0a0a0a] border border-zinc-800 rounded-2xl overflow-hidden shadow-2xl group">
      {/* Visual Ambiance Icon */}
      <div className="absolute top-0 right-0 p-4 opacity-[0.02] group-hover:opacity-[0.05] transition-opacity pointer-events-none">
        <ShieldCheck className="w-32 h-32 text-emerald-500" />
      </div>

      <header className="p-5 border-b border-zinc-800 bg-white/2 flex justify-between items-center backdrop-blur-md">
        <h3 className="text-[10px] font-black text-zinc-400 uppercase tracking-[0.3em] flex items-center gap-3 font-mono">
          <Server className="w-4 h-4 text-emerald-500" />
          Persona_Vault // Strata_L3
        </h3>
        <span className="text-[9px] font-mono bg-black text-emerald-500 px-3 py-1 rounded-full border border-emerald-900/30">
          COUNT: {identities_collection?.length || 0}
        </span>
      </header>

      <div className="flex-1 overflow-y-auto max-h-150 p-4 space-y-4 custom-scrollbar relative z-10">
        {identities_collection?.length === 0 ? (
          <div className="h-60 flex flex-col items-center justify-center text-zinc-700 gap-4">
            <Trash2 className="w-8 h-8 opacity-20" />
            <p className="text-[9px] font-mono uppercase tracking-widest text-center">Empty_Bunker // Waiting_for_Injection</p>
          </div>
        ) : (
          identities_collection?.map((identity_record: Identity) => (
            <div key={identity_record.id} className="bg-black/40 border border-zinc-800 p-5 rounded-xl hover:border-emerald-500/30 transition-all duration-300 group/card">
              <div className="flex justify-between items-start mb-4">
                <div className="flex items-center gap-3">
                  <div className={cn(
                    "w-2 h-2 rounded-full shadow-[0_0_8px]",
                    identity_record.status === "active" ? "bg-emerald-500 shadow-emerald-500/30" : "bg-red-500"
                  )} />
                  <span className="font-mono text-xs text-zinc-100 font-bold truncate w-40">
                    {identity_record.email}
                  </span>
                </div>
                <Activity className="w-3.5 h-3.5 text-zinc-800 group-hover/card:text-emerald-500/40 transition-colors" />
              </div>

              <div className="grid grid-cols-2 gap-4 pt-4 border-t border-zinc-800/50">
                <div className="space-y-1">
                  <p className="text-[7px] text-zinc-600 uppercase font-black font-mono">Lease_Count</p>
                  <div className="flex items-center gap-1.5">
                    <RefreshCw className="w-3 h-3 text-zinc-700" />
                    <span className="text-[10px] font-mono text-zinc-400">{identity_record.usage_count}</span>
                  </div>
                </div>
                <div className="space-y-1 text-right">
                  <p className="text-[7px] text-zinc-600 uppercase font-black font-mono">Last_Active</p>
                  <div className="flex items-center justify-end gap-1.5">
                    <span className="text-[10px] font-mono text-zinc-400">
                      {identity_record.last_used_at ? formatDistanceToNow(new Date(identity_record.last_used_at)) : "Never"}
                    </span>
                    <Clock className="w-3 h-3 text-zinc-700" />
                  </div>
                </div>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

function InventorySkeleton() {
  return (
    <div className="space-y-4">
      {[1, 2, 3].map(i => <Skeleton key={i} className="h-32 w-full rounded-2xl bg-zinc-900/50" />)}
    </div>
  );
}
