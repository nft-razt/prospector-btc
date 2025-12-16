import { z } from "zod";
import { LinkAtom } from "../common.schema";

export const PublicFooterSchema = z.object({
  copyright: z.string().describe("Texto de derechos de autor con placeholder de año"),
  links: z.object({
    privacy: LinkAtom,
    terms: LinkAtom,
    github: LinkAtom,
    documentation: LinkAtom,
  }),
  disclaimer: z.string().describe("Descargo de responsabilidad legal o académico"),
  status: z.object({
    label: z.string().describe("Etiqueta de estado del sistema (ej: System Status)"),
    operational: z.string().describe("Texto para estado 'Operativo'"),
    degraded: z.string().describe("Texto para estado 'Degradado'"),
  }),
});

export type PublicFooterParams = z.infer<typeof PublicFooterSchema>;
