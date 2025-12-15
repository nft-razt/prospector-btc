/**
 * =================================================================
 * APARATO: STAT CARD (HUD ELEMENT)
 * CLASIFICACIÓN: ATOMIC UI
 * RESPONSABILIDAD: VISUALIZACIÓN DE MÉTRICA INDIVIDUAL
 * =================================================================
 */

import { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils/cn";
import { Card } from "@/components/ui/kit/card";

interface StatCardProps {
  label: string;
  value: string | number;
  subValue?: string;
  icon: LucideIcon;
  trend?: "up" | "down" | "neutral";
  color?: "emerald" | "blue" | "purple" | "amber";
  loading?: boolean;
}

export function StatCard({
  label,
  value,
  subValue,
  icon: Icon,
  color = "blue",
  loading = false,
}: StatCardProps) {
  const colorStyles = {
    emerald: "text-emerald-500 bg-emerald-500/10 border-emerald-500/20",
    blue: "text-blue-500 bg-blue-500/10 border-blue-500/20",
    purple: "text-purple-500 bg-purple-500/10 border-purple-500/20",
    amber: "text-amber-500 bg-amber-500/10 border-amber-500/20",
  };

  return (
    <Card className="relative overflow-hidden bg-black/40 border-white/5 backdrop-blur-sm group hover:border-white/10 transition-colors">
      {/* Background Glow Effect */}
      <div
        className={cn(
          "absolute -right-6 -top-6 h-24 w-24 rounded-full blur-3xl opacity-20 transition-opacity group-hover:opacity-30",
          colorStyles[color].split(" ")[0].replace("text-", "bg-"),
        )}
      />

      <div className="p-5 flex items-start justify-between">
        <div className="space-y-1 relative z-10">
          <p className="text-[10px] font-mono uppercase tracking-widest text-zinc-500 font-bold">
            {label}
          </p>

          {loading ? (
            <div className="h-8 w-24 bg-zinc-800/50 rounded animate-pulse my-1" />
          ) : (
            <h3 className="text-2xl font-black text-white tracking-tight tabular-nums">
              {value}
            </h3>
          )}

          {subValue && (
            <p className="text-[10px] font-mono text-zinc-600">{subValue}</p>
          )}
        </div>

        <div
          className={cn(
            "p-2 rounded-lg border shadow-[0_0_15px_rgba(0,0,0,0.5)]",
            colorStyles[color],
          )}
        >
          <Icon className="w-5 h-5" />
        </div>
      </div>

      {/* Decorative Tech Line */}
      <div className="absolute bottom-0 left-0 h-[2px] w-full bg-linear-to-r from-transparent via-white/5 to-transparent opacity-50" />
    </Card>
  );
}
