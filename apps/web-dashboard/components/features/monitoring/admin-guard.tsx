'use client';

import { useState, useEffect } from 'react';
import { Lock, ArrowRight, AlertTriangle, Terminal, ShieldCheck } from 'lucide-react';
import { apiClient } from '@prospector/api-client';
import { useHeimdall } from '@/hooks/use-heimdall';
import { Input } from '@/components/ui/kit/input';
import { Button } from '@/components/ui/kit/button';
import { cn } from '@/lib/utils/cn';

export function AdminGuard({ children }: { children: React.ReactNode }) {
  const logger = useHeimdall('AdminGuard');
  const [isUnlocked, setIsUnlocked] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [inputKey, setInputKey] = useState('');
  const [error, setError] = useState('');

  // Secreto Maestro (Fallback seguro)
  const MASTER_SECRET = process.env.NEXT_PUBLIC_ADMIN_PASSWORD || 'Netflix69';

  const authenticate = async () => {
    setIsLoading(true);
    setError('');
    logger.info('Iniciando secuencia de autenticación administrativa...');

    // 1. Validación Local (Primer Anillo)
    if (inputKey !== MASTER_SECRET) {
      logger.warn('Fallo de autenticación local: Passphrase incorrecta');
      setError('ACCESS DENIED: INCORRECT PASSPHRASE');
      setIsLoading(false);
      return;
    }

    // 2. Validación Remota (Segundo Anillo - Handshake)
    try {
      await apiClient.get('/admin/identities', {
        headers: { Authorization: `Bearer ${inputKey}` }
      });

      logger.info('Handshake administrativo exitoso. Acceso concedido.');
      sessionStorage.setItem('ADMIN_SESSION_TOKEN', inputKey);
      setIsUnlocked(true);
    } catch (e: any) {
      logger.error('Fallo en handshake remoto', { error: e.message });
      setError('CONNECTION REJECTED BY ORCHESTRATOR [401]');
      sessionStorage.removeItem('ADMIN_SESSION_TOKEN');
    } finally {
      setIsLoading(false);
    }
  };

  // Auto-login (Persistencia de Sesión)
  useEffect(() => {
    const storedToken = sessionStorage.getItem('ADMIN_SESSION_TOKEN');
    if (storedToken === MASTER_SECRET) {
      setIsUnlocked(true);
    }
  }, [MASTER_SECRET]);

  if (isUnlocked) return <>{children}</>;

  return (
    <div className="min-h-screen bg-[#050505] flex items-center justify-center p-4 font-mono selection:bg-primary/30">
      <div className="max-w-md w-full bg-[#0a0a0a] border border-slate-800 p-10 rounded-2xl shadow-2xl relative overflow-hidden group">

        {/* Efecto de Ruido de Fondo */}
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-10 pointer-events-none"></div>

        {/* Header Visual */}
        <div className="text-center mb-10 relative z-10">
          <div className="mx-auto w-20 h-20 bg-slate-900/50 rounded-full flex items-center justify-center mb-6 border border-slate-700 shadow-[0_0_30px_rgba(0,0,0,0.5)] backdrop-blur-sm group-hover:border-primary/50 transition-colors">
            {isLoading ? (
              <div className="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin" />
            ) : (
              <Lock className="w-8 h-8 text-slate-400 group-hover:text-primary transition-colors" />
            )}
          </div>
          <h1 className="text-2xl font-black text-white tracking-[0.2em] uppercase">Restricted Area</h1>
          <div className="flex items-center justify-center gap-2 mt-3 text-[10px] text-slate-500">
            <span className="w-2 h-2 bg-red-500 rounded-full animate-pulse"></span>
            AUTHORIZATION LEVEL 5 REQUIRED
          </div>
        </div>

        {/* Formulario */}
        <div className="space-y-6 relative z-10">
          <div className="relative group/input">
            <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
              <Terminal className="h-4 w-4 text-slate-600 group-focus-within/input:text-primary transition-colors" />
            </div>
            <Input
              type="password"
              value={inputKey}
              onChange={(e) => { setInputKey(e.target.value); setError(''); }}
              placeholder="ENTER PASSPHRASE"
              disabled={isLoading}
              className="pl-10 text-center tracking-widest text-primary font-bold bg-black/50 border-slate-700 focus:border-primary/50 h-12"
              onKeyDown={(e) => e.key === 'Enter' && authenticate()}
              hasError={!!error}
            />
          </div>

          <Button
            onClick={authenticate}
            disabled={isLoading || inputKey.length < 3}
            variant="cyber"
            className="w-full h-12 text-xs font-bold tracking-[0.2em]"
            isLoading={isLoading}
          >
            {!isLoading && 'AUTHENTICATE'}
            {!isLoading && <ArrowRight className="w-4 h-4 ml-2 group-hover:translate-x-1 transition-transform" />}
          </Button>

          {error && (
            <div className="flex items-center justify-center gap-2 text-red-500 text-[10px] bg-red-950/10 py-3 rounded border border-red-900/30 animate-in fade-in slide-in-from-bottom-2">
              <AlertTriangle className="w-3 h-3" />
              <span className="font-mono font-bold">{error}</span>
            </div>
          )}
        </div>

        <div className="mt-8 text-[10px] text-center text-slate-700 font-mono flex justify-center items-center gap-2">
            <ShieldCheck className="w-3 h-3" />
            SECURE CONNECTION :: HYDRA-ZERO PROTOCOL
        </div>
      </div>
    </div>
  );
}
