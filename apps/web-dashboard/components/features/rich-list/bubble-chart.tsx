/**
 * =================================================================
 * APARATO: WEALTH DISTRIBUTION BUBBLE CHART (V25.0 - GOLD MASTER)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN ANALÍTICA DEL CENSO BITCOIN
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el motor de renderizado de la "Rich List" Zombie.
 * Mapea el espacio de riqueza de Bitcoin en una matriz tridimensional:
 * - Eje X: Temporalidad (Año de última actividad).
 * - Eje Y: Densidad de Nodos (Escala Logarítmica).
 * - Tamaño (Z): Volumen de Capital (Bitcoin Balance).
 *
 * ✅ RESOLUCIÓN TS2339: Implementación de interfaces nominales para
 * los componentes internos de Recharts, garantizando paridad de tipos.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import {
  ScatterChart,
  Scatter,
  XAxis,
  YAxis,
  ZAxis,
  Tooltip,
  ResponsiveContainer,
  Cell,
} from "recharts";
import { useQuery } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { Globe, Skull, Activity, Info, TrendingUp } from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

// --- SINAPSIS DE INFRAESTRUCTURA (L2 & L4) ---
import { strategicCensus, type WealthCluster, type WealthCategory } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { cn } from "@/lib/utils/cn";

/**
 * Interface de propiedades para el componente de Tooltip personalizado.
 * Resuelve la ambigüedad de 'payload' en el motor de Turbopack.
 */
interface CustomTooltipProperties {
  active?: boolean;
  payload?: Array<{
    payload: WealthCluster;
  }>;
}

/**
 * Componente interno: HUD de Información de Cluster.
 * Provee telemetría instantánea al operador durante el escrutinio visual.
 */
const ClusterTelemetryTooltip: React.FC<CustomTooltipProperties> = ({ active, payload }) => {
  if (active && payload && payload.length > 0) {
    const cluster_data: WealthCluster = payload[0].payload;

    return (
      <div className="bg-black/95 border border-zinc-800 p-5 rounded-xl shadow-2xl backdrop-blur-xl animate-in fade-in zoom-in-95 duration-200">
        <div className="flex items-center gap-3 mb-4 border-b border-white/10 pb-3">
          <div
            className={cn(
              "w-2.5 h-2.5 rounded-full",
              cluster_data.is_zombie_target
                ? "bg-red-500 animate-pulse shadow-[0_0_10px_#ef4444]"
                : "bg-emerald-500"
            )}
          />
          <p className="text-[11px] font-black text-white uppercase font-mono tracking-widest">
            {cluster_data.display_label}
          </p>
        </div>

        <div className="space-y-2.5 text-[10px] font-mono">
          <div className="flex justify-between gap-12">
            <span className="text-zinc-500 uppercase font-bold">Balance_Total</span>
            <span className="text-zinc-100 font-black">
              {cluster_data.balance_bitcoin.toLocaleString()} BTC
            </span>
          </div>
          <div className="flex justify-between gap-12">
            <span className="text-zinc-500 uppercase font-bold">Node_Density</span>
            <span className="text-zinc-100 font-black">
              {cluster_data.wallet_count.toLocaleString()}
            </span>
          </div>
          <div className="flex justify-between gap-12">
            <span className="text-zinc-500 uppercase font-bold">Chronology</span>
            <span className="text-zinc-100 font-black italic">
              Anno {cluster_data.last_activity_year}
            </span>
          </div>

          {cluster_data.is_zombie_target && (
            <div className="mt-4 pt-3 border-t border-red-900/50 flex items-center gap-2 text-red-500 font-black tracking-tighter animate-pulse">
              <Skull className="w-4 h-4" />
              <span className="uppercase">VULNERABILITY_THRESHOLD_DETECTED</span>
            </div>
          )}
        </div>
      </div>
    );
  }
  return null;
};

/**
 * Organismo Visual: WealthBubbleChart.
 * Orquesta la visualización de la estratificación de riqueza arqueológica.
 */
export function WealthBubbleChart(): React.ReactElement {
  const translations = useTranslations("Dashboard");

  /**
   * ADQUISICIÓN DE INTELIGENCIA ESTRATÉGICA (L4)
   * Recupera la distribución de riqueza desde el Cuartel General (Supabase).
   */
  const { data: wealth_clusters, isLoading, isError } = useQuery<WealthCluster[]>({
    queryKey: ["wealth-distribution-matrix-v25"],
    queryFn: () => strategicCensus.getWealthDistribution(),
    staleTime: 3600000, // Los datos de censo son históricos (1 hora de persistencia)
  });

  /**
   * MAPEO DE IDENTIDAD CROMÁTICA
   * Asigna colores basados en la clasificación forense de la Tesis.
   */
  const get_strata_color = (category_identifier: WealthCategory): string => {
    const color_palette: Record<WealthCategory, string> = {
      Satoshi_Era: "#ef4444", // Rojo: Origen Genesis
      Lost_Coins: "#f59e0b",  // Ámbar: Entropía estancada
      Whales: "#3b82f6",      // Azul: Capital moderno
      Exchanges: "#8b5cf6",   // Púrpura: Custodia masiva
      Retail: "#10b981",      // Esmeralda: Actividad económica
    };
    return color_palette[category_identifier] || "#71717a";
  };

  return (
    <Card className="bg-[#050505] border-zinc-800 h-[500px] flex flex-col shadow-2xl relative overflow-hidden group">
      {/* 1. CABECERA DE MANDO ANALÍTICO */}
      <CardHeader className="border-b border-white/5 bg-white/2 flex flex-row items-center justify-between p-6 z-20">
        <div className="space-y-1.5">
          <CardTitle className="text-[10px] font-black text-zinc-400 uppercase tracking-[0.4em] flex items-center gap-3 font-mono">
            <Globe className="w-4 h-4 text-primary" />
            {translations("analytics_page.title")} // CENSUS_STRATIFICATION
          </CardTitle>
          <div className="flex items-center gap-2.5">
            <div className="h-1.5 w-1.5 rounded-full bg-primary animate-pulse shadow-[0_0_8px_#10b981]" />
            <p className="text-[8px] text-zinc-600 font-mono uppercase tracking-widest font-bold">
              Strategic Link: Verified // Engine_B_Operational
            </p>
          </div>
        </div>

        <div className="flex items-center gap-4">
          <div className="bg-red-500/10 border border-red-500/20 px-3 py-1.5 rounded-lg flex items-center gap-2.5 shadow-[0_0_20px_rgba(239,68,68,0.1)]">
            <Skull className="w-4 h-4 text-red-500" />
            <span className="text-[10px] font-black text-red-500 uppercase font-mono tracking-tighter">
              Zombie_Trace: Enabled
            </span>
          </div>
          <button className="p-2 hover:bg-white/5 rounded-full transition-colors">
             <Info className="w-4 h-4 text-zinc-700" />
          </button>
        </div>
      </CardHeader>

      {/* 2. ÁREA DE PROCESAMIENTO VISUAL */}
      <CardContent className="flex-1 p-10 relative">
        <AnimatePresence>
          {isLoading && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="absolute inset-0 flex flex-col items-center justify-center bg-black/80 z-30 backdrop-blur-md gap-5"
            >
              <Activity className="w-10 h-10 text-primary animate-spin" />
              <span className="text-[11px] font-mono text-primary font-black animate-pulse tracking-[0.5em] uppercase">
                Synchronizing_Archaeological_Strata...
              </span>
            </motion.div>
          )}
        </AnimatePresence>

        <ResponsiveContainer width="100%" height="100%">
          <ScatterChart margin={{ top: 20, right: 30, bottom: 40, left: 20 }}>
            <XAxis
              type="number"
              dataKey="last_activity_year"
              name="Activity_Year"
              domain={[2009, 2025]}
              stroke="#27272a"
              fontSize={10}
              tickFormatter={(year_value: number) => `'${year_value.toString().slice(-2)}`}
              tick={{ fill: "#52525b", fontFamily: "monospace" }}
            />
            <YAxis
              type="number"
              dataKey="wallet_count"
              name="Node_Density"
              stroke="#27272a"
              fontSize={10}
              scale="log"
              domain={["auto", "auto"]}
              tick={{ fill: "#52525b", fontFamily: "monospace" }}
            />
            <ZAxis
              type="number"
              dataKey="balance_bitcoin"
              range={[100, 3500]}
              name="Cluster_Wealth_Magnitude"
            />
            <Tooltip
              content={<ClusterTelemetryTooltip />}
              cursor={{ strokeDasharray: "4 4", stroke: "#3f3f46", strokeWidth: 1 }}
            />
            <Scatter name="UTXO_Statistical_Clusters" data={wealth_clusters}>
              {wealth_clusters?.map((cluster_item: WealthCluster, cluster_index: number) => (
                <Cell
                  key={`cluster-identity-${cluster_item.cluster_identifier}-${cluster_index}`}
                  fill={get_strata_color(cluster_item.wealth_category)}
                  className="hover:opacity-100 opacity-50 transition-all duration-700 cursor-crosshair"
                  style={{
                    filter: cluster_item.is_zombie_target
                      ? "drop-shadow(0 0 12px rgba(239, 68, 68, 0.8))"
                      : "none",
                  }}
                />
              ))}
            </Scatter>
          </ScatterChart>
        </ResponsiveContainer>
      </CardContent>

      {/* 3. LEYENDA TÉCNICA Y ESTATUS DE CAPA */}
      <footer className="p-5 border-t border-white/5 bg-black/60 flex flex-col md:flex-row justify-between items-center px-10 gap-6">
        <div className="flex flex-wrap items-center gap-8 text-[10px] font-black text-zinc-500 font-mono uppercase tracking-tight">
          <div className="flex items-center gap-2.5 group">
            <div className="w-2 h-2 rounded-full bg-red-500 shadow-[0_0_8px_#ef4444]" />
            <span className="group-hover:text-zinc-300 transition-colors">Satoshi_Era</span>
          </div>
          <div className="flex items-center gap-2.5 group">
            <div className="w-2 h-2 rounded-full bg-blue-500 shadow-[0_0_8px_#3b82f6]" />
            <span className="group-hover:text-zinc-300 transition-colors">Institutional_Custody</span>
          </div>
          <div className="flex items-center gap-2.5 group">
            <div className="w-2 h-2 rounded-full bg-emerald-500 shadow-[0_0_8px_#10b981]" />
            <span className="group-hover:text-zinc-300 transition-colors">Retail_Strata</span>
          </div>
        </div>

        <div className="flex items-center gap-5">
          <div className="flex items-center gap-2.5 px-3 py-1 bg-zinc-900/50 border border-white/10 rounded-full">
            <TrendingUp className="w-3.5 h-3.5 text-primary" />
            <span className="text-[9px] font-black text-zinc-400 font-mono uppercase tracking-widest">
              Audit_Saturation: Optimized
            </span>
          </div>
          <div className="h-5 w-px bg-zinc-800" />
          <span className="text-[9px] font-black text-zinc-700 font-mono uppercase tracking-[0.4em]">
            Stratum_L4 // Master_V25.0
          </span>
        </div>
      </footer>

      {/* Errores de Sincronización */}
      {isError && (
        <div className="absolute bottom-20 left-1/2 -translate-x-1/2 px-6 py-3 bg-red-950/40 border border-red-500/50 rounded-xl backdrop-blur-md flex items-center gap-4 animate-bounce">
           <Skull className="w-5 h-5 text-red-500" />
           <p className="text-[10px] font-black text-red-200 font-mono uppercase tracking-widest">
             Critical_Link_Fault: Strategic_Uplink_Severed
           </p>
        </div>
      )}
    </Card>
  );
}
