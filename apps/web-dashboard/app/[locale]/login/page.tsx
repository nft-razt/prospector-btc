/**
 * =================================================================
 * APARATO: LOGIN PAGE
 * CLASIFICACIÓN: VIEW LAYER (SERVER COMPONENT)
 * RESPONSABILIDAD: PUNTO DE ENTRADA DE AUTENTICACIÓN
 * =================================================================
 */

import { useTranslations } from 'next-intl';
import { Button } from '@/components/ui/kit/button'; // ✅ Importación correcta
import { auth, signIn } from '@/lib/auth/config';
import { redirect } from 'next/navigation';
import { ShieldCheck, Cpu } from 'lucide-react';

export default async function LoginPage() {
  const t = useTranslations('Auth');
  const session = await auth();

  // Si ya está logueado, redirigir al dashboard
  if (session?.user) {
    redirect('/dashboard');
  }

  return (
    <div className="flex min-h-screen flex-col items-center justify-center p-4 bg-[#050505]">

      {/* Background FX */}
      <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none" />

      <div className="w-full max-w-md space-y-8 relative z-10 p-8 border border-white/5 rounded-2xl bg-black/50 backdrop-blur-xl shadow-2xl">

        {/* Header */}
        <div className="flex flex-col items-center justify-center text-center space-y-4">
          <div className="h-16 w-16 bg-primary/10 rounded-2xl border border-primary/20 flex items-center justify-center shadow-[0_0_30px_rgba(16,185,129,0.2)]">
            <Cpu className="w-8 h-8 text-primary animate-pulse" />
          </div>

          <h2 className="text-3xl font-black tracking-tighter text-white font-mono">
            {t('login_title')}
          </h2>

          <div className="flex items-center gap-2 text-[10px] font-mono text-zinc-500 uppercase tracking-widest bg-white/5 px-3 py-1 rounded-full">
            <ShieldCheck className="w-3 h-3 text-emerald-500" />
            Hydra-Zero Protocol
          </div>
        </div>

        {/* Auth Actions */}
        <div className="mt-8 space-y-4">
          <form
            action={async () => {
              "use server"
              await signIn("google", { redirectTo: "/dashboard" });
            }}
          >
            <Button
              variant="outline"
              className="w-full h-12 border-zinc-700 bg-zinc-900/50 hover:bg-zinc-800 text-zinc-200 font-mono text-xs uppercase tracking-wider gap-3"
              type="submit"
            >
              {/* Google Logo SVG simple */}
              <svg className="h-4 w-4" aria-hidden="true" focusable="false" data-prefix="fab" data-icon="google" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 488 512"><path fill="currentColor" d="M488 261.8C488 403.3 391.1 504 248 504 110.8 504 0 393.2 0 256S110.8 8 248 8c66.8 0 123 24.5 166.3 64.9l-67.5 64.9C258.5 52.6 94.3 116.6 94.3 256c0 86.5 69.1 156.6 153.7 156.6 98.2 0 135-70.4 140.8-106.9H248v-85.3h236.1c2.3 12.7 3.9 24.9 3.9 41.4z"></path></svg>
              {t('login_google')}
            </Button>
          </form>
        </div>

        {/* Footer */}
        <div className="pt-6 border-t border-white/5 text-center">
          <p className="text-[10px] text-zinc-600 font-mono">
            {t('login_footer')}
          </p>
        </div>

      </div>
    </div>
  );
}
