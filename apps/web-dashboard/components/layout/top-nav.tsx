'use client';

import { Breadcrumbs } from './breadcrumbs';
import { UserNav } from './user-nav';
import { ThemeToggle } from './theme-toggle';

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
        <div className="h-8 w-px bg-gradient-to-b from-transparent via-border to-transparent mx-1" />

        <UserNav user={user} />
      </div>
    </div>
  );
}
