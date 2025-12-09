'use client';

import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '@prospector/api-client';
import {
  Save, RefreshCw, Shield, Clock, User, Server,
  HelpCircle, Copy, Check, ExternalLink, Terminal
} from 'lucide-react';

// Tipado Frontend
interface Identity {
  id: string;
  platform: string;
  email: string;
  usage_count: number;
  last_used_at: string | null;
  status: 'active' | 'expired' | 'revoked';
}

export function IdentityVault() {
  const queryClient = useQueryClient();
  const [jsonInput, setJsonInput] = useState('');
  const [emailInput, setEmailInput] = useState('');
  const [platformInput, setPlatformInput] = useState('google_colab');
  const [showGuide, setShowGuide] = useState(false);

  // 1. FETCH INVENTORY
  const { data: identities } = useQuery({
    queryKey: ['identities'],
    queryFn: async () => {
      const res = await apiClient.get<Identity[]>('/admin/identities');
      return res.data;
    },
    refetchInterval: 5000,
  });

  // 2. UPLOAD MUTATION
  const uploadMutation = useMutation({
    mutationFn: async () => {
      let cookies;
      try {
        cookies = JSON.parse(jsonInput);
      } catch {
        throw new Error("El JSON de cookies no es válido. Revisa el formato.");
      }

      await apiClient.post('/admin/identities', {
        platform: platformInput,
        email: emailInput,
        cookies: cookies,
        user_agent: navigator.userAgent
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['identities'] });
      setJsonInput('');
      setEmailInput('');
      alert('✅ IDENTIDAD ASEGURADA EN LA BÓVEDA');
    },
    onError: (e) => alert(`❌ Error: ${e}`)
  });

  return (
    <div className="grid grid-cols-1 xl:grid-cols-3 gap-8">

      {/* --- COLUMNA 1: FORMULARIO DE INYECCIÓN --- */}
      <div className="xl:col-span-2 space-y-6">
        <div className="bg-[#0f0f0f] border border-slate-800 rounded-xl p-6 relative overflow-hidden group">
          <div className="absolute top-0 right-0 p-4 opacity-50 group-hover:opacity-100 transition-opacity">
            <button
              onClick={() => setShowGuide(!showGuide)}
              className="flex items-center gap-2 text-xs text-emerald-500 hover:text-emerald-400 bg-emerald-950/30 px-3 py-1 rounded-full border border-emerald-900/50"
            >
              <HelpCircle className="w-3 h-3" />
              {showGuide ? 'Hide Instructions' : 'How to get Cookies?'}
            </button>
          </div>

          <h2 className="text-xl font-black text-white mb-6 flex items-center gap-3">
            <Shield className="w-6 h-6 text-emerald-500" />
            IDENTITY INJECTION
          </h2>

          {/* Instrucciones Desplegables */}
          {showGuide && (
            <div className="mb-6 bg-slate-900/50 border border-slate-700 rounded-lg p-4 text-xs text-slate-300 space-y-2 animate-in fade-in slide-in-from-top-2">
              <h3 className="font-bold text-white mb-2">🕵️ Protocolo de Extracción de Identidad:</h3>
              <ol className="list-decimal list-inside space-y-1 ml-2">
                <li>Instala la extensión <strong>"Cookie-Editor"</strong> o <strong>"EditThisCookie"</strong> en tu navegador.</li>
                <li>Ve a <a href="https://colab.research.google.com" target="_blank" className="text-emerald-400 underline">Google Colab</a> y asegúrate de estar logueado.</li>
                <li>Abre la extensión y haz clic en <strong>"Export"</strong> (formato JSON).</li>
                <li>Pega el contenido del portapapeles en el campo "Credentials JSON" abajo.</li>
                <li>Introduce el email asociado para rastrear el uso.</li>
              </ol>
            </div>
          )}

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
             <div className="space-y-1">
               <label className="text-[10px] uppercase tracking-widest text-slate-500 font-bold">Platform Target</label>
               <div className="relative">
                 <Server className="w-4 h-4 absolute left-3 top-3 text-slate-500" />
                 <select
                   value={platformInput}
                   onChange={e => setPlatformInput(e.target.value)}
                   className="w-full bg-black border border-slate-700 p-2 pl-10 rounded text-sm text-white focus:border-emerald-500 outline-none appearance-none"
                 >
                   <option value="google_colab">Google Colab (GPU)</option>
                   <option value="ideogram">Ideogram (AI Gen)</option>
                   <option value="kaggle">Kaggle Kernels</option>
                 </select>
               </div>
             </div>

             <div className="space-y-1">
               <label className="text-[10px] uppercase tracking-widest text-slate-500 font-bold">Associated Email</label>
               <div className="relative">
                 <User className="w-4 h-4 absolute left-3 top-3 text-slate-500" />
                 <input
                   type="email"
                   placeholder="operator@gmail.com"
                   value={emailInput}
                   onChange={e => setEmailInput(e.target.value)}
                   className="w-full bg-black border border-slate-700 p-2 pl-10 rounded text-sm text-white focus:border-emerald-500 outline-none"
                 />
               </div>
             </div>
          </div>

          <div className="space-y-1 mb-6">
            <label className="text-[10px] uppercase tracking-widest text-slate-500 font-bold">Credentials JSON (Cookies)</label>
            <div className="relative">
                <Terminal className="w-4 h-4 absolute left-3 top-3 text-slate-600" />
                <textarea
                    value={jsonInput}
                    onChange={(e) => setJsonInput(e.target.value)}
                    placeholder='Paste [ { "domain": ".google.com", ... } ] here'
                    className="w-full h-40 bg-black border border-slate-700 rounded p-4 pl-10 font-mono text-[10px] text-emerald-500 focus:border-emerald-500 outline-none resize-none scrollbar-thin scrollbar-thumb-slate-800"
                    spellCheck={false}
                />
            </div>
          </div>

          <button
            onClick={() => uploadMutation.mutate()}
            disabled={uploadMutation.isPending || !jsonInput || !emailInput}
            className="w-full bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 disabled:cursor-not-allowed text-black font-black text-sm uppercase tracking-widest py-3 rounded flex justify-center items-center gap-2 transition-all shadow-[0_0_20px_rgba(16,185,129,0.2)] hover:shadow-[0_0_30px_rgba(16,185,129,0.4)]"
          >
            {uploadMutation.isPending ? <RefreshCw className="animate-spin w-4 h-4"/> : <Shield className="w-4 h-4" />}
            {uploadMutation.isPending ? 'ENCRYPTING & UPLOADING...' : 'SECURE CREDENTIALS IN VAULT'}
          </button>
        </div>
      </div>

      {/* --- COLUMNA 2: INVENTARIO EN TIEMPO REAL --- */}
      <div className="space-y-6">
        <div className="bg-[#0f0f0f] border border-slate-800 rounded-xl overflow-hidden flex flex-col h-full">
            <div className="p-6 border-b border-slate-800 bg-slate-900/30 flex justify-between items-center">
                <h3 className="text-sm font-bold text-slate-300 uppercase tracking-widest flex items-center gap-2">
                    <Server className="w-4 h-4" /> Active Personas
                </h3>
                <span className="text-xs bg-slate-800 text-slate-400 px-2 py-1 rounded">
                    {identities?.length || 0} Units
                </span>
            </div>

            <div className="flex-1 overflow-y-auto max-h-[500px] scrollbar-thin scrollbar-thumb-slate-800 p-2 space-y-2">
                {identities?.length === 0 && (
                    <div className="text-center p-8 text-slate-600 text-xs italic">
                        Vault is empty. Inject identities to start operations.
                    </div>
                )}

                {identities?.map((id) => (
                    <div key={id.id} className="bg-black/50 border border-slate-800 p-4 rounded-lg hover:border-slate-600 transition-colors group">
                        <div className="flex justify-between items-start mb-2">
                            <div className="flex items-center gap-2">
                                <div className={`w-2 h-2 rounded-full ${id.status === 'active' ? 'bg-emerald-500 shadow-[0_0_8px_#10b981]' : 'bg-red-500'}`} />
                                <span className="font-mono text-xs text-white font-bold">{id.email}</span>
                            </div>
                            <span className="text-[10px] text-slate-500 bg-slate-900 px-2 rounded uppercase">{id.platform}</span>
                        </div>

                        <div className="grid grid-cols-2 gap-2 text-[10px] text-slate-500 font-mono mt-3">
                            <div className="flex items-center gap-1">
                                <RefreshCw className="w-3 h-3" />
                                <span className="text-emerald-400">{id.usage_count}</span> leases
                            </div>
                            <div className="flex items-center gap-1 justify-end">
                                <Clock className="w-3 h-3" />
                                {id.last_used_at ? new Date(id.last_used_at).toLocaleTimeString() : 'Never'}
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
      </div>

    </div>
  );
}
