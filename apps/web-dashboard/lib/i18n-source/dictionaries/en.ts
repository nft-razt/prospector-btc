// libs/shared/i18n-config/src/lib/dictionaries/en.ts

import { AppLocale } from "../schema";

/**
 * Diccionario Maestro en Inglés (Default Locale).
 *
 * NOTA DE ARQUITECTURA:
 * Este objeto es la FUENTE DE VERDAD. El script de build (`pnpm i18n:gen`)
 * valida este objeto contra `AppLocaleSchema` antes de generar los archivos
 * `messages/en.json` que consume Next-Intl.
 */
export const enDictionary: AppLocale = {
  Common: {
    loading: "Initializing Systems...",
    error: "Critical Failure",
    copy: "Copy to Clipboard",
    success: "Operation Successful",
  },
  Landing: {
    hero: {
      title: "PROSPECTOR BTC",
      subtitle: "Industrial Grade Entropy Audit & Cryptographic Learning Suite",
      cta_primary: {
        label: "Initialize System",
        tooltip: "Start Free Tier",
      },
    },
    pricing: {
      observer_title: "Observer Node",
      observer_desc: "Access to public telemetry and basic network status.",
      operator_title: "Operator Node",
      operator_desc:
        "Full mining capabilities, priority queue, and deep entropy analysis.",
      currency: "USD/mo",
      cta_free: "INITIALIZE",
      cta_pro: "SUBSCRIBE PRO",
    },
  },
  Dashboard: {
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
    // Sección del Panóptico (Vigilancia)
    fleet: {
      title: "Visual Surveillance",
      live_feed: "LIVE FEED",
      no_signal: "NO VISUAL FEED DETECTED",
      deploy_hint: "Deploy workers via Provisioner to establish uplink.",
    },
  },
  Auth: {
    login_title: "Identify Yourself",
    login_google: "Authenticate via Google",
    login_footer: "Secure Connection // TLS 1.3",
    logout: "Logging out...",
  },
  System: {
    not_found: {
      title: "SIGNAL LOST",
      description:
        "The requested coordinates do not correspond to any known sector in the Prospector network.",
      error_code: "ERR_404_VOID",
      cta_return: "Return to Command Center",
    },
  },
};
