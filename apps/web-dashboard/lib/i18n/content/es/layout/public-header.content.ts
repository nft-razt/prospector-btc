/**
 * =================================================================
 * APARATO: PUBLIC HEADER CONTENT (ES)
 * CLASIFICACIÓN: LAYOUT CONTENT (ESTRATO L5)
 * RESPONSABILIDAD: DICCIONARIO DE NAVEGACIÓN PÚBLICA EN ESPAÑOL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la interfaz de navegación para visitantes, asegurando
 * una terminología coherente con la Tesis Doctoral. Define los
 * puntos de entrada al sistema de auditoría y la marca Prospector.
 * =================================================================
 */

import { type PublicHeaderParams } from "../../../schemas/layout/public-header.schema";

export const publicHeaderContent: PublicHeaderParams = {
  /** Identidad Visual y Marca */
  brand: "PROSPECTOR",

  /** Enlaces de Navegación Estratégica */
  nav: {
    features: "Capacidades",
    pricing: "Niveles de Acceso",
    about: "La Tesis Doctoral",
  },

  /** Acciones Globales de Usuario */
  actions: {
    login: "Acceso de Operador",
    get_started: "Inicializar Nodo",
  },

  /** Mensajería de Estado del Sistema */
  banner: {
    text: "Estado del Sistema: V10.8 Operativo",
    link: {
      label: "Ver Métricas",
      href: "/status",
    },
  },
};
