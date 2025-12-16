// apps/web-dashboard/middleware.ts
/**
 * =================================================================
 * APARATO: MIDDLEWARE KERNEL
 * RESPONSABILIDAD: ORQUESTACIÓN DE HANDLERS EN EL BORDE
 * CADENA: VISITOR -> AUTH -> I18N
 * =================================================================
 */

import { NextRequest, NextResponse } from "next/server";
import { authHandler } from "@/lib/handlers/auth";
import { i18nHandler } from "@/lib/handlers/i18n";
import { visitorHandler } from "@/lib/handlers/visitor";

export default async function middleware(req: NextRequest) {
  const { pathname } = req.nextUrl;

  // 0. Bypass de Activos Estáticos
  if (
    pathname.startsWith("/api/") ||
    pathname.startsWith("/_next") ||
    pathname.includes(".")
  ) {
    return NextResponse.next();
  }

  // 1. Análisis de Visitante (Inteligencia)
  await visitorHandler(req);

  // 2. Seguridad (Auth Guard - Actualmente Pasivo)
  const authResponse = await authHandler(req);
  if (authResponse) return authResponse;

  // 3. Internacionalización (Redirección final y reescritura)
  return i18nHandler(req);
}

export const config = {
  // Matcher optimizado para excluir archivos estáticos e imágenes
  matcher: ["/((?!api|_next|_vercel|.*\\..*).*)"],
};
