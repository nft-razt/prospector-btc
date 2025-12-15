/**
 * =================================================================
 * APARATO: WORKFLOW RUN HISTORY
 * CLASIFICACIÓN: MONITORING COMPONENT
 * RESPONSABILIDAD: VISUALIZACIÓN DE ESTADO CI/CD (GITHUB ACTIONS)
 * =================================================================
 */

"use client";

import { useQuery } from "@tanstack/react-query";
import { controlApi, type WorkflowRun } from "@prospector/api-client";
import {
  Activity,
  CheckCircle2,
  XCircle,
  Clock,
  ExternalLink,
  PlayCircle,
  StopCircle,
  AlertOctagon,
} from "lucide-react";
import { formatDistanceToNow } from "date-fns";
import { Skeleton } from "@/components/ui/kit/skeleton";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@/components/ui/kit/card";
import { cn } from "@/lib/utils/cn";

export function RunHistory() {
  const {
    data: runs,
    isLoading,
    isError,
    refetch,
  } = useQuery<WorkflowRun[]>({
    queryKey: ["workflow-runs"],
    queryFn: controlApi.getWorkflowRuns,
    refetchInterval: 5000, // Polling activo para ver progreso en vivo
    retry: 2,
  });

  // Skeleton de Carga
  if (isLoading) {
    return (
      <Card className="bg-[#0f0f0f] border-slate-800 h-full flex flex-col">
        <CardHeader className="pb-2 border-b border-white/5">
          <Skeleton className="h-5 w-32 bg-zinc-800" />
        </CardHeader>
        <CardContent className="p-0">
          {[1, 2, 3].map((i) => (
            <div
              key={i}
              className="p-4 border-b border-white/5 flex justify-between"
            >
              <Skeleton className="h-8 w-8 rounded-full bg-zinc-800" />
              <div className="space-y-2 flex-1 ml-4">
                <Skeleton className="h-3 w-24 bg-zinc-800" />
                <Skeleton className="h-2 w-16 bg-zinc-800" />
              </div>
            </div>
          ))}
        </CardContent>
      </Card>
    );
  }

  // Estado de Error (Probablemente GitHub Token inválido)
  if (isError) {
    return (
      <Card className="bg-[#0f0f0f] border-red-900/30 h-full flex flex-col justify-center items-center p-6 text-center">
        <div className="p-3 bg-red-500/10 rounded-full mb-3">
          <AlertOctagon className="w-6 h-6 text-red-500" />
        </div>
        <h3 className="text-xs font-bold text-red-400 uppercase tracking-widest">
          Signal Lost
        </h3>
        <p className="text-[10px] text-zinc-500 mt-1 mb-4">
          Cannot retrieve C2 telemetry.
        </p>
        <button
          onClick={() => refetch()}
          className="text-[10px] text-zinc-300 hover:text-white underline decoration-zinc-700"
        >
          RETRY CONNECTION
        </button>
      </Card>
    );
  }

  return (
    <Card className="bg-[#0f0f0f] border-slate-800 h-full flex flex-col relative overflow-hidden">
      <CardHeader className="pb-3 border-b border-white/5 bg-black/20">
        <CardTitle className="flex items-center gap-2 text-white text-xs font-bold uppercase tracking-widest">
          <Activity className="w-3.5 h-3.5 text-emerald-500" />
          Deploy Operations
        </CardTitle>
      </CardHeader>

      <div className="flex-1 overflow-y-auto custom-scrollbar">
        {runs?.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-40 text-zinc-600 space-y-2">
            <Clock className="w-6 h-6 opacity-20" />
            <span className="text-[10px] font-mono">NO HISTORY FOUND</span>
          </div>
        ) : (
          <div className="divide-y divide-white/5">
            {runs?.map((run) => (
              <RunItem key={run.id} run={run} />
            ))}
          </div>
        )}
      </div>

      {/* Footer Indicador */}
      <div className="p-2 bg-zinc-950 border-t border-white/5 flex justify-between items-center text-[9px] text-zinc-600 font-mono px-4">
        <span>SYNC: AUTO (5s)</span>
        <span className="flex items-center gap-1">
          <span className="w-1.5 h-1.5 bg-emerald-500 rounded-full animate-pulse" />
          LIVE
        </span>
      </div>
    </Card>
  );
}

function RunItem({ run }: { run: WorkflowRun }) {
  const getStatusConfig = () => {
    if (run.status === "in_progress" || run.status === "queued") {
      return {
        icon: PlayCircle,
        color: "text-yellow-500",
        bg: "bg-yellow-500/10",
        animate: true,
      };
    }
    if (run.conclusion === "success") {
      return {
        icon: CheckCircle2,
        color: "text-emerald-500",
        bg: "bg-emerald-500/10",
        animate: false,
      };
    }
    if (run.conclusion === "cancelled") {
      return {
        icon: StopCircle,
        color: "text-zinc-500",
        bg: "bg-zinc-500/10",
        animate: false,
      };
    }
    return {
      icon: XCircle,
      color: "text-red-500",
      bg: "bg-red-500/10",
      animate: false,
    };
  };

  const config = getStatusConfig();
  const Icon = config.icon;

  return (
    <div className="p-3 flex items-center justify-between hover:bg-white/5 transition-colors group">
      <div className="flex items-center gap-3">
        <div className={cn("p-1.5 rounded-full flex-shrink-0", config.bg)}>
          <Icon
            className={cn(
              "w-4 h-4",
              config.color,
              config.animate && "animate-pulse",
            )}
          />
        </div>

        <div className="flex flex-col min-w-0">
          <span className="text-xs font-bold text-zinc-200 truncate group-hover:text-white transition-colors">
            {run.name}{" "}
            <span className="text-zinc-600 font-mono">#{run.run_number}</span>
          </span>
          <div className="flex items-center gap-2">
            <span
              className={cn(
                "text-[9px] uppercase font-bold tracking-wider",
                config.color,
              )}
            >
              {run.conclusion || run.status}
            </span>
            <span className="text-[9px] text-zinc-600 font-mono">
              •{" "}
              {formatDistanceToNow(new Date(run.created_at), {
                addSuffix: true,
              })}
            </span>
          </div>
        </div>
      </div>

      <a
        href={run.html_url}
        target="_blank"
        rel="noreferrer"
        className="p-2 text-zinc-600 hover:text-white hover:bg-white/10 rounded transition-all opacity-0 group-hover:opacity-100"
        title="View Logs on GitHub"
      >
        <ExternalLink className="w-3.5 h-3.5" />
      </a>
    </div>
  );
}
