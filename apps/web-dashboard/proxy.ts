/**
 * =================================================================
 * APARATO: SOVEREIGN EDGE PROXY (V16.1)
 * CLASIFICACIÓN: INFRASTRUCTURE GATEWAY (ESTRATO L4)
 * RESPONSABILIDAD: ORQUESTACIÓN DE PETICIONES EN EL BORDE
 * =================================================================
 */

import { NextRequest, NextResponse } from "next/server";
import { authHandler } from "@/lib/handlers/auth";
import { i18nHandler } from "@/lib/handlers/i18n";
import { visitorHandler } from "@/lib/handlers/visitor";

export default async function proxy(request: NextRequest): Promise<NextResponse> {
  const { pathname } = request.nextUrl;

  // BYPASS DE ACTIVOS ESTÁTICOS
  if (
    pathname.startsWith("/api/") ||
    pathname.startsWith("/_next") ||
    pathname.includes(".")
  ) {
    return NextResponse.next();
  }

  // INTELIGENCIA DE VISITANTE
  await visitorHandler(request);

  // ESCUDO DE SEGURIDAD
  const authentication_response = await authHandler(request);
  if (authentication_response) return authentication_response;

  // RUTEO LOCALIZADO
  return i18nHandler(request);
}

export const config = {
  matcher: ["/((?!api|_next/static|_next/image|favicon.ico|robots.txt|sitemap.xml|.*\\..*).*)"],
};
