import { z } from "zod";

/**
 * Primitiva para enlaces de navegación.
 */
export const LinkAtom = z.object({
  label: z.string().describe("Texto visible del enlace"),
  href: z.string().describe("Destino URL (interno o externo)"),
});

/**
 * Primitiva para botones de llamada a la acción (CTA).
 */
export const CTAAtom = z.object({
  label: z.string().describe("Texto del botón"),
  tooltip: z.string().optional().describe("Texto de ayuda flotante"),
});

/**
 * Esquema de textos globales del sistema.
 */
export const CommonSchema = z.object({
  loading: z.string().describe("Estado de carga genérico"),
  error: z.string().describe("Mensaje de error genérico"),
  copy: z.string().describe("Acción de copiar"),
  success: z.string().describe("Acción exitosa"),
  actions: z.object({
    confirm: z.string(),
    cancel: z.string(),
    back: z.string(),
  }),
});

export type CommonParams = z.infer<typeof CommonSchema>;
