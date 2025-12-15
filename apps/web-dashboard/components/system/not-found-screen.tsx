import Link from "next/link";
import { AlertTriangle, Home } from "lucide-react";

interface NotFoundProps {
  texts: {
    title: string;
    description: string;
    error_code: string;
    cta_return: string;
  };
  redirectPath: string;
}

export function NotFoundScreen({ texts, redirectPath }: NotFoundProps) {
  return (
    <div className="min-h-screen w-full bg-[#030303] flex flex-col items-center justify-center relative overflow-hidden font-mono text-center px-4">
      {/* Background Grid FX */}
      <div className="absolute inset-0 bg-[linear-gradient(rgba(20,20,20,0)_1px,transparent_1px),linear-gradient(90deg,rgba(20,20,20,0)_1px,transparent_1px)] bg-[size:40px_40px] opacity-20 [mask-image:radial-gradient(ellipse_at_center,black_40%,transparent_100%)] pointer-events-none" />

      {/* Glitch Container */}
      <div className="relative z-10 max-w-md w-full p-8 border border-red-900/30 bg-red-950/5 rounded-2xl backdrop-blur-sm">
        <div className="flex justify-center mb-6">
          <div className="h-16 w-16 bg-red-500/10 rounded-full flex items-center justify-center border border-red-500/20 shadow-[0_0_30px_rgba(239,68,68,0.15)] animate-pulse">
            <AlertTriangle className="h-8 w-8 text-red-500" />
          </div>
        </div>

        <h1 className="text-4xl font-black text-white tracking-widest mb-2 uppercase">
          {texts.title}
        </h1>

        <div className="text-[10px] text-red-400 font-bold tracking-[0.2em] mb-6 border-y border-red-900/50 py-1">
          :: {texts.error_code} ::
        </div>

        <p className="text-zinc-500 text-sm mb-8 leading-relaxed">
          {texts.description}
        </p>

        <Link
          href={redirectPath}
          className="inline-flex items-center gap-2 px-6 py-3 bg-zinc-100 hover:bg-white text-black font-bold rounded-lg transition-all text-xs uppercase tracking-wider hover:shadow-[0_0_20px_rgba(255,255,255,0.2)]"
        >
          <Home className="w-4 h-4" />
          {texts.cta_return}
        </Link>
      </div>

      <div className="absolute bottom-8 text-[10px] text-zinc-800">
        PROSPECTOR SYSTEM // ANOMALY DETECTED
      </div>
    </div>
  );
}
