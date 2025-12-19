import { z } from "zod";
import { LinkAtom } from "../common.schema";

export const PublicHeaderSchema = z.object({
  brand: z.string().describe("Nombre de la marca o aplicación"),
  nav: z.object({
    features: z.string().describe("Enlace a características"),
    pricing: z.string().describe("Enlace a precios"),
    about: z.string().describe("Enlace a acerca de"),
  }),
  actions: z.object({
    login: z.string().describe("Botón de inicio de sesión"),
    get_started: z.string().describe("Botón de registro principal"),
  }),
  // Espacio para futuras alertas globales en el header
  banner: z
    .object({
      text: z.string().optional(),
      link: LinkAtom.optional(),
    })
    .optional(),
});

export type PublicHeaderParams = z.infer<typeof PublicHeaderSchema>;
