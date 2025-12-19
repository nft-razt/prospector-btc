// apps/web-dashboard/lib/handlers/visitor.ts
/**
 * =================================================================
 * APARATO: VISITOR INTELLIGENCE HANDLER
 * RESPONSABILIDAD: ANÁLISIS DE CONTEXTO DEL CLIENTE (IP, LOCALE)
 * ESTÁNDAR: EDGE COMPATIBLE (NEXT.JS MIDDLEWARE)
 * =================================================================
 */

import { NextRequest, NextResponse } from "next/server";
import { routing } from "@/lib/schemas/routing";
import { match } from "@formatjs/intl-localematcher";
import Negotiator from "negotiator";

/**
 * Interfaz de Telemetría del Visitante.
 * Se inyectará en los headers para consumo de la app.
 */
export interface VisitorContext {
  ip: string;
  country: string;
  preferredLocale: string;
  userAgent: string;
}

/**
 * Determina el idioma óptimo basado en los headers del navegador.
 */
function getLocale(request: NextRequest): string {
  // 1. Prioridad: Cookie de usuario (si ya eligió idioma)
  // const cookieLocale = request.cookies.get("NEXT_LOCALE")?.value;
  // if (cookieLocale) return cookieLocale;

  // 2. Negociación de Contenido
  const headers = {
    "accept-language": request.headers.get("accept-language") || "",
  };
  const languages = new Negotiator({ headers }).languages();

  try {
    return match(languages, routing.locales, routing.defaultLocale);
  } catch (e) {
    return routing.defaultLocale; // Fallback seguro a Inglés
  }
}

/**
 * Handler Principal.
 * Analiza la petición entrante y enriquece el contexto.
 * NO realiza la redirección final (eso lo hace i18nHandler), pero prepara el terreno.
 */
export async function visitorHandler(
  req: NextRequest,
): Promise<NextResponse | null> {
  const { pathname } = req.nextUrl;

  // Ignorar archivos estáticos y API
  if (
    pathname.startsWith("/_next") ||
    pathname.startsWith("/api") ||
    pathname.includes(".")
  ) {
    return null;
  }

  // 1. Detección de Locale
  const locale = getLocale(req);
  const pathnameHasLocale = routing.locales.some(
    (loc) => pathname.startsWith(`/${loc}/`) || pathname === `/${loc}`,
  );

  // Si la URL no tiene idioma, permitimos que el middleware de i18n maneje la redirección,
  // pero ya hemos calculado cuál debería ser.

  // 2. Extracción de Geo-IP (Vercel Edge específico o Headers estándar)
  const ip = req.ip || req.headers.get("x-forwarded-for") || "127.0.0.1";
  const country =
    req.geo?.country || req.headers.get("x-vercel-ip-country") || "UNKNOWN";
  const userAgent = req.headers.get("user-agent") || "Unknown";

  // 3. Telemetría de Depuración (Solo en Server Logs)
  // console.log(`[VISITOR] IP: ${ip} | Country: ${country} | Locale Target: ${locale}`);

  // Retornamos null para permitir que la cadena de middleware continúe.
  // La información extraída se podría inyectar en headers si fuera necesario para el backend.
  return null;
}
