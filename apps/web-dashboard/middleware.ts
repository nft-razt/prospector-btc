import { NextRequest, NextResponse } from 'next/server';
import { authHandler } from '@/lib/handlers/auth';
import { i18nHandler } from '@/lib/handlers/i18n';

export default async function middleware(req: NextRequest) {
  const { pathname } = req.nextUrl;

  // 1. Exclusiones de activos estáticos y API interna
  if (
    pathname.startsWith('/api/') ||
    pathname.startsWith('/_next') ||
    pathname.includes('.')
  ) {
    return NextResponse.next();
  }

  // 2. Seguridad (Auth Guard)
  const authResponse = await authHandler(req);
  if (authResponse) return authResponse;

  // 3. Internacionalización
  return i18nHandler(req);
}

export const config = {
  matcher: ['/((?!api|_next|_vercel|.*\\..*).*)']
};
