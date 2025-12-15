/**
 * =================================================================
 * APARATO: COOKIE PURIFIER
 * RESPONSABILIDAD: FILTRADO DE CREDENCIALES DE SESIÓN CRÍTICAS
 * NIVEL: ELITE PRODUCTION
 * =================================================================
 */

// Lista blanca estricta de cookies de autenticación de Google
const CRITICAL_COOKIE_NAMES = new Set([
  "SID",
  "HSID",
  "SSID",
  "APISID",
  "SAPISID",
  "__Secure-1PSID",
  "__Secure-3PSID",
  "__Secure-1PAPISID",
  "__Secure-3PAPISID",
  "NID",
  "SIDCC",
  "__Secure-1PSIDCC",
  "__Secure-3PSIDCC",
]);

const ALLOWED_DOMAINS = [
  ".google.com",
  "google.com",
  ".colab.research.google.com",
];

export interface PlaywrightCookie {
  name: string;
  value: string;
  domain: string;
  path: string;
  secure: boolean;
  httpOnly: boolean;
  sameSite: "Strict" | "Lax" | "None";
  expires?: number;
}

/**
 * Transforma y purifica cookies crudas (JSON) en formato Playwright seguro.
 */
export function purifyCookies(rawCookies: any[]): PlaywrightCookie[] {
  if (!Array.isArray(rawCookies)) return [];

  return rawCookies
    .filter((cookie) => {
      // 1. Validar Nombre
      if (!CRITICAL_COOKIE_NAMES.has(cookie.name)) return false;
      // 2. Validar Dominio
      const domain = cookie.domain || "";
      return ALLOWED_DOMAINS.some((d) => domain.endsWith(d));
    })
    .map(
      (cookie) =>
        ({
          name: cookie.name,
          value: cookie.value,
          domain: cookie.domain || ".google.com",
          path: cookie.path || "/",
          secure: true, // Google Auth siempre es HTTPS
          httpOnly: cookie.httpOnly ?? false,
          sameSite: "None", // Crítico para contextos cross-site
          expires: cookie.expirationDate || cookie.expires || undefined,
        }) as PlaywrightCookie,
    );
}
