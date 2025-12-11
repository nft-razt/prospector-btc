'use client';

import { useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import { adminApi, type WorkerSnapshot } from '@prospector/api-client';
import { useTranslations } from 'next-intl';
import { Eye, Clock, SignalHigh, WifiOff, Monitor } from 'lucide-react';

import { cn } from '@/lib/utils/cn';
import { useHeimdall } from '@/hooks/use-heimdall';
import { Skeleton } from '@/components/ui/kit/skeleton';

export function FleetGrid() {
  const t = useTranslations('Dashboard.fleet');
  const logger = useHeimdall('FleetGrid');

  // 🔥 TIPADO EXPLÍCITO <WorkerSnapshot[]>
  // Esto le dice a TS: "Espera un array, aunque la API falle al inferirlo"
  const { data: snapshots, isLoading, isError, error } = useQuery<WorkerSnapshot[]>({
    queryKey: ['worker-snapshots'],
    queryFn: adminApi.getWorkerSnapshots,
    refetchInterval: 3000,
    retry: 1,
  });

  useEffect(() => {
    if (isError) {
      logger.error('Pérdida de señal con el enjambre visual', { error: error?.message });
    }
  }, [isError, error, logger]);

  // --- ESTADO: CARGANDO ---
  if (isLoading) {
    return (
      <div className="space-y-4">
        <div className="flex justify-between items-end pb-2 border-b border-zinc-800">
           <Skeleton className="h-5 w-40 bg-zinc-800" />
           <Skeleton className="h-5 w-20 bg-zinc-800" />
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {[...Array(4)].map((_, i) => (
            <Skeleton key={i} className="aspect-video rounded-lg bg-zinc-900/50 border border-zinc-800" />
          ))}
        </div>
      </div>
    );
  }

  // --- ESTADO: ERROR / VACÍO ---
  // Validación defensiva: (!snapshots) cubre null/undefined
  if (isError || !snapshots || snapshots.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center p-12 border border-dashed border-zinc-800 rounded-xl bg-zinc-900/10 text-zinc-500 animate-in fade-in zoom-in-95 duration-300">
        {isError ? (
          <WifiOff className="w-12 h-12 mb-4 text-red-500/50" />
        ) : (
          <Monitor className="w-12 h-12 mb-4 opacity-30" />
        )}
        <p className="text-sm font-mono tracking-tight uppercase font-bold text-zinc-400">
          {isError ? t('connection_lost') : t('no_signal')}
        </p>
        <p className="text-xs mt-2 text-zinc-600 max-w-xs text-center">
          {isError ? t('check_orchestrator') : t('deploy_hint')}
        </p>
      </div>
    );
  }

  // --- ESTADO: OPERATIVO ---
  return (
    <div className="space-y-4">
      <div className="flex justify-between items-end pb-2 border-b border-zinc-800">
        <h3 className="text-sm font-bold text-zinc-100 uppercase tracking-widest flex items-center gap-2">
          <Eye className="w-4 h-4 text-emerald-500" />
          {t('title')} <span className="text-zinc-500 font-mono">[{snapshots.length}]</span>
        </h3>
        <span className="text-[10px] text-emerald-500 font-mono flex items-center gap-1 bg-emerald-950/30 px-2 py-1 rounded border border-emerald-900/50 shadow-[0_0_10px_rgba(16,185,129,0.1)]">
          <SignalHigh className="w-3 h-3 animate-pulse" />
          {t('live_feed')}
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {snapshots.map((snap) => (
          <FleetNodeCard key={snap.worker_id} snap={snap} />
        ))}
      </div>
    </div>
  );
}

// --- ATOMIC NODE CARD ---
function FleetNodeCard({ snap }: { snap: WorkerSnapshot }) {
  // ... (El resto del componente FleetNodeCard se mantiene igual)
  const getStatusStyle = (s: string) => {
    switch (s) {
      case 'running': return 'text-emerald-500 border-emerald-500/20 bg-emerald-500/10';
      case 'captcha': return 'text-red-500 border-red-500/50 bg-red-500/10 animate-pulse';
      case 'error': return 'text-amber-500 border-amber-500/20 bg-amber-500/10';
      default: return 'text-slate-500 border-slate-500/20 bg-slate-500/10';
    }
  };

  return (
    <div className={cn(
      "relative group overflow-hidden rounded-lg border bg-black transition-all duration-300 hover:shadow-2xl hover:scale-[1.02]",
      snap.status === 'captcha' ? 'border-red-500/50 shadow-[0_0_20px_rgba(239,68,68,0.2)]' : 'border-zinc-800 hover:border-emerald-500/30'
    )}>
      {/* HUD Overlay */}
      <div className="absolute top-0 left-0 w-full bg-gradient-to-b from-black/90 to-transparent p-3 flex justify-between items-start z-10 pointer-events-none">
        <div className="flex flex-col">
          <span className="text-[10px] font-mono text-zinc-300 font-bold truncate w-24 drop-shadow-md">
            {snap.worker_id}
          </span>
          <span className="text-[8px] text-zinc-500 uppercase font-bold tracking-wider">
            RELAY_01
          </span>
        </div>
        <div className={cn("px-2 py-0.5 rounded text-[8px] font-black uppercase tracking-wider backdrop-blur-md border", getStatusStyle(snap.status))}>
          {snap.status}
        </div>
      </div>

      {/* Image Feed */}
      <div className="aspect-video bg-zinc-900 relative">
        {/* eslint-disable-next-line @next/next/no-img-element */}
        <img
          src={snap.snapshot_base64}
          alt={`Feed ${snap.worker_id}`}
          className="w-full h-full object-cover opacity-70 group-hover:opacity-100 transition-opacity duration-500"
        />

        {/* Timestamp */}
        <div className="absolute bottom-2 right-2 flex items-center gap-1 bg-black/80 backdrop-blur px-2 py-1 rounded text-[9px] text-zinc-400 font-mono border border-zinc-800">
          <Clock className="w-3 h-3" />
          {new Date(snap.timestamp).toLocaleTimeString()}
        </div>
      </div>

      {/* CRT Effect Overlay */}
      <div className="absolute inset-0 bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.25)_50%),linear-gradient(90deg,rgba(255,0,0,0.06),rgba(0,255,0,0.02),rgba(0,0,255,0.06))] z-20 bg-[length:100%_2px,3px_100%] pointer-events-none opacity-20" />
    </div>
  );
}
