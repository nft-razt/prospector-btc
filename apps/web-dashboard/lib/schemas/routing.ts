import { defineRouting } from "next-intl/routing";
import { createNavigation } from "next-intl/navigation";

/**
 * CONFIGURACIÓN DE ENRUTAMIENTO I18N
 * Fuente única de verdad para la navegación localizada.
 */
export const routing = defineRouting({
  // Idiomas soportados por el sistema
  locales: ["en", "es"],
  defaultLocale: "en",
  // Prefijo de URL: /en/dashboard, /es/dashboard
  localePrefix: "as-needed",
});

// Exportación de utilidades de navegación tipadas
export const { Link, redirect, usePathname, useRouter } =
  createNavigation(routing);
