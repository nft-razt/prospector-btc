/**
 * =================================================================
 * APARATO: DASHBOARD PAGE (MISSION CONTROL)
 * CLASIFICACIÓN: VIEW LAYER (SERVER COMPONENT)
 * RESPONSABILIDAD: COMPOSICIÓN ESTRATÉGICA DE WIDGETS OPERATIVOS
 * ESTADO: GOLD MASTER (V5.0)
 * =================================================================
 */

import { useTranslations } from 'next-intl';
import { Rocket, Shield, Eye, Zap, Activity } from 'lucide-react';

// --- APARATOS DE DOMINIO (Features) ---
import { IdentityVault } from '@/components/features/identity/identity-vault';
import { FleetGrid } from '@/components/features/network/fleet-grid';
import { SwarmLauncher } from '@/components/features/control/swarm-launcher';
import { RunHistory } from '@/components/features/control/run-history';
import { SystemMonitor } from '@/components/features/monitoring/system-monitor';

// --- APARATOS DE UI/MARKETING ---
import { AdSlot } from '@/components/ui/marketing/ad-slot';

/**
 * Página principal del panel de control protegido.
 * Orquesta la interfaz de operaciones tácticas dividida en estratos funcionales:
 * 1. Telemetría L1 (System Monitor)
 * 2. Comando y Control (C2)
 * 3. Gestión de Acceso (IAM)
 * 4. Inteligencia Visual (ISR)
 */
export default function DashboardPage() {
  const t = useTranslations('Dashboard');

  return (
    <div className="space-y-12 animate-in fade-in duration-700 slide-in-from-bottom-4 pb-24">

      {/* 0. ZONA DE MONETIZACIÓN (Discreta) */}
      <div className="w-full opacity-80 hover:opacity-100 transition-opacity duration-500">
         <AdSlot />
      </div>

      {/* HEADER DE CONTEXTO TÁCTICO */}
      <div className="flex flex-col gap-2 border-l-2 border-primary/50 pl-6 py-1">
        <h1 className="text-4xl font-black text-white tracking-tighter uppercase font-mono flex items-center gap-3">
          <Zap className="w-8 h-8 text-primary animate-pulse-slow" />
          {t('sidebar.overview')}
        </h1>
        <div className="flex items-center gap-3">
            <span className="h-1.5 w-1.5 rounded-full bg-emerald-500 animate-pulse" />
            <p className="text-zinc-500 text-xs font-mono tracking-[0.2em] uppercase">
            :: HYDRA-ZERO PROTOCOL :: ACTIVE SESSION
            </p>
        </div>
      </div>

      <div className="grid gap-16">

        {/* -----------------------------------------------------------
            ESTRATO 1: TELEMETRÍA EN TIEMPO REAL (L1)
            Visión global del rendimiento del enjambre.
           ----------------------------------------------------------- */}
        <section className="space-y-6">
            <div className="flex items-center justify-between pb-2 border-b border-zinc-800">
                <h2 className="text-lg font-bold text-amber-500 flex items-center gap-3 font-mono tracking-wider">
                    <Activity className="w-5 h-5" />
                    LIVE TELEMETRY
                </h2>
                <span className="text-[10px] font-mono font-bold bg-amber-950/30 text-amber-400 px-2 py-1 rounded border border-amber-900/50">
                    REAL-TIME
                </span>
            </div>

            <SystemMonitor />
        </section>

        {/* -----------------------------------------------------------
            ESTRATO 2: COMANDO Y CONTROL (C2)
            Despliegue de infraestructura y monitoreo de CI/CD.
           ----------------------------------------------------------- */}
        <section className="space-y-6">
            <div className="flex items-center justify-between pb-2 border-b border-zinc-800">
                <h2 className="text-lg font-bold text-purple-500 flex items-center gap-3 font-mono tracking-wider">
                    <Rocket className="w-5 h-5" />
                    COMMAND & CONTROL
                </h2>
                <span className="text-[10px] font-mono font-bold bg-purple-950/30 text-purple-400 px-2 py-1 rounded border border-purple-900/50">
                    INFRA OPS
                </span>
            </div>

            <div className="grid grid-cols-1 xl:grid-cols-3 gap-6 h-full items-stretch">
                {/* Panel de Lanzamiento (2/3 de ancho en pantallas grandes) */}
                <div className="xl:col-span-2 h-full">
                    <SwarmLauncher />
                </div>
                {/* Historial de Ejecuciones (1/3 de ancho) */}
                <div className="xl:col-span-1 h-full min-h-[300px]">
                    <RunHistory />
                </div>
            </div>
        </section>

        {/* -----------------------------------------------------------
            ESTRATO 3: GESTIÓN DE IDENTIDAD (IAM)
            Bóveda de credenciales y rotación de sesiones.
           ----------------------------------------------------------- */}
        <section className="space-y-6">
           <div className="flex items-center justify-between pb-2 border-b border-zinc-800">
              <h2 className="text-lg font-bold text-emerald-500 flex items-center gap-3 font-mono tracking-wider">
                 <Shield className="w-5 h-5" />
                 IDENTITY VAULT
              </h2>
              <span className="text-[10px] font-mono font-bold bg-emerald-950/30 text-emerald-400 px-2 py-1 rounded border border-emerald-900/50">
                  CREDENTIALS
              </span>
           </div>

           {/* El componente IdentityVault maneja su propio layout interno (Injector + Inventory) */}
           <IdentityVault />
        </section>

        {/* -----------------------------------------------------------
            ESTRATO 4: VIGILANCIA VISUAL (PANÓPTICO)
            Telemetría visual en tiempo real de los nodos.
           ----------------------------------------------------------- */}
        <section className="space-y-6">
           <div className="flex items-center justify-between pb-2 border-b border-zinc-800">
              <h2 className="text-lg font-bold text-blue-500 flex items-center gap-3 font-mono tracking-wider">
                 <Eye className="w-5 h-5" />
                 VISUAL SURVEILLANCE
              </h2>
              <span className="text-[10px] font-mono font-bold bg-blue-950/30 text-blue-400 px-2 py-1 rounded border border-blue-900/50">
                  LIVE FEED
              </span>
           </div>

           <FleetGrid />
        </section>

      </div>
    </div>
  );
}
