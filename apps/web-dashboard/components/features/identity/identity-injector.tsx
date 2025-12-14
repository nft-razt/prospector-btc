'use client';

import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { Shield, Server, Terminal, HelpCircle, UploadCloud } from 'lucide-react';

import { useHeimdall } from '@/hooks/use-heimdall';
import { adminApi } from '@prospector/api-client';
import { InjectionFormSchema, type InjectionFormValues } from './schemas';

import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/kit/card';
import { Button } from '@/components/ui/kit/button';
import { Input } from '@/components/ui/kit/input';
import { cn } from '@/lib/utils/cn';

/**
 * APARATO: IDENTITY INJECTOR
 * Formulario transaccional para la inyección segura de credenciales (Cookies).
 * Realiza sanitización previa y validación de estructura JSON.
 */
export function IdentityInjector() {
  const logger = useHeimdall('IdentityInjector');
  const queryClient = useQueryClient();
  const [showGuide, setShowGuide] = useState(false);

  // 1. Configuración del Formulario (RHF + Zod)
  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting }
  } = useForm<InjectionFormValues>({
    resolver: zodResolver(InjectionFormSchema),
    defaultValues: {
      platform: 'google_colab',
      email: '',
      cookiesJson: ''
    }
  });

  // 2. Mutación (TanStack Query)
  const mutation = useMutation({
    mutationFn: async (data: InjectionFormValues) => {
      // Parsing seguro garantizado por Zod
      const parsedCookies = JSON.parse(data.cookiesJson);

      await adminApi.uploadIdentity({
        platform: data.platform,
        email: data.email,
        cookies: parsedCookies,
        userAgent: navigator.userAgent
        // CORRECCIÓN: Eliminado 'provider' redundante. 'platform' ya cumple esa función.
      });
    },
    onMutate: () => logger.info('Iniciando secuencia de inyección de identidad...'),
    onSuccess: () => {
      toast.success('Identity Secured in Vault', {
        description: 'Credentials distributed to swarm network.'
      });
      queryClient.invalidateQueries({ queryKey: ['identities'] });
      reset();
    },
    onError: (error: Error) => {
      logger.error('Fallo en inyección', { error: error.message });
      toast.error('Injection Failed', {
        description: error.message || 'Critical handshake error'
      });
    }
  });

  const onSubmit = (data: InjectionFormValues) => {
    mutation.mutate(data);
  };

  return (
    <Card className="h-full flex flex-col bg-[#0f0f0f] border-slate-800 relative overflow-hidden">
      {/* Visual Noise Pattern */}
      <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none" />

      <CardHeader>
        <div className="flex justify-between items-start">
          <div className="space-y-1">
            <CardTitle className="flex items-center gap-3 text-white">
              <Shield className="w-6 h-6 text-emerald-500" />
              IDENTITY INJECTION
            </CardTitle>
            <CardDescription>
              Provide valid session cookies to authorize worker nodes.
            </CardDescription>
          </div>
          <button
            type="button"
            onClick={() => setShowGuide(!showGuide)}
            className="text-xs flex items-center gap-2 text-emerald-500 hover:text-emerald-400 transition-colors"
          >
            <HelpCircle className="w-4 h-4" />
            {showGuide ? 'Hide Guide' : 'Extraction Guide'}
          </button>
        </div>
      </CardHeader>

      <CardContent className="relative z-10 flex-1">
        {/* Guía Colapsable */}
        {showGuide && (
          <div className="mb-6 bg-slate-900/50 border border-slate-700/50 rounded-lg p-4 text-xs text-slate-300 space-y-2 animate-in fade-in slide-in-from-top-2">
            <h4 className="font-bold text-white mb-2 font-mono uppercase tracking-wider">Protocol: Cookie Extraction</h4>
            <ol className="list-decimal list-inside space-y-1 ml-1 marker:text-emerald-500">
              <li>Install <strong>Cookie-Editor</strong> extension.</li>
              <li>Login to <span className="text-emerald-400">Google Colab</span>.</li>
              <li>Open extension → Click <strong>Export</strong> (JSON).</li>
              <li>Paste strictly into the terminal input below.</li>
            </ol>
          </div>
        )}

        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">

            {/* PLATFORM SELECTOR */}
            <div className="space-y-2">
              <label className="text-[10px] uppercase tracking-widest text-slate-500 font-bold font-mono">Platform Target</label>
              <div className="relative group">
                <Server className="w-4 h-4 absolute left-3 top-3 text-slate-500 group-focus-within:text-emerald-500 transition-colors" />
                <select
                  {...register('platform')}
                  className="w-full bg-black/50 border border-slate-800 p-2.5 pl-10 rounded-md text-sm text-white focus:border-emerald-500 outline-none appearance-none font-mono transition-all"
                >
                  <option value="google_colab">Google Colab (GPU/TPU)</option>
                  <option value="kaggle">Kaggle Kernels</option>
                  <option value="ideogram">Ideogram (GenAI)</option>
                </select>
              </div>
            </div>

            {/* EMAIL INPUT */}
            <div className="space-y-2">
              <label className="text-[10px] uppercase tracking-widest text-slate-500 font-bold font-mono">Owner Email</label>
              <div className="relative group">
                <Input
                  {...register('email')}
                  type="email"
                  placeholder="operator@gmail.com"
                  hasError={!!errors.email}
                  className="pl-4"
                />
                {errors.email && (
                  <span className="absolute right-3 top-3 text-[10px] text-destructive font-bold">{errors.email.message}</span>
                )}
              </div>
            </div>
          </div>

          {/* COOKIES JSON AREA */}
          <div className="space-y-2">
            <label className="text-[10px] uppercase tracking-widest text-slate-500 font-bold font-mono flex justify-between">
              <span>Credentials Payload (JSON)</span>
              {errors.cookiesJson && <span className="text-destructive">{errors.cookiesJson.message}</span>}
            </label>
            <div className="relative">
              <Terminal className="w-4 h-4 absolute left-3 top-3 text-slate-600" />
              <textarea
                {...register('cookiesJson')}
                placeholder='[ { "domain": ".google.com", ... } ]'
                className={cn(
                  "w-full h-40 bg-black/80 border rounded-md p-4 pl-10 font-mono text-[10px] text-emerald-500 outline-none resize-none scrollbar-thin scrollbar-thumb-slate-800 transition-all",
                  errors.cookiesJson
                    ? "border-destructive focus:border-destructive placeholder:text-destructive/30"
                    : "border-slate-800 focus:border-emerald-500/50"
                )}
                spellCheck={false}
              />
            </div>
          </div>

          <Button
            type="submit"
            variant="cyber"
            className="w-full h-12 text-sm"
            isLoading={isSubmitting || mutation.isPending}
          >
            {!isSubmitting && <UploadCloud className="w-4 h-4 mr-2" />}
            {isSubmitting ? 'ENCRYPTING & UPLOADING...' : 'SECURE IN VAULT'}
          </Button>
        </form>
      </CardContent>
    </Card>
  );
}
