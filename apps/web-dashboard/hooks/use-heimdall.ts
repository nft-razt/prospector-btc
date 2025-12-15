import { useMemo } from "react";
import { createLogger } from "@prospector/heimdall-ts";

/**
 * APARATO: HEIMDALL HOOK
 * Provee una instancia del logger configurada con el contexto del componente.
 * Memoizado para evitar recreación de instancias en cada render.
 *
 * @param context - Nombre del componente o módulo (ej: 'Sidebar', 'AuthGuard').
 */
export function useHeimdall(context: string) {
  const logger = useMemo(() => createLogger(context), [context]);
  return logger;
}
