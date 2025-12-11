import { useTranslations } from 'next-intl';
import { IdentityVault } from '@/components/features/identity/identity-vault';
import { FleetGrid } from '@/components/features/network/fleet-grid';
import { AdSlot } from '@/components/ui/marketing/ad-slot';
import { Separator } from '@radix-ui/react-separator'; // O tu componente UI si lo tienes encapsulado

export default function DashboardPage() {
  const t = useTranslations('Dashboard');

  return (
    <div className="space-y-8 animate-in fade-in duration-500">

      {/* 1. SECCIÓN DE MARKETING / MONETIZACIÓN (Discreta) */}
      <div className="w-full">
         <AdSlot />
      </div>

      <div className="grid gap-8">

        {/* 2. HEADER DE SECCIÓN */}
        <div className="flex flex-col gap-2">
          <h1 className="text-3xl font-black text-white tracking-tight">
            {t('sidebar.overview')}
          </h1>
          <p className="text-zinc-500 text-sm">
            Real-time operations monitor & entropy injection interface.
          </p>
        </div>

        {/* 3. MÓDULO DE IDENTIDAD (Ahora en Features/Identity) */}
        <section className="space-y-4">
           <div className="flex items-center justify-between">
              <h2 className="text-xl font-bold text-emerald-500 flex items-center gap-2">
                 ⚡ ACTIVE CREDENTIALS
              </h2>
           </div>
           <IdentityVault />
        </section>

        <div className="h-px bg-zinc-800 w-full my-4" />

        {/* 4. MÓDULO DE VIGILANCIA (Ahora en Features/Network) */}
        <section className="space-y-4">
           <div className="flex items-center justify-between">
              <h2 className="text-xl font-bold text-blue-500 flex items-center gap-2">
                 👁️ VISUAL SURVEILLANCE
              </h2>
           </div>
           <FleetGrid />
        </section>

      </div>
    </div>
  );
}
