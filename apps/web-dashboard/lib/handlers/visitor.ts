/**
 * =================================================================
 * APARATO: VISITOR INTELLIGENCE HANDLER (V16.5 - RESILIENT)
 * CLASIFICACIÓN: MIDDLEWARE LOGIC
 * RESPONSABILIDAD: EXTRACCIÓN DETERMINISTA DE CONTEXTO
 * =================================================================
 */

import { NextRequest } from "next/server";

export interface VisitorContext {
  ip_address: string;
  country_code: string;
  user_agent: string;
}

/**
 * Extrae metadatos del visitante utilizando cabeceras de red para
 * garantizar compatibilidad universal y evitar errores de tipado.
 */
export async function visitorHandler(request: NextRequest): Promise<VisitorContext> {
  const ip_address =
    (request as any).ip ||
    request.headers.get("x-forwarded-for")?.split(",")[0] ||
    "127.0.0.1";

  const country_code =
    (request as any).geo?.country ||
    request.headers.get("x-vercel-ip-country") ||
    "UNKNOWN";

  const user_agent = request.headers.get("user-agent") || "PROSPECTOR_AGENT";

  return { ip_address, country_code, user_agent };
}
