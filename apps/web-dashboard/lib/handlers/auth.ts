// apps/web-dashboard/lib/handlers/auth.ts
/**
 * =================================================================
 * APARATO: AUTHENTICATION HANDLER (SECURITY GATEKEEPER)
 * CLASIFICACIÓN: MIDDLEWARE LOGIC
 * RESPONSABILIDAD: PROTECCIÓN DE RUTAS Y GESTIÓN DE FLUJO DE SESIÓN
 * ESTRATEGIA: PERÍMETRO DEFENSIVO CON CONCIENCIA DE LOCALE
 * =================================================================
 */

import { NextRequest, NextResponse } from "next/server";
import { auth } from "@/lib/auth/config";
import { routing } from "@/lib/schemas/routing";

// --- CONFIGURACIÓN DE PERÍMETROS ---

/**
 * Rutas que requieren estrictamente una sesión activa.
 * Cualquier sub-ruta también será protegida (ej: /dashboard/lab).
 */
const PROTECTED_ROUTES = ["/dashboard", "/admin", "/settings"];

/**
 * Rutas exclusivas para usuarios NO autenticados.
 * Si un usuario con sesión intenta entrar aquí, será redirigido al dashboard.
 */
const AUTH_ROUTES = ["/login", "/register", "/forgot-password"];

/**
 * Analiza la solicitud entrante y determina si se debe permitir el paso,
 * redirigir a login, o redirigir al dashboard.
 *
 * @param req - La solicitud Next.js entrante.
 * @returns NextResponse para redirigir o null para continuar la cadena.
 */
export async function authHandler(
  req: NextRequest,
): Promise<NextResponse | null> {
  const { pathname, search } = req.nextUrl;

  // 1. Obtención de Sesión (Low Latency Check)
  // NextAuth v5 recupera la sesión de manera eficiente en el Edge.
  const session = await auth();
  const isLoggedIn = !!session?.user;

  // 2. Detección de Locale (Contexto Internacional)
  // Analizamos la URL para ver si ya tiene un prefijo de idioma válido.
  // Ej: "/es/dashboard" -> locale: "es", pathWithoutLocale: "/dashboard"
  const segments = pathname.split("/").filter(Boolean);
  const hasLocale = routing.locales.includes(segments[0] as any);

  const locale = hasLocale ? segments[0] : routing.defaultLocale;

  // Normalizamos el path para comparar contra nuestras constantes de configuración
  // Si la URL es "/es/dashboard/lab", el pathNormalized será "/dashboard/lab"
  const pathNormalized = hasLocale
    ? `/${segments.slice(1).join("/")}`
    : `/${segments.join("/")}`;

  // Normalizamos a raíz si está vacío
  const cleanPath = pathNormalized || "/";

  // 3. Lógica de Protección (Guardia Perimetral)
  const isProtectedRoute = PROTECTED_ROUTES.some((route) =>
    cleanPath.startsWith(route),
  );

  // ESCENARIO A: Usuario ANÓNIMO intenta acceder a RUTA PROTEGIDA
  // Acción: Denegar acceso y redirigir a Login conservando la intención (callbackUrl).
  if (isProtectedRoute && !isLoggedIn) {
    // Construimos la URL de login manteniendo el idioma actual
    const loginUrl = new URL(`/${locale}/login`, req.url);

    // Agregamos la URL original como callback para redirigir al usuario después de loguearse
    // Esto mejora drásticamente la UX.
    const callbackUrl = encodeURIComponent(`${pathname}${search}`);
    loginUrl.searchParams.set("callbackUrl", callbackUrl);

    return NextResponse.redirect(loginUrl);
  }

  // 4. Lógica de Redirección Inversa (Guest Only Routes)
  const isAuthRoute = AUTH_ROUTES.some((route) => cleanPath.startsWith(route));

  // ESCENARIO B: Usuario AUTENTICADO intenta acceder a LOGIN/REGISTER
  // Acción: Redirigir al Dashboard (No tiene sentido que se loguee de nuevo).
  if (isAuthRoute && isLoggedIn) {
    const dashboardUrl = new URL(`/${locale}/dashboard`, req.url);
    return NextResponse.redirect(dashboardUrl);
  }

  // ESCENARIO C: Rutas Públicas (Landing, About, Legal) o Acceso Permitido
  // Acción: Retornar null. Esto indica al `middleware.ts` principal que este handler
  // ha aprobado la solicitud y se debe pasar al siguiente eslabón (i18nHandler).
  return null;
}
