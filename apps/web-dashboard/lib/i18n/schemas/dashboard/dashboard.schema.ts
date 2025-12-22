/**
 * =================================================================
 * APARATO: DASHBOARD I18N SCHEMA (V47.0 - STRATEGIC EDITION)
 * CLASIFICACIÓN: DOMAIN CONTRACT (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN SOBERANA DE TEXTOS DEL PANEL DE CONTROL
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la Fuente Única de Verdad (SSoT) para el motor de
 * internacionalización. Valida recursivamente que todos los
 * diccionarios de idioma (EN/ES) posean las llaves necesarias para
 * renderizar la telemetría de hardware, la auditoría forense y el
 * estado de paridad de los motores gemelos (Turso/Supabase).
 * =================================================================
 */

import { z } from "zod";

export const DashboardSchema = z.object({
  /** Navegación lateral del centro de mando */
  sidebar: z.object({
    overview: z.string().describe("Título de la sección general de mando"),
    network: z.string().describe("Título de la sección de red de nodos distributed"),
    analytics_deep: z.string().describe("Título de la sección de análisis estratégico de esfuerzo"),
    wallet_lab: z.string().describe("Título de la sección de laboratorio de vulnerabilidad forense"),
    academy: z.string().describe("Título de la sección de formación técnica académica"),
    settings: z.string().describe("Título de la sección de configuración de infraestructura soberana"),
  }),

  /** Cabecera de sesión y estado neural */
  header: z.object({
    welcome: z.string().describe("Mensaje de bienvenida personalizada al operador"),
    status_online: z.string().describe("Etiqueta de estado de sincronización neural activa"),
  }),

  /** Navegación de identidad de usuario */
  user_nav: z.object({
    profile: z.string().describe("Acceso al perfil de identidad del operador"),
    billing: z.string().describe("Gestión de suscripciones y cuotas de cómputo"),
    settings: z.string().describe("Ajustes de seguridad y cifrado local"),
    logout: z.string().describe("Protocolo de terminación de sesión activa"),
  }),

  /** Monitor visual del enjambre (Panóptico) */
  fleet: z.object({
    title: z.string().describe("Título del monitor de vigilancia visual de flota"),
    live_feed: z.string().describe("Etiqueta de transmisión de video en tiempo real"),
    no_signal: z.string().describe("Mensaje ante la ausencia de señal visual del nodo"),
    deploy_hint: z.string().describe("Instrucciones para inicializar unidades de red"),
    connection_lost: z.string().describe("Mensaje de fallo en el enlace táctico visual"),
  }),

  /** Estrato Experimental y QA */
  lab: z.object({
    title: z.string().describe("Título general del estrato de pruebas experimentales"),
    interceptor_title: z.string().describe("Título del motor de interceptación de entropía"),
    forge_title: z.string().describe("Título del motor de cristalización de escenarios"),
    scan_btn: z.string().describe("Texto de activación de secuencia de escaneo manual"),
    inject_btn: z.string().describe("Texto de inyección de material para tickets dorados"),
    no_scenarios: z.string().describe("Mensaje de ausencia de experimentos activos en el ledger"),
  }),

  /** Bóveda Zero-Knowledge de Identidades */
  vault: z.object({
    title: z.string().describe("Título de la bóveda de identidades cifradas"),
    injection_badge: z.string().describe("Etiqueta indicadora de protección AES-256-GCM"),
    encrypting: z.string().describe("Mensaje de progreso durante el cifrado local en navegador"),
    secure_btn: z.string().describe("Texto del botón de persistencia en el ledger táctico"),
    empty_vault: z.string().describe("Mensaje indicando que el pool de identidades está vacío"),
  }),

  /** Página de Analítica Estratégica (Métricas Frías) */
  analytics_page: z.object({
    title: z.string().describe("Título principal de la vista de analítica de datos"),
    subtitle: z.string().describe("Subtítulo con la versión del kernel de cómputo"),
    effort_distribution: z.string().describe("Título de la visualización de distribución de esfuerzo"),
    hardware_efficiency: z.string().describe("Título del gráfico de eficiencia de hardware distribuido"),
    geographical_nodes: z.string().describe("Título del mapa de geolocalización de unidades activas"),
    time_series_label: z.string().describe("Etiqueta descriptiva del eje temporal de auditoría"),
    metrics: z.object({
      hashes_per_watt: z.string().describe("Métrica de eficiencia energética proyectada"),
      avg_latency: z.string().describe("Métrica de latencia media del apretón de manos neural"),
      collision_prob: z.string().describe("Cálculo estadístico de probabilidad de colisión"),
    }),
  }),

  /** Métricas resumidas de hashrate y censo */
  analytics: z.object({
    total_effort: z.string().describe("Etiqueta del volumen total de esfuerzo computacional"),
    hash_unit: z.string().describe("Unidad de medida para magnitudes masivas de hashes"),
    efficiency: z.string().describe("Etiqueta del ratio de eficiencia del enjambre"),
    zombie_rate: z.string().describe("Tasa de descubrimiento de identidades con alta entropía"),
  }),

  /** ESTRATO NUEVO: Estado de Paridad de Motores (V110.0) */
  archival_status: z.object({
    engine_b_parity: z.string().describe("Título del monitor de paridad con el Motor Estratégico"),
    strategic_vault_link: z.string().describe("Subtítulo que indica el enlace con el archivo Supabase"),
    archival_integrity: z.string().describe("Etiqueta del porcentaje de integridad del archivo"),
    sync_drift_detected: z.string().describe("Mensaje de alerta detallando la brecha de sincronización"),
    total_archived_missions: z.string().describe("Conteo acumulado de misiones en almacenamiento frío"),
  }),

  /** ESTRATO NUEVO: Ledger de Auditoría Inmutable (L4) */
  audit_trail: z.object({
    title: z.string().describe("Título del historial de misiones certificadas"),
    column_mission: z.string().describe("Cabecera para el identificador único de misión"),
    column_strategy: z.string().describe("Cabecera para la metodología algorítmica aplicada"),
    column_effort: z.string().describe("Cabecera para el volumen de hashes auditados"),
    column_status: z.string().describe("Cabecera para el estatus de certificación forense"),
    column_footprint: z.string().describe("Cabecera para la huella de verificación hexadecimal"),
    empty_state: z.string().describe("Mensaje cuando no existen reportes migrados al archivo"),
  }),

  /** ESTRATO NUEVO: Denominaciones de Estrategias Criptográficas */
  strategies: z.object({
    sequential: z.string().describe("Nombre de la auditoría por rangos secuenciales U256"),
    dictionary: z.string().describe("Nombre de la auditoría basada en diccionarios de entropía"),
    static_handshake: z.string().describe("Nombre de la verificación de secretos específicos"),
    forensic_archaeology: z.string().describe("Nombre de la recuperación de patrones de PRNG históricos"),
  }),
});

export type DashboardParams = z.infer<typeof DashboardSchema>;
