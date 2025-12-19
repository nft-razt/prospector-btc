/**
 * =================================================================
 * APARATO: IDENTITY INJECTOR (V36.2 - LINK RESOLVED)
 * CLASIFICACIÓN: SECURITY COMPONENT (L5)
 * RESPONSABILIDAD: CIFRADO CLIENT-SIDE Y PROVISIÓN DE IDENTIDAD
 *
 * ESTRATEGIA DE ÉLITE:
 * - Zero-Knowledge: El servidor nunca recibe el JSON en claro.
 * - AES-GCM 256: Cifrado simétrico de alta seguridad antes de la ingesta.
 * - Validation: Esquema Zod para asegurar integridad del payload.
 * =================================================================
 */

"use client";

import React from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import {
  Shield,
  UploadCloud,
  Lock,
  Key,
  Info,
  ShieldAlert,
} from "lucide-react";
import { useTranslations } from "next-intl";

// --- SINAPSIS DE ÉLITE (Tipado Estricto) ---
import { VaultCryptoEngine } from "@prospector/crypto-vault";
import { adminApi } from "@prospector/api-client";
import { InjectionFormSchema, type InjectionFormValues } from "./schemas";

// --- ÁTOMOS UI ---
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
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
    formState: { errors, isSubmitting },
  } = useForm<InjectionFormValues>({
    resolver: zodResolver(InjectionFormSchema),
    defaultValues: {
      platform: "google_colab",
    },
  });

  /**
   * MUTACIÓN DE INGESTIÓN SEGURA
   * Realiza el proceso de "Mirror Masking" inyectando la identidad cifrada.
   */
  const mutation = useMutation({
    mutationFn: async (data: InjectionFormValues) => {
      // 1. ADQUISICIÓN DE LLAVE DE PASO (Local Passphrase)
      const masterKey = process.env.NEXT_PUBLIC_ADMIN_PASSWORD || "Netflix69";

      // 2. PROTOCOLO ZERO-KNOWLEDGE (Cifrado en el Navegador)
      // Transformamos el string de cookies en un búnker cifrado
      const encryptedPayload = await VaultCryptoEngine.encrypt(
        data.cookiesJson,
        masterKey,
      );

      // 3. TRANSMISIÓN TÁCTICA
      // Enviamos el objeto cifrado al Orquestador (Stratum L3)
      await adminApi.uploadIdentity({
        platform: data.platform,
        email: data.email,
        cookies: encryptedPayload, // El backend guarda esto como un JSON opaco
        userAgent: navigator.userAgent,
      });
    },
    onSuccess: () => {
      toast.success("VAULT_SYNCHRONIZED", {
        description:
          "The identity bunker has been secured in the Tactical Ledger.",
        icon: <ShieldCheck className="w-4 h-4 text-emerald-500" />,
      });
      queryClient.invalidateQueries({ queryKey: ["identities"] });
      reset();
    },
    onError: (error: Error) => {
      console.error("🔥 [INJECTION_FAULT]:", error.message);
      toast.error("VAULT_FAILURE", {
        description: "Decryption handshake or database sync failed.",
        icon: <ShieldAlert className="w-4 h-4 text-red-500" />,
      });
    },
  });

  const onFormSubmit = (values: InjectionFormValues) => {
    mutation.mutate(values);
  };

  return (
    <Card className="h-full bg-[#0f0f0f] border-zinc-800 relative overflow-hidden group shadow-2xl">
      {/* Background Ambience FX */}
      <div className="absolute inset-0 bg-linear-to-br from-emerald-500/5 to-transparent pointer-events-none opacity-40 group-hover:opacity-100 transition-opacity duration-1000" />

      <CardHeader className="relative z-10 border-b border-white/5 bg-white/2">
        <div className="flex justify-between items-start">
          <div className="space-y-1">
            <CardTitle className="flex items-center gap-3 text-white font-mono text-lg uppercase tracking-widest">
              <Key className="w-5 h-5 text-emerald-500" />
              {t("title")}
              <span className="text-[8px] bg-emerald-500/10 text-emerald-400 px-2 py-0.5 rounded border border-emerald-500/30 font-black">
                ZK_VAULT_V2
              </span>
            </CardTitle>
            <CardDescription className="text-zinc-500 text-[10px] font-mono uppercase tracking-tighter">
              Secure injection gate for distributed compute identities.
            </CardDescription>
          </div>
          <Shield className="w-5 h-5 text-zinc-800 group-hover:text-emerald-500/40 transition-colors" />
        </div>
      </CardHeader>

      <CardContent className="p-6 space-y-6 relative z-10">
        <form onSubmit={handleSubmit(onFormSubmit)} className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {/* PLATFORM SELECTOR */}
            <div className="space-y-2">
              <label className="text-[9px] font-black text-zinc-600 uppercase font-mono tracking-widest flex items-center gap-2">
                <div className="w-1 h-1 rounded-full bg-emerald-500" /> Target
                Runtime
              </label>
              <select
                {...register("platform")}
                className="w-full bg-black border border-zinc-800 rounded-md h-11 px-4 text-xs font-mono text-zinc-300 focus:border-emerald-500/50 outline-none transition-all cursor-pointer hover:bg-zinc-900/50"
              >
                <option value="google_colab">Google Colab (Tesla T4/L4)</option>
                <option value="kaggle">Kaggle Kernels (P100)</option>
                <option value="ideogram">Ideogram AI (Compute)</option>
              </select>
            </div>

            {/* EMAIL IDENTIFIER */}
            <div className="space-y-2">
              <label className="text-[9px] font-black text-zinc-600 uppercase font-mono tracking-widest flex items-center gap-2">
                <div className="w-1 h-1 rounded-full bg-emerald-500" /> Operator
                Email
              </label>
              <Input
                {...register("email")}
                placeholder="operator@prospector.io"
                className="bg-black border-zinc-800 text-xs h-11 focus:border-emerald-500/50 font-mono"
                hasError={!!errors.email}
              />
            </div>
          </div>

          {/* COOKIES TEXTAREA */}
          <div className="space-y-2">
            <div className="flex justify-between items-end mb-1">
              <label className="text-[9px] font-black text-zinc-600 uppercase font-mono tracking-widest">
                Identity Material (JSON Cookies)
              </label>
              {errors.cookiesJson && (
                <span className="text-[8px] text-red-500 font-bold animate-pulse uppercase">
                  :: {errors.cookiesJson.message} ::
                </span>
              )}
            </div>
            <div className="relative group/textarea">
              <textarea
                {...register("cookiesJson")}
                className={cn(
                  "w-full h-48 bg-black/50 border border-zinc-800 rounded-lg p-5 font-mono text-[10px] text-emerald-500 outline-none focus:border-emerald-500/40 transition-all resize-none custom-scrollbar shadow-inner",
                  errors.cookiesJson &&
                    "border-red-900/50 focus:border-red-500/50",
                )}
                placeholder="Paste [ { 'name': 'SID', 'value': '...' }, ... ]"
                spellCheck={false}
              />
              <div className="absolute bottom-3 right-4 flex items-center gap-2 text-[8px] text-zinc-700 font-mono font-bold select-none group-focus-within/textarea:text-emerald-500/50 transition-colors">
                <Lock className="w-3 h-3" /> CLIENT_SIDE_ENCRYPTION_ACTIVE
              </div>
            </div>
          </div>

          <Button
            type="submit"
            variant="cyber"
            className="w-full h-14 font-black tracking-[0.4em] uppercase text-xs"
            isLoading={mutation.isPending || isSubmitting}
          >
            {mutation.isPending ? (
              "PROTECTING_DATA..."
            ) : (
              <span className="flex items-center gap-4">
                <UploadCloud className="w-5 h-5" /> {t("secure_btn")}
              </span>
            )}
          </Button>
        </form>

        <div className="pt-6 border-t border-white/5 flex items-start gap-4 text-zinc-600">
          <div className="p-2 bg-zinc-900/50 rounded-lg border border-white/5">
            <Info className="w-4 h-4 text-emerald-500/50" />
          </div>
          <p className="text-[9px] font-mono leading-relaxed uppercase opacity-60">
            Compliance Notice: All identity material is sanitized and
            AES-256-GCM encrypted locally. Decryption keys are ephemeral and
            never persist on the Orchestrator's tactical database.
          </p>
        </div>
      </CardContent>
    </Card>
  );
}

// Sub-component Helper
function ShieldCheck(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      {...props}
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
      <path d="m9 12 2 2 4-4" />
    </svg>
  );
}
