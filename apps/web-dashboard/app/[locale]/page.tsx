import { useTranslations } from 'next-intl';
import { Link } from '@/lib/schemas/routing'; // ✅ RUTA CORREGIDA
import { ShieldCheck, Zap, Globe, Cpu, ArrowRight } from 'lucide-react';
import { Button } from '@/components/ui/kit/button';
import { AdSlot } from '@/components/ui/marketing/ad-slot';

export default function LandingPage() {
  const t = useTranslations('Landing');

  return (
    <div className="flex flex-col min-h-screen bg-background text-foreground overflow-hidden selection:bg-primary/30">

      {/* 1. LAYER: BACKGROUND FX */}
      <div className="fixed inset-0 z-0 pointer-events-none">
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" />
        <div className="absolute left-0 right-0 top-0 -z-10 m-auto h-[310px] w-[310px] rounded-full bg-primary/20 opacity-20 blur-[100px]" />
      </div>

      <main className="relative z-10 flex-1 flex flex-col items-center pt-20 pb-24 px-4 sm:px-6 lg:px-8">

        {/* 2. SECTION: HERO */}
        <div className="text-center max-w-5xl mx-auto mb-24 animate-in fade-in zoom-in-95 duration-700 slide-in-from-bottom-4">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-secondary/50 border border-secondary text-[10px] uppercase tracking-widest font-mono text-primary mb-8 hover:bg-secondary/70 transition-colors cursor-default">
            <span className="relative flex h-2 w-2">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary opacity-75"></span>
              <span className="relative inline-flex rounded-full h-2 w-2 bg-primary"></span>
            </span>
            System Online // v4.0
          </div>

          <h1 className="text-6xl md:text-8xl font-black tracking-tighter mb-6 bg-gradient-to-b from-white via-white/90 to-white/50 bg-clip-text text-transparent drop-shadow-2xl">
            {t('hero.title')}
          </h1>

          <p className="text-xl md:text-2xl text-muted-foreground mb-12 max-w-2xl mx-auto leading-relaxed font-light">
            {t('hero.subtitle')}
          </p>

          <div className="flex flex-col sm:flex-row gap-6 justify-center items-center">
            <Link href="/login">
              <Button size="lg" variant="cyber" className="h-14 px-8 text-base shadow-[0_0_30px_-5px_rgba(16,185,129,0.3)]">
                {t('hero.cta_primary.label')}
                <ArrowRight className="ml-2 w-5 h-5 group-hover:translate-x-1 transition-transform" />
              </Button>
            </Link>

            <div className="flex items-center gap-4 text-sm text-muted-foreground font-mono">
              <span className="flex items-center gap-2">
                <ShieldCheck className="w-4 h-4 text-primary" /> Audited
              </span>
              <span className="w-px h-4 bg-border" />
              <span className="flex items-center gap-2">
                <Cpu className="w-4 h-4 text-primary" /> Distributed
              </span>
            </div>
          </div>
        </div>

        {/* 3. SECTION: SPONSORED SLOT */}
        <div className="w-full max-w-4xl mb-24 opacity-80 hover:opacity-100 transition-opacity">
          <AdSlot />
        </div>

        {/* 4. SECTION: PRICING MATRIX */}
        <div className="grid md:grid-cols-2 gap-8 w-full max-w-5xl">

          {/* TIER: OBSERVER (Free) */}
          <div className="relative group bg-card/50 backdrop-blur-sm border border-border p-8 rounded-3xl hover:border-primary/30 transition-all duration-300 flex flex-col">
            <div className="absolute inset-0 bg-gradient-to-b from-primary/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity rounded-3xl" />

            <div className="flex items-center gap-4 mb-6 relative z-10">
              <div className="p-3 bg-secondary rounded-2xl text-muted-foreground group-hover:text-primary transition-colors">
                <Globe className="w-6 h-6" />
              </div>
              <div>
                <h3 className="text-xl font-bold text-foreground font-mono uppercase tracking-wide">
                  {t('pricing.observer_title')}
                </h3>
                <span className="text-xs text-muted-foreground font-mono">Public Access</span>
              </div>
            </div>

            <p className="text-muted-foreground mb-8 text-sm flex-1 leading-relaxed border-l-2 border-border pl-4">
              {t('pricing.observer_desc')}
            </p>

            <div className="relative z-10 pt-6 border-t border-border mt-auto">
              <div className="flex items-baseline gap-2 mb-6">
                <span className="text-4xl font-mono font-bold text-foreground">$0</span>
                <span className="text-sm text-muted-foreground">/ {t('pricing.currency').split('/')[1]}</span>
              </div>

              <Link href="/login" className="w-full block">
                <Button variant="outline" className="w-full border-border hover:bg-secondary font-mono tracking-wider">
                  {t('pricing.cta_free')}
                </Button>
              </Link>
            </div>
          </div>

          {/* TIER: OPERATOR (Pro) */}
          <div className="relative group bg-gradient-to-b from-primary/10 to-transparent border border-primary/20 p-8 rounded-3xl overflow-hidden hover:shadow-[0_0_50px_-10px_rgba(16,185,129,0.1)] transition-all duration-300 flex flex-col">
            <div className="absolute top-0 right-0 bg-primary text-black text-[10px] font-black px-4 py-1.5 rounded-bl-2xl uppercase tracking-widest shadow-lg">
              Recommended
            </div>

            <div className="flex items-center gap-4 mb-6 relative z-10">
              <div className="p-3 bg-primary/20 rounded-2xl text-primary shadow-[0_0_15px_rgba(16,185,129,0.2)]">
                <Zap className="w-6 h-6" />
              </div>
              <div>
                <h3 className="text-xl font-bold text-white font-mono uppercase tracking-wide">
                  {t('pricing.operator_title')}
                </h3>
                <span className="text-xs text-primary font-mono font-bold">Full Capabilities</span>
              </div>
            </div>

            <p className="text-primary/80 mb-8 text-sm flex-1 leading-relaxed border-l-2 border-primary/30 pl-4">
              {t('pricing.operator_desc')}
            </p>

            <div className="relative z-10 pt-6 border-t border-primary/20 mt-auto">
              <div className="flex items-baseline gap-2 mb-6">
                <span className="text-4xl font-mono font-bold text-white">$49</span>
                <span className="text-sm text-primary/70">/ {t('pricing.currency').split('/')[1]}</span>
              </div>

              <Button variant="default" className="w-full bg-primary text-primary-foreground hover:bg-emerald-400 font-mono font-bold tracking-wider shadow-lg shadow-primary/20">
                {t('pricing.cta_pro')}
              </Button>
            </div>
          </div>

        </div>
      </main>

      <footer className="py-8 border-t border-border/50 bg-black/20 text-center relative z-10">
        <p className="text-[10px] text-muted-foreground font-mono uppercase tracking-[0.2em]">
          Prospector Suite © 2025 // MIT License // Academic Research
        </p>
      </footer>
    </div>
  );
}
