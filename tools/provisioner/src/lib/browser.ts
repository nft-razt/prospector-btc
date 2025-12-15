// tools/provisioner/src/lib/browser.ts
import { chromium } from "playwright-extra";
import stealth from "puppeteer-extra-plugin-stealth";
import { BrowserContext, Browser } from "playwright";
import { FingerprintGenerator } from "fingerprint-generator";
import { FingerprintInjector } from "fingerprint-injector";
import axios from "axios";
import * as fs from "fs";
import * as path from "path";

import { config } from "../config";
import { purifyCookies } from "./cookie-purifier";

// Activación global de Stealth (Evasión de detección básica)
chromium.use(stealth());

interface LeasedIdentity {
  id: string;
  email: string;
  credentials_json: string;
  user_agent: string;
}

export interface BrowserContextResult {
  browser: Browser;
  context: BrowserContext;
  identityEmail: string | null;
}

/**
 * Fábrica de contextos de navegación avanzados.
 * Genera huellas digitales únicas para cada worker para evitar vinculación de sesiones.
 */
export class BrowserFactory {
  // Configuración del generador de huellas (Chrome Desktop Moderno)
  private static fingerprintGenerator = new FingerprintGenerator({
    browsers: [{ name: "chrome", minVersion: 115 }],
    devices: ["desktop"],
    operatingSystems: ["windows", "linux"], // Colab espera OS de escritorio
  });

  private static fingerprintInjector = new FingerprintInjector();

  /**
   * Crea un navegador y contexto aislados.
   */
  static async createContext(): Promise<BrowserContextResult> {
    // 1. Generación de Fingerprint
    const fingerprint = this.fingerprintGenerator.getFingerprint();

    // 2. Lanzamiento del Motor (Chromium)
    const browser = await chromium.launch({
      headless: config.HEADLESS,
      args: [
        "--disable-blink-features=AutomationControlled", // CRÍTICO
        "--no-sandbox",
        "--disable-setuid-sandbox",
        "--disable-infobars",
        "--ignore-certificate-errors",
        "--disable-dev-shm-usage",
        "--disable-gpu", // Ahorro de recursos en CI
        `--window-size=${fingerprint.screen.width},${fingerprint.screen.height}`,
      ],
    });

    // 3. Configuración del Contexto
    const context = await browser.newContext({
      userAgent: fingerprint.navigator.userAgent,
      viewport: {
        width: fingerprint.screen.width,
        height: fingerprint.screen.height,
      },
      locale: "en-US",
      timezoneId: "America/New_York", // Consistencia con IPs de centros de datos US
      permissions: ["clipboard-read", "clipboard-write"],
      deviceScaleFactor: 1,
    });

    // 4. Inyección de Huella Digital (Sobreescritura de navigator.*)
    await this.fingerprintInjector.attachFingerprintToPlaywright(
      context,
      fingerprint,
    );

    // 5. Inyección de Identidad (Cookies)
    const identityEmail = await this.injectIdentity(context);

    return { browser, context, identityEmail };
  }

  /**
   * Orquesta la obtención e inyección de cookies.
   */
  private static async injectIdentity(
    context: BrowserContext,
  ): Promise<string | null> {
    let rawCookies: any[] = [];
    let identityEmail: string | null = null;
    let source = "NONE";

    // A. Intento Remoto (The Vault API)
    if (config.ORCHESTRATOR_URL && config.WORKER_AUTH_TOKEN) {
      try {
        const response = await axios.get<LeasedIdentity>(
          `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/lease`,
          {
            params: { platform: "google_colab" },
            headers: { Authorization: `Bearer ${config.WORKER_AUTH_TOKEN}` },
            timeout: 5000,
          },
        );

        if (response.data) {
          rawCookies = JSON.parse(response.data.credentials_json);
          identityEmail = response.data.email;
          source = `VAULT (${identityEmail})`;
        }
      } catch (e: any) {
        // Silencioso: Es normal si no hay identidades disponibles o la API está offline
      }
    }

    // B. Fallback Local (Desarrollo / Debug)
    if (rawCookies.length === 0 && config.GOOGLE_COOKIES_JSON) {
      try {
        // Intenta parsear si es string JSON directo
        rawCookies = JSON.parse(config.GOOGLE_COOKIES_JSON);
        source = "ENV_VAR";
        identityEmail = "env-user@local";
      } catch {
        // Si no es JSON, asume que es una ruta de archivo
        const localPath = path.resolve(config.GOOGLE_COOKIES_JSON);
        if (fs.existsSync(localPath)) {
          try {
            rawCookies = JSON.parse(fs.readFileSync(localPath, "utf-8"));
            source = "LOCAL_FILE";
            identityEmail = "file-user@local";
          } catch {}
        }
      }
    } else if (rawCookies.length === 0) {
      // Intento final: archivo por defecto
      const defaultPath = path.resolve("cookies.json");
      if (fs.existsSync(defaultPath)) {
        try {
          rawCookies = JSON.parse(fs.readFileSync(defaultPath, "utf-8"));
          source = "DEFAULT_FILE";
          identityEmail = "default@local";
        } catch {}
      }
    }

    // C. Purificación e Inyección
    if (rawCookies.length > 0) {
      const cleanCookies = purifyCookies(rawCookies);
      if (cleanCookies.length > 0) {
        await context.addCookies(cleanCookies);
        console.log(
          `✅ [IDENTITY] ${cleanCookies.length} cookies inyectadas. Fuente: ${source}`,
        );
        return identityEmail;
      }
    }

    console.warn(
      "⚠️ [IDENTITY] Iniciando en modo ANÓNIMO (Sin login). Capacidad limitada.",
    );
    return null;
  }
}
