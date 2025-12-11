import { NextRequest, NextResponse } from 'next/server';
import { auth } from '../../../../auth'; // Ruta relativa a la raíz de la app donde está auth.ts
import { routing } from '../../i18n/routing';

/**
 * Verifica la sesión y protege las rutas privadas.
 * Retorna NextResponse si debe redirigir, o null si permite el paso.
 */
export async function authHandler(req: NextRequest): Promise<NextResponse | null> {
  const { pathname } = req.nextUrl;

  // 1. Zonas
  const isDashboard = pathname.includes('/dashboard') || pathname.includes('/admin');
  const isLoginPage = pathname.includes('/login');

  // 2. Verificación de Sesión (Server-Side)
  const session = await auth();
  const isLoggedIn = !!session?.user;

  // 3. Regla: Visitante intenta entrar a Dashboard -> Login
  if (isDashboard && !isLoggedIn) {
    const locale = req.nextUrl.locale || routing.defaultLocale;
    const loginUrl = new URL(`/${locale}/login`, req.url);
    // Podríamos agregar ?callbackUrl=... aquí
    return NextResponse.redirect(loginUrl);
  }

  // 4. Regla: Usuario Logueado intenta entrar a Login -> Dashboard
  if (isLoginPage && isLoggedIn) {
    const locale = req.nextUrl.locale || routing.defaultLocale;
    return NextResponse.redirect(new URL(`/${locale}/dashboard`, req.url));
  }

  // Permite continuar al siguiente handler
  return null;
}
