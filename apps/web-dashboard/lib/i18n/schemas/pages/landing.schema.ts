import { z } from "zod";
import { CTAAtom } from "../common.schema";

/**
 * Esquema para una "Cápsula" (Tarjeta de decisión de entrada).
 */
const AuthCapsuleSchema = z.object({
  title: z.string().describe("Título principal de la tarjeta"),
  description: z.string().describe("Descripción de la propuesta de valor"),
  cta: z.string().describe("Texto del botón de acción"),
  badge: z.string().optional().describe("Etiqueta opcional (ej: 'Recomendado')"),
});

export const LandingPageSchema = z.object({
  meta: z.object({
    title: z.string().describe("Título SEO de la página"),
    description: z.string().describe("Descripción Meta SEO"),
  }),
  hero: z.object({
    badge: z.string().describe("Píldora de estado o versión (ej: V7.0 ONLINE)"),
    title: z.string().describe("Título principal H1"),
    subtitle: z.string().describe("Subtítulo descriptivo H2"),
    cta_primary: CTAAtom,
  }),
  capsules: z.object({
    login: AuthCapsuleSchema,
    register: AuthCapsuleSchema,
  }),
  pricing_preview: z.object({
    title: z.string(),
    observer_tier: z.string(),
    operator_tier: z.string(),
  }),
});

export type LandingPageParams = z.infer<typeof LandingPageSchema>;
