"use client";

import { usePathname } from "next/navigation";
import { Fragment } from "react";
import { ChevronRight, Home } from "lucide-react";
import { cn } from "@/lib/utils/cn";

/**
 * ÁTOMO: BREADCRUMBS
 * Genera la ruta visual basada en la URL actual, ignorando el locale.
 */
export function Breadcrumbs() {
  const pathname = usePathname();

  // Limpieza de ruta: eliminar locale y dividir
  const segments = pathname
    .split("/")
    .filter(Boolean)
    .filter((segment) => !["en", "es"].includes(segment));

  const title =
    segments.length > 0
      ? segments[segments.length - 1].replace(/-/g, " ")
      : "Dashboard";

  return (
    <div className="flex flex-col justify-center">
      <h2 className="text-lg font-bold text-foreground capitalize tracking-tight leading-none font-mono">
        {title}
      </h2>

      <div className="flex items-center gap-1 text-[10px] text-muted-foreground font-mono uppercase mt-1">
        <Home className="w-3 h-3 opacity-50" />
        <span className="opacity-50">ROOT</span>

        {segments.map((segment, idx) => (
          <Fragment key={segment}>
            <ChevronRight className="w-3 h-3 text-primary/40" />
            <span
              className={cn(
                "tracking-wider",
                idx === segments.length - 1
                  ? "text-primary font-bold"
                  : "opacity-50",
              )}
            >
              {segment.replace(/-/g, " ")}
            </span>
          </Fragment>
        ))}
      </div>
    </div>
  );
}
