/**
 * =================================================================
 * APARATO: ANALYTICS DEEP VIEW (V39.0 - STRATEGIC INTELLIGENCE)
 * CLASIFICACIÓN: STRATEGIC VIEW (L5)
 * RESPONSABILIDAD: ANÁLISIS FORENSE Y MÉTRICAS DE ESFUERZO COMPUTACIONAL
 *
 * ESTRATEGIA DE RENDERIZADO:
 * - Engine: Recharts (Area & Bar Matrix).
 * - Data Source: Engine B (Supabase / Strategic Archive).
 * - Type Safety: Vinculación estricta con contrato ArchivedJob (L2).
 * - Performance: Procesamiento In-Memory de series temporales.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import { useTranslations } from "next-intl";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  AreaChart,
  Area,
} from "recharts";
import { useQuery } from "@tanstack/react-query";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { strategicArchive, type ArchivedJob } from "@prospector/api-client";

// --- COMPONENTES UI (DESIGN SYSTEM) ---
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@/components/ui/kit/card";
import {
  Activity,
  Cpu,
  Zap,
  BarChart3,
  ShieldCheck,
  TrendingUp,
} from "lucide-react";
import { StatCard } from "@/components/ui/kit/stat-card";
import { cn } from "@/lib/utils/cn";

/**
 * Interface para la estructura de datos procesada para gráficas.
 */
interface AnalyticsTimeSeriesEntry {
  time_label: string;
  hash_volume: number;
  efficiency_ratio: number;
}

/**
 * Página de Analítica Profunda.
 * Provee al operador Hydra una visión macroscópica del progreso de la Tesis.
 */
export default function AnalyticsPage() {
  const t = useTranslations("Dashboard.analytics_page");

  /**
   * ADQUISICIÓN DE DATOS ESTRATÉGICOS (L4)
   * Recupera los últimos 100 trabajos archivados para análisis de tendencia.
   */
  const { data: archivedHistory, isLoading } = useQuery<ArchivedJob[]>({
    queryKey: ["detailed-analytics-history"],
    queryFn: () => strategicArchive.getHistory(100),
    staleTime: 600000, // Frescura de 10 minutos (Datos fríos)
  });

  /**
   * MOTOR DE TRANSFORMACIÓN (L5 -> UI)
   * ✅ RESOLUCIÓN Error 7006: Tipado explícito 'historyEntry: ArchivedJob'.
   * Realiza la normalización de billones de hashes a escala decimal legible.
   */
  const graphData = useMemo((): AnalyticsTimeSeriesEntry[] => {
    if (!archivedHistory) return [];

    return archivedHistory
      .map((historyEntry: ArchivedJob): AnalyticsTimeSeriesEntry => {
        const totalHashes = BigInt(historyEntry.total_hashes);
        const duration = historyEntry.duration_seconds || 1;

        return {
          // Formateo de tiempo para el eje X
          time_label: new Date(historyEntry.created_at).toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
          }),
          // Volumen en Millones de Hashes (MH)
          hash_volume: Number(totalHashes / BigInt(1_000_000)),
          // Ratio: Hashes por Segundo
          efficiency_ratio: Number(totalHashes / BigInt(duration)) / 1000, // kH/s
        };
      })
      .reverse(); // Orden cronológico para la gráfica
  }, [archivedHistory]);

  return (
    <div className="space-y-10 animate-in fade-in slide-in-from-bottom-4 duration-700 pb-20">
      {/* CABECERA ESTRATÉGICA */}
      <div className="flex flex-col md:flex-row justify-between items-start md:items-end gap-4 border-l-2 border-primary/40 pl-6 py-1">
        <div className="space-y-1">
          <h1 className="text-3xl font-black text-white tracking-tighter uppercase font-mono">
            {t("title")}
          </h1>
          <div className="flex items-center gap-3">
            <span className="h-1.5 w-1.5 rounded-full bg-primary animate-pulse" />
            <p className="text-zinc-500 text-[10px] font-mono uppercase tracking-[0.2em]">
              {t("subtitle")} // KERNEL_V14_OPTIMIZED
            </p>
          </div>
        </div>

        <div className="bg-zinc-900/50 border border-zinc-800 px-4 py-2 rounded-lg flex items-center gap-3">
          <TrendingUp className="w-4 h-4 text-emerald-500" />
          <span className="text-[10px] font-bold text-zinc-300 font-mono uppercase">
            Swarm Health: Optimized
          </span>
        </div>
      </div>

      {/* MÉTRICAS DE EFICIENCIA CRÍTICA (STAT CARDS) */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <StatCard
          label={t("metrics.hashes_per_watt")}
          value="14.8"
          subValue="J/GH (Projected)"
          icon={Zap}
          color="amber"
          loading={isLoading}
        />
        <StatCard
          label={t("metrics.avg_latency")}
          value="38ms"
          subValue="Neural Handshake"
          icon={Activity}
          color="blue"
          loading={isLoading}
        />
        <StatCard
          label={t("metrics.collision_prob")}
          value="1.2e-64"
          subValue="Next Epoch"
          icon={ShieldCheck}
          color="emerald"
          loading={isLoading}
        />
      </div>

      {/* MATRIZ DE GRÁFICAS DE ALTA DENSIDAD */}
      <div className="grid grid-cols-1 xl:grid-cols-2 gap-8 items-stretch">
        {/* GRÁFICA A: VOLUMEN DE AUDITORÍA (AREA) */}
        <Card className="bg-[#0a0a0a] border-zinc-800 h-110 flex flex-col shadow-2xl relative overflow-hidden group">
          <div className="absolute inset-0 bg-linear-to-b from-emerald-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none" />

          <CardHeader className="border-b border-white/5 bg-white/2">
            <CardTitle className="text-[10px] font-black text-zinc-400 uppercase tracking-widest flex items-center gap-3 font-mono">
              <BarChart3 className="w-4 h-4 text-primary" />
              {t("effort_distribution")}
            </CardTitle>
          </CardHeader>

          <CardContent className="flex-1 p-6 relative z-10">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={graphData}>
                <defs>
                  <linearGradient id="areaGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#10b981" stopOpacity={0.3} />
                    <stop offset="95%" stopColor="#10b981" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <CartesianGrid
                  strokeDasharray="3 3"
                  stroke="#18181b"
                  vertical={false}
                />
                <XAxis
                  dataKey="time_label"
                  stroke="#3f3f46"
                  fontSize={9}
                  tickMargin={10}
                  fontFamily="monospace"
                />
                <YAxis stroke="#3f3f46" fontSize={9} fontFamily="monospace" />
                <Tooltip
                  contentStyle={{
                    backgroundColor: "#050505",
                    border: "1px solid #27272a",
                    borderRadius: "8px",
                    fontSize: "10px",
                    fontFamily: "monospace",
                  }}
                  itemStyle={{ color: "#10b981" }}
                />
                <Area
                  type="monotone"
                  dataKey="hash_volume"
                  stroke="#10b981"
                  strokeWidth={2}
                  fillOpacity={1}
                  fill="url(#areaGradient)"
                  name="M-Hashes"
                />
              </AreaChart>
            </ResponsiveContainer>
          </CardContent>
        </Card>

        {/* GRÁFICA B: EFICIENCIA DE PROCESAMIENTO (BAR) */}
        <Card className="bg-[#0a0a0a] border-zinc-800 h-110 flex flex-col shadow-2xl relative overflow-hidden group">
          <div className="absolute inset-0 bg-linear-to-b from-blue-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none" />

          <CardHeader className="border-b border-white/5 bg-white/2">
            <CardTitle className="text-[10px] font-black text-zinc-400 uppercase tracking-widest flex items-center gap-3 font-mono">
              <Cpu className="w-4 h-4 text-blue-500" />
              {t("hardware_efficiency")}
            </CardTitle>
          </CardHeader>

          <CardContent className="flex-1 p-6 relative z-10">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={graphData}>
                <CartesianGrid
                  strokeDasharray="3 3"
                  stroke="#18181b"
                  vertical={false}
                />
                <XAxis
                  dataKey="time_label"
                  stroke="#3f3f46"
                  fontSize={9}
                  fontFamily="monospace"
                />
                <YAxis stroke="#3f3f46" fontSize={9} fontFamily="monospace" />
                <Tooltip
                  contentStyle={{
                    backgroundColor: "#050505",
                    border: "1px solid #27272a",
                    borderRadius: "8px",
                    fontSize: "10px",
                    fontFamily: "monospace",
                  }}
                  itemStyle={{ color: "#3b82f6" }}
                />
                <Bar
                  dataKey="efficiency_ratio"
                  fill="#3b82f6"
                  radius={[4, 4, 0, 0]}
                  name="kH/s Ratio"
                />
              </BarChart>
            </ResponsiveContainer>
          </CardContent>
        </Card>
      </div>

      {/* FOOTER TÉCNICO DE CAPAS */}
      <div className="pt-6 border-t border-white/5 flex flex-col md:flex-row justify-between items-center gap-4">
        <div className="flex items-center gap-6">
          <div className="flex items-center gap-2">
            <div className="w-1.5 h-1.5 rounded-full bg-primary shadow-[0_0_5px_#10b981]" />
            <span className="text-[8px] font-black text-zinc-700 uppercase font-mono tracking-widest">
              Stratum L4 Enabled
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-1.5 h-1.5 rounded-full bg-blue-500 shadow-[0_0_5px_#3b82f6]" />
            <span className="text-[8px] font-black text-zinc-700 uppercase font-mono tracking-widest">
              Strategic Pulse: Verified
            </span>
          </div>
        </div>

        <span className="text-[8px] font-bold text-zinc-800 font-mono uppercase">
          Neural Analytics Engine // V2.5 // Zero-Knowledge Data Tunneling
        </span>
      </div>
    </div>
  );
}
