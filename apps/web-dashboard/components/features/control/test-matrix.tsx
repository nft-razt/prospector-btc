/**
 * =================================================================
 * APARATO: TEST MATRIX WIDGET
 * RESPONSABILIDAD: VISUALIZACIÓN RECURSIVA DEL ESTADO DE CALIDAD
 * INTEGRACIÓN: GITHUB ACTIONS API
 * =================================================================
 */

import { useQuery } from "@tanstack/react-query";
import { CheckCircle2, XCircle, AlertTriangle, Terminal } from "lucide-react";
import { controlApi } from "@prospector/api-client";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@/components/ui/kit/card";
import { cn } from "@/lib/utils/cn";

export function TestMatrix() {
  // Consumimos el historial de ejecuciones (que incluye 'test' jobs)
  const { data: runs } = useQuery({
    queryKey: ["workflow-runs"],
    queryFn: controlApi.getWorkflowRuns,
    refetchInterval: 10000,
  });

  const latestRun = runs?.[0]; // El más reciente

  // Estado calculado recursivo
  const status = {
    core: latestRun?.conclusion === "success" ? "stable" : "critical",
    domain: latestRun?.conclusion === "success" ? "stable" : "critical",
    infra: latestRun?.conclusion === "success" ? "stable" : "warning",
  };

  return (
    <Card className="bg-[#0f0f0f] border-slate-800">
      <CardHeader className="pb-2 border-b border-white/5">
        <CardTitle className="flex items-center justify-between text-xs font-bold uppercase tracking-widest text-white">
          <span className="flex items-center gap-2">
            <Terminal className="w-4 h-4 text-purple-500" />
            System Integrity
          </span>
          <span
            className={cn(
              "px-2 py-0.5 rounded text-[9px]",
              latestRun?.status === "in_progress"
                ? "bg-blue-500/20 text-blue-400 animate-pulse"
                : latestRun?.conclusion === "success"
                  ? "bg-emerald-500/20 text-emerald-400"
                  : "bg-red-500/20 text-red-400",
            )}
          >
            {latestRun?.status === "in_progress"
              ? "RUNNING TESTS..."
              : latestRun?.conclusion || "UNKNOWN"}
          </span>
        </CardTitle>
      </CardHeader>

      <CardContent className="p-4 grid gap-3">
        {/* NIVEL MACRO: ESTRATOS GEOLÓGICOS */}
        <TestStratum label="CORE MATH (Rust)" status={status.core} />
        <TestStratum label="DOMAIN LOGIC" status={status.domain} />
        <TestStratum label="INFRASTRUCTURE" status={status.infra} />

        {/* NIVEL MICRO: ACCIÓN */}
        {latestRun && (
          <a
            href={latestRun.html_url}
            target="_blank"
            className="mt-2 text-[10px] text-zinc-500 hover:text-white underline decoration-zinc-700 text-center block"
          >
            VIEW DIAGNOSTIC LOGS
          </a>
        )}
      </CardContent>
    </Card>
  );
}

function TestStratum({ label, status }: { label: string; status: string }) {
  const config = {
    stable: {
      icon: CheckCircle2,
      color: "text-emerald-500",
      bg: "bg-emerald-500/10",
    },
    warning: {
      icon: AlertTriangle,
      color: "text-amber-500",
      bg: "bg-amber-500/10",
    },
    critical: { icon: XCircle, color: "text-red-500", bg: "bg-red-500/10" },
  }[status] || {
    icon: AlertTriangle,
    color: "text-zinc-500",
    bg: "bg-zinc-500/10",
  };

  const Icon = config.icon;

  return (
    <div className="flex items-center justify-between p-2 rounded bg-black/40 border border-white/5">
      <span className="text-[10px] font-mono text-zinc-300">{label}</span>
      <div className={cn("p-1 rounded-full", config.bg)}>
        <Icon className={cn("w-3 h-3", config.color)} />
      </div>
    </div>
  );
}
