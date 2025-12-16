// INICIO DEL ARCHIVO [apps/web-dashboard/app/not-found.tsx]
// =================================================================
// APARATO: GLOBAL NOT FOUND (404)
// ESTADO: RESTAURADO (REACT COMPONENT ONLY)
// =================================================================

// ✅ CORRECCIÓN 1: Uso de Alias '@' para evitar el infierno de rutas relativas (../../)
import { enRegistry } from "@/lib/i18n/registry";
import { NotFoundScreen } from "@/components/system/not-found-screen";

// Importación de estilos globales para asegurar renderizado correcto fuera del layout
import "./global.css";

export default function GlobalNotFound() {
  // Acceso tipado a la fuente de verdad (SSoT)
  const texts = enRegistry.System.not_found;

  return (
    <html lang="en">
      <body className="bg-[#050505] text-white antialiased">
        <NotFoundScreen
          texts={{
            title: texts.title,
            description: texts.description,
            error_code: texts.error_code,
            cta_return: texts.cta_return
          }}
          redirectPath="/"
        />
      </body>
    </html>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/not-found.tsx]
