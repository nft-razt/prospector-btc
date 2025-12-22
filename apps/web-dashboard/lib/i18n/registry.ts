/**
 * =================================================================
 * APARATO: I18N CONTENT REGISTRY (MASTER SOURCE)
 * CLASIFICACIÓN: INFRASTRUCTURE CORE (ESTRATO L4)
 * RESPONSABILIDAD: ENSAMBLAJE SOBERANO DE DICCIONARIOS LOCALIZADOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el punto de consolidación de la Fuente Única de Verdad (SSoT).
 * Vincula las implementaciones de idioma en Inglés y Español, asegurando
 * que ambas satisfagan el esquema 'AppLocale'. Este diseño permite
 * detectar en tiempo de compilación cualquier discrepancia estructural,
 * cumpliendo con la política de CERO REGRESIONES en la interfaz.
 * =================================================================
 */

import { type AppLocale } from "./schema";

// --- IMPORTACIONES DE CONTENIDO: ESTRATO INGLÉS (EN) ---
import { commonContent as commonEn } from "./content/en/common.content";
import { publicHeaderContent as headerEn } from "./content/en/layout/public-header.content";
import { publicFooterContent as footerEn } from "./content/en/layout/public-footer.content";
import { landingPageContent as landingEn } from "./content/en/pages/landing.content";
import { dashboardContent as dashboardEn } from "./content/en/dashboard/dashboard.content";
import { authContent as authEn } from "./content/en/auth/auth.content";
import { systemContent as systemEn } from "./content/en/system/system.content";

// --- IMPORTACIONES DE CONTENIDO: ESTRATO ESPAÑOL (ES) ---
import { commonContent as commonEs } from "./content/es/common.content";
import { publicHeaderContent as headerEs } from "./content/es/layout/public-header.content";
import { publicFooterContent as footerEs } from "./content/es/layout/public-footer.content";
import { landingPageContent as landingEs } from "./content/es/pages/landing.content";
import { dashboardContent as dashboardEs } from "./content/es/dashboard/dashboard.content";
import { authContent as authEs } from "./content/es/auth/auth.content";
import { systemContent as systemEs } from "./content/es/system/system.content";

/**
 * DICCIONARIO SOBERANO: INGLÉS (BASE)
 * Implementación canónica utilizada como referencia técnica.
 */
export const enRegistry: AppLocale = {
  Common: commonEn,
  PublicHeader: headerEn,
  PublicFooter: footerEn,
  Landing: landingEn,
  Dashboard: dashboardEn,
  Auth: authEn,
  System: systemEn,
};

/**
 * DICCIONARIO SOBERANO: ESPAÑOL
 * Implementación nivelada para el mercado hispanohablante.
 * RESOLUCIÓN: Satisface el contrato 'AppLocale' garantizando paridad total con EN.
 */
export const esRegistry: AppLocale = {
  Common: commonEs,
  PublicHeader: headerEs,
  PublicFooter: footerEs,
  Landing: landingEs,
  Dashboard: dashboardEs,
  Auth: authEs,
  System: systemEs,
};

/**
 * MAPA GLOBAL DE LOCALIZACIÓN
 * Utilizado por el compilador para iterar y generar los artefactos JSON.
 */
export const localizationMap: Record<string, AppLocale> = {
  en: enRegistry,
  es: esRegistry,
};
