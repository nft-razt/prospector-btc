// =================================================================
// APARATO: BROWSER FACTORY (STEALTH + IDENTITY AWARE)
// RESPONSABILIDAD: CREACIÓN DE CONTEXTOS DE NAVEGACIÓN INDETECTABLES
// NIVEL: ELITE PRODUCTION
// =================================================================

import { chromium } from 'playwright-extra';
import stealth from 'puppeteer-extra-plugin-stealth';
import { BrowserContext, Browser } from 'playwright';
import axios from 'axios';
import * as fs from 'fs';
import * as path from 'path';

// Importación de configuración soberana
import { config } from '../config';

// Inyección del plugin de evasión al nivel del driver
chromium.use(stealth());

/**
 * Interfaz para la respuesta de identidad del Orquestador
 */
interface LeasedIdentity {
  id: string;
  platform: string;
  email: string;
  credentials_json: string; // JSON stringificado de las cookies
  user_agent: string;
}

export class BrowserFactory {
  /**
   * Inicializa un navegador y un contexto seguro con identidad inyectada.
   * Aplica parches anti-detección a nivel de argumentos de lanzamiento.
   */
  static async createContext(): Promise<{ context: BrowserContext; browser: Browser }> {
    console.log('🚀 [BROWSER] Inicializando motor Chromium Stealth...');

    // 1. Lanzamiento del Navegador
    // Usamos argumentos específicos para ocultar la automatización
    const browser = await chromium.launch({
      headless: config.HEADLESS,
      channel: 'chrome', // Intenta usar el binario real de Google Chrome si está instalado
      args: [
        '--disable-blink-features=AutomationControlled', // CRÍTICO: Oculta 'navigator.webdriver'
        '--no-sandbox',
        '--disable-setuid-sandbox',
        '--disable-infobars',
        '--window-position=0,0',
        '--ignore-certificate-errors',
        '--disable-web-security',
        '--disable-features=IsolateOrigins,site-per-process', // Mejora performance en iframes
      ],
    });

    // 2. Creación del Contexto
    // Configuramos huellas digitales básicas para parecer un humano real
    const context = await browser.newContext({
      viewport: { width: 1366, height: 768 },
      // User Agent base (será sobrescrito si la identidad trae uno específico)
      userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
      locale: 'en-US',
      timezoneId: 'America/New_York',
      permissions: ['clipboard-read', 'clipboard-write'], // Necesario para inyección de payload
    });

    // 3. Inyección de Identidad (The Iron Vault Protocol)
    await this.injectIdentity(context);

    return { context, browser };
  }

  /**
   * Lógica de obtención y aplicación de cookies.
   * Prioridad: API (Vault) > ENV (Cloud) > FILE (Local)
   */
  private static async injectIdentity(context: BrowserContext) {
    let cookies: any[] = [];
    let source = 'NONE';

    // A. ESTRATEGIA: THE VAULT (API REST)
    // Intentamos arrendar una identidad gestionada por el Orquestador
    if (config.ORCHESTRATOR_URL && config.WORKER_AUTH_TOKEN) {
      try {
        console.log('📡 [IDENTITY] Contactando a The Vault (Orchestrator) para arrendamiento...');

        const response = await axios.get<LeasedIdentity>(
          `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/lease`,
          {
            params: { platform: 'google_colab' },
            headers: { 'Authorization': `Bearer ${config.WORKER_AUTH_TOKEN}` },
            timeout: 5000 // Fail fast si el orquestador no responde
          }
        );

        if (response.data) {
          const identity = response.data;
          console.log(`🔑 [IDENTITY] Identidad Arrendada: ${identity.email}`);

          // Parseamos las cookies que vienen como string JSON en la DB
          cookies = JSON.parse(identity.credentials_json);
          source = 'VAULT_API';

          // Si la identidad tiene un User-Agent específico guardado, lo aplicamos
          if (identity.user_agent && identity.user_agent.length > 5) {
             // Nota: Playwright no permite cambiar UA dinámicamente fácil en un contexto ya creado
             // sin usar route/fulfillment, pero podemos establecer headers extra.
             // Para v3.0 simple, confiamos en el UA del contexto base o implementamos reinicio de contexto.
             // Aquí solo logueamos la intención.
             // console.log(`ℹ️ [IDENTITY] Target UA: ${identity.user_agent}`);
          }
        }
      } catch (e: any) {
        const status = e.response?.status;
        if (status === 404) {
           console.warn('⚠️ [IDENTITY] Stock agotado en The Vault. No hay cuentas disponibles.');
        } else {
           console.warn(`⚠️ [IDENTITY] Fallo de conexión con The Vault: ${e.message}. Intentando fallback...`);
        }
      }
    }

    // B. ESTRATEGIA: VARIABLES DE ENTORNO (CI/CD)
    if (cookies.length === 0 && config.GOOGLE_COOKIES_JSON) {
      try {
        // Soporte para JSON directo
        if (config.GOOGLE_COOKIES_JSON.startsWith('[')) {
            cookies = JSON.parse(config.GOOGLE_COOKIES_JSON);
            source = 'ENV_VAR';
        }
      } catch (e) {
        console.error('❌ [IDENTITY] Error parseando GOOGLE_COOKIES_JSON del entorno.');
      }
    }

    // C. ESTRATEGIA: ARCHIVO LOCAL (DESARROLLO)
    if (cookies.length === 0) {
      const cookiePath = path.resolve('cookies.json');
      if (fs.existsSync(cookiePath)) {
        try {
          const content = fs.readFileSync(cookiePath, 'utf-8');
          cookies = JSON.parse(content);
          source = 'LOCAL_FILE';
        } catch (e) {
          console.error(`❌ [IDENTITY] Archivo local corrupto: ${cookiePath}`);
        }
      }
    }

    // D. APLICACIÓN FINAL
    if (cookies.length > 0) {
      try {
        await context.addCookies(cookies);
        console.log(`✅ [IDENTITY] Identidad inyectada exitosamente. Fuente: ${source}`);
      } catch (e) {
        console.error('❌ [IDENTITY] Error crítico aplicando cookies al navegador:', e);
        throw new Error('IdentityInjectionFailed');
      }
    } else {
      console.warn('⚠️ [IDENTITY] ADVERTENCIA: Iniciando en modo ANÓNIMO (Sin login). Es probable que Colab requiera autenticación manual.');
      // No lanzamos error para permitir debugging manual en modo headful
    }
  }
}
