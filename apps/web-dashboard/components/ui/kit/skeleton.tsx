import { cn } from "@/lib/utils/cn";

/**
 * APARATO: SKELETON
 * Placeholder animado para estados de carga asíncrona.
 * Simula el flujo de datos ("Pulse effect").
 */
function Skeleton({
  className,
  ...props
}: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn("animate-pulse rounded-md bg-muted/50", className)}
      {...props}
    />
  );
}

export { Skeleton };
