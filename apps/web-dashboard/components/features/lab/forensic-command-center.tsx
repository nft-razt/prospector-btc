/**
 * =================================================================
 * APARATO: FORENSIC COMMAND CENTER (V14.9 - TYPE SECURED)
 * CLASIFICACIÓN: FEATURE ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: MANDO Y CERTIFICACIÓN DEL ESTRATO CRIPTOGRÁFICO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la interfaz de ignición para misiones de humo.
 * ✅ RESOLUCIÓN TS17002: Corrección de la simetría de cierre en CardHeader.
 * =================================================================
 */

"use client";

import React, { useState, useCallback } from "react";
import { Microscope, Play, Binary } from "lucide-react";
import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";
import { labApi, type CertificationIgnitionResponse } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";

export function ForensicCommandCenter(): React.ReactElement {
  const [tactical_logs, set_tactical_logs] = useState<string[]>([]);

  const push_tactical_log = useCallback((message: string) => {
    set_tactical_logs(prev => [`[${new Date().toLocaleTimeString()}] ${message}`, ...prev].slice(0, 10));
  }, []);

  const ignition_mutation = useMutation({
    mutationFn: () => labApi.triggerCertificationMission(),
    onSuccess: (response: CertificationIgnitionResponse) => {
      push_tactical_log(`IGNITION_SUCCESS: Mission_${response.mission_identifier.substring(0, 8)}`);
      toast.success("MISSION_IGNITED", { description: "Certification sequence active." });
    },
    onError: (error: Error) => {
      push_tactical_log(`FATAL_ERROR: ${error.message}`);
      toast.error("IGNITION_FAILED");
    }
  });

  return (
    <Card className="bg-[#050505] border-primary/20 shadow-2xl relative overflow-hidden">
      {/* ✅ RESOLUCIÓN TS17002: Sellado simétrico del Header */}
      <CardHeader className="border-b border-white/5 bg-white/2 p-6">
        <CardTitle className="text-xs font-black text-primary uppercase tracking-[0.4em] font-mono flex items-center gap-3">
          <Microscope className="w-4 h-4" />
          Forensic_Control_Gate
        </CardTitle>
      </CardHeader>

      <CardContent className="p-8 space-y-8">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-10">
          <div className="space-y-6">
            <div className="bg-zinc-950 border border-white/5 p-6 rounded-2xl">
              <h4 className="text-[10px] font-black text-zinc-500 uppercase tracking-widest flex items-center gap-2 mb-4">
                <Binary className="w-3 h-3 text-primary" /> Target_Vector_DNA
              </h4>
              <div className="space-y-2 font-mono text-[10px]">
                <div className="flex justify-between">
                  <span className="text-zinc-600 uppercase">Snapshot_ID</span>
                  <span className="text-zinc-300">WIN_XP_SP3_GOLD</span>
                </div>
              </div>
            </div>

            <Button
              onClick={() => ignition_mutation.mutate()}
              disabled={ignition_mutation.isPending}
              variant="cyber"
              className="w-full h-16 text-sm font-black tracking-[0.5em]"
              isLoading={ignition_mutation.isPending}
            >
              <Play className="w-4 h-4 mr-3 fill-primary" />
              INITIATE_SMOKE_TEST
            </Button>
          </div>

          <div className="bg-black border border-zinc-800 rounded-2xl p-5 h-48 overflow-y-auto custom-scrollbar font-mono text-[9px] text-emerald-500/70">
            {tactical_logs.map((log, index) => (
              <p key={index} className="mb-1 leading-tight">{log}</p>
            ))}
            {tactical_logs.length === 0 && <p className="text-zinc-800 uppercase animate-pulse">Awaiting_Command...</p>}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
