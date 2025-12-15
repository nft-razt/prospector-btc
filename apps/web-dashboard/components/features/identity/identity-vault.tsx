"use client";

import { IdentityInjector } from "./identity-injector";
import { IdentityInventory } from "./identity-inventory";

/**
 * FEATURE: IDENTITY VAULT (Layout)
 *
 * Contenedor principal para el subsistema de gestión de identidades.
 * Implementa un diseño responsivo de dos columnas:
 * 1. Columna Principal (2/3): Formulario de Inyección (Escritura).
 * 2. Columna Lateral (1/3): Inventario de Identidades (Lectura).
 */
export function IdentityVault() {
  return (
    <div className="grid grid-cols-1 xl:grid-cols-3 gap-6 h-full items-start animate-in fade-in duration-500 slide-in-from-bottom-4">
      {/* AREA DE COMANDO (INPUT) */}
      <div className="xl:col-span-2 h-full">
        <IdentityInjector />
      </div>

      {/* AREA DE ESTADO (OUTPUT) */}
      <div className="xl:col-span-1 h-full min-h-[400px]">
        <IdentityInventory />
      </div>
    </div>
  );
}
