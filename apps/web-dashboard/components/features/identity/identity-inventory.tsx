/**
 * =================================================================
 * APARATO: IDENTITY INVENTORY HUD (V41.0 - TAILWIND CANONICAL)
 * CLASIFICACIÓN: FEATURE UI (L5)
 * RESPONSABILIDAD: VISUALIZACIÓN Y MONITOREO DE LA BÓVEDA DE ACCESO
 *
 * ESTRATEGIA DE ÉLITE:
 * - Data Source: adminApi unificado (L4).
 * - Type Safety: Vinculación estricta con contrato Identity V40.0 (L2).
 * - UI Standards: Tailwind v4 con clases canónicas y sombras atómicas.
 * - UX: Formateo de tiempos relativo y estados de error resilientes.
 * =================================================================
 */

"use client";

import React from "react";
import { useQuery } from "@tanstack/react-query";
import { formatDistanceToNow } from "date-fns";
import {
  Server,
  RefreshCw,
  Clock,
  AlertTriangle,
  Trash2,
  ShieldCheck,
} from "lucide-react";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { adminApi, type Identity } from "@prospector/api-client";
import { cn } from "@/lib/utils/cn";

// --- ÁTOMOS UI ---
import { Skeleton } from "@/components/ui/kit/skeleton";

/**
 * HUD de inventario para la gestión de identidades en tiempo real.
 * Provee al operador una visión clara del pool de identidades cifradas.
 */
export function IdentityInventory() {
  /**
   * ADQUISICIÓN DE IDENTIDADES (L3/L4)
   * Utiliza el adaptador administrativo nivelado para recuperar la Bóveda.
   */
  const {
    data: identities,
    isLoading,
    isError,
    refetch,
  } = useQuery<Identity[]>({
    queryKey: ["identities"],
    queryFn: () => adminApi.listIdentities(),
    refetchInterval: 15000, // Sincronización cada 15 segundos
  });

  if (isLoading) return <InventorySkeleton />;

  if (isError) {
    return (
      <div className="p-10 border border-red-900/30 bg-red-950/5 rounded-xl text-center space-y-4">
        <AlertTriangle className="w-8 h-8 text-red-500 mx-auto animate-pulse" />
        <p className="text-[10px] font-mono text-red-500 uppercase tracking-widest font-bold">
          VAULT_UPLINK_SEVERED
        </p>
        <button
          onClick={() => refetch()}
          className="text-[9px] font-black text-white hover:text-red-400 underline transition-colors"
        >
          RE-ESTABLISH HANDSHAKE
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#0f0f0f] border border-zinc-800 rounded-xl overflow-hidden shadow-2xl">
      {/* HUD HEADER */}
      <div className="p-4 border-b border-zinc-800 bg-zinc-900/30 flex justify-between items-center backdrop-blur-md">
        <h3 className="text-xs font-black text-zinc-300 uppercase tracking-widest flex items-center gap-2">
          <Server className="w-3.5 h-3.5 text-emerald-500" />
          Active Personas
        </h3>
        <span className="text-[10px] font-mono bg-black text-emerald-500 px-3 py-0.5 rounded-full border border-emerald-900/30 shadow-[0_0_10px_rgba(16,185,129,0.1)]">
          SECURED: {identities?.length || 0}
        </span>
      </div>

      {/* LIST CANVAS */}
      <div className="flex-1 overflow-y-auto max-h-150 scrollbar-thin p-3 space-y-3 custom-scrollbar">
        {identities?.length === 0 ? (
          <div className="h-40 flex flex-col items-center justify-center text-zinc-600 space-y-3">
            <Trash2 className="w-6 h-6 opacity-20" />
            <p className="text-[9px] font-mono uppercase tracking-widest text-center">
              Vault is Empty // Injection Required
            </p>
          </div>
        ) : (
          identities?.map((identity) => (
            <IdentityCard key={identity.id} identity={identity} />
          ))
        )}
      </div>

      {/* TACTICAL FOOTER */}
      <div className="p-2 border-t border-white/5 bg-black/40 flex justify-center">
        <span className="text-[7px] font-black text-zinc-800 font-mono uppercase tracking-[0.4em]">
          Identity Stratum L3 // Zero-Knowledge Pool
        </span>
      </div>
    </div>
  );
}

/**
 * ÁTOMO COMPUESTO: IdentityCard
 * Renderiza la telemetría individual de una identidad con efectos visuales de alta fidelidad.
 */
function IdentityCard({ identity }: { identity: Identity }) {
  const statusConfig = {
    active: "bg-emerald-500 shadow-[0_0_8px_#10b981]",
    ratelimited: "bg-amber-500 shadow-[0_0_8px_#f59e0b]",
    expired: "bg-red-500 shadow-[0_0_8px_#ef4444]",
    revoked: "bg-zinc-700 shadow-none",
  };

  return (
    <div className="bg-black/40 border border-zinc-800 p-4 rounded-lg hover:border-emerald-500/30 transition-all duration-300 group relative overflow-hidden">
      {/*
        ✅ CANONICAL CLASS RESOLUTION:
        bg-emerald-500/[0.01] -> bg-emerald-500/1
      */}
      <div className="absolute inset-0 bg-emerald-500/1 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none" />

      <div className="flex justify-between items-start mb-4 relative z-10">
        <div className="flex items-center gap-3">
          {/* Indicador de Estado con Sombra y Pulsación */}
          <div
            className={cn(
              "w-1.5 h-1.5 rounded-full animate-pulse transition-all",
              statusConfig[identity.status as keyof typeof statusConfig],
            )}
          />
          <span className="font-mono text-xs text-zinc-100 font-bold tracking-tighter truncate w-32">
            {identity.email}
          </span>
        </div>
        <ShieldCheck className="w-3.5 h-3.5 text-emerald-500/30 group-hover:text-emerald-500 transition-colors" />
      </div>

      <div className="grid grid-cols-2 gap-4 pt-3 border-t border-zinc-800/50 relative z-10">
        <div className="space-y-1">
          <p className="text-[7px] text-zinc-600 uppercase font-black font-mono">
            Usage Record
          </p>
          <div className="flex items-center gap-1.5">
            <RefreshCw className="w-3 h-3 text-zinc-700" />
            <span className="text-[10px] font-mono text-zinc-400">
              {identity.usage_count}{" "}
              <span className="opacity-40 uppercase">Leases</span>
            </span>
          </div>
        </div>

        <div className="space-y-1 text-right">
          <p className="text-[7px] text-zinc-600 uppercase font-black font-mono">
            Temporal Status
          </p>
          <div className="flex items-center justify-end gap-1.5">
            <span className="text-[10px] font-mono text-zinc-400">
              {identity.last_used_at
                ? formatDistanceToNow(new Date(identity.last_used_at), {
                    addSuffix: true,
                  })
                : "Idle"}
            </span>
            <Clock className="w-3 h-3 text-zinc-700" />
          </div>
        </div>
      </div>
    </div>
  );
}

/**
 * SKELETON: InventorySkeleton
 */
function InventorySkeleton() {
  return (
    <div className="h-125 bg-[#0f0f0f] border border-zinc-800 rounded-xl p-4 space-y-4">
      <div className="flex justify-between mb-8">
        <Skeleton className="h-4 w-32 bg-zinc-900" />
        <Skeleton className="h-4 w-10 bg-zinc-900" />
      </div>
      {[1, 2, 3].map((index) => (
        <Skeleton
          key={`identity-skeleton-${index}`}
          className="h-24 w-full bg-zinc-900/50 rounded-lg"
        />
      ))}
    </div>
  );
}
