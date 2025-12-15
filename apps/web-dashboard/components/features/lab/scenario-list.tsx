/**
 * =================================================================
 * APARATO: SCENARIO LIST
 * RESPONSABILIDAD: VISUALIZACIÓN DE PRUEBAS ACTIVAS
 * =================================================================
 */

"use client";

import { useQuery } from "@tanstack/react-query";
import { apiClient } from "@prospector/api-client";
import { TestScenario } from "@prospector/api-contracts"; // Asegurar que este export exista
import { Card } from "@/components/ui/kit/card";
import { CheckCircle2, Clock, PlayCircle } from "lucide-react";
import { cn } from "@/lib/utils/cn";

export function ScenarioList() {
  const { data: scenarios, isLoading } = useQuery({
    queryKey: ["scenarios"],
    queryFn: async () =>
      (await apiClient.get<TestScenario[]>("/lab/scenarios")).data,
  });

  if (isLoading)
    return (
      <div className="text-center text-xs text-slate-500 py-10 font-mono">
        LOADING LAB DATA...
      </div>
    );

  return (
    <div className="grid gap-4">
      {scenarios?.map((scenario) => (
        <Card
          key={scenario.id}
          className="bg-[#0f0f0f] border-slate-800 p-4 flex items-center justify-between group hover:border-emerald-500/30 transition-all"
        >
          <div className="flex flex-col gap-1">
            <div className="flex items-center gap-2">
              <span
                className={cn(
                  "w-2 h-2 rounded-full",
                  scenario.status === "verified"
                    ? "bg-emerald-500"
                    : "bg-amber-500 animate-pulse",
                )}
              />
              <h4 className="text-sm font-bold text-white font-mono uppercase">
                {scenario.name}
              </h4>
            </div>
            <div className="flex items-center gap-2 text-[10px] text-slate-500 font-mono">
              <span className="bg-slate-900 px-1 rounded border border-slate-800">
                {scenario.target_address.substring(0, 8)}...
              </span>
              <span>
                Created: {new Date(scenario.created_at).toLocaleDateString()}
              </span>
            </div>
          </div>

          <div className="flex items-center gap-2">
            {scenario.status === "idle" && (
              <button className="p-2 bg-emerald-900/20 text-emerald-500 rounded hover:bg-emerald-500 hover:text-black transition-colors">
                <PlayCircle className="w-4 h-4" />
              </button>
            )}
            {scenario.status === "verified" && (
              <CheckCircle2 className="w-5 h-5 text-emerald-500" />
            )}
          </div>
        </Card>
      ))}

      {scenarios?.length === 0 && (
        <div className="text-center py-10 border border-dashed border-slate-800 rounded-lg">
          <p className="text-xs text-slate-600 font-mono">
            NO ACTIVE SCENARIOS
          </p>
        </div>
      )}
    </div>
  );
}
