/**
 * =================================================================
 * APARATO: SIDEBAR COMMAND CENTER
 * CLASIFICACIÓN: ESTRUCTURAL / UX PERCEPTIVA
 * RESPONSABILIDAD: NAVEGACIÓN Y ESTADO DE SALUD GLOBAL
 * =================================================================
 */

"use client";

import { usePathname } from "next/navigation";
import { useTranslations } from "next-intl";
import { motion } from "framer-motion";
import {
  Cpu,
  Activity,
  Zap,
  Database,
  ShieldAlert,
  Terminal,
  Globe,
} from "lucide-react";

import { MAIN_NAVIGATION } from "@/config/navigation";
import { SidebarItem } from "./sidebar-item";
import { Card } from "@/components/ui/kit/card";
import { cn } from "@/lib/utils/cn";

/**
 * Organismo de navegación lateral con telemetría integrada.
 * Implementa visualización de estado de capas (L1, L2, L3).
 */
export function Sidebar() {
  const pathname = usePathname();
  const t = useTranslations("Dashboard.sidebar");

  const isRouteActive = (href: string) => {
    const cleanPath = pathname.replace(/^\/(en|es)/, "") || "/";
    return cleanPath.startsWith(href);
  };

  return (
    <div className="flex flex-col h-full bg-black/40 backdrop-blur-xl border-r border-white/5 select-none overflow-hidden">
      {/* 1. BRANDING & CORE STATUS */}
      <div className="p-6">
        <div className="flex items-center gap-4 mb-8">
          <div className="relative">
            <div className="absolute inset-0 bg-primary/20 blur-lg rounded-full animate-pulse" />
            <div className="relative h-10 w-10 bg-black border border-primary/40 text-primary rounded-xl flex items-center justify-center shadow-[0_0_20px_rgba(16,185,129,0.1)]">
              <Cpu className="h-6 w-6" />
            </div>
          </div>
          <div className="flex flex-col">
            <span className="text-sm font-black tracking-[0.2em] text-white leading-none font-mono">
              PROSPECTOR
            </span>
            <span className="text-[8px] text-primary font-bold tracking-[0.3em] mt-1.5 uppercase opacity-70">
              U256 // HYDRA-ZERO
            </span>
          </div>
        </div>

        {/* 2. NAVIGATION GRID */}
        <nav className="space-y-1">
          {MAIN_NAVIGATION.map((item) => (
            <SidebarItem
              key={item.href}
              item={item}
              isActive={isRouteActive(item.href)}
              label={t(item.translationKey as any)}
            />
          ))}
        </nav>
      </div>

      {/* 3. SYSTEM STRATA MONITOR (Real-time Feedback) */}
      <div className="mt-auto p-4 space-y-3">
        <Card className="bg-zinc-900/30 border-white/5 p-4 overflow-hidden relative group">
          <div className="absolute top-0 right-0 p-2 opacity-20 group-hover:opacity-100 transition-opacity">
            <Activity className="w-3 h-3 text-primary animate-pulse" />
          </div>

          <h4 className="text-[9px] font-black text-zinc-500 uppercase tracking-widest mb-3 flex items-center gap-2">
            <Database className="w-3 h-3" /> System Strata
          </h4>

          <div className="space-y-2.5">
            <StrataStatus label="Orchestrator L3" status="online" />
            <StrataStatus label="Turso Vault" status="online" />
            <StrataStatus label="Swarm Mesh" status="active" />
          </div>
        </Card>

        {/* 4. SECURITY TOKEN INFO */}
        <div className="px-2 py-3 flex items-center justify-between border-t border-white/5">
          <div className="flex items-center gap-2">
            <ShieldAlert className="w-3 h-3 text-amber-500" />
            <span className="text-[8px] text-zinc-500 font-mono uppercase font-bold">
              Encrypted Link
            </span>
          </div>
          <span className="text-[8px] text-emerald-500 font-mono font-bold tracking-tighter">
            TLS_AES_256
          </span>
        </div>
      </div>
    </div>
  );
}

/** Átomo de visualización de capa */
function StrataStatus({ label, status }: { label: string; status: string }) {
  return (
    <div className="flex items-center justify-between group/strata">
      <span className="text-[10px] text-zinc-400 font-mono group-hover/strata:text-zinc-200 transition-colors">
        {label}
      </span>
      <div className="flex items-center gap-1.5">
        <div
          className={cn(
            "h-1 w-1 rounded-full",
            status === "online" || status === "active"
              ? "bg-emerald-500 shadow-[0_0_5px_#10b981]"
              : "bg-red-500",
          )}
        />
        <span className="text-[8px] uppercase font-black text-zinc-600 tracking-tighter">
          {status}
        </span>
      </div>
    </div>
  );
}
