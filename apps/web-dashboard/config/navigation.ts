import {
  LayoutDashboard,
  Network,
  ShieldCheck,
  GraduationCap,
  Settings,
  type LucideIcon
} from 'lucide-react';

export interface RouteItem {
  href: string;
  translationKey: string;
  icon: LucideIcon;
  /**
   * 'exact': Solo activo si pathname === href
   * 'includes': Activo si pathname empieza con href
   */
  matchMode: 'exact' | 'includes';
}

/**
 * Mapeo estático de la navegación principal.
 * SSoT (Single Source of Truth) para el Sidebar.
 */
export const MAIN_NAVIGATION: RouteItem[] = [
  {
    href: '/dashboard',
    translationKey: 'overview',
    icon: LayoutDashboard,
    matchMode: 'exact'
  },
  {
    href: '/dashboard/network',
    translationKey: 'network',
    icon: Network,
    matchMode: 'includes'
  },
  {
    href: '/dashboard/wallet-lab',
    translationKey: 'wallet_lab',
    icon: ShieldCheck,
    matchMode: 'includes'
  },
  {
    href: '/dashboard/academy',
    translationKey: 'academy',
    icon: GraduationCap,
    matchMode: 'includes'
  },
  {
    href: '/dashboard/settings',
    translationKey: 'settings',
    icon: Settings,
    matchMode: 'includes'
  },
];
