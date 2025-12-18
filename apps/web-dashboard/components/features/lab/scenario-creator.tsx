/**
 * =================================================================
 * APARATO: SCENARIO CREATOR HUD (V29.0)
 * CLASIFICACIÓN: FEATURE UI (L5)
 * RESPONSABILIDAD: INTERFAZ DE CRISTALIZACIÓN DE GOLDEN TICKETS
 *
 * ESTRATEGIA DE IMPLEMENTACIÓN:
 * - Validation: Integridad garantizada vía CreateScenarioSchema (L2).
 * - Mutation: Persistencia asíncrona mediante TanStack Query v5.
 * - Style: Tailwind CSS v4 con clases canónicas lineales.
 * - UX: Feedback táctico mediante Sonner (Toasts).
 * =================================================================
 */

"use client";

import React from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { FlaskConical, Plus, ShieldAlert, Sparkles, Terminal } from "lucide-react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { labApi } from "@prospector/api-client";
import {
  CreateScenarioSchema,
  type CreateScenarioPayload
} from "@prospector/api-contracts";

// --- COMPONENTES ATÓMICOS ---
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent
} from "@/components/ui/kit/card";
import { Input } from "@/components/ui/kit/input";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

/**
 * Organismo para la forja de escenarios de prueba deterministas.
 * Permite al operador inyectar entropía conocida para validar la red.
 */
export function ScenarioCreator() {
  const queryClient = useQueryClient();

  /**
   * Configuración del Formulario Reactivo.
   * Utiliza el esquema soberano definido en la capa de dominio.
   */
  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isValid }
  } = useForm<CreateScenarioPayload>({
    resolver: zodResolver(CreateScenarioSchema),
    mode: "onChange"
  });

  /**
   * Mutación de persistencia.
   * Transforma la frase semilla en un registro de base de datos auditable.
   */
  const crystallizationMutation = useMutation({
    mutationFn: (payload: CreateScenarioPayload) => labApi.createScenario(payload),
    onSuccess: () => {
      toast.success("SCENARIO_CRYSTALLIZED", {
        description: "Golden Ticket successfully injected into the tactical ledger."
      });
      // Invalidación de caché para refrescar la lista de experimentos
      queryClient.invalidateQueries({ queryKey: ["scenarios"] });
      reset();
    },
    onError: (error: Error) => {
      console.error("🔥 [LAB_FORGE_FAILURE]:", error.message);
      toast.error("VAULT_LINK_ERROR", {
        description: "Failed to establish secure handshake with the laboratory."
      });
    },
  });

  /**
   * Handler de envío de formulario.
   * @param data Atributos validados del escenario.
   */
  const onHandleSubmit = (data: CreateScenarioPayload): void => {
    crystallizationMutation.mutate(data);
  };

  return (
    <Card className="bg-[#0a0a0a] border-zinc-800 relative overflow-hidden group shadow-2xl">
      {/*
        ✅ CANONICAL CLASS RESOLUTION:
        bg-gradient-to-br -> bg-linear-to-br
      */}
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
        <form onSubmit={handleSubmit(onHandleSubmit)} className="space-y-6">
          <div className="space-y-5">
            {/* DESIGNATION FIELD */}
            <div className="space-y-2">
              <label className="text-[9px] font-bold text-zinc-600 uppercase tracking-widest font-mono flex items-center gap-2">
                <Terminal className="w-3 h-3" /> Operation Designation
              </label>
              <Input
                {...register("name")}
                placeholder="e.g. ALPHA-VULN-2013"
                className={cn(
                  "bg-black/40 border-zinc-800 font-mono text-xs uppercase h-11 transition-all",
                  errors.name && "border-red-900/50 focus:border-red-500"
                )}
              />
              {errors.name && (
                <span className="text-[9px] text-red-500 font-bold font-mono animate-in fade-in slide-in-from-top-1">
                  ERR_INVALID_DESIGNATION: {errors.name.message}
                </span>
              )}
            </div>

            {/* SEED PHRASE FIELD */}
            <div className="space-y-2">
              <label className="text-[9px] font-bold text-zinc-600 uppercase tracking-widest font-mono flex items-center gap-2">
                <ShieldAlert className="w-3 h-3" /> Entropy Seed Phrase
              </label>
              <Input
                {...register("secret_phrase")}
                type="password"
                placeholder="UNSECURED PLAIN TEXT SOURCE"
                className={cn(
                  "bg-black/40 border-zinc-800 font-mono text-xs text-emerald-400 h-11 tracking-widest transition-all",
                  errors.secret_phrase && "border-red-900/50 focus:border-red-500"
                )}
              />
              {errors.secret_phrase && (
                <span className="text-[9px] text-red-500 font-bold font-mono animate-in fade-in slide-in-from-top-1">
                  ERR_INSUFFICIENT_ENTROPY: {errors.secret_phrase.message}
                </span>
              )}
            </div>
          </div>

          <div className="pt-6 border-t border-white/5">
            <Button
              type="submit"
              variant="cyber"
              className="w-full border-emerald-500/30 text-emerald-500 hover:bg-emerald-500 hover:text-black shadow-[0_0_20px_rgba(16,185,129,0.1)] h-12 transition-all font-bold tracking-widest"
              isLoading={crystallizationMutation.isPending}
              disabled={!isValid || crystallizationMutation.isPending}
            >
              <Sparkles className="w-4 h-4 mr-2" />
              CRYSTALLIZE GOLDEN TICKET
            </Button>
          </div>
        </form>
      </CardContent>

      {/* TACTICAL FOOTER */}
      <div className="p-3 bg-black/40 border-t border-white/5 flex justify-center">
        <span className="text-[8px] font-black text-zinc-800 font-mono uppercase tracking-[0.3em]">
          Laboratory Stratum // Secure Injection Point
        </span>
      </div>
    </Card>
  );
}
