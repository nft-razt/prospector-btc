import { AdminGuard } from '../../components/admin-guard';
import { IdentityVault } from '../../components/identity-vault';
import { Activity, Server } from 'lucide-react';

export default function AdminPage() {
  return (
    <AdminGuard>
      <main className="min-h-screen bg-[#050505] text-slate-200 p-6 md:p-10 font-mono selection:bg-emerald-500/30">

        {/* HEADER */}
        <header className="mb-12 border-b border-slate-900 pb-6 flex justify-between items-end">
          <div>
            <h1 className="text-3xl font-black text-white tracking-tighter flex items-center gap-3">
              <span className="text-red-500">///</span> COMMAND CENTER
            </h1>
            <p className="text-slate-600 text-xs mt-1 uppercase tracking-[0.2em]">
              Prospector System Internal Administration
            </p>
          </div>
          <div className="flex gap-4">
             <div className="flex items-center gap-2 text-xs text-slate-500 px-3 py-1 bg-slate-900/50 rounded border border-slate-800">
                <Server className="w-3 h-3" />
                <span>ORCHESTRATOR: ONLINE</span>
             </div>
             <div className="flex items-center gap-2 text-xs text-emerald-500/50 px-3 py-1 bg-emerald-950/10 rounded border border-emerald-900/20">
                <Activity className="w-3 h-3" />
                <span>SYSTEM SECURE</span>
             </div>
          </div>
        </header>

        {/* CONTENT */}
        <div className="max-w-6xl mx-auto space-y-12">

          {/* SECTION 1: IDENTITY */}
          <section>
            <div className="flex items-center gap-4 mb-6">
              <div className="h-px bg-slate-800 flex-1" />
              <h2 className="text-sm font-bold text-slate-500 uppercase tracking-widest">
                Identity Management
              </h2>
              <div className="h-px bg-slate-800 flex-1" />
            </div>

            <IdentityVault />
          </section>

          {/* SECTION 2: FLEET CONTROL (Placeholder para futuro) */}
          <section className="opacity-50 pointer-events-none grayscale">
            <div className="flex items-center gap-4 mb-6">
              <div className="h-px bg-slate-800 flex-1" />
              <h2 className="text-sm font-bold text-slate-700 uppercase tracking-widest">
                Fleet Override (Coming Soon)
              </h2>
              <div className="h-px bg-slate-800 flex-1" />
            </div>

            <div className="grid grid-cols-4 gap-4">
                <div className="bg-slate-900/50 border border-slate-800 h-32 rounded-xl flex items-center justify-center text-xs text-slate-700">
                    EMERGENCY SHUTDOWN
                </div>
                <div className="bg-slate-900/50 border border-slate-800 h-32 rounded-xl flex items-center justify-center text-xs text-slate-700">
                    FORCE RESPAWN
                </div>
            </div>
          </section>

        </div>
      </main>
    </AdminGuard>
  );
}
