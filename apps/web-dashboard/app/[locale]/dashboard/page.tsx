/**
 * =================================================================
 * APARATO: MISSION CONTROL DASHBOARD (V56.0 - TYPE SECURED)
 * CLASIFICACIÓN: VIEW LAYER ORGANISM (L5)
 * RESPONSABILIDAD: ORQUESTACIÓN DE TELEMETRÍA Y COMANDO ESTRATÉGICO
 *
 * ESTRATEGIA DE ÉLITE:
 * - Namespace Avoidance: Uso de importaciones granulares para evitar TS2833.
 * - Explicit JSX Return: Garantiza compatibilidad con React 19.
 * - Zero-Abbreviations: Cumplimiento de nomenclatura Prospector-V8.5.
 * =================================================================
 */

import React from "react"; // Importación por defecto necesaria para React 19
import { useTranslations } from "next-intl";
import { Zap, Activity, ShieldCheck, Rocket, Eye, Database, Terminal } from "lucide-react";

// --- ESTRATO DE COMPONENTES ---
import { SystemMonitor } from "@/components/features/monitoring/system-monitor";
import { SwarmLauncher } from "@/components/features/control/swarm-launcher";
import { IdentityVault } from "@/components/features/identity/identity-vault";
import { AuditTrailHUD } from "@/components/features/monitoring/audit-trail-hud";
import { FleetGrid } from "@/components/features/network/fleet-grid";
import { AdSlot } from "@/components/ui/marketing/ad-slot";

/**
 * Puesto de Mando Central del sistema Prospector.
 *
 * @returns {JSX.Element} Interfaz de control nivelada.
 */
export default function DashboardMissionControlPage(): React.ReactElement {
  const t = useTranslations("Dashboard");

  return (
    <div className="space-y-10 animate-in fade-in slide-in-from-bottom-4 duration-1000 pb-24">

      <div className="w-full opacity-60 hover:opacity-100 transition-opacity duration-500">
        <AdSlot />
      </div>

      <header className="flex flex-col gap-2 border-l-4 border-primary pl-6 py-2">
        <h1 className="text-4xl font-black text-white tracking-tighter uppercase font-mono flex items-center gap-4">
          <Zap className="w-10 h-10 text-primary animate-pulse" />
          {t("sidebar.overview")}
        </h1>
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2">
            <span className="h-2 w-2 rounded-full bg-emerald-500 animate-ping" />
            <p className="text-zinc-500 text-xs font-mono tracking-[0.3em] uppercase">
              Protocol_V8.5 // Active_Session_Authorized
            </p>
          </div>
        </div>
      </header>

      <div className="grid grid-cols-1 xl:grid-cols-12 gap-10 items-start">
        <div className="xl:col-span-8 space-y-12">
          <section className="space-y-6">
            <div className="flex items-center gap-3 pb-2 border-b border-zinc-800">
              <Activity className="w-5 h-5 text-amber-500" />
              <h2 className="text-sm font-black text-zinc-400 uppercase tracking-widest font-mono">
                Real-Time Swarm Telemetry
              </h2>
            </div>
            <SystemMonitor />
          </section>

          <section className="space-y-6">
            <div className="flex items-center gap-3 pb-2 border-b border-zinc-800">
              <Database className="w-5 h-5 text-blue-500" />
              <h2 className="text-sm font-black text-zinc-400 uppercase tracking-widest font-mono">
                Immutable Audit Trail // Stratum L4
              </h2>
            </div>
            <div className="h-[600px]">
              <AuditTrailHUD />
            </div>
          </section>

          <section className="space-y-6">
            <div className="flex items-center gap-3 pb-2 border-b border-zinc-800">
              <Eye className="w-5 h-5 text-purple-500" />
              <h2 className="text-sm font-black text-zinc-400 uppercase tracking-widest font-mono">
                Visual Grid Surveillance
              </h2>
            </div>
            <FleetGrid />
          </section>
        </div>

        <aside className="xl:col-span-4 space-y-12 sticky top-24">
          <section className="space-y-6">
            <div className="flex items-center gap-3 pb-2 border-b border-zinc-800">
              <Rocket className="w-5 h-5 text-primary" />
              <h2 className="text-sm font-black text-zinc-400 uppercase tracking-widest font-mono">
                C2 Deployment Center
              </h2>
            </div>
            <SwarmLauncher />
          </section>

          <section className="space-y-6">
            <div className="flex items-center gap-3 pb-2 border-b border-zinc-800">
              <ShieldCheck className="w-5 h-5 text-emerald-500" />
              <h2 className="text-sm font-black text-zinc-400 uppercase tracking-widest font-mono">
                ZK Identity Vault
              </h2>
            </div>
            <IdentityVault />
          </section>

          <section className="bg-black/80 border border-zinc-800 rounded-xl p-6 font-mono">
            <div className="flex items-center gap-3 mb-4">
              <Terminal className="w-4 h-4 text-zinc-500" />
              <span className="text-[10px] text-zinc-500 uppercase font-bold">System_Kernel_Output</span>
            </div>
            <div className="space-y-2">
              <p className="text-[9px] text-emerald-500/70">{"[OK] Handshake L3 established."}</p>
              <p className="text-[9px] text-blue-500/70">{"[INFO] Audit Trail synchronized."}</p>
            </div>
          </section>
        </aside>
      </div>

      <footer className="pt-12 border-t border-white/5 flex justify-between items-center opacity-40 hover:opacity-100 transition-opacity">
        <span className="text-[8px] font-black text-zinc-500 font-mono uppercase tracking-[0.3em]">
          Prospector OS // Academic Release 2025
        </span>
      </footer>
    </div>
  );
}
