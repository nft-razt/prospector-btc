// apps/web-dashboard/config/navigation.ts
/**
 * =================================================================
 * APARATO: NAVIGATION CONFIG (FIXED)
 * RESPONSABILIDAD: MAPEO DE RUTAS DEL SISTEMA DE COMANDO
 * ESTADO: ALINEADO CON ESTRUCTURA DE ARCHIVOS (/dashboard/lab)
 * =================================================================
 */

import {
  LayoutDashboard,
  Network,
  ShieldCheck,
  GraduationCap,
  Settings,
  FlaskConical, // Icono más apropiado para "Lab"
  type LucideIcon,
} from "lucide-react";

export interface RouteItem {
  href: string;
  translationKey: string;
  icon: LucideIcon;
  /**
   * 'exact': Solo activo si pathname === href
   * 'includes': Activo si pathname empieza con href
   */
  matchMode: "exact" | "includes";
}

/**
 * Mapeo estático de la navegación principal.
 * SSoT (Single Source of Truth) para el Sidebar.
 */
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
  {
    // ✅ CORRECCIÓN: Ruta actualizada a '/dashboard/lab' para coincidir con page.tsx
    // Cambiamos el icono a FlaskConical para denotar experimentación científica.
    href: "/dashboard/lab",
    translationKey: "wallet_lab", // Se mantiene la clave i18n
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
