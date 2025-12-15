// tools/provisioner/src/typings.d.ts
// =================================================================
// APARATO: TYPE SHIMS (SILENCIADOR DE TS2307/TS2305)
// OBJETIVO: Tipado estricto para librerías sin soporte oficial
// =================================================================

declare module "fingerprint-generator" {
  export class FingerprintGenerator {
    constructor(options?: any);
    getFingerprint(options?: any): any;
  }
}

declare module "fingerprint-injector" {
  export class FingerprintInjector {
    constructor(options?: any);
    attachFingerprintToPlaywright(
      context: any,
      fingerprint: any,
    ): Promise<void>;
  }
}

declare module "ghost-cursor-playwright" {
  import { Page } from "playwright";

  // Definición de la interfaz del objeto Cursor
  export interface GhostCursor {
    toggleRandomMove(random: boolean): void;
    click(selector?: string | any, options?: any): Promise<void>;
    move(selector: string | any, options?: any): Promise<void>;
    moveTo(destination: { x: number; y: number }): Promise<void>;
  }

  // Exportación de la función factoría
  export function createCursor(
    page: Page,
    start?: { x: number; y: number },
    performRandomMove?: boolean,
  ): Promise<GhostCursor>;
}
