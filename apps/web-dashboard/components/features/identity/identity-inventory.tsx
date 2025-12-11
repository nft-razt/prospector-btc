'use client';

import { useQuery } from '@tanstack/react-query';
import { RefreshCw, Clock, Server, AlertTriangle, Trash2 } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';

import { apiClient } from '@prospector/api-client';
import { useHeimdall } from '@/hooks/use-heimdall';
import { Skeleton } from '@/components/ui/kit/skeleton'; // Nuestro nuevo átomo
import { Card } from '@/components/ui/kit/card';
import { cn } from '@/lib/utils/cn';

// Definición local del tipo para renderizado (debería venir de api-client idealmente)
interface IdentityItem {
  id: string;
  platform: string;
  email: string;
  usage_count: number;
  last_used_at: string | null;
  status: 'active' | 'expired' | 'revoked' | 'ratelimited';
}

export function IdentityInventory() {
  useHeimdall('IdentityInventory');

  const { data: identities, isLoading, isError, refetch } = useQuery({
    queryKey: ['identities'],
    queryFn: async () => {
      const res = await apiClient.get<IdentityItem[]>('/admin/identities');
      return res.data;
    },
    refetchInterval: 10000, // Refresh cada 10s
  });

  if (isLoading) return <InventorySkeleton />;

  if (isError) return (
    <div className="p-6 border border-destructive/20 bg-destructive/10 rounded-xl text-center text-destructive text-sm font-mono flex flex-col items-center gap-2">
      <AlertTriangle className="w-6 h-6" />
      <span>CONNECTION TO VAULT FAILED</span>
      <button onClick={() => refetch()} className="underline opacity-80 hover:opacity-100">Retry Link</button>
    </div>
  );

  return (
    <div className="flex flex-col h-full bg-[#0f0f0f] border border-slate-800 rounded-xl overflow-hidden shadow-xl">
        {/* Header */}
        <div className="p-4 border-b border-slate-800 bg-slate-900/30 flex justify-between items-center backdrop-blur-sm">
            <h3 className="text-xs font-bold text-slate-300 uppercase tracking-widest flex items-center gap-2">
                <Server className="w-3.5 h-3.5 text-emerald-500" />
                Active Personas
            </h3>
            <span className="text-[10px] font-mono bg-slate-800 text-slate-400 px-2 py-0.5 rounded border border-slate-700">
                TOTAL: {identities?.length || 0}
            </span>
        </div>

        {/* List Canvas */}
        <div className="flex-1 overflow-y-auto max-h-[600px] scrollbar-thin scrollbar-thumb-slate-800 p-3 space-y-3">
            {identities?.length === 0 && (
                <div className="h-full flex flex-col items-center justify-center text-slate-600 space-y-4 py-12">
                    <Trash2 className="w-8 h-8 opacity-20" />
                    <p className="text-xs italic font-mono text-center max-w-[200px]">
                        Vault is empty. Inject credentials to initialize operations.
                    </p>
                </div>
            )}

            {identities?.map((id) => (
                <IdentityCard key={id.id} identity={id} />
            ))}
        </div>
    </div>
  );
}

// --- SUB-COMPONENTES DE UI (Local Atoms) ---

function IdentityCard({ identity }: { identity: IdentityItem }) {
  const statusColors = {
    active: 'bg-emerald-500 shadow-[0_0_8px_#10b981]',
    ratelimited: 'bg-amber-500 shadow-[0_0_8px_#f59e0b]',
    expired: 'bg-red-500 shadow-[0_0_8px_#ef4444]',
    revoked: 'bg-slate-500',
  };

  return (
    <div className="bg-black/40 border border-slate-800 p-3 rounded-lg hover:border-emerald-500/30 hover:bg-slate-900/20 transition-all group">
        <div className="flex justify-between items-start mb-2">
            <div className="flex items-center gap-2.5 overflow-hidden">
                <div className={cn("w-1.5 h-1.5 rounded-full", statusColors[identity.status])} />
                <span className="font-mono text-xs text-zinc-200 font-bold truncate tracking-tight">
                    {identity.email}
                </span>
            </div>
            <span className={cn(
              "text-[9px] px-1.5 py-0.5 rounded uppercase font-bold tracking-wider",
              identity.status === 'active' ? 'bg-emerald-950/50 text-emerald-500' : 'bg-slate-900 text-slate-500'
            )}>
                {identity.platform.replace('_', ' ')}
            </span>
        </div>

        <div className="grid grid-cols-2 gap-2 text-[10px] text-slate-500 font-mono mt-3 pt-2 border-t border-slate-800/50">
            <div className="flex items-center gap-1.5">
                <RefreshCw className="w-3 h-3 text-slate-600" />
                <span className="text-emerald-400 group-hover:text-emerald-300 transition-colors">
                    {identity.usage_count}
                </span>
                <span className="opacity-50">leases</span>
            </div>
            <div className="flex items-center gap-1.5 justify-end">
                <Clock className="w-3 h-3 text-slate-600" />
                <span className="opacity-70">
                    {identity.last_used_at
                      ? formatDistanceToNow(new Date(identity.last_used_at), { addSuffix: true })
                      : 'Never'}
                </span>
            </div>
        </div>
    </div>
  );
}

function InventorySkeleton() {
  return (
    <div className="h-[400px] bg-[#0f0f0f] border border-slate-800 rounded-xl p-4 space-y-4">
      <div className="flex justify-between items-center mb-6">
        <Skeleton className="h-4 w-32 bg-slate-800" />
        <Skeleton className="h-4 w-10 bg-slate-800" />
      </div>
      {[...Array(3)].map((_, i) => (
        <Skeleton key={i} className="h-24 w-full bg-slate-900/50 rounded-lg" />
      ))}
    </div>
  );
}
