// apps/web-dashboard/app/[locale]/page.tsx
/**
 * =================================================================
 * APARATO: LANDING PAGE (PUBLIC ENTRY)
 * CLASIFICACIÓN: VIEW LAYER
 * RESPONSABILIDAD: PRIMERA IMPRESIÓN Y ENRUTAMIENTO DE USUARIO
 * =================================================================
 */

import { useTranslations } from "next-intl";
import { Link } from "@/lib/schemas/routing";
import { PublicHeader } from "@/components/layout/public-header";
import { Footer } from "@/components/layout/footer"; // Reusamos el footer existente o creamos PublicFooter
import { Button } from "@/components/ui/kit/button";
import { ArrowRight, ShieldCheck, Database, Key } from "lucide-react";
import { cn } from "@/lib/utils/cn";

export default function LandingPage() {
  const t = useTranslations("Landing");
  const common = useTranslations("Common");

  return (
    <div className="flex flex-col min-h-screen bg-[#050505] selection:bg-emerald-500/30">
      <PublicHeader />

      <main className="flex-1 flex flex-col relative pt-20">
        {/* BACKGROUND FX */}
        <div className="absolute inset-0 pointer-events-none z-0">
           <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[1000px] h-[600px] bg-emerald-500/5 blur-[120px] rounded-full opacity-40" />
           <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03]" />
        </div>

        {/* HERO SECTION */}
        <div className="relative z-10 flex flex-col items-center justify-center pt-24 pb-16 px-4 text-center max-w-5xl mx-auto">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-emerald-500/20 bg-emerald-500/10 text-emerald-400 text-[10px] font-mono tracking-widest uppercase mb-6 animate-in fade-in slide-in-from-bottom-4 duration-700">
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" />
            {t("hero.badge")}
          </div>

          <h1 className="text-5xl md:text-7xl font-black text-white tracking-tighter mb-8 leading-tight animate-in fade-in slide-in-from-bottom-4 duration-700 delay-100">
            {t("hero.title")}
          </h1>

          <p className="text-lg text-zinc-400 max-w-2xl mx-auto mb-16 font-light leading-relaxed animate-in fade-in slide-in-from-bottom-4 duration-700 delay-200">
            {t("hero.subtitle")}
          </p>

          {/* ACTION CAPSULES */}
          <div className="grid md:grid-cols-2 gap-6 w-full max-w-3xl animate-in fade-in slide-in-from-bottom-8 duration-700 delay-300">

            {/* LOGIN CAPSULE */}
            <div className="group relative p-8 rounded-2xl bg-zinc-900/40 border border-white/5 hover:border-emerald-500/30 transition-all duration-500 hover:bg-zinc-900/60 backdrop-blur-sm text-left">
              <div className="absolute inset-0 bg-gradient-to-br from-emerald-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity rounded-2xl" />

              <div className="relative z-10 flex flex-col h-full">
                <div className="w-10 h-10 rounded-lg bg-zinc-800 flex items-center justify-center mb-6 text-zinc-400 group-hover:text-white transition-colors">
                   <Database className="w-5 h-5" />
                </div>

                <h3 className="text-xl font-bold text-white mb-2 font-mono uppercase tracking-wide">
                  {t("capsules.login.title")}
                </h3>
                <p className="text-sm text-zinc-500 mb-8 flex-1">
                  {t("capsules.login.description")}
                </p>

                <Link href="/login" className="w-full">
                  <Button variant="outline" className="w-full justify-between group-hover:border-emerald-500/50 group-hover:text-emerald-400 font-mono text-xs">
                    {t("capsules.login.cta")}
                    <ArrowRight className="w-4 h-4 ml-2 group-hover:translate-x-1 transition-transform" />
                  </Button>
                </Link>
              </div>
            </div>

            {/* REGISTER CAPSULE */}
            <div className="group relative p-8 rounded-2xl bg-zinc-900/40 border border-white/5 hover:border-blue-500/30 transition-all duration-500 hover:bg-zinc-900/60 backdrop-blur-sm text-left">
               <div className="absolute inset-0 bg-gradient-to-br from-blue-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity rounded-2xl" />

               <div className="relative z-10 flex flex-col h-full">
                <div className="w-10 h-10 rounded-lg bg-zinc-800 flex items-center justify-center mb-6 text-zinc-400 group-hover:text-white transition-colors">
                   <Key className="w-5 h-5" />
                </div>

                <h3 className="text-xl font-bold text-white mb-2 font-mono uppercase tracking-wide">
                  {t("capsules.register.title")}
                </h3>
                <p className="text-sm text-zinc-500 mb-8 flex-1">
                  {t("capsules.register.description")}
                </p>

                <Link href="/register" className="w-full">
                  <Button variant="cyber" className="w-full justify-between font-mono text-xs">
                    {t("capsules.register.cta")}
                    <ShieldCheck className="w-4 h-4 ml-2" />
                  </Button>
                </Link>
              </div>
            </div>

          </div>
        </div>
      </main>

      <Footer />
    </div>
  );
}
