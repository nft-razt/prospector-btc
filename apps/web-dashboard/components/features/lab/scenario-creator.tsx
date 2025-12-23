/**
 * =================================================================
 * APARATO: SCENARIO CREATOR HUD (V14.6 - FORM SECURED)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: INTERFAZ DE CRISTALIZACIÓN DE GOLDEN TICKETS
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la forja de experimentos. Resuelve los errores TS2345
 * y TS2339 al sincronizar los nombres de los campos del formulario
 * con el contrato soberano: 'operation_name' y 'entropy_seed_phrase'.
 * =================================================================
 */

"use client";

import React from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  FlaskConical,
  Plus,
  ShieldAlert,
  Sparkles,
  Terminal
} from "lucide-react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { labApi } from "@prospector/api-client";
import {
  CreateScenarioSchema,
  type CreateScenarioPayload
} from "@prospector/api-contracts";

// --- COMPONENTES ATÓMICOS ---
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@/components/ui/kit/card";
import { Input } from "@/components/ui/kit/input";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

export function ScenarioCreator() {
  const query_client = useQueryClient();

  /**
   * CONFIGURACIÓN DEL MOTOR DE FORMULARIO
   * ✅ RESOLUCIÓN TS2345/TS2339: Mapeo estricto al esquema de dominio.
   */
  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isValid },
  } = useForm<CreateScenarioPayload>({
    resolver: zodResolver(CreateScenarioSchema),
    mode: "onChange",
  });

  const crystallization_mutation = useMutation({
    mutationFn: (payload: CreateScenarioPayload) => labApi.createScenario(payload),
    onSuccess: () => {
      toast.success("SCENARIO_CRYSTALLIZED", {
        description: "Golden Ticket successfully injected into the tactical ledger."
      });
      query_client.invalidateQueries({ queryKey: ["cryptographic-scenarios-inventory-v14.6"] });
      reset();
    },
    onError: (error: Error) => {
      toast.error("VAULT_LINK_ERROR", { description: error.message });
    }
  });

  const on_handle_submit = (data: CreateScenarioPayload): void => {
    crystallization_mutation.mutate(data);
  };

  return (
    <Card className="bg-[#0a0a0a] border-zinc-800 relative overflow-hidden group shadow-2xl">
      <div className="absolute inset-0 bg-linear-to-br from-emerald-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-1000 pointer-events-none" />

      <CardHeader className="relative z-10">
        <CardTitle className="text-[10px] font-black text-emerald-500 uppercase tracking-[0.2em] flex items-center gap-3 font-mono">
          <FlaskConical className="w-4 h-4" />
          Scenario Forge // Ignition
        </CardTitle>
        <CardDescription className="text-[10px] text-zinc-500 font-mono uppercase tracking-tight">
          Inject known entropy vectors to validate swarm audit integrity.
        </CardDescription>
      </CardHeader>

      <CardContent className="relative z-10">
        <form onSubmit={handleSubmit(on_handle_submit)} className="space-y-6">
          <div className="space-y-5">

            {/* CAMPO: operation_name (Resolución TS2345) */}
            <div className="space-y-2">
              <label className="text-[9px] font-bold text-zinc-600 uppercase tracking-widest font-mono flex items-center gap-2">
                <Terminal className="w-3 h-3" /> Operation Designation
              </label>
              <Input
                {...register("operation_name")}
                placeholder="e.g. ALPHA-VULN-2013"
                className={cn(
                  "bg-black/40 border-zinc-800 font-mono text-xs uppercase h-11 transition-all",
                  errors.operation_name && "border-red-900/50 focus:border-red-500"
                )}
                hasError={!!errors.operation_name}
              />
              {errors.operation_name && (
                <span className="text-[9px] text-red-500 font-bold font-mono animate-in fade-in">
                  ERR_INVALID_DESIGNATION: {errors.operation_name.message}
                </span>
              )}
            </div>

            {/* CAMPO: entropy_seed_phrase (Resolución TS2345) */}
            <div className="space-y-2">
              <label className="text-[9px] font-bold text-zinc-600 uppercase tracking-widest font-mono flex items-center gap-2">
                <ShieldAlert className="w-3 h-3" /> Entropy Seed Phrase
              </label>
              <Input
                {...register("entropy_seed_phrase")}
                type="password"
                placeholder="UNSECURED PLAIN TEXT SOURCE"
                className={cn(
                  "bg-black/40 border-zinc-800 font-mono text-xs text-emerald-400 h-11 tracking-widest transition-all",
                  errors.entropy_seed_phrase && "border-red-900/50 focus:border-red-500"
                )}
                hasError={!!errors.entropy_seed_phrase}
              />
              {errors.entropy_seed_phrase && (
                <span className="text-[9px] text-red-500 font-bold font-mono animate-in fade-in">
                  ERR_INSUFFICIENT_ENTROPY: {errors.entropy_seed_phrase.message}
                </span>
              )}
            </div>
          </div>

          <div className="pt-6 border-t border-white/5">
            <Button
              type="submit"
              variant="cyber"
              className="w-full border-emerald-500/30 text-emerald-500 hover:bg-emerald-500 hover:text-black shadow-[0_0_20px_rgba(16,185,129,0.1)] h-12 transition-all font-bold tracking-widest"
              isLoading={crystallization_mutation.isPending}
              disabled={!isValid || crystallization_mutation.isPending}
            >
              <Sparkles className="w-4 h-4 mr-2" />
              CRYSTALLIZE GOLDEN TICKET
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
