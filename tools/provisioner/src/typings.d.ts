// =================================================================
// APARATO: TYPE SHIMS (SILENCIADOR DE TS2307)
// OBJETIVO: Permitir compilación de librerías sin tipos oficiales
// =================================================================

declare module 'fingerprint-generator' {
    export class FingerprintGenerator {
        constructor(options?: any);
        getFingerprint(options?: any): any;
    }
}

declare module 'fingerprint-injector' {
    export class FingerprintInjector {
        constructor(options?: any);
        attachFingerprintToPlaywright(context: any, fingerprint: any): Promise<void>;
    }
}

declare module 'ghost-cursor-playwright' {
    import { Page } from 'playwright';
    export function createCursor(page: Page, start?: any, performRandomMove?: boolean): Promise<any>;
}
