import { useTranslations } from 'next-intl';
import { Link } from '@/lib/schemas/routing'; // ✅ RUTA CRÍTICA CORREGIDA
import { ShieldCheck, Zap, Globe, ArrowRight } from 'lucide-react';
import { Button } from '@/components/ui/kit/button';
import { AdSlot } from '@/components/ui/marketing/ad-slot';

export default function LandingPage() {
  const t = useTranslations('Landing');

  return (
    <div className="flex flex-col w-full h-full">

      {/* BACKGROUND FX LAYER */}
      <div className="fixed inset-0 z-0 pointer-events-none">
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" />
        <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[400px] bg-primary/10 opacity-30 blur-[120px] rounded-full pointer-events-none" />
      </div>

      <div className="relative z-10 flex flex-col items-center pt-24 pb-24 px-4 sm:px-6 lg:px-8 w-full max-w-7xl mx-auto">

        {/* HERO SECTION */}
        <div className="text-center max-w-4xl mx-auto mb-20 animate-in fade-in zoom-in-95 duration-700 slide-in-from-bottom-4">
          <h1 className="text-5xl md:text-7xl font-black tracking-tighter mb-6 bg-gradient-to-b from-white via-zinc-200 to-zinc-500 bg-clip-text text-transparent">
            {t('hero.title')}
          </h1>

          <p className="text-lg md:text-xl text-zinc-400 mb-10 max-w-2xl mx-auto font-light leading-relaxed">
            {t('hero.subtitle')}
          </p>

          <div className="flex flex-col sm:flex-row gap-6 justify-center items-center">
            <Link href="/login">
              <Button size="lg" variant="cyber" className="h-12 px-8 text-sm shadow-[0_0_30px_-5px_rgba(16,185,129,0.3)] hover:shadow-[0_0_40px_-5px_rgba(16,185,129,0.5)]">
                {t('hero.cta_primary.label')}
                <ArrowRight className="ml-2 w-4 h-4 group-hover:translate-x-1 transition-transform" />
              </Button>
            </Link>
          </div>
        </div>

        {/* ADS / MONETIZATION */}
        <div className="w-full max-w-3xl mb-24 opacity-60 hover:opacity-100 transition-opacity duration-500">
          <AdSlot />
        </div>

        {/* PRICING GRID */}
        <div className="grid md:grid-cols-2 gap-6 w-full max-w-4xl">
          {/* FREE TIER */}
          <div className="relative group bg-zinc-900/30 border border-white/5 p-8 rounded-2xl hover:border-zinc-700 transition-all">
            <div className="flex items-center gap-4 mb-6">
              <div className="p-3 bg-zinc-800/50 rounded-xl text-zinc-400 group-hover:text-white transition-colors">
                <Globe className="w-6 h-6" />
              </div>
              <div>
                <h3 className="text-lg font-bold text-white font-mono uppercase">{t('pricing.observer_title')}</h3>
                <span className="text-xs text-zinc-500 font-mono">Public Telemetry</span>
              </div>
            </div>
            <p className="text-zinc-400 mb-8 text-sm h-12 leading-relaxed border-l border-zinc-800 pl-4">
              {t('pricing.observer_desc')}
            </p>
            <div className="pt-6 border-t border-white/5 mt-auto">
              <Link href="/login" className="w-full block">
                <Button variant="outline" className="w-full font-mono border-zinc-800 bg-transparent hover:bg-zinc-800 text-zinc-300">
                  {t('pricing.cta_free')}
                </Button>
              </Link>
            </div>
          </div>

          {/* PRO TIER */}
          <div className="relative group bg-gradient-to-b from-primary/5 to-transparent border border-primary/20 p-8 rounded-2xl hover:shadow-[0_0_30px_-10px_rgba(16,185,129,0.1)] transition-all">
            <div className="absolute top-0 right-0 bg-primary text-black text-[9px] font-black px-3 py-1 rounded-bl-xl uppercase tracking-widest">
              Elite Access
            </div>
            <div className="flex items-center gap-4 mb-6">
              <div className="p-3 bg-primary/10 rounded-xl text-primary border border-primary/20">
                <Zap className="w-6 h-6" />
              </div>
              <div>
                <h3 className="text-lg font-bold text-white font-mono uppercase">{t('pricing.operator_title')}</h3>
                <span className="text-xs text-primary font-mono font-bold">Unrestricted Mining</span>
              </div>
            </div>
            <p className="text-emerald-500/80 mb-8 text-sm h-12 leading-relaxed border-l border-primary/20 pl-4">
              {t('pricing.operator_desc')}
            </p>
            <div className="pt-6 border-t border-primary/10 mt-auto">
              <Button variant="default" className="w-full bg-primary text-black hover:bg-emerald-400 font-mono font-bold tracking-wider">
                {t('pricing.cta_pro')}
              </Button>
            </div>
          </div>
        </div>

      </div>
    </div>
  );
}
