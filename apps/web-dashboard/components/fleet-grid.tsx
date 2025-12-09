// apps/web-dashboard/components/fleet-grid.tsx
'use client';

import { useQuery } from '@tanstack/react-query';
import { adminApi } from '@prospector/api-client';
import { Activity, AlertTriangle, Eye, Monitor, Clock } from 'lucide-react';

export function FleetGrid() {
  // 1. Polling de alta frecuencia (3s) para video casi en vivo
  const { data: snapshots, isLoading } = useQuery({
    queryKey: ['worker-snapshots'],
    queryFn: adminApi.getWorkerSnapshots,
    refetchInterval: 3000,
  });

  if (isLoading) {
    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 animate-pulse">
        {[...Array(4)].map((_, i) => (
          <div key={i} className="aspect-video bg-zinc-900 rounded-lg border border-zinc-800" />
        ))}
      </div>
    );
  }

  if (!snapshots || snapshots.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center p-12 border border-dashed border-zinc-800 rounded-xl bg-zinc-900/20 text-zinc-500">
        <Monitor className="w-10 h-10 mb-4 opacity-50" />
        <p className="text-sm font-mono tracking-tight">NO VISUAL FEED DETECTED</p>
        <p className="text-xs mt-2">Deploy workers via Provisioner to establish uplink.</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {/* Header de la Sección */}
      <div className="flex justify-between items-end pb-2 border-b border-zinc-800">
        <h3 className="text-sm font-bold text-zinc-100 uppercase tracking-widest flex items-center gap-2">
          <Eye className="w-4 h-4 text-emerald-500" />
          Visual Surveillance ({snapshots.length} Nodes)
        </h3>
        <span className="text-[10px] text-zinc-500 font-mono flex items-center gap-1">
          <div className="w-2 h-2 bg-emerald-500 rounded-full animate-pulse" />
          LIVE FEED
        </span>
      </div>

      {/* Grilla de Cámaras */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {snapshots.map((snap) => (
          <div
            key={snap.worker_id}
            className={`
              relative group overflow-hidden rounded-lg border bg-black transition-all duration-300
              ${getStatusBorder(snap.status)}
            `}
          >
            {/* Status Bar (Top Overlay) */}
            <div className="absolute top-0 left-0 w-full bg-gradient-to-b from-black/90 to-transparent p-3 flex justify-between items-start z-10">
              <div className="flex flex-col">
                <span className="text-[10px] font-mono text-zinc-300 font-bold truncate w-24">
                  {snap.worker_id}
                </span>
                <span className="text-[8px] text-zinc-500 uppercase">{snap.status}</span>
              </div>

              <div className={`p-1.5 rounded-md ${getStatusBadge(snap.status)}`}>
                 {getIcon(snap.status)}
              </div>
            </div>

            {/* Screen Feed */}
            <div className="aspect-video bg-zinc-900 relative">
              <img
                src={snap.snapshot_base64}
                alt={`Feed ${snap.worker_id}`}
                className="w-full h-full object-cover opacity-70 group-hover:opacity-100 transition-opacity duration-300 grayscale group-hover:grayscale-0"
              />

              {/* Timestamp Overlay (Bottom) */}
              <div className="absolute bottom-2 right-2 flex items-center gap-1 bg-black/80 backdrop-blur px-2 py-1 rounded text-[9px] text-zinc-400 font-mono border border-zinc-800">
                  <Clock className="w-3 h-3" />
                  {new Date(snap.timestamp).toLocaleTimeString()}
              </div>
            </div>

            {/* Scanline Effect (CSS trick for cyber feel) */}
            <div className="absolute inset-0 bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.25)_50%),linear-gradient(90deg,rgba(255,0,0,0.06),rgba(0,255,0,0.02),rgba(0,0,255,0.06))] z-[5] bg-[length:100%_2px,3px_100%] pointer-events-none opacity-20" />
          </div>
        ))}
      </div>
    </div>
  );
}

// --- Helpers de Estilo ---

function getStatusBorder(status: string) {
    switch (status) {
        case 'captcha': return 'border-red-500/50 shadow-[0_0_15px_rgba(239,68,68,0.2)]';
        case 'error': return 'border-amber-500/50';
        default: return 'border-zinc-800 hover:border-zinc-600';
    }
}

function getStatusBadge(status: string) {
    switch (status) {
        case 'running': return 'bg-emerald-500/10 text-emerald-500';
        case 'captcha': return 'bg-red-500 text-white animate-bounce';
        case 'error': return 'bg-amber-500/10 text-amber-500';
        default: return 'bg-zinc-800 text-zinc-500';
    }
}

function getIcon(status: string) {
    switch (status) {
        case 'running': return <Activity className="w-3 h-3" />;
        case 'captcha': return <Eye className="w-3 h-3" />;
        case 'error': return <AlertTriangle className="w-3 h-3" />;
        default: return <Monitor className="w-3 h-3" />;
    }
}
