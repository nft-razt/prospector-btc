"use client";

import { useEffect } from "react";
import { useTranslations } from "next-intl";
import { AlertOctagon, RotateCcw, ShieldAlert } from "lucide-react";
import { Button } from "@/components/ui/kit/button";
import { useHeimdall } from "@/hooks/use-heimdall";

interface ErrorProps {
  error: Error & { digest?: string };
  reset: () => void;
}

/**
 * APARATO: ROUTE ERROR BOUNDARY
 * Escudo de contención para fallos de renderizado en tiempo de ejecución.
 * Evita que un error en un widget rompa toda la aplicación.
 */
export default function DashboardError({ error, reset }: ErrorProps) {
  const t = useTranslations("Common"); // Asumiendo claves genéricas
  const logger = useHeimdall("ErrorBoundary");

  useEffect(() => {
    // Reportar el incidente a Heimdall (Console en Dev, JSON en Prod)
    logger.error("CRITICAL RENDER FAILURE", {
      message: error.message,
      digest: error.digest,
      stack: error.stack,
    });
  }, [error, logger]);

  return (
    <div className="h-full w-full flex flex-col items-center justify-center min-h-[400px] p-6 text-center animate-in fade-in zoom-in-95 duration-300">
      {/* Iconografía de Alerta */}
      <div className="relative mb-6 group">
        <div className="absolute inset-0 bg-red-500/20 blur-xl rounded-full opacity-50 group-hover:opacity-100 transition-opacity" />
        <div className="relative bg-black border border-red-500/50 p-4 rounded-2xl shadow-[0_0_30px_rgba(220,38,38,0.2)]">
          <AlertOctagon className="w-12 h-12 text-red-500" />
        </div>
      </div>

      <h2 className="text-2xl font-black text-white tracking-widest uppercase mb-2">
        System Malfunction
      </h2>

      <div className="bg-red-950/30 border border-red-900/50 px-4 py-2 rounded mb-6 font-mono text-[10px] text-red-300">
        ERR_DIGEST: {error.digest || "UNKNOWN_EXCEPTION"}
      </div>

      <p className="text-slate-400 max-w-md mb-8 text-sm leading-relaxed">
        The interface encountered an unrecoverable state while processing the
        visual feed. Navigation systems remain operational.
      </p>

      <div className="flex gap-4">
        <Button
          onClick={reset}
          variant="default"
          className="bg-red-600 hover:bg-red-500 text-white shadow-[0_0_20px_rgba(220,38,38,0.4)]"
        >
          <RotateCcw className="w-4 h-4 mr-2" />
          ATTEMPT REBOOT
        </Button>

        <Button
          variant="outline"
          onClick={() => window.location.reload()}
          className="border-slate-700 hover:bg-slate-800"
        >
          <ShieldAlert className="w-4 h-4 mr-2" />
          HARD RELOAD
        </Button>
      </div>

      <div className="mt-12 text-[10px] text-slate-600 font-mono">
        PROSPECTOR SAFETY PROTOCOL // ISO-9001
      </div>
    </div>
  );
}
