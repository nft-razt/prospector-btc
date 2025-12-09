'use client';

import { useState, useEffect } from 'react';
import { Lock, Unlock, ArrowRight, AlertTriangle, Terminal } from 'lucide-react';
import { apiClient } from '@prospector/api-client';

export function AdminGuard({ children }: { children: React.ReactNode }) {
  const [isUnlocked, setIsUnlocked] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [inputKey, setInputKey] = useState('');
  const [error, setError] = useState('');

  // Sincronización con el "Secreto Maestro"
  // Si no se define en .env, el hardcode de emergencia es 'Netflix69'
  const MASTER_SECRET = process.env.NEXT_PUBLIC_ADMIN_PASSWORD || 'Netflix69';

  const authenticate = async () => {
    setIsLoading(true);
    setError('');

    // 1. Validación Local (Primer Anillo de Seguridad)
    if (inputKey !== MASTER_SECRET) {
      setError('ACCESS DENIED: INCORRECT PASSPHRASE');
      setIsLoading(false);
      return;
    }

    // 2. Validación Remota (Segundo Anillo de Seguridad)
    // Usamos el token para intentar listar identidades. Si el token (Netflix69)
    // no coincide con WORKER_AUTH_TOKEN en el servidor, esto fallará.
    try {
      await apiClient.get('/admin/identities', {
        headers: { Authorization: `Bearer ${inputKey}` }
      });

      // Si llegamos aquí, el token es válido en el servidor.
      sessionStorage.setItem('ADMIN_SESSION_TOKEN', inputKey);
      setIsUnlocked(true);
    } catch (e: any) {
      console.error("Auth Handshake Failed", e);
      // Mensaje críptico para seguridad por oscuridad
      setError('CONNECTION REJECTED BY ORCHESTRATOR [401]');
      sessionStorage.removeItem('ADMIN_SESSION_TOKEN');
    } finally {
      setIsLoading(false);
    }
  };

  // Auto-login si ya hay sesión válida
  useEffect(() => {
    const storedToken = sessionStorage.getItem('ADMIN_SESSION_TOKEN');
    if (storedToken === MASTER_SECRET) {
      setIsUnlocked(true);
    }
  }, [MASTER_SECRET]);

  if (isUnlocked) return <>{children}</>;

  return (
    <div className="min-h-screen bg-[#050505] flex items-center justify-center p-4 font-mono selection:bg-emerald-500/30">
      <div className="max-w-md w-full bg-[#0a0a0a] border border-slate-800 p-10 rounded-2xl shadow-2xl relative overflow-hidden group">

        {/* Efecto de Matriz de Fondo */}
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-10 pointer-events-none"></div>
        <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-transparent via-emerald-600 to-transparent opacity-0 group-hover:opacity-100 group-hover:animate-scan transition-opacity" />

        <div className="text-center mb-10 relative z-10">
          <div className="mx-auto w-20 h-20 bg-slate-900/80 rounded-full flex items-center justify-center mb-6 border border-slate-700 shadow-[0_0_30px_rgba(0,0,0,0.8)] backdrop-blur-sm">
            {isLoading ? (
              <div className="w-8 h-8 border-2 border-emerald-500 border-t-transparent rounded-full animate-spin" />
            ) : (
              <Lock className="w-8 h-8 text-slate-400 group-hover:text-emerald-500 transition-colors" />
            )}
          </div>
          <h1 className="text-2xl font-black text-white tracking-[0.2em] uppercase">Restricted Area</h1>
          <div className="flex items-center justify-center gap-2 mt-3 text-[10px] text-slate-500">
            <span className="w-2 h-2 bg-red-500 rounded-full animate-pulse"></span>
            AUTHORIZATION LEVEL 5 REQUIRED
          </div>
        </div>

        <div className="space-y-6 relative z-10">
          <div className="relative group/input">
            <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
              <Terminal className="h-4 w-4 text-slate-600 group-focus-within/input:text-emerald-500 transition-colors" />
            </div>
            <input
              type="password"
              value={inputKey}
              onChange={(e) => { setInputKey(e.target.value); setError(''); }}
              placeholder="ENTER PASSPHRASE"
              disabled={isLoading}
              className="w-full bg-black/50 border border-slate-700 text-emerald-500 text-center py-4 px-10 rounded-lg focus:outline-none focus:border-emerald-500/50 focus:ring-1 focus:ring-emerald-500/20 transition-all placeholder:text-slate-800 disabled:opacity-50 text-sm tracking-widest"
              onKeyDown={(e) => e.key === 'Enter' && authenticate()}
            />
          </div>

          <button
            onClick={authenticate}
            disabled={isLoading || inputKey.length < 3}
            className="w-full bg-emerald-600/10 hover:bg-emerald-600/20 text-emerald-500 border border-emerald-500/30 py-4 rounded-lg flex items-center justify-center gap-3 transition-all uppercase text-xs font-bold tracking-[0.2em] group disabled:opacity-30 disabled:cursor-not-allowed hover:shadow-[0_0_20px_rgba(16,185,129,0.1)]"
          >
            {isLoading ? 'ESTABLISHING SECURE LINK...' : 'AUTHENTICATE'}
            {!isLoading && <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />}
          </button>

          {error && (
            <div className="flex items-center justify-center gap-2 text-red-500 text-[10px] bg-red-950/20 py-2 rounded border border-red-900/30 animate-in fade-in slide-in-from-bottom-2">
              <AlertTriangle className="w-3 h-3" />
              <span className="font-mono">{error}</span>
            </div>
          )}
        </div>

        <div className="mt-8 text-[10px] text-center text-slate-700 font-mono">
            SECURE CONNECTION :: TLS 1.3 :: HYDRA-ZERO PROTOCOL
        </div>
      </div>
    </div>
  );
}
