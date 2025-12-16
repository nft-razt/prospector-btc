// apps/web-dashboard/components/layout/public-header.tsx
/**
 * =================================================================
 * APARATO: PUBLIC HEADER
 * CLASIFICACIÓN: ATOMIC ORGANISM
 * RESPONSABILIDAD: NAVEGACIÓN Y MARCA PARA VISITANTES
 * =================================================================
 */

import { useTranslations } from "next-intl";
import { Link } from "@/lib/schemas/routing";
import { Button } from "@/components/ui/kit/button";
import { Cpu, Terminal } from "lucide-react";

export function PublicHeader() {
  const t = useTranslations("PublicHeader");

  return (
    <header className="fixed top-0 left-0 right-0 z-50 h-16 border-b border-white/5 bg-black/60 backdrop-blur-xl transition-all duration-300">
      <div className="mx-auto flex h-full max-w-7xl items-center justify-between px-6">
        {/* LOGO AREA */}
        <Link href="/" className="flex items-center gap-3 group">
          <div className="h-8 w-8 bg-emerald-500/10 rounded-lg border border-emerald-500/20 flex items-center justify-center group-hover:bg-emerald-500/20 group-hover:shadow-[0_0_15px_rgba(16,185,129,0.3)] transition-all">
            <Cpu className="w-5 h-5 text-emerald-500" />
          </div>
          <span className="text-lg font-bold tracking-tighter text-white font-mono leading-none">
            {t("brand")}
          </span>
        </Link>

        {/* NAVIGATION LINKS (Desktop) */}
        <nav className="hidden md:flex items-center gap-8">
          {["features", "pricing", "about"].map((key) => (
            <Link
              key={key}
              href={`#${key}`}
              className="text-xs font-mono text-zinc-400 hover:text-white uppercase tracking-wider transition-colors"
            >
              {t(`nav.${key}` as any)}
            </Link>
          ))}
        </nav>

        {/* ACTIONS */}
        <div className="flex items-center gap-4">
          <Link href="/login">
            <Button
              variant="ghost"
              size="sm"
              className="text-zinc-400 hover:text-white font-mono uppercase tracking-wider text-xs hover:bg-white/5"
            >
              {t("actions.login")}
            </Button>
          </Link>

          <Link href="/register">
            <Button
              variant="cyber"
              size="sm"
              className="hidden sm:flex h-9 text-xs"
            >
               <Terminal className="w-3 h-3 mr-2" />
               {t("actions.get_started")}
            </Button>
          </Link>
        </div>
      </div>
    </header>
  );
}
