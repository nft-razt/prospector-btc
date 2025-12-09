// tools/provisioner/src/lib/browser.ts
// =================================================================
// APARATO: BROWSER FACTORY (ELITE STEALTH)
// RESPONSABILIDAD: CREACIÓN DE CONTEXTOS DE NAVEGACIÓN Y GESTIÓN DE IDENTIDAD
// MEJORA: TRAZABILIDAD DE IDENTIDAD (RETURN METADATA)
// =================================================================

import { chromium } from 'playwright-extra';
import stealth from 'puppeteer-extra-plugin-stealth';
import { BrowserContext, Browser } from 'playwright';
import { FingerprintGenerator } from 'fingerprint-generator';
import { FingerprintInjector } from 'fingerprint-injector';
import axios from 'axios';
import * as fs from 'fs';
import * as path from 'path';

import { config } from '../config';
import { purifyCookies } from './cookie-purifier';

// Activación del plugin de sigilo a nivel global
chromium.use(stealth());

/** Estructura de respuesta del endpoint de Lease */
interface LeasedIdentity {
  id: string;
  email: string;
  credentials_json: string;
  user_agent: string;
}

/** Resultado de la creación del contexto */
export interface BrowserContextResult {
  browser: Browser;
  context: BrowserContext;
  /** Email de la identidad inyectada (si existe) para reportes de fallo */
  identityEmail: string | null;
}

export class BrowserFactory {
  private static fingerprintGenerator = new FingerprintGenerator({
    browsers: [{ name: 'chrome', minVersion: 110 }],
    devices: ['desktop'],
    operatingSystems: ['windows', 'linux'],
  });

  private static fingerprintInjector = new FingerprintInjector();

  /**
   * Crea un navegador y contexto configurados con huella digital única y credenciales.
   */
  static async createContext(): Promise<BrowserContextResult> {
    console.log('🎭 [BROWSER] Generando identidad digital sintética...');

    // 1. Generación de Fingerprint (Hardware Spoofing)
    const fingerprint = this.fingerprintGenerator.getFingerprint();

    // 2. Lanzamiento del Motor (Chromium)
    const browser = await chromium.launch({
      headless: config.HEADLESS,
      args: [
        '--disable-blink-features=AutomationControlled',
        '--no-sandbox',
        '--disable-setuid-sandbox',
        '--disable-infobars',
        '--ignore-certificate-errors',
        '--disable-dev-shm-usage',
        '--disable-gpu',
        `--window-size=${fingerprint.screen.width},${fingerprint.screen.height}`
      ],
    });

    // 3. Configuración del Contexto
    const context = await browser.newContext({
      userAgent: fingerprint.navigator.userAgent,
      viewport: {
        width: fingerprint.screen.width,
        height: fingerprint.screen.height
      },
      locale: 'en-US',
      timezoneId: 'America/New_York',
      permissions: ['clipboard-read', 'clipboard-write'],
      deviceScaleFactor: 1,
    });

    // 4. Inyección de Huella Digital (Evasión Activa)
    await this.fingerprintInjector.attachFingerprintToPlaywright(context, fingerprint);

    // 5. Inyección de Identidad (Cookies)
    const identityEmail = await this.injectIdentity(context);

    return { browser, context, identityEmail };
  }

  /**
   * Obtiene e inyecta cookies de sesión.
   * Prioridad: 1. The Vault (API) -> 2. Archivo Local -> 3. Anónimo.
   * @returns El email de la identidad inyectada o null.
   */
  private static async injectIdentity(context: BrowserContext): Promise<string | null> {
    let rawCookies: any[] = [];
    let identityEmail: string | null = null;
    let source = 'NONE';

    // A. Intento Remoto (The Vault)
    if (config.ORCHESTRATOR_URL && config.WORKER_AUTH_TOKEN) {
      try {
        console.log('📡 [IDENTITY] Solicitando credenciales a The Vault...');
        const response = await axios.get<LeasedIdentity>(
          `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/lease`,
          {
            params: { platform: 'google_colab' },
            headers: { 'Authorization': `Bearer ${config.WORKER_AUTH_TOKEN}` },
            timeout: 5000
          }
        );

        if (response.data) {
          rawCookies = JSON.parse(response.data.credentials_json);
          identityEmail = response.data.email;
          source = `VAULT (${identityEmail})`;

          // Sincronizamos User-Agent si la identidad tiene uno específico
          // Esto es vital para que Google no detecte cambio de navegador
          if (response.data.user_agent) {
             // Nota: En Playwright el UA se define al crear el contexto.
             // Aquí ya es tarde para cambiarlo a nivel de contexto root,
             // pero las cookies suelen ser tolerantes si el fingerprint es consistente.
          }
        }
      } catch (e: any) {
        console.warn(`⚠️ [IDENTITY] Fallo en Vault (Offline/Empty): ${e.message}`);
      }
    }

    // B. Fallback Local (Desarrollo)
    if (rawCookies.length === 0) {
      const localPath = path.resolve('cookies.json');
      if (fs.existsSync(localPath)) {
        try {
          rawCookies = JSON.parse(fs.readFileSync(localPath, 'utf-8'));
          source = 'LOCAL_FILE';
          identityEmail = 'local-dev-user@localhost';
        } catch {}
      }
    }

    // C. Purificación e Inyección
    if (rawCookies.length > 0) {
      const cleanCookies = purifyCookies(rawCookies);
      if (cleanCookies.length > 0) {
        await context.addCookies(cleanCookies);
        console.log(`✅ [IDENTITY] ${cleanCookies.length} cookies inyectadas. Fuente: ${source}`);
        return identityEmail;
      } else {
        console.error('❌ [IDENTITY] Cookies inválidas tras purificación.');
      }
    } else {
      console.warn('⚠️ [IDENTITY] Iniciando en modo ANÓNIMO (Sin login).');
    }

    return null;
  }
}
