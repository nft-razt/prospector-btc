/**
 * =================================================================
 * APARATO: NAVIGATION CONFIG (V25.0 - ANALYTICS ENABLED)
 * RESPONSABILIDAD: MAPEO DE RUTAS DEL SISTEMA DE COMANDO
 * ESTADO: GOLD MASTER // NO ABBREVIATIONS
 * =================================================================
 */

import {
  LayoutDashboard,
  Network,
  Settings,
  FlaskConical,
  BarChart3, // Icono para Analytics
  GraduationCap,
  type LucideIcon,
} from "lucide-react";

export interface RouteItem {
  href: string;
  translationKey: string;
  icon: LucideIcon;
  matchMode: "exact" | "includes";
}

export const MAIN_NAVIGATION: RouteItem[] = [
  {
    href: "/dashboard",
    translationKey: "overview",
    icon: LayoutDashboard,
    matchMode: "exact",
  },
  {
    href: "/dashboard/network",
    translationKey: "network",
    icon: Network,
    matchMode: "includes",
  },
  // ✅ NUEVA RUTA ESTRATÉGICA
  {
    href: "/dashboard/analytics",
    translationKey: "analytics_deep",
    icon: BarChart3,
    matchMode: "includes",
  },
  {
    href: "/dashboard/lab",
    translationKey: "wallet_lab",
    icon: FlaskConical,
    matchMode: "includes",
  },
  {
    href: "/dashboard/academy",
    translationKey: "academy",
    icon: GraduationCap,
    matchMode: "includes",
  },
  {
    href: "/dashboard/settings",
    translationKey: "settings",
    icon: Settings,
    matchMode: "includes",
  },
];
