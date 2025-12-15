/**
 * =================================================================
 * APARATO: MANUAL VERIFIER (THE INTERCEPTOR)
 * RESPONSABILIDAD: PRUEBA DE CONCEPTO EN TIEMPO REAL
 * UX: FEEDBACK VISUAL DE "MATCH"
 * =================================================================
 */

"use client";

import { useState } from "react";
import { useMutation } from "@tanstack/react-query";
import {
  Search,
  ShieldCheck,
  AlertCircle,
  Fingerprint,
  Database,
} from "lucide-react";
import { toast } from "sonner";

import { apiClient } from "@prospector/api-client";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@/components/ui/kit/card";
import { Input } from "@/components/ui/kit/input";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

export function ManualVerifier() {
  const [input, setInput] = useState("");

  const mutation = useMutation({
    mutationFn: async (secret: string) => {
      // Llamada al endpoint que acabamos de crear
      return (await apiClient.post<any>("/lab/verify", { secret })).data;
    },
    onError: () => toast.error("Verification System Offline"),
  });

  const result = mutation.data;

  return (
    <Card className="bg-[#0f0f0f] border-slate-800 h-full flex flex-col">
      <CardHeader className="border-b border-slate-800 bg-slate-900/20">
        <CardTitle className="text-xs font-bold text-blue-400 uppercase tracking-widest flex items-center gap-2">
          <Search className="w-4 h-4" />
          The Interceptor
        </CardTitle>
      </CardHeader>

      <CardContent className="p-6 space-y-6 flex-1">
        <div className="space-y-2">
          <label className="text-[10px] uppercase font-bold text-slate-500 font-mono">
            Input Entropy (Phrase / Key)
          </label>
          <div className="flex gap-2">
            <Input
              value={input}
              onChange={(e) => setInput(e.target.value)}
              placeholder="e.g. correct horse battery staple"
              className="font-mono text-xs bg-black border-slate-700"
            />
            <Button
              variant="default"
              className="bg-blue-600 hover:bg-blue-500 text-white"
              onClick={() => mutation.mutate(input)}
              isLoading={mutation.isPending}
              disabled={!input}
            >
              VERIFY
            </Button>
          </div>
        </div>

        {/* RESULTS PANEL */}
        {result && (
          <div
            className={cn(
              "rounded-lg border p-4 space-y-3 transition-all animate-in fade-in slide-in-from-bottom-2",
              result.is_target
                ? "bg-emerald-950/20 border-emerald-500/50 shadow-[0_0_20px_rgba(16,185,129,0.1)]"
                : "bg-slate-900/50 border-slate-700",
            )}
          >
            <div className="flex items-center justify-between">
              <span className="text-[10px] font-mono uppercase text-slate-500">
                Derivation Result
              </span>
              {result.is_target ? (
                <span className="flex items-center gap-1 text-[10px] font-bold text-emerald-400 bg-emerald-950/50 px-2 py-0.5 rounded border border-emerald-900">
                  <Database className="w-3 h-3" /> MATCH FOUND IN DB
                </span>
              ) : (
                <span className="flex items-center gap-1 text-[10px] font-bold text-slate-500 bg-slate-950 px-2 py-0.5 rounded border border-slate-800">
                  <AlertCircle className="w-3 h-3" /> NO DATABASE MATCH
                </span>
              )}
            </div>

            <div className="space-y-1">
              <div className="flex items-center gap-2 text-xs font-mono text-slate-300">
                <Fingerprint className="w-3 h-3 text-slate-500" />
                <span className="truncate">{result.address}</span>
              </div>
              {result.matched_scenario && (
                <div className="text-xs text-emerald-400 font-mono pl-5">
                  ↳ Linked to Scenario:{" "}
                  <strong>{result.matched_scenario}</strong>
                </div>
              )}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
