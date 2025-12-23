/**
 * =================================================================
 * APARATO: AUTHENTICATION HANDLER (V17.0 - EDGE OPTIMIZED)
 * CLASIFICACIÓN: MIDDLEWARE LOGIC (ESTRATO L4)
 * RESPONSABILIDAD: PROTECCIÓN DE RUTAS Y GESTIÓN DE SESIÓN
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el guardián de acceso para el Proxy. Utiliza el
 * protocolo JWT 'getToken' para verificar la identidad del operador
 * sin incurrir en peticiones de red pesadas, garantizando que
 * el enjambre sea inaccesible para agentes no autorizados.
 * =================================================================
 */

import { NextRequest, NextResponse } from "next/server";
import { getToken } from "next-auth/jwt";
import { routing } from "@/lib/schemas/routing";

/** Rutas que requieren autorización de nivel 'Operator'. */
const PROTECTED_DOMAIN_ROUTES = ["/dashboard", "/admin", "/settings", "/lab"];

/** Rutas destinadas únicamente a agentes sin identificar. */
const AUTH_ACCESS_ROUTES = ["/login", "/register"];

/**
 * Orquestador de acceso para el flujo de peticiones.
 *
 * @param request - La solicitud entrante capturada en el borde.
 * @returns {Promise<NextResponse | null>} Respuesta de redirección o nulo para continuar.
 */
export async function authHandler(
  request: NextRequest,
): Promise<NextResponse | null> {
  const { pathname, search } = request.nextUrl;

  /**
   * ADQUISICIÓN DE TOKEN DE IDENTIDAD (L4)
   * En el Middleware/Proxy, getToken es la única forma resiliente
   * de validar la sesión en NextAuth v4.
   */
  const identity_token = await getToken({
    req: request,
    secret: process.env.AUTH_SECRET
  });

  const is_agent_authenticated = !!identity_token;

  /**
   * NORMALIZACIÓN DE LOCALE (I18N SYNC)
   */
  const url_segments = pathname.split("/").filter(Boolean);
  const current_locale = routing.locales.includes(url_segments[0] as any)
    ? url_segments[0]
    : routing.defaultLocale;

  const normalized_path = routing.locales.includes(url_segments[0] as any)
    ? `/${url_segments.slice(1).join("/")}`
    : `/${url_segments.join("/")}`;

  const clean_path = normalized_path || "/";

  /**
   * ESCENARIO A: ACCESO NO AUTORIZADO A ESTRATO PROTEGIDO
   * Acción: Redirigir al portal de identificación conservando el callback.
   */
  const is_target_protected = PROTECTED_DOMAIN_ROUTES.some((route) =>
    clean_path.startsWith(route),
  );

  if (is_target_protected && !is_agent_authenticated) {
    const login_destination_url = new URL(`/${current_locale}/login`, request.url);
    const origin_callback = encodeURIComponent(`${pathname}${search}`);
    login_destination_url.searchParams.set("callbackUrl", origin_callback);

    return NextResponse.redirect(login_destination_url);
  }

  /**
   * ESCENARIO B: AGENTE AUTENTICADO EN RUTA DE LOGIN
   * Acción: Redirigir al Centro de Mando (Dashboard).
   */
  const is_in_auth_route = AUTH_ACCESS_ROUTES.some((route) =>
    clean_path.startsWith(route)
  );

  if (is_in_auth_route && is_agent_authenticated) {
    const dashboard_url = new URL(`/${current_locale}/dashboard`, request.url);
    return NextResponse.redirect(dashboard_url);
  }

  // Permiso concedido: Continuar al estrato de Internacionalización (L5)
  return null;
}
