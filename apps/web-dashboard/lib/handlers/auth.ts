import { NextRequest, NextResponse } from "next/server";
import { auth } from "@/lib/auth/config";
import { routing } from "@/lib/schemas/routing";

export async function authHandler(
  req: NextRequest,
): Promise<NextResponse | null> {
  const { pathname } = req.nextUrl;
  const session = await auth();
  const isLoggedIn = !!session?.user;

  // Detectar locale actual del pathname (ej: /es/dashboard -> es)
  // Si no hay locale en la URL, asumimos el default para la lógica de redirección
  const segments = pathname.split("/");
  const locale = routing.locales.includes(segments[1] as any)
    ? segments[1]
    : routing.defaultLocale;

  const isDashboard =
    pathname.includes("/dashboard") || pathname.includes("/admin");
  const isLoginPage = pathname.includes("/login");

  // 1. Protección de Rutas Privadas
  if (isDashboard && !isLoggedIn) {
    const url = new URL(`/${locale}/login`, req.url);
    // Preservar la URL de retorno si es necesario (opcional)
    return NextResponse.redirect(url);
  }

  // 2. Redirección si ya está logueado
  if (isLoginPage && isLoggedIn) {
    const url = new URL(`/${locale}/dashboard`, req.url);
    return NextResponse.redirect(url);
  }

  return null;
}
