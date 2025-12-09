import React from 'react';
import { Server } from 'lucide-react';
import { type WorkerHeartbeat } from '@prospector/api-client';

interface TelemetryTableProps {
  workers: WorkerHeartbeat[];
  isLoading: boolean;
}

export function TelemetryTable({ workers, isLoading }: TelemetryTableProps) {
  // Lógica de renderizado de la tabla (copiada y limpiada del page.tsx original)
  return (
    <div className="bg-[#0f0f0f] border border-slate-800 rounded-xl overflow-hidden shadow-2xl">
        {/* ... Header ... */}
        <div className="overflow-x-auto">
            <table className="w-full text-left text-sm">
                {/* ... Thead y Tbody ... */}
            </table>
        </div>
    </div>
  );
}
