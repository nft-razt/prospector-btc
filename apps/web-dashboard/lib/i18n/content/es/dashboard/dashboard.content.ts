/**
 * =================================================================
 * APARATO: DASHBOARD I18N CONTENT (ES) (V47.0 - EDICIÓN ESTRATÉGICA)
 * CLASIFICACIÓN: DOMAIN CONTENT (ESTRATO L5)
 * RESPONSABILIDAD: DICCIONARIO MAESTRO EN ESPAÑOL PARA EL CENTRO DE MANDO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la versión en español de la interfaz del operador,
 * utilizando terminología técnica soberana y académica. Esta versión
 * garantiza que el operador tenga visibilidad total sobre la cadena de
 * integridad de la auditoría y la paridad de los motores tácticos.
 * =================================================================
 */

import { type DashboardParams } from "../../../schemas/dashboard/dashboard.schema";

export const dashboardContent: DashboardParams = {
  /** Navegación Lateral del Sistema */
  sidebar: {
    overview: "Centro de Mando de la Misión",
    network: "Rejilla de Inteligencia Distribuida",
    analytics_deep: "Analítica Estratégica de Esfuerzo",
    wallet_lab: "Laboratorio de Vulnerabilidad Criptográfica",
    academy: "Academia Técnica de Hydra",
    settings: "Configuración de Infraestructura Soberana",
  },

  /** Cabecera y Sincronización Neural */
  header: {
    welcome: "Bienvenido de nuevo, Operador",
    status_online: "ENLACE_NEURAL_SINCRONIZADO",
  },

  /** Navegación de Identidad del Operador */
  user_nav: {
    profile: "Perfil de Identidad del Operador",
    billing: "Suscripción y Cuotas de Cómputo",
    settings: "Ajustes de Seguridad y Cifrado Local",
    logout: "Protocolo de Terminación de Sesión Activa",
  },

  /** Vigilancia Visual del Enjambre (Panóptico) */
  fleet: {
    title: "Vigilancia Visual en Tiempo Real",
    live_feed: "TRANSMISIÓN_DE_VIDEO_ACTIVA",
    no_signal: "NO SE DETECTA SEÑAL VISUAL DE LA UNIDAD",
    deploy_hint: "Inicialice unidades en la rejilla mediante el Provisionador para establecer el enlace.",
    connection_lost: "ENLACE TÁCTICO VISUAL INTERRUMPIDO // REESTABLECIENDO CONEXIÓN",
  },

  /** Estrato de Investigación y Certificación Forense */
  lab: {
    title: "Estrato de Investigación Experimental",
    interceptor_title: "Motor Interceptor de Entropía Neural",
    forge_title: "Forja y Cristalizador de Escenarios",
    scan_btn: "INICIALIZAR SECUENCIA DE ESCANEO",
    inject_btn: "CRISTALIZAR BOLETO DORADO",
    no_scenarios: "NO EXISTEN EXPERIMENTOS CRIPTOGRÁFICOS ACTIVOS EN EL LEDGER",
  },

  /** Bóveda de Identidad Zero-Knowledge */
  vault: {
    title: "Bóveda de Identidad de Conocimiento Cero",
    injection_badge: "PROTECCIÓN AES-256-GCM ACTIVA",
    encrypting: "CIFRANDO_PAYLOAD_DE_IDENTIDAD_LOCALMENTE...",
    secure_btn: "ASEGURAR EN EL LEDGER TÁCTICO",
    empty_vault: "El búnker de identidad está vacío. Se requiere inyección manual.",
  },

  /** Analítica de Esfuerzo Estratégico (Engine B) */
  analytics_page: {
    title: "Analítica de Esfuerzo Estratégico",
    subtitle: "Kernel de Auditoría Computacional // Versión 10.8 Operativa",
    effort_distribution: "Distribución Global del Esfuerzo de Auditoría",
    hardware_efficiency: "Ratio de Eficiencia de Hardware Distribuido",
    geographical_nodes: "Mapa de Distribución Geográfica de Nodos",
    time_series_label: "Secuencia Temporal de Auditoría",
    metrics: {
      hashes_per_watt: "Eficiencia Energética Proyectada (H/W)",
      avg_latency: "Latencia Media del Apretón de Manos Neural",
      collision_prob: "Probabilidad Estadística de Colisión",
    },
  },

  /** Telemetría de Alta Frecuencia */
  analytics: {
    total_effort: "Esfuerzo Global de Auditoría Computacional",
    hash_unit: "Cuatrillones de Identidades Potenciales Escaneadas",
    efficiency: "Ratio de Eficiencia de Procesamiento del Enjambre",
    zombie_rate: "Velocidad de Descubrimiento de Direcciones Zombie",
  },

  /** Monitor de Paridad de Motores Gemelos */
  archival_status: {
    engine_b_parity: "Monitor de Paridad del Motor B",
    strategic_vault_link: "Enlace de Archivo Estratégico (Supabase)",
    archival_integrity: "Integridad de la Cadena de Archivo",
    sync_drift_detected: "DERIVA_DE_SINCRONIZACIÓN: {count} misiones pendientes de migración estratégica.",
    total_archived_missions: "Total de Misiones Certificadas en Almacenamiento Frío",
  },

  /** Ledger de Auditoría Inmutable (Huella Forense) */
  audit_trail: {
    title: "Ledger de Auditoría de Misiones Inmutable",
    column_mission: "Identificador de la Misión",
    column_strategy: "Estrategia Aplicada",
    column_effort: "Volumen Computacional",
    column_status: "Estado de la Certificación",
    column_footprint: "Huella de Verificación (Hexadecimal)",
    empty_state: "El Archivo Estratégico está esperando la migración de datos desde el Estrato L3.",
  },

  /** Motores de Estrategia Criptográfica */
  strategies: {
    sequential: "Auditoría de Rango Secuencial U256",
    dictionary: "Apretón de Manos por Diccionario de Entropía",
    static_handshake: "Verificación de Secreto Específico",
    forensic_archaeology: "Recuperación de Patrones PRNG Históricos",
  },
};
