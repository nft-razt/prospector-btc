// =================================================================
// APARATO: BROWSER FACTORY (FINGERPRINTING & STEALTH)
// RESPONSABILIDAD: EVASIÓN DE DETECCIÓN BIOMÉTRICA Y DE HARDWARE
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

// 1. Activar Stealth Base
chromium.use(stealth());

interface LeasedIdentity {
  id: string;
  email: string;
  credentials_json: string;
  user_agent: string;
}

export class BrowserFactory {
  // Generador de huellas digitales sintéticas
  private static fingerprintGenerator = new FingerprintGenerator({
    browsers: [{ name: 'chrome', minVersion: 110 }],
    devices: ['desktop'],
    operatingSystems: ['windows', 'linux'], // Diversidad de OS
  });

  private static fingerprintInjector = new FingerprintInjector();

  static async createContext(): Promise<{ context: BrowserContext; browser: Browser }> {
    console.log('🎭 [BROWSER] Generando identidad digital sintética...');

    // A. Generar Fingerprint Único
    const fingerprint = this.fingerprintGenerator.getFingerprint();

    // B. Configurar Navegador (Argumentos Anti-Bot)
    const browser = await chromium.launch({
      headless: config.HEADLESS,
      args: [
        '--disable-blink-features=AutomationControlled', // CRÍTICO
        '--no-sandbox',
        '--disable-setuid-sandbox',
        '--disable-infobars',
        '--ignore-certificate-errors',
        '--disable-dev-shm-usage', // Vital para Docker
        '--disable-gpu',           // Necesario en Render/Headless
        `--window-size=${fingerprint.screen.width},${fingerprint.screen.height}`
      ],
    });

    // C. Crear Contexto con Datos del Fingerprint
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

    // D. Inyectar Fingerprint (WebGL, Canvas, Audio override)
    await this.fingerprintInjector.attachFingerprintToPlaywright(context, fingerprint);

    // E. Inyectar Identidad (Cookies)
    await this.injectIdentity(context);

    return { context, browser };
  }

  private static async injectIdentity(context: BrowserContext) {
    let rawCookies: any[] = [];
    let source = 'NONE';

    // Prioridad 1: The Vault (API)
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
          source = `VAULT (${response.data.email})`;
        }
      } catch (e: any) {
        console.warn(`⚠️ [IDENTITY] Fallo en Vault: ${e.message}`);
      }
    }

    // Prioridad 2: Archivo Local (Dev)
    if (rawCookies.length === 0) {
      const localPath = path.resolve('cookies.json');
      if (fs.existsSync(localPath)) {
        try {
          rawCookies = JSON.parse(fs.readFileSync(localPath, 'utf-8'));
          source = 'LOCAL_FILE';
        } catch {}
      }
    }

    // Purificación e Inyección
    if (rawCookies.length > 0) {
      const cleanCookies = purifyCookies(rawCookies);
      if (cleanCookies.length > 0) {
        await context.addCookies(cleanCookies);
        console.log(`✅ [IDENTITY] ${cleanCookies.length} cookies inyectadas. Fuente: ${source}`);
      } else {
        console.error('❌ [IDENTITY] Cookies inválidas tras purificación.');
      }
    } else {
      console.warn('⚠️ [IDENTITY] Iniciando en modo ANÓNIMO.');
    }
  }
}
