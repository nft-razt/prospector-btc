import { type LandingPageParams } from "../../../schemas/pages/landing.schema";

export const landingPageContent = {
  meta: {
    title: "Prospector // Hydra-Zero Protocol",
    description: "Distributed Cryptographic Audit System targeting secp256k1.",
  },
  hero: {
    badge: "SYSTEM OPERATIONAL // V7.5",
    title: "Distributed Entropy Archaeology",
    subtitle: "Join the global swarm. Auditing the immutable ledger through probabilistic mathematics and ephemeral computing.",
    cta_primary: {
      label: "Initialize System",
      tooltip: "Start Free Tier",
    },
  },
  capsules: {
    login: {
      title: "Active Operators",
      description: "Access Mission Control dashboard via secure handshake.",
      cta: "ACCESS CONSOLE",
      badge: "SECURE",
    },
    register: {
      title: "New Deployment",
      description: "Provision a new node identity and join the research grid.",
      cta: "INITIALIZE PROTOCOL",
      badge: "OPEN",
    },
  },
  pricing_preview: {
    title: "Access Tiers",
    observer_tier: "Observer Node",
    operator_tier: "Operator Node",
  },
} satisfies LandingPageParams;
