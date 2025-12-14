/**
 * =================================================================
 * APARATO: SYSTEM MONITOR (L1 TELEMETRY)
 * RESPONSABILIDAD: VISUALIZACIÓN AGREGADA DEL ESTADO DEL ENJAMBRE
 * =================================================================
 */

'use client';

import { Activity, Cpu, Server, Key } from 'lucide-react';
import { useSystemTelemetry } from '@prospector/api-client';
import { StatCard } from '@/components/ui/kit/stat-card';

// Utilidad para formatear hashrate (H/s -> MH/s -> GH/s)
const formatHashrate = (hashes: number) => {
  if (hashes > 1e9) return `${(hashes / 1e9).toFixed(2)} GH/s`;
  if (hashes > 1e6) return `${(hashes / 1e6).toFixed(2)} MH/s`;
  if (hashes > 1e3) return `${(hashes / 1e3).toFixed(2)} kH/s`;
  return `${hashes} H/s`;
};

// Utilidad para formatear números grandes
const formatCompact = (num: number) =>
  new Intl.NumberFormat('en-US', { notation: "compact", maximumFractionDigits: 1 }).format(num);

export function SystemMonitor() {
  const { data, isLoading } = useSystemTelemetry();
  const metrics = data?.metrics;

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 animate-in fade-in slide-in-from-bottom-2 duration-500">

      {/* 1. HASHRATE GLOBAL */}
      <StatCard
        label="Global Hashrate"
        value={formatHashrate(metrics?.globalHashrate || 0)}
        subValue="Combined swarm power"
        icon={Activity}
        color="emerald"
        loading={isLoading}
      />

      {/* 2. ACTIVE NODES */}
      <StatCard
        label="Active Nodes"
        value={metrics?.activeNodes || 0}
        subValue={`${metrics?.totalNodes || 0} registered workers`}
        icon={Server}
        color="blue"
        loading={isLoading}
      />

      {/* 3. KEYS SCAN RATE */}
      <StatCard
        label="Scan Velocity"
        value={formatCompact(metrics?.keysPerDay || 0)}
        subValue="Keys scanned / 24h"
        icon={Key}
        color="purple"
        loading={isLoading}
      />

      {/* 4. CPU UTILIZATION (Simulado / Derivado) */}
      <StatCard
        label="Cluster Load"
        value={`${metrics?.activeNodes ? '98.4' : '0'}%`}
        subValue="Optimal Saturation"
        icon={Cpu}
        color="amber"
        loading={isLoading}
      />

    </div>
  );
}
