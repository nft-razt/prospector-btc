/**
 * =================================================================
 * APARATO: DASHBOARD I18N CONTENT (V47.0 - STRATEGIC EDITION)
 * CLASIFICACIÓN: DOMAIN CONTENT (ESTRATO L5)
 * RESPONSABILIDAD: DICCIONARIO MAESTRO EN INGLÉS PARA EL CENTRO DE MANDO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la Fuente de Verdad (SSoT) de contenidos textuales para la
 * interfaz del operador. Utiliza terminología técnica soberana y
 * cumple con el contrato definido en 'DashboardSchema'. Esta versión
 * habilita la visualización de la cadena de integridad de auditoría
 * y el monitoreo de la deriva de datos entre Motor A y Motor B.
 * =================================================================
 */

import { type DashboardParams } from "../../../schemas/dashboard/dashboard.schema";

export const dashboardContent = {
  /** Navegación Lateral Estratégica */
  sidebar: {
    overview: "Mission Command Center",
    network: "Distributed Swarm Grid",
    analytics_deep: "Strategic Effort Analytics",
    wallet_lab: "Cryptographic Vulnerability Lab",
    academy: "Hydra Technical Academy",
    settings: "Sovereign Infrastructure Config",
  },

  /** Componentes de Cabecera y Estado Neural */
  header: {
    welcome: "Welcome back, Operator",
    status_online: "NEURAL_LINK_SYNCHRONIZED",
  },

  /** Gestión de Identidad de Operador */
  user_nav: {
    profile: "Operator Identity Profile",
    billing: "Subscription & Resource Quotas",
    settings: "Security & Local Encryption Settings",
    logout: "Terminate Active Session Protocol",
  },

  /** Vigilancia Visual del Enjambre (El Panóptico) */
  fleet: {
    title: "Real-Time Visual Surveillance",
    live_feed: "ACTIVE_VIDEO_TRANSMISSION",
    no_signal: "NO VISUAL SIGNAL DETECTED FROM GRID UNIT",
    deploy_hint: "Initialize grid units via Provisioner to establish a neural uplink.",
    connection_lost: "TACTICAL_VISUAL_LINK_SEVERED // RE-ESTABLISHING HANDSHAKE",
  },

  /** Estrato de Pruebas y Certificación Forense */
  lab: {
    title: "Experimental Research Stratum",
    interceptor_title: "Neural Entropy Interceptor",
    forge_title: "Scenario Forge & Crystallizer",
    scan_btn: "INITIALIZE SCAN SEQUENCE",
    inject_btn: "CRYSTALLIZE GOLDEN TICKET",
    no_scenarios: "NO ACTIVE CRYPTOGRAPHIC EXPERIMENTS IN LEDGER",
  },

  /** Bóveda Zero-Knowledge (Seguridad del Lado del Cliente) */
  vault: {
    title: "Zero-Knowledge Identity Vault",
    injection_badge: "AES-256-GCM PROTECTION ACTIVE",
    encrypting: "ENCRYPTING_IDENTITY_PAYLOAD_LOCALLY...",
    secure_btn: "SECURE IN TACTICAL LEDGER",
    empty_vault: "The Identity Bunker is empty. Manual injection required.",
  },

  /** Analítica de Esfuerzo Estratégico (Engine B Source) */
  analytics_page: {
    title: "Strategic Effort Analytics",
    subtitle: "Computational Audit Kernel // Version 10.8 Operational",
    effort_distribution: "Global Audit Effort Distribution",
    hardware_efficiency: "Distributed Hardware Efficiency Ratio",
    geographical_nodes: "Geographical Node Distribution Map",
    time_series_label: "Temporal Audit Sequence",
    metrics: {
      hashes_per_watt: "Projected Energy Efficiency (H/W)",
      avg_latency: "Average Neural Handshake Latency",
      collision_prob: "Statistical Collision Probability",
    },
  },

  /** Métricas de Telemetría Global */
  analytics: {
    total_effort: "Global Computational Audit Effort",
    hash_unit: "Quadrillions of Potential Identities Scanned",
    efficiency: "Swarm Processing Efficiency Ratio",
    zombie_rate: "Zombie Address Discovery Velocity",
  },

  /** Monitor de Paridad de Motores Gemelos (V110.0) */
  archival_status: {
    engine_b_parity: "Engine B Parity Monitor",
    strategic_vault_link: "Strategic Archival Link (Supabase)",
    archival_integrity: "Archival Chain Integrity",
    sync_drift_detected: "SYNC_DRIFT: {count} missions pending strategic migration.",
    total_archived_missions: "Total Certified Missions in Cold Storage",
  },

  /** Ledger de Auditoría Inmutable (Trazabilidad Tesis) */
  audit_trail: {
    title: "Immutable Mission Audit Ledger",
    column_mission: "Mission Identifier",
    column_strategy: "Applied Strategy",
    column_effort: "Computational Volume",
    column_status: "Certification Status",
    column_footprint: "Verification Footprint (Hex)",
    empty_state: "The Strategic Archive is awaiting data migration from Stratum L3.",
  },

  /** Denominaciones del Motor de Estrategias */
  strategies: {
    sequential: "Sequential U256 Range Audit",
    dictionary: "Entropy Dictionary Handshake",
    static_handshake: "Specific Secret Verification",
    forensic_archaeology: "Historical PRNG Pattern Recovery",
  },
} satisfies DashboardParams;
