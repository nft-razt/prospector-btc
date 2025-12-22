/**
 * =================================================================
 * APARATO: LANDING PAGE CONTENT (ES)
 * CLASIFICACIÓN: PAGE CONTENT (ESTRATO L5)
 * RESPONSABILIDAD: DICCIONARIO DE CONVERSIÓN Y PROPUESTA DE VALOR
 * =================================================================
 */

import { type LandingPageParams } from "../../../schemas/pages/landing.schema";

export const landingPageContent: LandingPageParams = {
  /** Metadatos para Optimización de Motores de Búsqueda (SEO) */
  meta: {
    title: "Prospector // Protocolo Hydra-Zero",
    description: "Sistema de auditoría criptográfica distribuida enfocado en la curva secp256k1.",
  },

  /** Sección Hero: Propuesta de Valor Primaria */
  hero: {
    badge: "SISTEMA OPERATIVO // VERSIÓN 10.8",
    title: "Arqueología de Entropía Distribuida",
    subtitle:
      "Únase al enjambre global. Auditoría del ledger inmutable mediante matemáticas probabilísticas y computación efímera.",
    cta_primary: {
      label: "Inicializar Sistema",
      tooltip: "Comenzar en Nivel Observador",
    },
  },

  /** Cápsulas de Identidad: Puertas de Entrada */
  capsules: {
    login: {
      title: "Operadores Activos",
      description: "Acceda al centro de mando mediante el apretón de manos seguro.",
      cta: "ACCEDER A LA CONSOLA",
      badge: "SEGURO",
    },
    register: {
      title: "Nuevo Despliegue",
      description: "Provisión de una nueva identidad de nodo para la rejilla de investigación.",
      cta: "INICIALIZAR PROTOCOLO",
      badge: "ABIERTO",
    },
  },

  /** Vista Previa de Niveles de Acceso */
  pricing_preview: {
    title: "Niveles de Acceso",
    observer_tier: "Nodo Observador",
    operator_tier: "Nodo Operador",
  },
};
