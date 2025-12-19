import { z } from "zod";

/**
 * Esquema para páginas de error del sistema (404, 500).
 */
export const SystemPagesSchema = z.object({
  not_found: z.object({
    title: z.string().describe("Título grande de error (ej: SIGNAL LOST)"),
    description: z.string().describe("Explicación amigable del error"),
    error_code: z.string().describe("Código técnico (ej: ERR_404_VOID)"),
    cta_return: z.string().describe("Texto del botón de regreso"),
  }),
  maintenance: z
    .object({
      title: z.string(),
      message: z.string(),
    })
    .optional(),
});

export type SystemParams = z.infer<typeof SystemPagesSchema>;
