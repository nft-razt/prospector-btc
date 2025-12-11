import { NextRequest, NextResponse } from 'next/server';
// ⚠️ CORRECCIÓN DE RUTAS: Usar alias absolutos para evitar romper la compilación
import { auth } from '@/lib/auth/config';
import { routing } from '@/lib/schemas/routing';

/**
 * APARATO: AUTH GUARD
 * Intercepta peticiones para proteger rutas privadas (/dashboard, /admin).
 */
export async function authHandler(req: NextRequest): Promise<NextResponse | null> {
  const { pathname } = req.nextUrl;

  const isDashboard = pathname.includes('/dashboard') || pathname.includes('/admin');
  const isLoginPage = pathname.includes('/login');

  const session = await auth();
  const isLoggedIn = !!session?.user;

  // 1. Visitante en Zona Privada -> Redirigir a Login
  if (isDashboard && !isLoggedIn) {
    const locale = req.nextUrl.locale || routing.defaultLocale;
    const loginUrl = new URL(`/${locale}/login`, req.url);
    return NextResponse.redirect(loginUrl);
  }

  // 2. Usuario Logueado en Login -> Redirigir a Dashboard
  if (isLoginPage && isLoggedIn) {
    const locale = req.nextUrl.locale || routing.defaultLocale;
    return NextResponse.redirect(new URL(`/${locale}/dashboard`, req.url));
  }

  return null;
}
