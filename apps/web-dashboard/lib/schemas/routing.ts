import { defineRouting } from 'next-intl/routing';
import { createNavigation } from 'next-intl/navigation'; // <-- CAMBIO AQUÍ

export const routing = defineRouting({
  // Idiomas soportados: Inglés (Default) y Español
  locales: ['en', 'es'],
  defaultLocale: 'en',
  localePrefix: 'as-needed'
});

// CAMBIO AQUÍ: Usar createNavigation en lugar de createSharedPathnamesNavigation
export const { Link, redirect, usePathname, useRouter } =
  createNavigation(routing);
