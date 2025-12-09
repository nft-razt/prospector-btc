import { AdminGuard } from '../../components/admin-guard';
import { IdentityVault } from '../../components/identity-vault';
import { FleetGrid } from '../../components/fleet-grid';
import { AdSlot } from '../../components/ad-slot';
import { Server, ShieldCheck } from 'lucide-react';

export default function AdminPage() {
  return (
    <AdminGuard>
      <main className="min-h-screen bg-black text-zinc-200 font-sans selection:bg-emerald-500/30">

        {/* 1. MONETIZATION LAYER */}
        <AdSlot />

        <div className="p-6 md:p-10 max-w-7xl mx-auto space-y-12">

          {/* 2. HEADER */}
          <header className="flex flex-col md:flex-row justify-between items-end border-b border-zinc-800 pb-8">
            <div>
              <h1 className="text-3xl font-bold tracking-tight text-white flex items-center gap-3">
                <span className="text-emerald-500 text-4xl">●</span> COMMAND CENTER
              </h1>
              <p className="text-zinc-500 text-sm mt-2">
                Hydra-Zero Orchestration & Surveillance Interface
              </p>
            </div>

            <div className="flex gap-3">
               <StatusBadge label="ORCHESTRATOR" active={true} icon={<Server className="w-3 h-3"/>} />
               <StatusBadge label="VAULT SECURE" active={true} icon={<ShieldCheck className="w-3 h-3"/>} />
            </div>
          </header>

          {/* 3. IDENTITY SECTION */}
          <section>
            <div className="mb-6">
              <h2 className="text-lg font-semibold text-white">Identity Provisioning</h2>
              <p className="text-xs text-zinc-500">Manage Google credentials for worker injection.</p>
            </div>
            <IdentityVault />
          </section>

          {/* 4. SURVEILLANCE SECTION (PANÓPTICO) */}
          <section>
            <FleetGrid />
          </section>

        </div>
      </main>
    </AdminGuard>
  );
}

// Sub-componente simple para el header
function StatusBadge({ label, active, icon }: { label: string, active: boolean, icon: any }) {
    return (
        <div className={`
            flex items-center gap-2 text-[10px] font-bold px-3 py-1.5 rounded-full border
            ${active
                ? 'bg-emerald-950/30 border-emerald-900 text-emerald-500'
                : 'bg-zinc-900 border-zinc-800 text-zinc-500'}
        `}>
            {icon}
            {label}
        </div>
    );
}
