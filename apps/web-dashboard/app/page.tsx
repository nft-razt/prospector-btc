'use client'; // Directiva vital: Este componente maneja estado y hooks del navegador

import React, { useMemo } from 'react';
import { useSystemStatus } from '@prospector/api-client'; // Nuestro aparato de Infraestructura
import {
  Activity,
  Cpu,
  Database,
  Search,
  AlertCircle,
  Wifi,
  WifiOff,
  Server
} from 'lucide-react';

// =================================================================
// UTILIDADES LOCALES (Formato Científico)
// =================================================================

/**
 * Convierte un entero raw en una unidad legible por humanos (MH/s, GH/s).
 */
const formatHashrate = (hashrate: number): string => {
  if (hashrate === 0) return '0 H/s';
  const k = 1000;
  const sizes = ['H/s', 'KH/s', 'MH/s', 'GH/s', 'TH/s'];
  const i = Math.floor(Math.log(hashrate) / Math.log(k));
  return `${parseFloat((hashrate / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
};

/**
 * Formatea fechas para logs de precisión.
 */
const formatTime = (isoString: string) => {
  try {
    return new Date(isoString).toLocaleTimeString('en-US', {
      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit'
    });
  } catch (e) {
    return '--:--:--';
  }
};

// =================================================================
// COMPONENTE PRINCIPAL
// =================================================================

export default function DashboardPage() {
  // 1. SINAPSIS DE DATOS (TanStack Query)
  // Polling agresivo (2s) para sensación de tiempo real.
  const { data: workers, isLoading, isError, error } = useSystemStatus();

  // 2. CÁLCULOS DERIVADOS (Memoized para performance)
  const metrics = useMemo(() => {
    if (!workers) return { activeNodes: 0, totalHashrate: 0 };

    return {
      activeNodes: workers.length,
      totalHashrate: workers.reduce((acc, w) => acc + w.hashrate, 0),
    };
  }, [workers]);

  // 3. ESTADO DEL SISTEMA (Semáforo)
  const systemState = isError
    ? 'critical'
    : isLoading
      ? 'connecting'
      : metrics.activeNodes > 0
        ? 'operational'
        : 'idle';

  return (
    <main className="min-h-screen bg-[#0a0a0a] text-slate-200 p-6 md:p-10 font-mono selection:bg-emerald-500/30">

      {/* --- HEADER: MISSION CONTROL --- */}
      <header className="flex flex-col md:flex-row justify-between items-start md:items-center mb-10 border-b border-slate-800 pb-6 gap-4">
        <div>
          <h1 className="text-3xl md:text-4xl font-black tracking-tighter text-white flex items-center gap-3">
            <span className="text-emerald-500">///</span> PROSPECTOR
          </h1>
          <p className="text-slate-500 text-sm mt-1 uppercase tracking-widest">
            Distributed Entropy Audit System <span className="text-emerald-500/50">//</span> v1.0.0-RC1
          </p>
        </div>

        {/* Status Badge */}
        <div className={`
          flex items-center gap-3 px-4 py-2 rounded-full border backdrop-blur-md transition-all
          ${systemState === 'operational' ? 'bg-emerald-950/30 border-emerald-500/30 text-emerald-400' : ''}
          ${systemState === 'critical' ? 'bg-red-950/30 border-red-500/30 text-red-400' : ''}
          ${systemState === 'connecting' ? 'bg-blue-950/30 border-blue-500/30 text-blue-400' : ''}
          ${systemState === 'idle' ? 'bg-slate-800/50 border-slate-700 text-slate-400' : ''}
        `}>
          {systemState === 'operational' && <Wifi className="w-4 h-4 animate-pulse" />}
          {systemState === 'critical' && <WifiOff className="w-4 h-4" />}
          {systemState === 'connecting' && <Activity className="w-4 h-4 animate-spin" />}
          {systemState === 'idle' && <Server className="w-4 h-4" />}

          <span className="text-xs font-bold uppercase tracking-wider">
            {systemState === 'operational' && 'SYSTEM ONLINE'}
            {systemState === 'critical' && 'CONNECTION LOST'}
            {systemState === 'connecting' && 'ESTABLISHING UPLINK...'}
            {systemState === 'idle' && 'WAITING FOR WORKERS'}
          </span>
        </div>
      </header>

      {/* --- METRICS GRID (HUD) --- */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <MetricCard
          icon={<Cpu className="w-5 h-5" />}
          label="Active Drones"
          value={metrics.activeNodes}
          subValue="Nodes reporting"
          color="blue"
          loading={isLoading}
        />
        <MetricCard
          icon={<Activity className="w-5 h-5" />}
          label="Global Hashrate"
          value={formatHashrate(metrics.totalHashrate)}
          subValue="Combined power"
          color="emerald"
          loading={isLoading}
        />
        <MetricCard
          icon={<Database className="w-5 h-5" />}
          label="Target Scope"
          value="52,403,201"
          subValue="UTXOs Indexed"
          color="purple"
          loading={false} // Static for now
        />
        <MetricCard
          icon={<Search className="w-5 h-5" />}
          label="Collisions"
          value="0"
          subValue="Confirmed Keys"
          color="amber"
          loading={false}
        />
      </div>

      {/* --- WORKER TELEMETRY TABLE --- */}
      <section className="bg-[#0f0f0f] border border-slate-800 rounded-xl overflow-hidden shadow-2xl">
        <div className="px-6 py-4 border-b border-slate-800 bg-slate-900/50 flex justify-between items-center">
          <h2 className="text-sm font-bold uppercase tracking-wider text-slate-400 flex items-center gap-2">
            <Server className="w-4 h-4" /> Swarm Telemetry
          </h2>
          <span className="text-xs text-slate-600">Auto-refresh: 2000ms</span>
        </div>

        {isError ? (
           <div className="p-12 text-center text-red-500 bg-red-950/10">
             <AlertCircle className="w-12 h-12 mx-auto mb-4 opacity-50" />
             <h3 className="text-lg font-bold">Orchestrator Unreachable</h3>
             <p className="text-sm opacity-70 mt-2 font-mono">
               {(error as any)?.message || "Connection refused by target machine"}
             </p>
           </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full text-left text-sm">
              <thead>
                <tr className="border-b border-slate-800 text-xs uppercase text-slate-500 bg-slate-900/20">
                  <th className="px-6 py-4 font-medium">Worker ID / Host</th>
                  <th className="px-6 py-4 font-medium">Hashrate</th>
                  <th className="px-6 py-4 font-medium">Current Job</th>
                  <th className="px-6 py-4 font-medium text-right">Last Heartbeat</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-800/50">
                {isLoading ? (
                  // SKELETON LOADING ROWS
                  [...Array(3)].map((_, i) => (
                    <tr key={i} className="animate-pulse">
                      <td className="px-6 py-4"><div className="h-4 w-32 bg-slate-800 rounded"></div></td>
                      <td className="px-6 py-4"><div className="h-4 w-20 bg-slate-800 rounded"></div></td>
                      <td className="px-6 py-4"><div className="h-4 w-24 bg-slate-800 rounded"></div></td>
                      <td className="px-6 py-4"><div className="h-4 w-16 bg-slate-800 rounded ml-auto"></div></td>
                    </tr>
                  ))
                ) : metrics.activeNodes === 0 ? (
                  // EMPTY STATE
                  <tr>
                    <td colSpan={4} className="px-6 py-12 text-center text-slate-600">
                      <div className="flex flex-col items-center gap-2">
                        <WifiOff className="w-8 h-8 opacity-50" />
                        <span>No workers connected to the hive.</span>
                      </div>
                    </td>
                  </tr>
                ) : (
                  // DATA ROWS
                  workers?.map((worker) => (
                    <tr key={worker.worker_id} className="group hover:bg-emerald-900/5 transition-colors">
                      <td className="px-6 py-4">
                        <div className="font-mono text-emerald-400 font-bold">
                          {worker.worker_id.split('-')[0]}...
                        </div>
                        <div className="text-xs text-slate-500">{worker.hostname}</div>
                      </td>
                      <td className="px-6 py-4 font-mono text-slate-200">
                        {formatHashrate(worker.hashrate)}
                      </td>
                      <td className="px-6 py-4">
                        <span className="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-slate-800 text-slate-300 border border-slate-700">
                          {worker.current_job_id ? 'MINING' : 'IDLE'}
                        </span>
                      </td>
                      <td className="px-6 py-4 text-right font-mono text-slate-400">
                        {formatTime(worker.timestamp)}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        )}
      </section>
    </main>
  );
}

// =================================================================
// SUB-COMPONENTE: METRIC CARD (Atomic UI)
// =================================================================

interface MetricCardProps {
  icon: React.ReactNode;
  label: string;
  value: string | number;
  subValue: string;
  color: 'blue' | 'emerald' | 'purple' | 'amber';
  loading: boolean;
}

function MetricCard({ icon, label, value, subValue, color, loading }: MetricCardProps) {
  // Mapeo de colores semánticos
  const colors = {
    blue: 'text-blue-500 bg-blue-500/10 border-blue-500/20',
    emerald: 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20',
    purple: 'text-purple-500 bg-purple-500/10 border-purple-500/20',
    amber: 'text-amber-500 bg-amber-500/10 border-amber-500/20',
  };

  return (
    <div className="bg-[#0f0f0f] border border-slate-800 p-6 rounded-xl relative overflow-hidden group hover:border-slate-700 transition-all">
      {/* Decorative Glow */}
      <div className={`absolute -right-4 -top-4 w-24 h-24 rounded-full blur-3xl opacity-0 group-hover:opacity-10 transition-opacity ${colors[color].split(' ')[0]}`} />

      <div className="flex justify-between items-start mb-4">
        <div className={`p-2 rounded-lg ${colors[color]}`}>
          {icon}
        </div>
      </div>

      <h3 className="text-xs font-bold text-slate-500 uppercase tracking-wider mb-1">
        {label}
      </h3>

      {loading ? (
        <div className="h-8 w-24 bg-slate-800 rounded animate-pulse mb-1" />
      ) : (
        <div className="text-3xl font-black text-slate-100 tracking-tight">
          {value}
        </div>
      )}

      <p className="text-xs text-slate-600 font-mono">
        {subValue}
      </p>
    </div>
  );
}
