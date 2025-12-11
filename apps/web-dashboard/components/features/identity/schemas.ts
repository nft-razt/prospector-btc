import { z } from 'zod';

/**
 * Esquema de validación para una cookie individual (Formato EditThisCookie/Playwright).
 */
const CookieSchema = z.object({
  domain: z.string(),
  name: z.string(),
  value: z.string(),
  path: z.string().optional(),
}).passthrough(); // Permitimos campos extra pero validamos los esenciales

/**
 * Esquema del formulario de inyección.
 * Valida que el JSON sea parseable Y tenga la estructura correcta.
 */
export const InjectionFormSchema = z.object({
  platform: z.enum(['google_colab', 'kaggle', 'ideogram']),
  email: z.string().email("Invalid email format"),

  // El input es un string (textarea), pero debe ser JSON válido de cookies
  cookiesJson: z.string()
    .min(10, "Cookies cannot be empty")
    .refine((val) => {
      try {
        const parsed = JSON.parse(val);
        return Array.isArray(parsed) && parsed.length > 0;
      } catch {
        return false;
      }
    }, { message: "Invalid JSON format or empty array" })
});

export type InjectionFormValues = z.infer<typeof InjectionFormSchema>;
