/**
 * =================================================================
 * APARATO: PUBLIC FOOTER CONTENT (ES)
 * CLASIFICACIÓN: LAYOUT CONTENT (ESTRATO L5)
 * RESPONSABILIDAD: DICCIONARIO DE CIERRE Y CUMPLIMIENTO EN ESPAÑOL
 * =================================================================
 */

import { type PublicFooterParams } from "../../../schemas/layout/public-footer.schema";

export const publicFooterContent: PublicFooterParams = {
  /** Información de Propiedad Intelectual */
  copyright: "© 2025 Grupo de Investigación Prospector. Licencia MIT.",

  /** Matriz de Enlaces Legales y Técnicos */
  links: {
    privacy: { label: "Privacidad de Datos", href: "/privacy" },
    terms: { label: "Términos del Protocolo", href: "/terms" },
    github: { label: "Código Fuente", href: "https://github.com/prospector-btc" },
    documentation: { label: "Documentación", href: "/docs" },
  },

  /** Descargo de Responsabilidad Académica */
  disclaimer:
    "Herramienta de investigación académica. No diseñada para uso ilícito. Utilizar bajo responsabilidad ética.",

  /** Indicadores de Salud de Infraestructura */
  status: {
    label: "Estado del Sistema",
    operational: "Sistemas Operativos",
    degraded: "Rendimiento Degradado",
  },
};
