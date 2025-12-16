// apps/web-dashboard/lib/i18n/registry.ts
/**
 * =================================================================
 * APARATO: I18N CONTENT REGISTRY (MASTER SOURCE)
 * RESPONSABILIDAD: ENSAMBLAJE DE ÁTOMOS DE CONTENIDO EN INGLÉS (BASE)
 * =================================================================
 */

import { type AppLocale } from "./schema";

// Importación de Contenidos Granulares (EN)
// Estos archivos deben existir en lib/i18n/content/en/...
import { commonContent } from "./content/en/common.content";
import { publicHeaderContent } from "./content/en/layout/public-header.content";
import { publicFooterContent } from "./content/en/layout/public-footer.content";
import { landingPageContent } from "./content/en/pages/landing.content";
import { dashboardContent } from "./content/en/dashboard/dashboard.content";
import { authContent } from "./content/en/auth/auth.content";
import { systemContent } from "./content/en/system/system.content";

/**
 * DICCIONARIO MAESTRO (INGLÉS)
 * Esta es la fuente de verdad que será validada y convertida a JSON.
 * Actúa como "Blueprint" para el resto de idiomas.
 */
export const enRegistry: AppLocale = {
  Common: commonContent,
  PublicHeader: publicHeaderContent,
  PublicFooter: publicFooterContent,
  Landing: landingPageContent,
  Dashboard: dashboardContent,
  Auth: authContent,
  System: systemContent,
};
