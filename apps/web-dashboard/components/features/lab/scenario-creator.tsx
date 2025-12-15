/**
 * =================================================================
 * APARATO: SCENARIO CREATOR
 * CLASIFICACIÓN: FEATURE UI (INTERACTIVE)
 * RESPONSABILIDAD: CREACIÓN INTUITIVA DE ESCENARIOS DE PRUEBA
 * UX: FEEDBACK VISUAL INMEDIATO
 * =================================================================
 */

"use client";

import { useState } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { FlaskConical, ArrowRight, Save, Key, Wallet } from "lucide-react";
import { toast } from "sonner";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import {
  CreateScenarioSchema,
  type CreateScenarioPayload,
} from "@prospector/api-contracts";
import { apiClient } from "@prospector/api-client"; // Asumimos que apiClient es genérico o extendido

import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { Input } from "@/components/ui/kit/input";

export function ScenarioCreator() {
  const queryClient = useQueryClient();
  const [preview, setPreview] = useState<{
    address: string;
    wif: string;
  } | null>(null);

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors, isValid },
  } = useForm<CreateScenarioPayload>({
    resolver: zodResolver(CreateScenarioSchema),
    mode: "onChange",
  });

  // Efecto simulado de derivación (En producción real, esto llamaría a un endpoint /tools/derive)
  // Para la demo UX, simulamos la respuesta visual.
  const phrase = watch("secret_phrase");

  // Mutación para guardar en DB
  const mutation = useMutation({
    mutationFn: async (data: CreateScenarioPayload) => {
      // Enviamos al backend que se encargará de la derivación real criptográfica y guardado
      return apiClient.post("/lab/scenarios", data);
    },
    onSuccess: () => {
      toast.success("Scenario Crystallized", {
        description: "Target injected into Test Lab.",
      });
      queryClient.invalidateQueries({ queryKey: ["scenarios"] });
      setPreview(null);
    },
    onError: () => toast.error("Failed to create scenario"),
  });

  return (
    <Card className="bg-[#0f0f0f] border-slate-800 relative overflow-hidden">
      {/* Fondo decorativo de laboratorio */}
      <div className="absolute top-0 right-0 p-32 bg-emerald-500/5 blur-[100px] rounded-full pointer-events-none" />

      <CardHeader>
        <CardTitle className="flex items-center gap-3 text-emerald-400 font-mono tracking-wider">
          <FlaskConical className="w-5 h-5" />
          CRYPTOGRAPHIC FORGE
        </CardTitle>
        <CardDescription>
          Define a known entropy source ("Golden Ticket") to validate swarm
          integrity.
        </CardDescription>
      </CardHeader>

      <CardContent className="space-y-6 relative z-10">
        <form
          onSubmit={handleSubmit((data) => mutation.mutate(data))}
          className="space-y-4"
        >
          <div className="space-y-2">
            <label className="text-[10px] uppercase font-bold text-slate-500 font-mono">
              Scenario Name
            </label>
            <Input
              {...register("name")}
              placeholder="e.g. Operation Alpha Test"
              className="bg-black/50 border-slate-700"
            />
            {errors.name && (
              <span className="text-red-500 text-[10px]">
                {errors.name.message}
              </span>
            )}
          </div>

          <div className="space-y-2">
            <label className="text-[10px] uppercase font-bold text-slate-500 font-mono">
              Secret Seed Phrase
            </label>
            <div className="relative">
              <Key className="absolute left-3 top-3 w-4 h-4 text-slate-600" />
              <Input
                {...register("secret_phrase")}
                placeholder="correct horse battery staple"
                className="bg-black/50 border-slate-700 pl-10 font-mono text-emerald-300"
              />
            </div>
            {errors.secret_phrase && (
              <span className="text-red-500 text-[10px]">
                {errors.secret_phrase.message}
              </span>
            )}
          </div>

          {/* VISUAL FEEDBACK AREA (UX MAGIC) */}
          {phrase && phrase.length > 5 && (
            <div className="p-4 bg-slate-900/50 rounded-lg border border-slate-800 space-y-2 animate-in fade-in slide-in-from-top-2">
              <div className="flex items-center justify-between text-[10px] text-slate-500 font-mono uppercase">
                <span>Predicted Artifacts</span>
                <span className="text-emerald-500">Auto-Calculated</span>
              </div>
              <div className="flex items-center gap-3 text-xs font-mono text-slate-300">
                <Wallet className="w-4 h-4 text-purple-500" />
                <span className="truncate">
                  1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa... (Simulation)
                </span>
              </div>
            </div>
          )}

          <Button
            type="submit"
            variant="cyber"
            className="w-full"
            isLoading={mutation.isPending}
            disabled={!isValid}
          >
            <Save className="w-4 h-4 mr-2" />
            INJECT SCENARIO
          </Button>
        </form>
      </CardContent>
    </Card>
  );
}
