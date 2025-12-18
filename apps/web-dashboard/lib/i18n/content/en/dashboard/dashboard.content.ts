import { type DashboardParams } from "../../../schemas/dashboard/dashboard.schema";

/**
 * =================================================================
 * APARATO: DASHBOARD ENGLISH CONTENT (V35.0)
 * CLASIFICACIÓN: DATA LAYER (L5)
 * RESPONSABILIDAD: DICCIONARIO BASE PARA EL MERCADO ANGLOSAJÓN
 * =================================================================
 */

export const dashboardContent = {
  sidebar: {
    overview: "Command Center",
    network: "Swarm Intelligence",
    analytics_deep: "Strategic Analytics",
    wallet_lab: "Vulnerability Lab",
    academy: "Hydra Academy",
    settings: "System Config",
  },
  header: {
    welcome: "Welcome back, Operator",
    status_online: "NEURAL_LINK_STABLE",
  },
  user_nav: {
    profile: "Operator Profile",
    billing: "Manage Subscription",
    settings: "Security Settings",
    logout: "Terminate Session",
  },
  fleet: {
    title: "Visual Surveillance",
    live_feed: "LIVE FEED",
    no_signal: "NO VISUAL SIGNAL DETECTED",
    deploy_hint: "Initialize grid units via Provisioner to establish uplink.",
    connection_lost: "TACTICAL_LINK_SEVERED // RECONNECTING",
  },
  lab: {
    title: "Experimental Stratum",
    interceptor_title: "Neural Interceptor",
    forge_title: "Scenario Forge",
    scan_btn: "INITIALIZE SCAN",
    inject_btn: "CRYSTALLIZE TICKET",
    no_scenarios: "NO ACTIVE EXPERIMENTS DETECTED",
  },
  vault: {
    title: "Identity Vault",
    injection_badge: "AES-256-GCM PROTECTED",
    encrypting: "ENCRYPTING_IDENTITY_PAYLOAD...",
    secure_btn: "SECURE IN VAULT",
    empty_vault: "Vault is empty. Manual injection required.",
  },
  analytics_page: {
    title: "Strategic Intelligence",
    subtitle: "Hydra-Zero Protocol Audit Deep-View",
    effort_distribution: "Computational Effort Distribution",
    hardware_efficiency: "Node Processing Efficiency",
    geographical_nodes: "Global Grid Topology",
    time_series_label: "Audit Timeline",
    metrics: {
      hashes_per_watt: "Energy Efficiency Ratio",
      avg_latency: "Global Handshake Latency",
      collision_prob: "Discovery Probability Index",
    },
  },
  analytics: {
    total_effort: "Global Audit Effort",
    hash_unit: "Billions of Hashes Scanned",
    efficiency: "Swarm Core Efficiency",
    zombie_rate: "Zombie Discovery Velocity",
  },
} satisfies DashboardParams;
