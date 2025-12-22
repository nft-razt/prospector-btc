/**
 * =================================================================
 * APARATO: SWARM INTELLIGENCE HEATMAP (V90.0 - HIGH DENSITY)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN ESPACIAL DEL ESFUERZO DE MINERÍA
 *
 * ESTRATEGIA DE ÉLITE:
 * - Hardware Acceleration: Uso de Canvas 2D con buffer de dibujo optimizado.
 * - Spatial Virtualization: Mapeo de 2^256 a una rejilla de 10,000 celdas.
 * - Bloom FX: Efecto de resplandor en zonas de alta actividad computacional.
 * =================================================================
 */

"use client";

import React, { useRef, useEffect, useMemo } from "react";
import { type SwarmHeatmapSegment } from "@prospector/api-contracts";
import { useNeuralLink } from "@prospector/api-client";
import { Box, Target, Zap } from "lucide-react";

interface CurveHeatmapProps {
  grid_resolution?: number; // Ej: 100 para una rejilla de 100x100
}

export function CurveHeatmap({ grid_resolution = 100 }: CurveHeatmapProps) {
  const canvas_reference = useRef<HTMLCanvasElement>(null);
  const { heatmap_data } = useNeuralLink(); // Hook nivelado para recibir segmentos

  /**
   * MOTOR DE RENDERIZADO TÁCTICO
   * Dibuja la ocupación del espacio secp256k1 en tiempo real.
   */
  useEffect(() => {
    const canvas = canvas_reference.current;
    if (!canvas) return;

    const context = canvas.getContext("2d");
    if (!context) return;

    const width = canvas.width;
    const height = canvas.height;
    const cell_size = width / grid_resolution;

    // 1. LIMPIEZA DE FRAME CON EFECTO DE PERSISTENCIA (Trail)
    context.fillStyle = "rgba(5, 5, 5, 0.15)"; // Rastro sutil
    context.fillRect(0, 0, width, height);

    // 2. DIBUJO DE REJILLA DE FONDO (Grid Architecture)
    context.strokeStyle = "rgba(16, 185, 129, 0.05)";
    context.lineWidth = 0.5;
    for (let i = 0; i <= grid_resolution; i++) {
      context.beginPath();
      context.moveTo(i * cell_size, 0);
      context.lineTo(i * cell_size, height);
      context.stroke();
      context.beginPath();
      context.moveTo(0, i * cell_size);
      context.lineTo(width, i * cell_size);
      context.stroke();
    }

    // 3. RENDERIZADO DE SEGMENTOS ACTIVOS (The Swarm Pulse)
    heatmap_data.forEach((segment: SwarmHeatmapSegment) => {
      // Mapeo unidimensional [0,1] a coordenadas 2D (Escaneo de izquierda a derecha)
      const total_cells = grid_resolution * grid_resolution;
      const target_cell_index = Math.floor(segment.normalized_start * total_cells);

      const x_coordinate = (target_cell_index % grid_resolution) * cell_size;
      const y_coordinate = Math.floor(target_cell_index / grid_resolution) * cell_size;

      // Estilo de Ignición: Gradiente basado en intensidad
      const alpha_channel = 0.3 + (segment.intensity * 0.7);
      context.shadowBlur = 10 * segment.intensity;
      context.shadowColor = "#10b981";
      context.fillStyle = `rgba(16, 185, 129, ${alpha_channel})`;

      context.fillRect(
        x_coordinate + 1,
        y_coordinate + 1,
        cell_size - 2,
        cell_size - 2
      );

      context.shadowBlur = 0; // Reset para el siguiente
    });
  }, [heatmap_data, grid_resolution]);

  return (
    <div className="bg-[#0a0a0a] border border-zinc-800 rounded-2xl p-6 space-y-6 relative overflow-hidden shadow-2xl group">
      {/* HUD DE METADATOS ESPACIALES */}
      <div className="flex justify-between items-center relative z-10">
        <div className="space-y-1">
          <h3 className="text-[10px] font-black text-emerald-500 uppercase tracking-[0.4em] font-mono flex items-center gap-3">
            <Target className="w-4 h-4 animate-pulse" />
            Keyspace Exploration Matrix
          </h3>
          <p className="text-[8px] text-zinc-500 font-mono uppercase">Proyected Strata: 2^256 Secp256k1 Curve</p>
        </div>

        <div className="flex items-center gap-4">
          <div className="flex flex-col items-end">
            <span className="text-[8px] text-zinc-600 font-bold uppercase font-mono">Grid Accuracy</span>
            <span className="text-[10px] text-zinc-300 font-mono font-black">1.2e-72 bits/px</span>
          </div>
          <Zap className="w-5 h-5 text-amber-500" />
        </div>
      </div>

      {/* LIENZO DE AUDITORÍA (CANVAS) */}
      <div className="relative aspect-square w-full bg-black rounded-lg border border-white/5 overflow-hidden">
        <canvas
          ref={canvas_reference}
          width={800}
          height={800}
          className="w-full h-full cursor-crosshair transition-opacity duration-1000"
        />

        {/* Marcadores de Frontera */}
        <div className="absolute top-2 left-2 px-2 py-1 bg-black/80 border border-white/10 rounded text-[7px] font-mono text-zinc-500">
          START_OFFSET: 0x00...00
        </div>
        <div className="absolute bottom-2 right-2 px-2 py-1 bg-black/80 border border-white/10 rounded text-[7px] font-mono text-zinc-500">
          END_OFFSET: 0xFF...FF
        </div>
      </div>

      {/* LEYENDA TÉCNICA */}
      <div className="pt-4 border-t border-white/5 flex justify-between items-center">
        <div className="flex gap-6">
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 rounded-sm bg-emerald-500/20 border border-emerald-500/50" />
            <span className="text-[8px] font-black text-zinc-600 uppercase font-mono">Idle Range</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 rounded-sm bg-emerald-500 shadow-[0_0_8px_#10b981]" />
            <span className="text-[8px] font-black text-zinc-400 uppercase font-mono">Active Swarm Audit</span>
          </div>
        </div>

        <span className="text-[8px] font-bold text-zinc-800 font-mono uppercase tracking-[0.2em]">
          Spatial Intelligence Stratum V2.5
        </span>
      </div>
    </div>
  );
}
