// libs/shared/i18n-config/src/lib/schema.ts
import { z } from 'zod';

// --- ÁTOMOS REUTILIZABLES (PRIMITIVAS UI) ---

/** Esquema para botones de llamada a la acción */
const CTASchema = z.object({
  label: z.string(),
  tooltip: z.string().optional(),
});

/** Esquema para elementos de navegación */
const NavItemSchema = z.object({
  label: z.string(),
  description: z.string().optional(),
});

// --- ÁTOMOS DE DOMINIO (SUBSISTEMAS) ---

/** Textos comunes transversales a toda la aplicación */
const CommonSchema = z.object({
  loading: z.string(),
  error: z.string(),
  copy: z.string(),
  success: z.string(),
});

/** Textos específicos de la Landing Page (Marketing) */
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

/** Textos del Dashboard Privado (SaaS) */
const DashboardSchema = z.object({
  sidebar: z.object({
    overview: z.string(),
    network: z.string(),
    wallet_lab: z.string(),
    academy: z.string(),
    settings: z.string(),
  }),
  header: z.object({
    welcome: z.string(),
    status_online: z.string(),
  }),
  user_nav: z.object({
    profile: z.string(),
    billing: z.string(),
    settings: z.string(),
    logout: z.string(),
  }),
});

/** Textos de Autenticación y Sesión */
const AuthSchema = z.object({
  login_title: z.string(),
  login_google: z.string(),
  login_footer: z.string(),
  logout: z.string(),
});

/** Textos de Errores del Sistema (404/500) */
const SystemPagesSchema = z.object({
  not_found: z.object({
    title: z.string(),
    description: z.string(),
    error_code: z.string(),
    cta_return: z.string(),
  }),
});

// --- SCHEMA MAESTRO (COMPOSICIÓN) ---
export const AppLocaleSchema = z.object({
  Common: CommonSchema,
  Landing: LandingSchema,
  Dashboard: DashboardSchema,
  Auth: AuthSchema,
  System: SystemPagesSchema,
});

// Inferencia de Tipo para TypeScript (Intellisense en toda la app)
export type AppLocale = z.infer<typeof AppLocaleSchema>;
