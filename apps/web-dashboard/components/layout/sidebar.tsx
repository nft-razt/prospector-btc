'use client';

import { usePathname } from 'next/navigation';
import { useTranslations } from 'next-intl';
import { Cpu, ServerCrash } from 'lucide-react';
import { MAIN_NAVIGATION, type RouteItem } from '@/config/navigation';
import { SidebarItem } from './sidebar-item';
import { Card } from '@/components/ui/kit/card';

/**
 * ORGANISMO: SIDEBAR
 * Navegación lateral principal. Orquesta los ítems y muestra estado del sistema.
 */
export function Sidebar() {
  const pathname = usePathname();
  const t = useTranslations('Dashboard.sidebar');

  // Lógica de coincidencia de rutas robusta
  const isRouteActive = (route: RouteItem) => {
    // Normalizamos quitando el locale si existe (ej: /es/dashboard -> /dashboard)
    const cleanPath = pathname.replace(/^\/(en|es)/, '') || '/';

    if (route.matchMode === 'exact') {
      return cleanPath === route.href || cleanPath === `${route.href}/`;
    }
    return cleanPath.startsWith(route.href);
  };

  return (
    <div className="flex flex-col h-full py-4 space-y-4 text-muted-foreground select-none">
      {/* 1. BRANDING */}
      <div className="px-6 py-2 flex items-center gap-3 mb-4">
        <div className="h-9 w-9 bg-black border border-primary/30 text-primary rounded-xl flex items-center justify-center shadow-[0_0_20px_rgba(16,185,129,0.15)] backdrop-blur-sm">
            <Cpu className="h-5 w-5 animate-pulse-slow" />
        </div>
        <div className="flex flex-col justify-center">
            <span className="text-foreground font-black tracking-widest text-sm leading-none font-mono">
              PROSPECTOR
            </span>
            <span className="text-[9px] text-primary/80 font-mono tracking-[0.3em] mt-1.5 uppercase">
              Suite v4.0
            </span>
        </div>
      </div>

      {/* 2. NAVIGATION LIST */}
      <nav className="flex-1 px-3 space-y-1 overflow-y-auto scrollbar-thin scrollbar-thumb-muted">
        {MAIN_NAVIGATION.map((route) => (
          <SidebarItem
            key={route.href}
            item={route}
            isActive={isRouteActive(route)}
            label={t(route.translationKey as any)}
          />
        ))}
      </nav>

      {/* 3. FOOTER STATUS */}
      <div className="px-4 mt-auto">
        <Card className="bg-black/40 border-primary/10 p-4 backdrop-blur-sm">
           <div className="flex items-center gap-2 mb-2">
              <span className="relative flex h-2 w-2">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
              </span>
              <span className="text-[10px] font-bold text-foreground font-mono tracking-widest">
                SYSTEM ONLINE
              </span>
           </div>
           <div className="flex justify-between items-center text-[9px] text-muted-foreground font-mono">
             <span>Latency:</span>
             <span className="text-emerald-500 font-bold">24ms</span>
           </div>
        </Card>
      </div>
    </div>
  );
}
