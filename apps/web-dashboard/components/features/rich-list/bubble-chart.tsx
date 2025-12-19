/**
 * =================================================================
 * APARATO: WEALTH DISTRIBUTION BUBBLE CHART (V23.0)
 * CLASIFICACIÓN: FEATURE ORGANISM (L5)
 * RESPONSABILIDAD: VISUALIZACIÓN ANALÍTICA DEL CENSO BITCOIN
 *
 * ESTRATEGIA DE RENDERIZADO:
 * - Motor: Recharts (Scatter Matrix).
 * - Datos: Capa L4 (Supabase) via api-client.
 * - Optimización: Cache de 1 hora para datos arqueológicos.
 * - Estilo: Tailwind CSS v4 con clases canónicas.
 * =================================================================
 */

"use client";

import React from "react";
import {
  ScatterChart,
  Scatter,
  XAxis,
  YAxis,
  ZAxis,
  Tooltip,
  ResponsiveContainer,
  Cell,
  type TooltipProps,
} from "recharts";
import { useQuery } from "@tanstack/react-query";
import { strategicCensus } from "@prospector/api-client";
import {
  type WealthCluster,
  type WealthCategory,
} from "@prospector/api-contracts";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@/components/ui/kit/card";
import { Globe, Skull, Info, Activity } from "lucide-react";
import { cn } from "@/lib/utils/cn";

/**
 * Interface para los valores de las etiquetas de los ejes.
 */
interface AxisTickProps {
  x: number;
  y: number;
  payload: {
    value: number;
  };
}

/**
 * Componente de HUD flotante (Tooltip) con tipado estricto.
 * Provee telemetría detallada del cluster al operador en hover.
 *
 * @param active - Estado de visibilidad del tooltip.
 * @param payload - Datos del cluster seleccionado.
 */
const ClusterTooltip = ({ active, payload }: TooltipProps<number, string>) => {
  if (active && payload && payload.length) {
    const data = payload[0].payload as WealthCluster;
    return (
      <div className="bg-black/95 border border-zinc-800 p-4 rounded-xl shadow-2xl backdrop-blur-md animate-in fade-in zoom-in-95 duration-200">
        <div className="flex items-center gap-2 mb-3 border-b border-white/10 pb-2">
          <div
            className={cn(
              "w-2 h-2 rounded-full",
              data.is_zombie_target
                ? "bg-red-500 animate-pulse"
                : "bg-emerald-500",
            )}
          />
          <p className="text-[10px] font-black text-white uppercase font-mono tracking-widest">
            {data.display_label}
          </p>
        </div>

        <div className="space-y-2 text-[9px] font-mono">
          <div className="flex justify-between gap-8">
            <span className="text-zinc-500 uppercase">Balance Total</span>
            <span className="text-zinc-100 font-bold">
              {data.balance_bitcoin.toLocaleString()} BTC
            </span>
          </div>
          <div className="flex justify-between gap-8">
            <span className="text-zinc-500 uppercase">Densidad de Nodos</span>
            <span className="text-zinc-100 font-bold">
              {data.wallet_count.toLocaleString()}
            </span>
          </div>
          <div className="flex justify-between gap-8">
            <span className="text-zinc-500 uppercase">Última Actividad</span>
            <span className="text-zinc-100 font-bold">
              Año {data.last_activity_year}
            </span>
          </div>

          {data.is_zombie_target && (
            <div className="mt-3 pt-2 border-t border-red-900/50 flex items-center gap-2 text-red-500 font-black tracking-tighter">
              <Skull className="w-3 h-3" />
              <span>DETECCIÓN DE ALTA ENTROPÍA</span>
            </div>
          )}
        </div>
      </div>
    );
  }
  return null;
};

/**
 * Organismo visual principal para la visualización de la "Rich List" Zombie.
 */
export function WealthBubbleChart() {
  // ADQUISICIÓN DE INTELIGENCIA ESTRATÉGICA (L4)
  const { data: clusters, isLoading } = useQuery<WealthCluster[]>({
    queryKey: ["wealth-distribution-clusters"],
    queryFn: () => strategicCensus.getWealthDistribution(),
    staleTime: 3600000, // Los datos de censo son históricos (1h cache)
    gcTime: 86400000, // Persistencia en memoria por 24h
  });

  /**
   * Mapeo de identidad cromática según la clasificación forense.
   */
  const getCategoryColor = (category: WealthCategory): string => {
    const palette: Record<WealthCategory, string> = {
      Satoshi_Era: "#ef4444", // Rojo: Origen Genesis / Hal Finney
      Lost_Coins: "#f59e0b", // Ámbar: Monedas sin movimiento > 10 años
      Whales: "#3b82f6", // Azul: Grandes acumuladores modernos
      Exchanges: "#8b5cf6", // Púrpura: Billeteras de custodia masiva
      Retail: "#10b981", // Esmeralda: Actividad económica estándar
    };
    return palette[category] || "#71717a";
  };

  return (
    <Card className="bg-[#050505] border-zinc-800 h-125 flex flex-col shadow-2xl overflow-hidden group">
      {/* CABECERA TÉCNICA */}
      <CardHeader className="border-b border-white/5 bg-white/2 flex flex-row items-center justify-between p-5">
        <div className="space-y-1">
          <CardTitle className="text-[10px] font-black text-zinc-400 uppercase tracking-[0.3em] flex items-center gap-3 font-mono">
            <Globe className="w-4 h-4 text-primary" />
            The Bitcoin Census // Wealth Stratification
          </CardTitle>
          <div className="flex items-center gap-2">
            <span className="h-1 w-1 rounded-full bg-emerald-500 animate-pulse" />
            <p className="text-[8px] text-zinc-600 font-mono uppercase tracking-widest">
              Strategic Data Uplink: Verified
            </p>
          </div>
        </div>

        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2 px-2.5 py-1.5 bg-red-500/10 border border-red-500/20 rounded shadow-[0_0_15px_rgba(239,68,68,0.1)]">
            <Skull className="w-3.5 h-3.5 text-red-500" />
            <span className="text-[9px] font-black text-red-500 uppercase font-mono tracking-tight">
              Zombie Trace Active
            </span>
          </div>
          <Info className="w-4 h-4 text-zinc-700 hover:text-zinc-400 transition-colors cursor-help" />
        </div>
      </CardHeader>

      {/* ÁREA DE RENDERIZADO MATRICIAL */}
      <CardContent className="flex-1 p-8 relative">
        {isLoading && (
          <div className="absolute inset-0 flex flex-col items-center justify-center bg-black/60 z-30 backdrop-blur-sm gap-4">
            <Activity className="w-8 h-8 text-primary animate-pulse" />
            <span className="text-[10px] font-mono text-primary font-black animate-pulse tracking-[0.4em] uppercase">
              Synchronizing_Stratum_L4
            </span>
          </div>
        )}

        <ResponsiveContainer width="100%" height="100%">
          <ScatterChart margin={{ top: 20, right: 30, bottom: 30, left: 10 }}>
            <XAxis
              type="number"
              dataKey="last_activity_year"
              name="Activity Year"
              domain={[2009, 2025]}
              stroke="#27272a"
              fontSize={10}
              // ✅ RESOLUCIÓN Error 7006: Tipado estricto del parámetro
              tickFormatter={(val: number) => `'${val.toString().slice(-2)}`}
              label={{
                value: "Timeline (Last Seen)",
                position: "bottom",
                fontSize: 9,
                fill: "#52525b",
                dy: 20,
                fontFamily: "monospace",
                fontWeight: "bold",
              }}
            />
            <YAxis
              type="number"
              dataKey="wallet_count"
              name="Node Count"
              stroke="#27272a"
              fontSize={10}
              scale="log"
              domain={["auto", "auto"]}
              label={{
                value: "Node Density (Log)",
                angle: -90,
                position: "left",
                fontSize: 9,
                fill: "#52525b",
                dx: -15,
                fontFamily: "monospace",
                fontWeight: "bold",
              }}
            />
            <ZAxis
              type="number"
              dataKey="balance_bitcoin"
              range={[60, 2500]}
              name="Cluster Wealth"
            />
            <Tooltip
              content={<ClusterTooltip />}
              cursor={{ strokeDasharray: "4 4", stroke: "#3f3f46" }}
            />
            <Scatter name="UTXO Clusters" data={clusters}>
              {clusters?.map((cluster, index) => (
                <Cell
                  key={`cluster-index-${cluster.cluster_identifier}-${index}`}
                  fill={getCategoryColor(cluster.wealth_category)}
                  className="hover:opacity-100 opacity-60 transition-all duration-700 cursor-crosshair"
                  style={{
                    // ✅ NO REGRESSIONS: Preservación de brillo condicional
                    filter: cluster.is_zombie_target
                      ? "drop-shadow(0 0 10px rgba(239, 68, 68, 0.7))"
                      : "none",
                  }}
                />
              ))}
            </Scatter>
          </ScatterChart>
        </ResponsiveContainer>
      </CardContent>

      {/* PIE DE PÁGINA: LEYENDA TÉCNICA */}
      <div className="p-4 border-t border-white/5 bg-black/40 flex flex-col md:flex-row justify-between items-center px-8 gap-4">
        <div className="flex flex-wrap items-center gap-6 text-[9px] font-bold text-zinc-500 font-mono uppercase">
          <div className="flex items-center gap-2">
            <div className="w-1.5 h-1.5 rounded-full bg-red-500 shadow-[0_0_8px_#ef4444]" />
            Satoshi Era
          </div>
          <div className="flex items-center gap-2">
            <div className="w-1.5 h-1.5 rounded-full bg-blue-500 shadow-[0_0_8px_#3b82f6]" />
            Institutional
          </div>
          <div className="flex items-center gap-2">
            <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-[0_0_8px_#10b981]" />
            Retail Grid
          </div>
        </div>

        <div className="flex items-center gap-3">
          <div className="h-4 w-px bg-zinc-800" />
          <span className="text-[9px] font-black text-zinc-700 font-mono uppercase tracking-[0.2em]">
            Stratum L4 // Strategic Engine V2.1
          </span>
        </div>
      </div>
    </Card>
  );
}
