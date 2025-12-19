/**
 * =================================================================
 * APARATO: MANUAL VERIFIER (THE INTERCEPTOR V30.0)
 * CLASIFICACIÓN: FEATURE UI (L5)
 * RESPONSABILIDAD: PRUEBA DE CONCEPTO CRIPTOGRÁFICA EN TIEMPO REAL
 *
 * ESTRATEGIA DE ÉLITE:
 * - Engine: Interacción directa con el motor matemático secp256k1 (Rust) vía API.
 * - Style: Aplicación de clases canónicas Tailwind v4 (shrink-0).
 * - Security: Validación de entrada y visualización segura de identidades derivadas.
 * - UX: Feedback táctico mediante estados de mutación y notificaciones reactivas.
 * =================================================================
 */

"use client";

import React, { useState } from "react";
import { useMutation } from "@tanstack/react-query";
import {
  Search,
  ShieldCheck,
  AlertCircle,
  Fingerprint,
  Cpu,
  RefreshCw,
} from "lucide-react";
import { toast } from "sonner";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { labApi } from "@prospector/api-client";
import { type EntropyResult } from "@prospector/api-contracts";

// --- COMPONENTES ATÓMICOS (UI KIT) ---
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@/components/ui/kit/card";
import { Input } from "@/components/ui/kit/input";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

/**
 * Organismo de inspección manual "The Interceptor".
 * Permite auditar frases semilla o claves hexadecimales contra el censo de objetivos.
 */
export function ManualVerifier() {
  /** Vector de entropía ingresado por el operador. */
  const [entropyVector, setEntropyVector] = useState<string>("");

  /**
   * Mutación para la verificación de entropía.
   * Realiza un handshake con el Orquestador para derivar material criptográfico.
   */
  const verificationMutation = useMutation<EntropyResult, Error, string>({
    mutationKey: ["cryptographic-interceptor-scan"],
    mutationFn: (secret: string) =>
      labApi.verifyEntropy({
        secret,
        type: "phrase",
      }),
    onSuccess: (data) => {
      if (data.is_target) {
        toast.success("🎯 COLLISION_DETECTED", {
          description: `Entropy linked to experiment: ${data.matched_scenario}`,
          duration: 6000,
        });
      } else {
        toast.info("SCAN_COMPLETE", {
          description:
            "Vector processed. No collisions found in the active ledger.",
        });
      }
    },
    onError: (error: Error) => {
      console.error("🔥 [INTERCEPTOR_CRITICAL]:", error.message);
      toast.error("NEURAL_LINK_FAILURE", {
        description: "The cryptographic engine is unresponsive or timed out.",
      });
    },
  });

  /** Resultado procesado de la última verificación exitosa. */
  const analysisResult = verificationMutation.data;

  /** Ejecuta la secuencia de escaneo. */
  const triggerScan = () => {
    if (entropyVector && !verificationMutation.isPending) {
      verificationMutation.mutate(entropyVector);
    }
  };

  return (
    <Card className="bg-[#0a0a0a] border-zinc-800 flex flex-col shadow-2xl relative overflow-hidden group">
      {/* Visual Glitch Layer (Ambientación Técnica) */}
      <div className="absolute top-0 right-0 p-4 opacity-5 group-hover:opacity-15 transition-opacity duration-1000 pointer-events-none">
        <Cpu className="w-16 h-16 text-blue-500 animate-pulse" />
      </div>

      <CardHeader className="border-b border-white/5 bg-white/2 p-4 relative z-10">
        <CardTitle className="text-[10px] font-black text-blue-400 uppercase tracking-[0.2em] flex items-center gap-3 font-mono">
          <Search className="w-4 h-4" />
          Neural Interceptor // Manual Scan
        </CardTitle>
      </CardHeader>

      <CardContent className="p-6 space-y-6 flex-1 relative z-10">
        {/* PANEL DE ENTRADA */}
        <div className="space-y-3">
          <label className="text-[9px] uppercase font-black text-zinc-600 font-mono tracking-widest flex justify-between">
            <span>Entropy Source (Phrase/Hex)</span>
            {verificationMutation.isPending && (
              <span className="text-blue-500 flex items-center gap-1.5">
                <RefreshCw className="w-2.5 h-2.5 animate-spin" />{" "}
                ANALYZING_VECTOR...
              </span>
            )}
          </label>
          <div className="flex gap-2">
            <Input
              value={entropyVector}
              onChange={(event) => setEntropyVector(event.target.value)}
              placeholder="e.g. correct horse battery staple"
              className="font-mono text-xs bg-black/50 border-zinc-800 focus:border-blue-500/50 text-zinc-300 h-11 transition-all"
              onKeyDown={(event) => event.key === "Enter" && triggerScan()}
              disabled={verificationMutation.isPending}
            />
            <Button
              variant="cyber"
              className="border-blue-500/40 text-blue-400 hover:bg-blue-500 hover:text-black px-6 font-bold"
              onClick={triggerScan}
              isLoading={verificationMutation.isPending}
              disabled={!entropyVector || verificationMutation.isPending}
            >
              SCAN
            </Button>
          </div>
        </div>

        {/* HUD DINÁMICO: RESULTADO DEL ANÁLISIS FORENSE */}
        {analysisResult && (
          <div
            className={cn(
              "rounded-xl border p-5 space-y-4 transition-all duration-500 animate-in fade-in slide-in-from-bottom-4",
              analysisResult.is_target
                ? "bg-emerald-950/20 border-emerald-500/50 shadow-[0_0_30px_rgba(16,185,129,0.1)]"
                : "bg-zinc-900/40 border-zinc-800",
            )}
          >
            <div className="flex items-center justify-between">
              <span className="text-[8px] font-black uppercase text-zinc-600 tracking-tighter font-mono">
                Cryptographic Identity
              </span>
              {analysisResult.is_target ? (
                <div className="flex items-center gap-2 px-2 py-0.5 bg-emerald-500/20 border border-emerald-500/40 rounded text-[9px] font-black text-emerald-400">
                  <ShieldCheck className="w-3 h-3" /> TARGET_MATCHED
                </div>
              ) : (
                <div className="flex items-center gap-2 px-2 py-0.5 bg-zinc-800 border border-zinc-700 rounded text-[9px] font-black text-zinc-500">
                  <AlertCircle className="w-3 h-3" /> NO_COLLISION
                </div>
              )}
            </div>

            <div className="space-y-4 font-mono">
              {/* Dirección Pública Derivada */}
              <div className="space-y-1.5">
                <span className="text-[7px] text-zinc-500 uppercase font-bold">
                  Public Key (Base58)
                </span>
                <div className="flex items-center gap-2 text-[10px] text-zinc-300 bg-black/40 p-2.5 rounded border border-white/5">
                  {/* ✅ CANONICAL CLASS RESOLVED: flex-shrink-0 -> shrink-0 */}
                  <Fingerprint className="w-3.5 h-3.5 text-zinc-600 shrink-0" />
                  <span className="truncate select-all">
                    {analysisResult.address}
                  </span>
                </div>
              </div>

              {/* Escenario Vinculado (Si aplica) */}
              {analysisResult.matched_scenario && (
                <div className="pt-3 border-t border-emerald-500/20">
                  <div className="text-[9px] text-emerald-500/80 font-black uppercase mb-1">
                    Coded Origin
                  </div>
                  <div className="text-xs text-emerald-400 font-bold tracking-tight">
                    ↳ {analysisResult.matched_scenario}
                  </div>
                </div>
              )}
            </div>
          </div>
        )}
      </CardContent>

      {/* PIE DE PÁGINA TÉCNICO */}
      <div className="p-3 border-t border-white/5 bg-black/40">
        <p className="text-[8px] text-center text-zinc-700 font-mono tracking-widest uppercase">
          Neural-Link V8.0 // Secure Discrete Log Handshake
        </p>
      </div>
    </Card>
  );
}
