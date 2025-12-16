// apps/web-dashboard/lib/i18n/schema.ts
/**
 * =================================================================
 * APARATO: I18N ROOT SCHEMA (AGGREGATOR)
 * RESPONSABILIDAD: VALIDACIÓN ESTRICTA Y RECURSIVA DEL DICCIONARIO
 * ESTADO: STRICT TYPED (NO ANY ALLOWED)
 * =================================================================
 */

import { z } from "zod";

// Importación de Átomos Granulares
import { CommonSchema } from "./schemas/common.schema";
import { PublicHeaderSchema } from "./schemas/layout/public-header.schema";
import { PublicFooterSchema } from "./schemas/layout/public-footer.schema";
import { LandingPageSchema } from "./schemas/pages/landing.schema";
import { DashboardSchema } from "./schemas/dashboard/dashboard.schema";
import { AuthSchema } from "./schemas/auth/auth.schema";
import { SystemPagesSchema } from "./schemas/system/system.schema";

/**
 * ESQUEMA RAÍZ (LA LEY SUPREMA DEL I18N)
 * Valida la estructura completa del diccionario generado.
 * Si un desarrollador agrega una clave en código pero no en el esquema, el build fallará.
 */
export const AppLocaleSchema = z.object({
  Common: CommonSchema,
  PublicHeader: PublicHeaderSchema,
  PublicFooter: PublicFooterSchema,
  Landing: LandingPageSchema,
  Dashboard: DashboardSchema,
  Auth: AuthSchema,
  System: SystemPagesSchema,
});

export type AppLocale = z.infer<typeof AppLocaleSchema>;
