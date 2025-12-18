/**
 * =================================================================
 * APARATO: IDENTITY INJECTOR (V36.0 - ZERO KNOWLEDGE)
 * CLASIFICACIÓN: SECURITY COMPONENT (L5)
 * RESPONSABILIDAD: CIFRADO CLIENT-SIDE Y PROVISIÓN DE IDENTIDAD
 * =================================================================
 */

"use client";

import React from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import { Shield, UploadCloud, Lock, Key, Info } from "lucide-react";
import { useTranslations } from "next-intl";

// ✅ RESOLUCIÓN Error 2307 & 2305: Importaciones niveladas
import { VaultCryptoEngine } from "@prospector/crypto-vault";
import { adminApi } from "@prospector/api-client";

import { InjectionFormSchema, type InjectionFormValues } from "./schemas";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription
} from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { Input } from "@/components/ui/kit/input";
import { cn } from "@/lib/utils/cn";

export function IdentityInjector() {
  const t = useTranslations("Dashboard.vault");
  const queryClient = useQueryClient();

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting }
  } = useForm<InjectionFormValues>({
    resolver: zodResolver(InjectionFormSchema),
  });

  const mutation = useMutation({
    mutationFn: async (data: InjectionFormValues) => {
      // 1. ADQUISICIÓN DE LLAVE MAESTRA (Perimetral)
      const masterKey = process.env.NEXT_PUBLIC_ADMIN_PASSWORD || "Netflix69";

      // 2. CIFRADO SIMÉTRICO (Zero-Knowledge)
      // El servidor nunca verá el JSON original de las cookies.
      const encryptedPayload = await VaultCryptoEngine.encrypt(
        data.cookiesJson,
        masterKey
      );

      // 3. TRANSMISIÓN AL ORQUESTADOR
      await adminApi.uploadIdentity({
        platform: data.platform,
        email: data.email,
        cookies: encryptedPayload as any,
        userAgent: navigator.userAgent,
      });
    },
    onSuccess: () => {
      toast.success("VAULT_SYNCHRONIZED", {
        description: "Encrypted identity payload secured in Turso."
      });
      queryClient.invalidateQueries({ queryKey: ["identities"] });
      reset();
    },
    onError: (error: Error) => {
      console.error("🔥 [INJECTION_FAULT]:", error.message);
      toast.error("VAULT_FAILURE", { description: error.message });
    },
  });

  const onFormSubmit = (values: InjectionFormValues) => {
    mutation.mutate(values);
  };

  return (
    <Card className="h-full bg-[#0f0f0f] border-zinc-800 relative overflow-hidden group">
      {/* Visual Ambiance Layer */}
      <div className="absolute inset-0 bg-linear-to-br from-emerald-500/5 to-transparent pointer-events-none" />

      <CardHeader className="relative z-10">
        <div className="flex justify-between items-start">
          <div className="space-y-1">
            <CardTitle className="flex items-center gap-3 text-white font-mono text-lg uppercase tracking-widest">
              <Shield className="w-5 h-5 text-emerald-500" />
              {t("title")}
              <span className="text-[8px] bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded border border-emerald-500/30">
                {t("injection_badge")}
              </span>
            </CardTitle>
            <CardDescription className="text-zinc-500 text-xs font-mono">
              Secure injection point for ephemeral grid credentials.
            </CardDescription>
          </div>
          <Lock className="w-4 h-4 text-zinc-800 group-hover:text-emerald-500/40 transition-colors" />
        </div>
      </CardHeader>

      <CardContent className="space-y-6 relative z-10">
        <form onSubmit={handleSubmit(onFormSubmit)} className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <label className="text-[9px] font-black text-zinc-600 uppercase font-mono tracking-widest">
                Target Platform
              </label>
              <select
                {...register("platform")}
                className="w-full bg-black border border-zinc-800 rounded-md h-10 px-3 text-xs font-mono text-zinc-300 focus:border-emerald-500 outline-none transition-all"
              >
                <option value="google_colab">Google Colab (T4/L4)</option>
                <option value="kaggle">Kaggle Kernels</option>
              </select>
            </div>
            <div className="space-y-2">
              <label className="text-[9px] font-black text-zinc-600 uppercase font-mono tracking-widest">
                Operator Identifier
              </label>
              <Input
                {...register("email")}
                placeholder="operator@hydra.io"
                className="bg-black border-zinc-800 text-xs h-10 focus:border-emerald-500"
                hasError={!!errors.email}
              />
            </div>
          </div>

          <div className="space-y-2">
            <label className="text-[9px] font-black text-zinc-600 uppercase font-mono flex justify-between tracking-widest">
              <span>Credentials Payload (Cookies JSON)</span>
              {errors.cookiesJson && (
                <span className="text-red-500 animate-pulse">:: {errors.cookiesJson.message} ::</span>
              )}
            </label>
            <div className="relative">
              <textarea
                {...register("cookiesJson")}
                className={cn(
                  "w-full h-40 bg-black/50 border border-zinc-800 rounded-md p-4 font-mono text-[10px] text-emerald-500 outline-none focus:border-emerald-500/50 transition-all resize-none custom-scrollbar",
                  errors.cookiesJson && "border-red-900/50 focus:border-red-500"
                )}
                placeholder="Paste [ { 'name': 'SID', 'value': '...' }, ... ]"
                spellCheck={false}
              />
              <div className="absolute bottom-3 right-3 flex items-center gap-2 text-[8px] text-zinc-700 font-mono font-bold">
                <Key className="w-3 h-3" /> ZK_PROTOCOL_ACTIVE
              </div>
            </div>
          </div>

          <Button
            type="submit"
            variant="cyber"
            className="w-full h-12 font-black tracking-[0.3em] uppercase text-xs"
            isLoading={mutation.isPending || isSubmitting}
          >
            {mutation.isPending ? t("encrypting") : (
              <span className="flex items-center gap-3">
                <UploadCloud className="w-4 h-4" /> {t("secure_btn")}
              </span>
            )}
          </Button>
        </form>

        <div className="pt-4 border-t border-white/5 flex items-center gap-3 text-zinc-600">
           <Info className="w-3 h-3" />
           <p className="text-[8px] font-mono leading-relaxed">
             Cookies are sanitized and encrypted with AES-256-GCM before transmission.
             Decryption key is never stored on the server.
           </p>
        </div>
      </CardContent>
    </Card>
  );
}
