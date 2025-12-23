/**
 * =================================================================
 * APARATO: MANUAL VERIFIER (THE INTERCEPTOR V14.8)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: PRUEBA DE CONCEPTO CRIPTOGRÁFICA EN TIEMPO REAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el protocolo 'The Interceptor'. Permite auditar vectores
 * manuales contra el censo. Resuelve todos los errores de propiedad
 * mapeando 'entropy_vector', 'is_target_collision', 'matched_scenario_name'
 * y 'derived_bitcoin_address'.
 * =================================================================
 */

"use client";

import React, { useState } from "react";
import { useMutation } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import {
  Search,
  ShieldCheck,
  AlertCircle,
  Fingerprint,
  Cpu,
  RefreshCw
} from "lucide-react";
import { toast } from "sonner";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { labApi, type EntropyResult } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Input } from "@/components/ui/kit/input";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

export function ManualVerifier(): React.ReactElement {
  const translations = useTranslations("Dashboard.lab");
  const [vector_input, set_vector_input] = useState<string>("");

  /**
   * MUTACIÓN DEL INTERCEPTOR
   * ✅ RESOLUCIÓN TS: Sincronización total con 'EntropyResult' (V13.0).
   */
  const verification_mutation = useMutation<EntropyResult, Error, string>({
    mutationFn: (secret: string) => labApi.verifyEntropy({
      entropy_vector: secret, // ✅ RESOLUCIÓN TS2353
      vector_type: "phrase"
    }),
    onSuccess: (data) => {
      if (data.is_target_collision) {
        toast.success("COLLISION_DETECTED", {
          description: `Vector linked to: ${data.matched_scenario_name}`,
          duration: 6000
        });
      }
    }
  });

  const analysis_result = verification_mutation.data;

  return (
    <Card className="bg-[#0a0a0a] border-zinc-800 shadow-2xl relative overflow-hidden group">
      <CardHeader className="border-b border-white/5 bg-white/2 p-5">
        <CardTitle className="text-[10px] font-black text-blue-400 uppercase tracking-[0.2em] flex items-center gap-3 font-mono">
          <Search className="w-4 h-4" />
          Neural_Interceptor // L5_Verification
        </CardTitle>
      </CardHeader>

      <CardContent className="p-6 space-y-6">
        <div className="space-y-3">
          <label className="text-[9px] uppercase font-black text-zinc-600 font-mono tracking-widest">
            Entropy_Source_Vector
          </label>
          <div className="flex gap-3">
            <Input
              value={vector_input}
              onChange={(e) => set_vector_input(e.target.value)}
              placeholder="PHRASE_OR_HEX_INPUT"
              className="font-mono text-xs bg-black/50 border-zinc-800 focus:border-blue-500/40 text-zinc-300 h-12"
            />
            <Button
              variant="cyber"
              className="border-blue-500/30 text-blue-400 hover:bg-blue-500 hover:text-black px-6"
              onClick={() => verification_mutation.mutate(vector_input)}
              isLoading={verification_mutation.isPending}
            >
              SCAN
            </Button>
          </div>
        </div>

        <AnimatePresence>
          {analysis_result && (
            <div className={cn(
              "rounded-xl border p-5 space-y-4 animate-in fade-in duration-500",
              analysis_result.is_target_collision
                ? "bg-emerald-950/20 border-emerald-500/50 shadow-[0_0_30px_rgba(16,185,129,0.1)]"
                : "bg-zinc-900/40 border-zinc-800"
            )}>
              <div className="flex items-center justify-between">
                <span className="text-[8px] font-black uppercase text-zinc-600 font-mono">Result_Metadata</span>
                {analysis_result.is_target_collision ? ( // ✅ RESOLUCIÓN TS2339
                  <div className="flex items-center gap-2 text-[9px] font-black text-emerald-400">
                    <ShieldCheck className="w-3 h-3" /> COLLISION_MATCH
                  </div>
                ) : (
                  <div className="flex items-center gap-2 text-[9px] font-black text-zinc-500">
                    <AlertCircle className="w-3 h-3" /> NO_IDENTITY_MATCH
                  </div>
                )}
              </div>

              <div className="space-y-4 font-mono">
                <div className="space-y-1.5">
                  <span className="text-[7px] text-zinc-500 uppercase font-bold">Derived_Bitcoin_Address</span>
                  <div className="flex items-center gap-3 text-[10px] text-zinc-300 bg-black/50 p-3 rounded-lg border border-white/5">
                    <Fingerprint className="w-4 h-4 text-zinc-700" />
                    <span className="select-all tracking-tight">{analysis_result.derived_bitcoin_address}</span>
                  </div>
                </div>

                {analysis_result.matched_scenario_name && ( // ✅ RESOLUCIÓN TS2551
                  <div className="pt-3 border-t border-emerald-500/20">
                    <span className="text-[8px] text-emerald-500/60 uppercase font-black">Scenario_Linkage</span>
                    <p className="text-xs text-emerald-400 font-bold tracking-widest mt-1">
                      ↳ {analysis_result.matched_scenario_name}
                    </p>
                  </div>
                )}
              </div>
            </div>
          )}
        </AnimatePresence>
      </CardContent>
    </Card>
  );
}

import { AnimatePresence } from "framer-motion";
