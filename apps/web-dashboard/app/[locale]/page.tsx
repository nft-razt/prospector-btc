import { useTranslations } from 'next-intl';
import { Link } from '@/i18n/routing';
import { ShieldCheck, Zap, Globe } from 'lucide-react';

export default function LandingPage() {
  const t = useTranslations('Landing');

  return (
    <div className="flex flex-col items-center justify-center py-24 px-4 sm:px-6 lg:px-8">

      {/* HERO SECTION */}
      <div className="text-center max-w-4xl mx-auto mb-20">
        <h1 className="text-5xl md:text-7xl font-black tracking-tighter bg-gradient-to-r from-emerald-400 to-cyan-500 bg-clip-text text-transparent mb-6">
          PROSPECTOR BTC
        </h1>
        <p className="text-xl text-slate-400 mb-10">{t('subtitle')}</p>
        <div className="flex gap-4 justify-center">
          <Link href="/login" className="px-8 py-3 bg-emerald-500 text-black font-bold rounded-full hover:bg-emerald-400 transition-all shadow-[0_0_20px_rgba(16,185,129,0.3)]">
            {t('cta_join')}
          </Link>
        </div>
      </div>

      {/* PRICING CAPSULES */}
      <div className="grid md:grid-cols-2 gap-8 w-full max-w-5xl">

        {/* FREE CAPSULE */}
        <div className="border border-slate-800 bg-[#0a0a0a] p-8 rounded-3xl hover:border-slate-600 transition-all group">
          <div className="flex items-center gap-3 mb-4">
            <div className="p-2 bg-slate-900 rounded-lg text-slate-400 group-hover:text-white transition-colors">
              <Globe className="w-6 h-6" />
            </div>
            <h3 className="text-2xl font-bold text-white">Observer Node</h3>
          </div>
          <p className="text-slate-500 mb-8 text-sm h-12">Access to public telemetry and basic network status.</p>
          <div className="text-4xl font-mono font-bold text-white mb-8">$0 <span className="text-sm text-slate-600">/mo</span></div>
          <button className="w-full py-3 border border-slate-700 text-white rounded-xl hover:bg-slate-800 transition-all font-mono text-sm">
            INITIALIZE
          </button>
        </div>

        {/* PRO CAPSULE (THE MONEY MAKER) */}
        <div className="border border-emerald-900/50 bg-emerald-950/10 p-8 rounded-3xl relative overflow-hidden group hover:shadow-[0_0_40px_rgba(16,185,129,0.1)] transition-all">
          <div className="absolute top-0 right-0 bg-emerald-500 text-black text-[10px] font-bold px-3 py-1 rounded-bl-xl">
            RECOMMENDED
          </div>
          <div className="flex items-center gap-3 mb-4">
            <div className="p-2 bg-emerald-900/30 rounded-lg text-emerald-400">
              <Zap className="w-6 h-6" />
            </div>
            <h3 className="text-2xl font-bold text-white">Operator Node</h3>
          </div>
          <p className="text-emerald-500/60 mb-8 text-sm h-12">Full mining capabilities, priority queue, and deep entropy analysis.</p>
          <div className="text-4xl font-mono font-bold text-white mb-8">$49 <span className="text-sm text-slate-600">/mo</span></div>
          <button className="w-full py-3 bg-emerald-500 text-black font-bold rounded-xl hover:bg-emerald-400 transition-all font-mono text-sm shadow-lg">
            {t('cta_sub')}
          </button>
        </div>

      </div>
    </div>
  );
}
