/**
 * =================================================================
 * APARATO: IDENTITY INJECTOR (V60.0 - ZERO-KNOWLEDGE PORTABLE)
 * CLASIFICACIÓN: SECURITY COMPONENT (ESTRATO L5)
 * RESPONSABILIDAD: CIFRADO CLIENT-SIDE Y PROVISIÓN DE IDENTIDAD SOBERANA
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el protocolo de inyección segura para el enjambre Hydra.
 * Realiza un cifrado AES-256-GCM en el navegador del operador antes de
 * la transmisión. Esta versión utiliza 'encryptPortable' vinculando
 * el material criptográfico al email del operador, garantizando que
 * la identidad sea reconstruible únicamente con la combinación de la
 * llave maestra y la identidad del sujeto, cumpliendo con la política
 * de CERO REGRESIONES y máxima soberanía.
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
  ShieldCheck,
  ShieldAlert
} from "lucide-react";
import { useTranslations } from "next-intl";

// --- SINAPSIS DE INFRAESTRUCTURA (ESTRATOS L1, L2, L4) ---
import { VaultCryptoEngine } from "@prospector/crypto-vault";
import { adminApi } from "@prospector/api-client";
import { InjectionFormSchema, type InjectionFormValues } from "./schemas";

// --- ÁTOMOS UI (DESIGN SYSTEM) ---
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

/**
 * Organismo de inyección de identidades.
 * Orquesta el flujo: Validación -> Cifrado Local -> Persistencia Táctica.
 *
 * @returns {React.ReactElement} El panel de mando de la bóveda.
 */
export function IdentityInjector(): React.ReactElement {
  const t = useTranslations("Dashboard.vault");
  const query_client = useQueryClient();

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
   * MUTACIÓN DE INGESTIÓN SOBERANA
   * Ejecuta el protocolo de cifrado determinista y sincroniza con el Orquestador.
   */
  const ingestion_mutation = useMutation({
    mutationFn: async (form_data: InjectionFormValues) => {
      // 1. ADQUISICIÓN DE LLAVE DE PASO (Local Context)
      const master_passphrase = process.env.NEXT_PUBLIC_ADMIN_PASSWORD || "Netflix69";

      /**
       * 2. PROTOCOLO ZERO-KNOWLEDGE PORTABLE (L1 Sync)
       * RESOLUCIÓN: Pasamos el email como componente de entropía para la sal.
       */
      const encrypted_payload = await VaultCryptoEngine.encryptPortable(
        form_data.cookiesJson,
        master_passphrase,
        form_data.email
      );

      // 3. TRANSMISIÓN TÁCTICA HACIA EL MOTOR A (TURSO)
      await adminApi.uploadIdentity({
        platform: form_data.platform,
        email: form_data.email,
        cookies: encrypted_payload, // El payload es ahora un objeto tipado EncryptedVaultPayload
        userAgent: navigator.userAgent,
      });
    },
    onSuccess: () => {
      toast.success("VAULT_SYNCHRONIZED", {
        description: "The identity bunker has been secured in the Tactical Ledger.",
        icon: <ShieldCheck className="w-4 h-4 text-emerald-500" />,
      });
      // Invalidación de caché para refrescar el inventario de identidades
      query_client.invalidateQueries({ queryKey: ["identities"] });
      reset();
    },
    onError: (error: Error) => {
      console.error("🔥 [INJECTION_FAULT]:", error.message);
      toast.error("VAULT_FAILURE", {
        description: "Decryption handshake or database sync failed. Check Master Key.",
        icon: <ShieldAlert className="w-4 h-4 text-red-500" />,
      });
    },
  });

  /**
   * Procesa el envío del formulario tras la validación de Zod.
   */
  const on_handle_submit = (values: InjectionFormValues) => {
    ingestion_mutation.mutate(values);
  };

  return (
    <Card className="h-full bg-[#0a0a0a] border-zinc-800 relative overflow-hidden group shadow-2xl">
      {/* Visual Ambiance FX: Tailwind v4 Linear Gradient */}
      <div className="absolute inset-0 bg-linear-to-br from-emerald-500/5 to-transparent pointer-events-none opacity-40 group-hover:opacity-100 transition-opacity duration-1000" />

      <CardHeader className="relative z-10 border-b border-white/5 bg-white/2">
        <div className="flex justify-between items-start">
          <div className="space-y-1">
            <CardTitle className="flex items-center gap-3 text-white font-mono text-lg uppercase tracking-widest">
              <Key className="w-5 h-5 text-emerald-500" />
              {t("title")}
              <span className="text-[8px] bg-emerald-500/10 text-emerald-400 px-2 py-0.5 rounded border border-emerald-500/30 font-black">
                ZK_PORTABLE_V60
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
        <form onSubmit={handleSubmit(on_handle_submit)} className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">

            {/* SELECTOR DE PLATAFORMA DE CÓMPUTO */}
            <div className="space-y-2">
              <label className="text-[9px] font-black text-zinc-600 uppercase font-mono tracking-widest flex items-center gap-2">
                <div className="w-1 h-1 rounded-full bg-emerald-500" /> Target Runtime
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

            {/* IDENTIFICADOR DE OPERADOR (CRÍTICO PARA SAL) */}
            <div className="space-y-2">
              <label className="text-[9px] font-black text-zinc-600 uppercase font-mono tracking-widest flex items-center gap-2">
                <div className="w-1 h-1 rounded-full bg-emerald-500" /> Operator Identity (Email)
              </label>
              <Input
                {...register("email")}
                placeholder="operator@prospector.io"
                className={cn(
                  "bg-black border-zinc-800 text-xs h-11 focus:border-emerald-500/50 font-mono",
                  errors.email && "border-red-900/50"
                )}
                hasError={!!errors.email}
              />
            </div>
          </div>

          {/* ÁREA DE INYECCIÓN DE MATERIAL DE SESIÓN (COOKIES) */}
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
                  errors.cookiesJson && "border-red-900/50 focus:border-red-500/50"
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
            isLoading={ingestion_mutation.isPending || isSubmitting}
          >
            {ingestion_mutation.isPending ? (
              t("encrypting")
            ) : (
              <span className="flex items-center gap-4">
                <UploadCloud className="w-5 h-5" /> {t("secure_btn")}
              </span>
            )}
          </Button>
        </form>

        {/* CLÁUSULA DE PRIVACIDAD TÉCNICA */}
        <div className="pt-6 border-t border-white/5 flex items-start gap-4 text-zinc-600">
          <div className="p-2 bg-zinc-900/50 rounded-lg border border-white/5">
            <Info className="w-4 h-4 text-emerald-500/50" />
          </div>
          <p className="text-[9px] font-mono leading-relaxed uppercase opacity-60">
            Compliance Notice: All identity material is sanitized and AES-256-GCM encrypted locally.
            The master passphrase never leaves this terminal. This node identity will be
            automatically rotated upon detection of session expiration.
          </p>
        </div>
      </CardContent>
    </Card>
  );
}
