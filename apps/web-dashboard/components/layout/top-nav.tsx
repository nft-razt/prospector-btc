/**
 * =================================================================
 * APARATO: TOP NAV STRATEGIC BAR
 * CLASIFICACIÓN: UX / CONTEXTO GLOBAL
 * RESPONSABILIDAD: BREADCRUMBS, UPLINK STATUS & USER ACTIONS
 * =================================================================
 */

'use client';

import { Breadcrumbs } from '@/components/layout/breadcrumbs';
import { UserNav } from '@/components/layout/user-nav';
import { ThemeToggle } from '@/components/layout/theme-toggle';
import { Globe, Wifi, ShieldCheck } from 'lucide-react';
import { motion } from 'framer-motion';

interface TopNavProps {
  user: {
    name?: string | null;
    email?: string | null;
    image?: string | null;
  }
}

export function TopNav({ user }: TopNavProps) {
  return (
    <div className="flex w-full items-center justify-between h-full px-2">

      {/* SECTOR IZQUIERDO: LOCALIZACIÓN */}
      <div className="flex items-center gap-6">
        <Breadcrumbs />
      </div>

      {/* SECTOR DERECHO: TELEMETRÍA DE RED & USER */}
      <div className="flex items-center gap-4">

        {/* NETWORK UPLINK INDICATOR */}
        <div className="hidden lg:flex items-center gap-6 px-4 py-1.5 bg-zinc-950/50 border border-white/5 rounded-full backdrop-blur-md">

            {/* Status Item: Latency */}
            <div className="flex items-center gap-2 border-r border-white/10 pr-4">
               <Wifi className="w-3 h-3 text-emerald-500" />
               <div className="flex flex-col">
                  <span className="text-[7px] text-zinc-500 uppercase font-black leading-none">Latency</span>
                  <span className="text-[10px] font-mono text-emerald-400 font-bold leading-none mt-1">24ms</span>
               </div>
            </div>

            {/* Status Item: Node Cluster */}
            <div className="flex items-center gap-2 border-r border-white/10 pr-4">
               <Globe className="w-3 h-3 text-primary" />
               <div className="flex flex-col">
                  <span className="text-[7px] text-zinc-500 uppercase font-black leading-none">Gateway</span>
                  <span className="text-[10px] font-mono text-zinc-300 font-bold leading-none mt-1">Render-US-East</span>
               </div>
            </div>

            {/* Status Item: Security */}
            <div className="flex items-center gap-2">
               <ShieldCheck className="w-3 h-3 text-blue-500" />
               <div className="flex flex-col">
                  <span className="text-[7px] text-zinc-500 uppercase font-black leading-none">Handshake</span>
                  <span className="text-[10px] font-mono text-blue-400 font-bold leading-none mt-1">Verified</span>
               </div>
            </div>
        </div>

        {/* ACTIONS */}
        <div className="flex items-center gap-2 pl-2">
           <ThemeToggle />
           <div className="h-6 w-px bg-white/10 mx-2" />
           <UserNav user={user} />
        </div>
      </div>
    </div>
  );
}
