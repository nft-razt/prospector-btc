import { type DashboardParams } from "../../../schemas/dashboard/dashboard.schema";

/**
 * =================================================================
 * APARATUS: DASHBOARD I18N CONTENT (V46.0 - STRATEGIC EDITION)
 * CLASSIFICATION: DOMAIN CONTENT (L5)
 * RESPONSIBILITY: ENGLISH DICTIONARY FOR THE MISSION CONTROL CENTER
 *
 * STRATEGIC IMPROVEMENTS:
 * - Deterministic Naming: Zero abbreviations in alignment with the core engine.
 * - Academic Depth: Specialized terminology for entropy audit reporting.
 * - Full Internationalization: Support for high-frequency telemetry alerts.
 * =================================================================
 */

export const dashboardContent = {
  sidebar: {
    overview: "Command & Control",
    network: "Swarm Intelligence Grid",
    analytics_deep: "Strategic Effort Analytics",
    wallet_lab: "Cryptographic Vulnerability Lab",
    academy: "Hydra Technical Academy",
    settings: "System Infrastructure Config",
  },
  header: {
    welcome: "Welcome back, Operator",
    status_online: "NEURAL_LINK_SYNCHRONIZED",
  },
  user_nav: {
    profile: "Operator Identity Profile",
    billing: "Subscription & Resource Quotas",
    settings: "Security & Encryption Settings",
    logout: "Terminate Active Session",
  },
  fleet: {
    title: "Real-Time Visual Surveillance",
    live_feed: "ACTIVE TRANSMISSION",
    no_signal: "NO VISUAL SIGNAL DETECTED FROM GRID",
    deploy_hint: "Initialize grid units via Provisioner to establish a neural uplink.",
    connection_lost: "TACTICAL_LINK_SEVERED // RE-ESTABLISHING HANDSHAKE",
  },
  lab: {
    title: "Experimental Stratum",
    interceptor_title: "Neural Interceptor Engine",
    forge_title: "Scenario Forge & Crystallizer",
    scan_btn: "INITIALIZE SCAN SEQUENCE",
    inject_btn: "CRYSTALLIZE GOLDEN TICKET",
    no_scenarios: "NO ACTIVE CRYPTOGRAPHIC EXPERIMENTS DETECTED",
  },
  vault: {
    title: "Zero-Knowledge Identity Vault",
    injection_badge: "AES-256-GCM PROTECTION ACTIVE",
    encrypting: "ENCRYPTING_IDENTITY_PAYLOAD_LOCALLY...",
    secure_btn: "SECURE IN TACTICAL LEDGER",
    empty_vault: "Bunker is currently empty. Manual identity injection required.",
  },
  analytics: {
    total_effort: "Global Computational Audit Effort",
    hash_unit: "Quadrillions of Potential Identities Scanned",
    efficiency: "Swarm Processing Efficiency Ratio",
    zombie_rate: "Zombie Address Discovery Velocity",
  },
  // Nivelación para el nuevo Audit Trail
  audit_trail: {
    title: "Immutable Mission History",
    column_mission: "Mission ID",
    column_strategy: "Applied Strategy",
    column_effort: "Computational Effort",
    column_status: "Audit Status",
    column_footprint: "Verification Footprint",
    empty_state: "The Strategic Archive is currently awaiting data migration from Stratum L3.",
  },
  strategies: {
    sequential: "Sequential U256 Range Audit",
    dictionary: "Entropy Dictionary Handshake",
    static_handshake: "Specific Secrect Verification",
    forensic_archaeology: "Historical PRNG Pattern Recovery",
  }
} satisfies DashboardParams;
