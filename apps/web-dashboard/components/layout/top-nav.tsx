'use client';

import { Breadcrumbs } from '@/components/layout/breadcrumbs';
import { UserNav } from '@/components/layout/user-nav';
import { ThemeToggle } from '@/components/layout/theme-toggle';

interface TopNavProps {
  user: {
    name?: string | null;
    email?: string | null;
    image?: string | null;
  }
}

/**
 * ORGANISMO: TOP NAVIGATION
 * Barra superior de contexto. Contiene Breadcrumbs y herramientas globales.
 */
export function TopNav({ user }: TopNavProps) {
  return (
    <div className="flex w-full items-center justify-between h-full">
      {/* 1. Contexto (Izquierda) */}
      <Breadcrumbs />

      {/* 2. Herramientas (Derecha) */}
      <div className="flex items-center gap-3">
        <div className="bg-card border border-border rounded-full p-1 flex items-center">
           <ThemeToggle />
        </div>

        {/* Separador vertical visual */}
        {/* CORRECCIÓN: bg-gradient-to-b -> bg-linear-to-b */}
        <div className="h-8 w-px bg-linear-to-b from-transparent via-border to-transparent mx-1" />

        <UserNav user={user} />
      </div>
    </div>
  );
}
