// libs/shared/i18n-config/src/lib/schema.ts

import { z } from "zod";

// =================================================================
// 🧩 ÁTOMOS REUTILIZABLES (PRIMITIVAS UI)
// =================================================================

/**
 * Esquema para botones de llamada a la acción (CTA).
 * Utilizado en landing pages, modales y formularios.
 */
const CTASchema = z.object({
  /** Texto visible del botón */
  label: z.string(),
  /** Texto de ayuda (tooltip) opcional para UX guiada */
  tooltip: z.string().optional(),
});

/**
 * Esquema para elementos de navegación.
 */
const NavItemSchema = z.object({
  label: z.string(),
  description: z.string().optional(),
});

// =================================================================
// 🏗️ SUBSISTEMAS (DOMINIO ESPECÍFICO)
// =================================================================

/**
 * Textos comunes transversales a toda la aplicación.
 * Mensajes de estado global, errores genéricos y feedback.
 */
const CommonSchema = z.object({
  loading: z.string().describe("Mensaje de carga por defecto (spinners)"),
  error: z.string().describe("Título genérico de error"),
  copy: z.string().describe("Acción de copiar al portapapeles"),
  success: z.string().describe("Mensaje de operación exitosa"),
});

/**
 * Textos específicos de la Landing Page (Marketing & Conversión).
 */
const LandingSchema = z.object({
  hero: z.object({
    title: z.string(),
    subtitle: z.string(),
    cta_primary: CTASchema,
  }),
  pricing: z.object({
    observer_title: z.string(),
    observer_desc: z.string(),
    operator_title: z.string(),
    operator_desc: z.string(),
    currency: z.string(),
    cta_free: z.string(),
    cta_pro: z.string(),
  }),
});

/**
 * Textos del Dashboard Privado (SaaS Application).
 * Contiene la estructura de la barra lateral, cabecera y módulos funcionales.
 */
const DashboardSchema = z.object({
  /** Navegación lateral principal */
  sidebar: z.object({
    overview: z.string(),
    network: z.string(),
    wallet_lab: z.string(),
    academy: z.string(),
    settings: z.string(),
  }),
  /** Cabecera superior y estados de conexión */
  header: z.object({
    welcome: z.string(),
    status_online: z.string(),
  }),
  /** Menú de usuario desplegable */
  user_nav: z.object({
    profile: z.string(),
    billing: z.string(),
    settings: z.string(),
    logout: z.string(),
  }),
  /**
   * Módulo de Vigilancia Visual (Fleet Grid).
   * @see libs/features/network/fleet-grid.tsx
   */
  fleet: z.object({
    title: z.string().describe("Título de la sección de vigilancia"),
    live_feed: z.string().describe("Indicador de transmisión en vivo"),
    no_signal: z.string().describe("Mensaje de estado vacío (sin workers)"),
    deploy_hint: z
      .string()
      .describe("Sugerencia de acción cuando no hay señal"),
  }),
});

/**
 * Textos de Autenticación y Gestión de Sesión.
 */
const AuthSchema = z.object({
  login_title: z.string(),
  login_google: z.string(),
  login_footer: z.string(),
  logout: z.string(),
});

/**
 * Textos de Páginas del Sistema (Errores HTTP, Mantenimiento).
 */
const SystemPagesSchema = z.object({
  not_found: z.object({
    title: z.string(),
    description: z.string(),
    error_code: z.string(),
    cta_return: z.string(),
  }),
});

// =================================================================
// 👑 SCHEMA MAESTRO (LA LEY)
// =================================================================

/**
 * Esquema raíz que valida la integridad completa del diccionario.
 * Usado por `tools/scripts/generate-i18n.ts`.
 */
export const AppLocaleSchema = z.object({
  Common: CommonSchema,
  Landing: LandingSchema,
  Dashboard: DashboardSchema,
  Auth: AuthSchema,
  System: SystemPagesSchema,
});

/**
 * Tipo inferido automáticamente para TypeScript.
 * Garantiza autocompletado e intellisense en toda la UI.
 */
export type AppLocale = z.infer<typeof AppLocaleSchema>;
