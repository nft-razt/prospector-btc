/**
 * =================================================================
 * APARATO: SWARM INTELLIGENCE HEATMAP (V11.8 - GOLD MASTER)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN ESPACIAL DEL ESFUERZO DE MINERÍA
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la rejilla de exploración de la curva secp256k1.
 * Mapea segmentos de búsqueda a coordenadas 2D. Resuelve errores
 * de tipado mediante el uso de 'normalized_start_position'
 * e 'intensity_weight' según el contrato soberano.
 * =================================================================
 */

"use client";

import React, { useRef, useEffect } from "react";
import { type SwarmHeatmapSegment } from "@prospector/api-contracts";
import { useNeuralLink } from "@prospector/api-client";
import { Target, Zap } from "lucide-react";

interface CurveHeatmapProperties {
  /** Resolución de la rejilla (ej: 80x80). */
  grid_resolution?: number;
}

export function CurveHeatmap({ grid_resolution = 100 }: CurveHeatmapProperties): React.ReactElement {
  const canvas_reference = useRef<HTMLCanvasElement>(null);
  const { heatmap_data } = useNeuralLink();

  useEffect(() => {
    const canvas = canvas_reference.current;
    if (!canvas) return;

    const context = canvas.getContext("2d");
    if (!context) return;

    const width = canvas.width;
    const height = canvas.height;
    const cell_size = width / grid_resolution;

    // Limpieza con rastro persistente
    context.fillStyle = "rgba(5, 5, 5, 0.2)";
    context.fillRect(0, 0, width, height);

    // Dibujo de rejilla base
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

    /**
     * RENDERIZADO DE SEGMENTOS ACTIVOS
     * ✅ RESOLUCIÓN TS2339: Uso de nombres nominales completos.
     */
    heatmap_data.forEach((segment: SwarmHeatmapSegment) => {
      const total_cells = grid_resolution * grid_resolution;
      const target_cell_index = Math.floor(segment.normalized_start_position * total_cells);

      const x_coordinate = (target_cell_index % grid_resolution) * cell_size;
      const y_coordinate = Math.floor(target_cell_index / grid_resolution) * cell_size;

      const alpha_channel = 0.3 + (segment.intensity_weight * 0.7);
      context.shadowBlur = 12 * segment.intensity_weight;
      context.shadowColor = "#10b981";
      context.fillStyle = `rgba(16, 185, 129, ${alpha_channel})`;

      context.fillRect(
        x_coordinate + 1,
        y_coordinate + 1,
        cell_size - 2,
        cell_size - 2
      );
      context.shadowBlur = 0;
    });
  }, [heatmap_data, grid_resolution]);

  return (
    <div className="bg-[#0a0a0a] border border-zinc-800 rounded-2xl p-6 space-y-6 relative overflow-hidden shadow-2xl group">
      <div className="flex justify-between items-center relative z-10">
        <div className="space-y-1">
          <h3 className="text-[10px] font-black text-emerald-500 uppercase tracking-[0.4em] font-mono flex items-center gap-3">
            <Target className="w-4 h-4 animate-pulse" />
            Keyspace Exploration Matrix
          </h3>
          <p className="text-[8px] text-zinc-500 font-mono uppercase tracking-widest">Stratum: 2^256 secp256k1</p>
        </div>
        <Zap className="w-5 h-5 text-amber-500" />
      </div>

      <div className="relative aspect-square w-full bg-black rounded-lg border border-white/5 overflow-hidden">
        <canvas
          ref={canvas_reference}
          width={800}
          height={800}
          className="w-full h-full cursor-crosshair transition-opacity duration-1000"
        />
      </div>

      <footer className="pt-4 border-t border-white/5 flex justify-between items-center">
        <span className="text-[8px] font-bold text-zinc-800 font-mono uppercase tracking-[0.2em]">
          Spatial Intelligence Stratum V3.0
        </span>
      </footer>
    </div>
  );
}
