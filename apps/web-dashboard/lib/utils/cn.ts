import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * UTILITY: CLASSNAME MERGER
 *
 * Fusiona clases de Tailwind resolviendo conflictos de especificidad en tiempo real.
 * Vital para permitir sobreescritura de estilos desde componentes padres.
 *
 * @param inputs - Lista variable de clases condicionales o strings.
 * @returns String de clases optimizado y deduplicado.
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
