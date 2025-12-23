/**
 * =================================================================
 * APARATO: SCENARIO LIST MANAGER (V14.6 - TYPE SECURED)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE EXPERIMENTOS CRIPTOGRÁFICOS
 * =================================================================
 */

"use client";

import React from "react";
import { useQuery } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import {
  CheckCircle2,
  Clock,
  PlayCircle,
  FlaskConical,
  Fingerprint,
  ChevronRight,
  Database,
  AlertCircle // ✅ RESOLUCIÓN TS2304
} from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { labApi, type TestScenario } from "@prospector/api-client";
import { Card } from "@/components/ui/kit/card";
import { Skeleton } from "@/components/ui/kit/skeleton"; // ✅ RESOLUCIÓN TS2552
import { cn } from "@/lib/utils/cn";

export function ScenarioList(): React.ReactElement {
  const translations = useTranslations("Dashboard.lab");

  const { data: scenarios_collection, isLoading, isError } = useQuery<TestScenario[]>({
    queryKey: ["cryptographic-scenarios-inventory-v14.6"],
    queryFn: () => labApi.listScenarios(),
    refetchInterval: 10000,
  });

  if (isLoading) return <ScenarioListSkeleton />;

  return (
    <div className="grid gap-5 animate-in fade-in duration-700">
      <AnimatePresence mode="popLayout">
        {scenarios_collection?.length === 0 ? (
          <div className="flex flex-col items-center justify-center p-20 border border-dashed border-zinc-800 rounded-2xl bg-zinc-950/20">
            <FlaskConical className="w-10 h-10 text-zinc-800 mb-4 animate-pulse" />
            <p className="text-[10px] font-mono text-zinc-600 uppercase tracking-[0.4em]">
              {translations("no_scenarios")}
            </p>
          </div>
        ) : (
          scenarios_collection?.map((experiment: TestScenario) => (
            <motion.div
              key={experiment.identifier}
              layout
              initial={{ opacity: 0, x: -10 }}
              animate={{ opacity: 1, x: 0 }}
            >
              <Card className="bg-[#0a0a0a] border-zinc-800 p-5 flex items-center justify-between hover:border-emerald-500/30 transition-all shadow-xl group">
                <div className="flex gap-6 items-center">
                  <div className="relative">
                    <div className={cn(
                      "w-2.5 h-2.5 rounded-full shadow-[0_0_8px]",
                      experiment.current_status === "verified" ? "bg-emerald-500 shadow-emerald-500/40" : "bg-amber-500 animate-pulse shadow-amber-500/40"
                    )} />
                  </div>

                  <div className="space-y-1.5">
                    <div className="flex items-center gap-3">
                      <h4 className="text-xs font-black text-white font-mono uppercase tracking-widest">
                        {experiment.operation_name}
                      </h4>
                      <span className="text-[8px] font-bold text-zinc-600 font-mono bg-zinc-950 px-2 py-0.5 rounded border border-white/5 uppercase">
                        TAG_{experiment.identifier.substring(0, 6)}
                      </span>
                    </div>

                    <div className="flex items-center gap-6 text-[10px] font-mono">
                      <div className="flex items-center gap-2 text-zinc-400">
                        <Fingerprint className="w-3.5 h-3.5 text-zinc-700" />
                        <span className="select-all tracking-tight">{experiment.target_bitcoin_address}</span>
                      </div>
                      <div className="flex items-center gap-2 text-zinc-600">
                        <Clock className="w-3.5 h-3.5" />
                        <span>{new Date(experiment.crystallized_at).toLocaleDateString()}</span>
                      </div>
                    </div>
                  </div>
                </div>

                <div className="flex items-center gap-4">
                  {experiment.current_status === "verified" ? (
                    <CheckCircle2 className="w-5 h-5 text-emerald-500" />
                  ) : (
                    <button className="p-2.5 bg-zinc-900 text-zinc-400 rounded-xl hover:bg-emerald-500 hover:text-black transition-all border border-white/5">
                      <PlayCircle className="w-5 h-5" />
                    </button>
                  )}
                  <ChevronRight className="w-4 h-4 text-zinc-800 group-hover:text-zinc-600 transition-colors" />
                </div>
              </Card>
            </motion.div>
          ))
        )}
      </AnimatePresence>

      {isError && (
        <div className="p-6 bg-red-950/10 border border-red-900/30 rounded-2xl flex items-center gap-4">
          <AlertCircle className="w-5 h-5 text-red-500" />
          <p className="text-[10px] font-mono text-red-400 font-black uppercase">
            Critical_Sync_Fault: Laboratory_Ledger_Unreachable
          </p>
        </div>
      )}
    </div>
  );
}

function ScenarioListSkeleton() {
  return (
    <div className="space-y-4">
      {[...Array(3)].map((_, i) => <Skeleton key={i} className="h-24 w-full bg-zinc-900/50 rounded-2xl" />)}
    </div>
  );
}
