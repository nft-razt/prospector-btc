import { NextRequest, NextResponse } from 'next/server';
import { auth } from '@/lib/auth/config'; // ✅ Ruta absoluta
import { routing } from '@/lib/schemas/routing'; // ✅ Ruta absoluta

export async function authHandler(req: NextRequest): Promise<NextResponse | null> {
  const { pathname } = req.nextUrl;
  const isDashboard = pathname.includes('/dashboard') || pathname.includes('/admin');
  const isLoginPage = pathname.includes('/login');

  const session = await auth();
  const isLoggedIn = !!session?.user;

  if (isDashboard && !isLoggedIn) {
    const locale = req.nextUrl.locale || routing.defaultLocale;
    return NextResponse.redirect(new URL(`/${locale}/login`, req.url));
  }

  if (isLoginPage && isLoggedIn) {
    const locale = req.nextUrl.locale || routing.defaultLocale;
    return NextResponse.redirect(new URL(`/${locale}/dashboard`, req.url));
  }

  return null;
}
