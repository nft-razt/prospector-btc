/**
 * =================================================================
 * APARATO: SWARM LAUNCHER CONTROL CENTER (V11.2 - TYPE HARDENED)
 * CLASIFICACIÓN: FEATURE COMPONENT (ESTRATO L5)
 * RESPONSABILIDAD: GESTIÓN DE CONFIGURACIÓN Y DESPACHO DE ENJAMBRE
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la interfaz de mando soberana para el despliegue de
 * infraestructura. Resuelve el conflicto de tipos entre Zod y
 * React Hook Form garantizando que la Fuente Única de Verdad (SSoT)
 * sea respetada en todo el ciclo de vida del formulario.
 * =================================================================
 */

"use client";

import React, { useState } from "react";
import { useForm, type SubmitHandler, type Resolver } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Rocket, Layers, Users, Activity } from "lucide-react";
import { toast } from "sonner";

// --- SINAPSIS CON EL DOMINIO Y EL CLIENTE DE API (L2 & L4) ---
import {
  SwarmLaunchSchema,
  type SwarmLaunchConfig,
} from "@prospector/api-contracts";
import { controlApi } from "@prospector/api-client";

// --- ÁTOMOS UI (DESIGN SYSTEM) ---
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardDescription,
} from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { Input } from "@/components/ui/kit/input";

// --- COMPONENTES DE SEGURIDAD (GATEKEEPERS) ---
import { PreFlightModal } from "./pre-flight-modal";

/**
 * Organismo de mando táctico para la ignición del enjambre.
 */
export function SwarmLauncher(): React.ReactElement {
  const query_client = useQueryClient();

  // Gestión de estado para el protocolo de verificación Pre-Flight
  const [is_pre_flight_visible, set_is_pre_flight_visible] = useState<boolean>(false);
  const [pending_mission_config, set_pending_mission_config] = useState<SwarmLaunchConfig | null>(null);

  /**
   * INICIALIZACIÓN DEL MOTOR DE FORMULARIO SOBERANO
   *
   * ✅ RESOLUCIÓN TS2322:
   * Aplicamos un cast al resolver para unificar la discrepancia entre
   * el esquema de entrada (opcional por defaults) y el contrato de salida.
   */
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors, isValid },
  } = useForm<SwarmLaunchConfig>({
    resolver: zodResolver(SwarmLaunchSchema) as Resolver<SwarmLaunchConfig>,
    mode: "onChange",
    defaultValues: {
      worker_count: 30,
      shard_count: 5,
      ref: "main",
    },
  });

  // Monitoreo en tiempo real de los parámetros para el HUD de estimación
  const current_worker_count = watch("worker_count");
  const current_shard_count = watch("shard_count");

  /**
   * MUTACIÓN DE DESPACHO C2 (L5 -> L4)
   */
  const ignition_mutation = useMutation({
    mutationFn: (mission_parameters: SwarmLaunchConfig) =>
      controlApi.launchSwarm(mission_parameters),
    onSuccess: () => {
      toast.success("SWARM_SEQUENCE_IGNITED", {
        description: "Ignition signal confirmed by the Sovereign Gateway.",
        duration: 5000,
      });
      set_is_pre_flight_visible(false);
      set_pending_mission_config(null);

      // Sincronización neural del historial
      setTimeout(() => {
        query_client.invalidateQueries({ queryKey: ["workflow-runs"] });
      }, 2500);
    },
    onError: (error: Error) => {
      toast.error("C2_HANDSHAKE_FAULT", {
        description: error.message || "Failed to communicate with GitHub Actions.",
      });
      set_is_pre_flight_visible(false);
    },
  });

  /**
   * MANEJADOR DE VALIDACIÓN EXITOSA
   *
   * ✅ RESOLUCIÓN TS2345:
   * Tipado explícito mediante SubmitHandler para garantizar paridad con handleSubmit.
   */
  const on_form_submit_validated: SubmitHandler<SwarmLaunchConfig> = (validated_data) => {
    set_pending_mission_config(validated_data);
    set_is_pre_flight_visible(true);
  };

  /**
   * Confirmación final del operador tras el chequeo de salud.
   */
  const handle_ignition_confirmed = (): void => {
    if (pending_mission_config) {
      ignition_mutation.mutate(pending_mission_config);
    }
  };

  return (
    <Card className="bg-[#0f0f0f] border-slate-800 h-full flex flex-col relative overflow-hidden shadow-2xl group">
      {/* Visual Ambiance Layer */}
      <div className="absolute top-0 right-0 p-24 bg-purple-500/5 blur-[120px] rounded-full pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-1000" />

      <CardHeader className="pb-6 border-b border-white/5 bg-white/2">
        <CardTitle className="flex items-center gap-3 text-white font-mono tracking-widest text-lg uppercase">
          <div className="p-2 bg-purple-500/10 rounded-lg border border-purple-500/20 shadow-[0_0_15px_rgba(168,85,247,0.2)]">
            <Rocket className="w-5 h-5 text-purple-500" />
          </div>
          Swarm Command Console
        </CardTitle>
        <CardDescription className="text-[10px] font-mono text-zinc-500 uppercase tracking-tighter">
          Orchestrating ephemeral compute grid units.
        </CardDescription>
      </CardHeader>

      <CardContent className="pt-8 flex-1 flex flex-col justify-between gap-8">
        <form onSubmit={handleSubmit(on_form_submit_validated)} className="space-y-8">
          <div className="grid grid-cols-2 gap-8">

            {/* ENTRADA: NODOS POR SEGMENTO */}
            <div className="space-y-3">
              <label className="text-[9px] uppercase font-black text-zinc-600 flex items-center gap-2 font-mono tracking-widest">
                <Users className="w-3 h-3" /> Nodes / Shard
              </label>
              <div className="relative group/input">
                <Input
                  type="number"
                  {...register("worker_count", { valueAsNumber: true })}
                  className="bg-black border-zinc-800 font-mono text-emerald-400 focus:border-purple-500/50 transition-all h-12 text-lg font-black"
                  hasError={!!errors.worker_count}
                  placeholder="30"
                />
              </div>
              {errors.worker_count && (
                <span className="text-red-500 text-[9px] font-bold font-mono animate-in fade-in">
                  {errors.worker_count.message}
                </span>
              )}
            </div>

            {/* ENTRADA: SEGMENTOS PARALELOS */}
            <div className="space-y-3">
              <label className="text-[9px] uppercase font-black text-zinc-600 flex items-center gap-2 font-mono tracking-widest">
                <Layers className="w-3 h-3" /> Parallel Shards
              </label>
              <div className="relative group/input">
                <Input
                  type="number"
                  {...register("shard_count", { valueAsNumber: true })}
                  className="bg-black border-zinc-800 font-mono text-purple-400 focus:border-purple-500/50 transition-all h-12 text-lg font-black"
                  hasError={!!errors.shard_count}
                  placeholder="5"
                />
              </div>
              {errors.shard_count && (
                <span className="text-red-500 text-[9px] font-bold font-mono animate-in fade-in">
                  {errors.shard_count.message}
                </span>
              )}
            </div>
          </div>

          {/* HUD DE MÉTRICAS PROYECTADAS */}
          <div className="p-5 rounded-xl bg-zinc-950 border border-white/5 flex items-center justify-between shadow-inner">
            <div className="flex items-center gap-4">
              <div className="h-2 w-2 rounded-full bg-emerald-500 animate-pulse shadow-[0_0_8px_#10b981]" />
              <span className="text-[10px] text-zinc-400 font-mono uppercase font-bold tracking-widest">
                Aggregated Capacity
              </span>
            </div>
            <div className="text-lg font-black text-white font-mono tracking-tighter">
              {(current_worker_count || 0) * (current_shard_count || 0)} UNITS
            </div>
          </div>

          <Button
            type="submit"
            variant="cyber"
            className="w-full h-14 text-xs font-black tracking-[0.4em]"
            isLoading={ignition_mutation.isPending}
            disabled={!isValid || ignition_mutation.isPending}
          >
            EXECUTE IGNITION SEQUENCE
          </Button>
        </form>

        <div className="pt-4 border-t border-white/5 flex justify-between items-center opacity-40">
          <span className="text-[8px] text-zinc-600 font-black font-mono uppercase tracking-[0.2em]">
            Protocol Link: Active
          </span>
          <Activity className="w-3 h-3 text-zinc-600" />
        </div>
      </CardContent>

      {/* COMPONENTE DE SEGURIDAD PRE-IGNICIÓN */}
      {is_pre_flight_visible && pending_mission_config && (
        <PreFlightModal
          isOpen={is_pre_flight_visible}
          onClose={() => set_is_pre_flight_visible(false)}
          onConfirm={handle_ignition_confirmed}
          config={{
            workerCount: pending_mission_config.worker_count,
            shardCount: pending_mission_config.shard_count,
          }}
        />
      )}
    </Card>
  );
}
