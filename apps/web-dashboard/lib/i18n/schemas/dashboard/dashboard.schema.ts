/**
 * =================================================================
 * APARATO: DASHBOARD I18N SCHEMA (V35.0)
 * CLASIFICACIÓN: DOMAIN CONTRACT (L2)
 * RESPONSABILIDAD: DEFINICIÓN SOBERANA DE TEXTOS DEL PANEL DE CONTROL
 * ESTADO: EXTENDED // FULL HOLISTIC COMPLIANCE
 * =================================================================
 */

import { z } from "zod";

export const DashboardSchema = z.object({
  sidebar: z.object({
    overview: z.string().describe("Título de la sección general"),
    network: z.string().describe("Título de la sección de red de nodos"),
    analytics_deep: z.string().describe("Título de la sección de análisis estratégico"),
    wallet_lab: z.string().describe("Título de la sección de laboratorio forense"),
    academy: z.string().describe("Título de la sección de formación académica"),
    settings: z.string().describe("Título de la sección de configuración de sistema"),
  }),
  header: z.object({
    welcome: z.string().describe("Mensaje de bienvenida al operador"),
    status_online: z.string().describe("Etiqueta de estado de conexión activa"),
  }),
  user_nav: z.object({
    profile: z.string().describe("Opción de perfil de usuario"),
    billing: z.string().describe("Opción de gestión de suscripción"),
    settings: z.string().describe("Opción de seguridad y ajustes"),
    logout: z.string().describe("Opción de cierre de sesión"),
  }),
  fleet: z.object({
    title: z.string().describe("Título del monitor visual de flota"),
    live_feed: z.string().describe("Etiqueta de transmisión en vivo"),
    no_signal: z.string().describe("Mensaje de ausencia de señal de video"),
    deploy_hint: z.string().describe("Instrucciones para inicializar el enjambre"),
    connection_lost: z.string().describe("Mensaje de pérdida de enlace táctico"),
  }),
  lab: z.object({
    title: z.string().describe("Título del estrato experimental"),
    interceptor_title: z.string().describe("Título del motor Interceptor neural"),
    forge_title: z.string().describe("Título del motor de forja de escenarios"),
    scan_btn: z.string().describe("Texto del botón de escaneo manual"),
    inject_btn: z.string().describe("Texto del botón de inyección de tickets"),
    no_scenarios: z.string().describe("Mensaje cuando no hay experimentos activos"),
  }),
  vault: z.object({
    title: z.string().describe("Título de la bóveda de identidades"),
    injection_badge: z.string().describe("Etiqueta de cifrado Zero-Knowledge"),
    encrypting: z.string().describe("Mensaje durante el proceso de cifrado"),
    secure_btn: z.string().describe("Texto del botón de subida segura"),
    empty_vault: z.string().describe("Mensaje cuando la bóveda está vacía"),
  }),
  analytics_page: z.object({
    title: z.string().describe("Título de la página de analítica profunda"),
    subtitle: z.string().describe("Subtítulo con versión del protocolo"),
    effort_distribution: z.string().describe("Título de la gráfica de esfuerzo"),
    hardware_efficiency: z.string().describe("Título de la gráfica de eficiencia"),
    geographical_nodes: z.string().describe("Título del mapa de geolocalización"),
    time_series_label: z.string().describe("Etiqueta del eje temporal"),
    metrics: z.object({
      hashes_per_watt: z.string().describe("Métrica de eficiencia energética"),
      avg_latency: z.string().describe("Métrica de latencia media de red"),
      collision_prob: z.string().describe("Cálculo de probabilidad de colisión"),
    }),
  }),
  analytics: z.object({
    total_effort: z.string().describe("Etiqueta de esfuerzo computacional global"),
    hash_unit: z.string().describe("Unidad de medida (Billones de hashes)"),
    efficiency: z.string().describe("Etiqueta de eficiencia del núcleo"),
    zombie_rate: z.string().describe("Tasa de descubrimiento de carteras zombie"),
  }),
});

export type DashboardParams = z.infer<typeof DashboardSchema>;
