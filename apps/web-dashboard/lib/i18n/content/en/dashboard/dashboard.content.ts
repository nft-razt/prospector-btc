import { type DashboardParams } from "../../../schemas/dashboard/dashboard.schema";

export const dashboardContent = {
  sidebar: {
    overview: "Command Center",
    network: "Swarm Intelligence",
    wallet_lab: "Wallet Forge",
    academy: "Crypto Academy",
    settings: "System Config",
  },
  header: {
    welcome: "Welcome back, Operator",
    status_online: "UPLINK ESTABLISHED",
  },
  user_nav: {
    profile: "Operator Profile",
    billing: "Billing & Plan",
    settings: "User Settings",
    logout: "Terminate Session",
  },
  fleet: {
    title: "Visual Surveillance",
    live_feed: "LIVE FEED",
    no_signal: "NO VISUAL FEED DETECTED",
    deploy_hint: "Deploy workers via Provisioner to establish uplink.",
    connection_lost: "UPLINK SEVERED // RECONNECTING",
  },
} satisfies DashboardParams;
