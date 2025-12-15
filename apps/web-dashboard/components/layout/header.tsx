"use client";

import { Link } from "@/lib/schemas/routing";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/kit/button";
import { Cpu, ShieldCheck } from "lucide-react";

export function Header() {
  const t = useTranslations("Dashboard.header");

  return (
    <header className="fixed top-0 left-0 right-0 z-50 h-16 border-b border-white/5 bg-black/60 backdrop-blur-xl transition-all duration-300">
      <div className="mx-auto flex h-full max-w-7xl items-center justify-between px-6">
        {/* LOGO AREA */}
        <Link href="/" className="flex items-center gap-3 group">
          <div className="h-8 w-8 bg-primary/10 rounded-lg border border-primary/20 flex items-center justify-center group-hover:bg-primary/20 group-hover:shadow-[0_0_15px_rgba(16,185,129,0.3)] transition-all">
            <Cpu className="w-5 h-5 text-primary" />
          </div>
          <div className="flex flex-col">
            <span className="text-lg font-bold tracking-tighter text-white font-mono leading-none">
              PROSPECTOR
            </span>
            <span className="text-[9px] text-primary/80 font-mono tracking-[0.3em] uppercase">
              Hydra-Zero
            </span>
          </div>
        </Link>

        {/* ACTIONS */}
        <div className="flex items-center gap-4">
          <div className="hidden md:flex items-center gap-2 text-[10px] text-zinc-500 font-mono border border-white/5 px-3 py-1 rounded-full bg-white/5">
            <ShieldCheck className="w-3 h-3 text-emerald-500" />
            <span>SYSTEM SECURE</span>
          </div>

          <Link href="/login">
            <Button
              variant="ghost"
              size="sm"
              className="text-zinc-400 hover:text-white font-mono uppercase tracking-wider text-xs hover:bg-white/5"
            >
              Operator Login
            </Button>
          </Link>
        </div>
      </div>
    </header>
  );
}
