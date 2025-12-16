import { type PublicHeaderParams } from "../../../schemas/layout/public-header.schema";

export const publicHeaderContent = {
  brand: "PROSPECTOR",
  nav: {
    features: "Capabilities",
    pricing: "Access Tiers",
    about: "The Thesis",
  },
  actions: {
    login: "Operator Login",
    get_started: "Initialize Node",
  },
  banner: {
    text: "System Status: V7.5 Operational",
    link: {
      label: "View Metrics",
      href: "/status",
    },
  },
} satisfies PublicHeaderParams;
