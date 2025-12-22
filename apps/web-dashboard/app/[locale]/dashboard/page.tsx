/**
 * =================================================================
 * APARATO: MISSION CONTROL DASHBOARD (V100.0 - HEATMAP ENABLED)
 * CLASIFICACIÓN: VIEW LAYER (ESTRATO L5)
 * RESPONSABILIDAD: ORQUESTACIÓN FINAL DE LA INTERFAZ DE MANDO
 * =================================================================
 */

import React from "react";
import { SystemMonitor } from "@/components/features/monitoring/system-monitor";
import { CurveHeatmap } from "@/components/features/monitoring/curve-heatmap";
import { AuditTrailHUD } from "@/components/features/monitoring/audit-trail-hud";
import { SwarmLauncher } from "@/components/features/control/swarm-launcher";
import { IdentityVault } from "@/components/features/identity/identity-vault";

export default function DashboardPage() {
  return (
    <div className="space-y-10 animate-in fade-in duration-1000 pb-20">

      {/* SECCIÓN 1: TELEMETRÍA GLOBAL Y MAPA DE CALOR */}
      <div className="grid grid-cols-1 xl:grid-cols-12 gap-10">

        {/* COLUMNA IZQUIERDA: MÉTRICAS Y MAPA (EL CEREBRO) */}
        <div className="xl:col-span-8 space-y-10">
          <SystemMonitor />

          <div className="grid grid-cols-1 md:grid-cols-2 gap-10">
             <CurveHeatmap grid_resolution={80} />
             <div className="h-full">
                <AuditTrailHUD />
             </div>
          </div>
        </div>

        {/* COLUMNA DERECHA: CONTROL Y SEGURIDAD (EL MANDO) */}
        <div className="xl:col-span-4 space-y-10">
          <SwarmLauncher />
          <IdentityVault />
        </div>

      </div>

      {/* FOOTER TÉCNICO SOBERANO */}
      <footer className="pt-10 border-t border-white/5 opacity-20 hover:opacity-100 transition-opacity">
        <p className="text-[9px] text-center font-mono uppercase tracking-[0.5em] text-zinc-500">
          Prospector Distributed Engine // Unified Jacobian Stratum // V10.6 Operational
        </p>
      </footer>
    </div>
  );
}
