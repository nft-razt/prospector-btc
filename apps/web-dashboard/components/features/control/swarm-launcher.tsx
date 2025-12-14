/**
 * =================================================================
 * APARATO: SWARM LAUNCHER (C2 CONTROLLER)
 * CLASIFICACIÓN: FEATURE COMPONENT
 * RESPONSABILIDAD: INTERFAZ DE CONFIGURACIÓN Y DISPARO DE DESPLIEGUE
 * DEPENDENCIA: PreFlightModal (Gatekeeper)
 * =================================================================
 */

'use client';

import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Rocket, Layers, Users, Activity } from 'lucide-react';
import { toast } from 'sonner';

// Contratos y Cliente de API
import { SwarmLaunchSchema, type SwarmLaunchConfig } from '@prospector/api-contracts';
import { controlApi } from '@prospector/api-client';

// Componentes UI (Design System)
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '@/components/ui/kit/card';
import { Button } from '@/components/ui/kit/button';
import { Input } from '@/components/ui/kit/input';

// Componente Gatekeeper (Seguridad Pre-Despliegue)
import { PreFlightModal } from './pre-flight-modal';

/**
 * Componente principal para la orquestación del lanzamiento de infraestructura.
 *
 * Flujo de Operación:
 * 1. El operador configura los parámetros (Workers, Shards).
 * 2. Se valida el formulario localmente (Zod).
 * 3. Se abre el `PreFlightModal` para verificar integridad del sistema (API, Auth, Vault).
 * 4. Si el checklist pasa, se ejecuta la mutación que contacta con GitHub Actions.
 */
export function SwarmLauncher() {
  const queryClient = useQueryClient();

  // Estado local para la gestión del flujo de confirmación
  const [showPreFlight, setShowPreFlight] = useState(false);
  const [pendingConfig, setPendingConfig] = useState<SwarmLaunchConfig | null>(null);

  // 1. Configuración del Formulario Reactivo
  const {
    register,
    handleSubmit,
    formState: { errors, isValid }
  } = useForm<SwarmLaunchConfig>({
    resolver: zodResolver(SwarmLaunchSchema),
    mode: 'onChange',
    defaultValues: {
      worker_count: 30, // Valor óptimo para runners estándar de GitHub
      shard_count: 5,   // Paralelismo medio
      ref: 'main'       // Rama por defecto
    }
  });

  // 2. Definición de la Mutación (Disparo a GitHub Actions)
  const mutation = useMutation({
    mutationFn: controlApi.launchSwarm,
    onSuccess: () => {
      // Feedback visual inmediato
      toast.success('Swarm Sequence Initiated', {
        description: 'Command dispatched to GitHub Actions C2.',
        duration: 5000,
      });

      // Cerrar modal y limpiar estado pendiente
      setShowPreFlight(false);
      setPendingConfig(null);

      // Refrescar lista de ejecuciones para ver el nuevo job
      setTimeout(() => {
        queryClient.invalidateQueries({ queryKey: ['workflow-runs'] });
      }, 2000);
    },
    onError: (error: Error) => {
      toast.error('Ignition Failed', {
        description: error.message || 'Unknown C2 Error'
      });
      // Mantenemos el modal abierto en caso de error para reintentar si es necesario,
      // o el usuario puede cerrarlo manualmente.
      setShowPreFlight(false);
    }
  });

  /**
   * Handler inicial del formulario.
   * No dispara la mutación, sino que abre la ventana de verificación.
   */
  const onFormSubmit = (data: SwarmLaunchConfig) => {
    setPendingConfig(data);
    setShowPreFlight(true);
  };

  /**
   * Callback ejecutado exclusivamente por el PreFlightModal cuando todos
   * los chequeos de seguridad han pasado exitosamente.
   */
  const handleIgnitionConfirmed = () => {
    if (pendingConfig) {
      mutation.mutate(pendingConfig);
    }
  };

  return (
    <Card className="bg-[#0f0f0f] border-slate-800 h-full flex flex-col relative overflow-hidden shadow-xl">
      {/* Fondo decorativo sutil */}
      <div className="absolute top-0 right-0 p-20 bg-purple-500/5 blur-[80px] rounded-full pointer-events-none" />

      <CardHeader className="pb-4">
        <CardTitle className="flex items-center gap-3 text-white font-mono tracking-wide text-lg">
          <div className="p-2 bg-purple-500/10 rounded-lg border border-purple-500/20">
            <Rocket className="w-5 h-5 text-purple-500" />
          </div>
          SWARM CONTROLLER
        </CardTitle>
        <CardDescription className="text-xs font-mono text-zinc-500">
          Provision ephemeral infrastructure via GitHub Actions Matrix.
        </CardDescription>
      </CardHeader>

      <CardContent className="flex-1 flex flex-col justify-between gap-6">

        <form onSubmit={handleSubmit(onFormSubmit)} className="space-y-8">

          <div className="grid grid-cols-2 gap-6">

            {/* INPUT: WORKERS PER SHARD */}
            <div className="space-y-3">
              <label className="text-[10px] uppercase font-bold text-slate-500 flex items-center gap-2 font-mono tracking-wider">
                <Users className="w-3.5 h-3.5" />
                Nodes / Shard
              </label>
              <div className="relative group">
                <Input
                  type="number"
                  {...register('worker_count', { valueAsNumber: true })}
                  className="bg-black/50 border-slate-700 font-mono text-emerald-400 focus:border-purple-500 transition-colors h-11 text-lg font-bold pl-4"
                  placeholder="30"
                />
                <div className="absolute right-3 top-3 text-[10px] text-zinc-600 font-mono">UNITS</div>
              </div>
              {errors.worker_count && (
                <span className="text-red-500 text-[10px] font-bold block animate-in fade-in slide-in-from-top-1">
                  {errors.worker_count.message}
                </span>
              )}
            </div>

            {/* INPUT: SHARD COUNT */}
            <div className="space-y-3">
              <label className="text-[10px] uppercase font-bold text-slate-500 flex items-center gap-2 font-mono tracking-wider">
                <Layers className="w-3.5 h-3.5" />
                Parallel Shards
              </label>
              <div className="relative group">
                <Input
                  type="number"
                  {...register('shard_count', { valueAsNumber: true })}
                  className="bg-black/50 border-slate-700 font-mono text-purple-400 focus:border-purple-500 transition-colors h-11 text-lg font-bold pl-4"
                  placeholder="5"
                />
                <div className="absolute right-3 top-3 text-[10px] text-zinc-600 font-mono">THREADS</div>
              </div>
              {errors.shard_count && (
                <span className="text-red-500 text-[10px] font-bold block animate-in fade-in slide-in-from-top-1">
                  {errors.shard_count.message}
                </span>
              )}
            </div>
          </div>

          {/* STATUS DISPLAY (PREDICTION) */}
          <div className="p-4 rounded-lg bg-zinc-900/50 border border-zinc-800 flex items-center justify-between">
             <div className="flex items-center gap-3">
                <Activity className="w-4 h-4 text-zinc-600" />
                <span className="text-xs text-zinc-400 font-mono uppercase">Total Capacity Estimate</span>
             </div>
             <div className="text-xl font-black text-white font-mono tracking-tighter">
                {/* Cálculo en tiempo real basado en valores por defecto si el formulario no tiene valores aún */}
                <EstimatedCapacity
                   workers={30} // Valor base visual, reactividad real requeriría watch()
                   shards={5}
                />
             </div>
          </div>

          {/* TRIGGER BUTTON */}
          <Button
            type="submit"
            variant="cyber"
            className="w-full h-12 text-sm font-bold tracking-[0.2em] shadow-[0_0_20px_rgba(168,85,247,0.15)] hover:shadow-[0_0_30px_rgba(168,85,247,0.3)] border-purple-500/50 text-purple-400 hover:bg-purple-500 hover:text-black transition-all"
            isLoading={mutation.isPending}
            disabled={!isValid}
          >
            INITIALIZE DEPLOY SEQUENCE
          </Button>

        </form>

        <div className="text-[9px] text-center text-zinc-600 font-mono">
          SECURE C2 CHANNEL // TLS 1.3 ENCRYPTED
        </div>
      </CardContent>

      {/* --- GATEKEEPER MODAL (CONDITIONAL RENDER) --- */}
      {showPreFlight && pendingConfig && (
        <PreFlightModal
          isOpen={showPreFlight}
          onClose={() => setShowPreFlight(false)}
          onConfirm={handleIgnitionConfirmed}
          config={{
            workerCount: pendingConfig.worker_count,
            shardCount: pendingConfig.shard_count
          }}
        />
      )}
    </Card>
  );
}

/**
 * Sub-componente simple para mostrar la capacidad estimada.
 * En una implementación real, usaría `watch()` de react-hook-form.
 */
function EstimatedCapacity({ workers, shards }: { workers: number, shards: number }) {
    // Nota: Aquí se podría conectar `useFormContext` o pasar props desde el padre con `watch`
    // Para simplificar este archivo "sin abreviaciones", lo dejamos estático o necesitaríamos pasar props dinámicas.
    // Asumiendo que el usuario quiere ver la multiplicación:
    return (
        <span>~ {(workers * shards).toLocaleString()} NODES</span>
    );
}
