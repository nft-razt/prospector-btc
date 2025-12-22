/**
 * =================================================================
 * APARATO: FORENSIC COMMAND CENTER (V72.1 - TYPE SECURED)
 * CLASIFICACIÓN: FEATURE ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: MANDO Y CERTIFICACIÓN DEL ESTRATO CRIPTOGRÁFICO
 * =================================================================
 */

"use client";

import React, { useState, useCallback } from "react";
import {
  ShieldCheck,
  Zap,
  Terminal as TerminalIcon,
  Activity,
  Binary,
  Microscope,
  Play
} from "lucide-react";
import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";

// --- SINAPSIS DE INFRAESTRUCTURA ---
// ✅ RESOLUCIÓN ERROR 2305: Importación ahora reconocida por el Barrel V47.0
import { labApi, type CertificationIgnitionResponse } from "@prospector/api-client";

// --- ÁTOMOS UI (DESIGN SYSTEM) ---
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

export function ForensicCommandCenter() {
  const [is_certified_active, set_is_certified_active] = useState(false);
  const [tactical_logs, set_tactical_logs] = useState<string[]>([]);

  const push_tactical_log = useCallback((message: string) => {
    const timestamp = new Date().toLocaleTimeString();
    set_tactical_logs(prev => [`[${timestamp}] ${message}`, ...prev].slice(0, 12));
  }, []);

  /**
   * MUTACIÓN SOBERANA: Ignición de Misión de Certificación.
   */
  const ignition_mutation = useMutation({
    mutationFn: () => labApi.triggerCertificationMission(),
    onMutate: () => {
      push_tactical_log("INITIATING_HANDSHAKE: Requesting golden vector from L3...");
    },
    onSuccess: (data: CertificationIgnitionResponse) => {
      push_tactical_log(`IGNITION_SUCCESS: Mission ID ${data.mission_id.substring(0, 8)} secured.`);
      push_tactical_log("DNA_SYNC: Windows XP SP3 Performance Template verified.");

      set_is_certified_active(true);
      toast.success("IGNITION_SEQUENCE_ACTIVE", {
        description: "Certification mission has been injected into the dispatch buffer."
      });
    },
    onError: (error: Error) => {
      push_tactical_log(`❌ FATAL_ERROR: Ignition collapsed -> ${error.message}`);
      toast.error("IGNITION_FAILED");
    }
  });

  return (
    <Card className="bg-[#050505] border-primary/20 shadow-[0_0_60px_rgba(16,185,129,0.05)] overflow-hidden relative group">
      <CardHeader className="border-b border-white/5 bg-white/2 p-6">
        <div className="flex justify-between items-center relative z-20">
          <div className="space-y-1">
            <CardTitle className="text-xs font-black text-primary uppercase tracking-[0.4em] flex items-center gap-3 font-mono">
              <Microscope className="w-4 h-4" />
              Forensic Certification Gate
            </CardTitle>
            <p className="text-[9px] text-zinc-500 font-mono uppercase">Integrity Verification Strata // V10.8</p>
          </div>

          <div className={cn(
            "px-3 py-1 rounded-full border text-[9px] font-black font-mono transition-all duration-1000",
            is_certified_active
              ? "bg-primary/10 border-primary text-primary animate-pulse"
              : "bg-zinc-900 border-zinc-800 text-zinc-600"
          )}>
            {is_certified_active ? "CERTIFICATION_IN_PROGRESS" : "SYSTEM_READY"}
          </div>
        </div>
      </CardHeader>

      <CardContent className="p-8 space-y-8 relative z-20">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-10">
          <div className="space-y-6">
            <div className="bg-zinc-950 border border-white/5 p-6 rounded-xl space-y-5">
              <h4 className="text-[10px] font-black text-zinc-400 uppercase tracking-widest flex items-center gap-2">
                <Binary className="w-3 h-3 text-primary" /> Immutable Target DNA
              </h4>

              <div className="space-y-3 font-mono">
                <div className="flex justify-between text-[10px]">
                  <span className="text-zinc-600 uppercase">Snapshot_ID</span>
                  <span className="text-zinc-300">WIN_XP_SP3_GOLD</span>
                </div>
                <div className="flex justify-between text-[10px]">
                  <span className="text-zinc-600 uppercase">QPC_Vector</span>
                  <span className="text-primary font-bold">GENESIS_BLOCK_1</span>
                </div>
              </div>
            </div>

            <Button
              onClick={() => ignition_mutation.mutate()}
              disabled={is_certified_active || ignition_mutation.isPending}
              variant="cyber"
              className="w-full h-16 text-sm font-black tracking-[0.5em]"
              isLoading={ignition_mutation.isPending}
            >
              {!ignition_mutation.isPending && (
                <>
                  <Play className="w-4 h-4 mr-3 fill-primary group-hover:fill-black transition-colors" />
                  INITIATE_SMOKE_TEST
                </>
              )}
            </Button>
          </div>

          <div className="bg-black border border-zinc-800 rounded-xl p-5 h-[220px] flex flex-col font-mono shadow-inner">
            <div className="flex-1 overflow-y-auto space-y-2 custom-scrollbar">
              {tactical_logs.map((log, index) => (
                <p key={index} className="text-[9px] text-emerald-500/70 leading-tight">
                  <span className="opacity-30 mr-2">»</span>{log}
                </p>
              ))}
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
