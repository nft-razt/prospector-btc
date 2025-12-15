// tools/provisioner/src/harvester.ts
// =================================================================
// APARATO: IDENTITY HARVESTER v2.0 (STEALTH EDITION)
// OBJETIVO: Evasión de detección de bots de Google
// =================================================================

import { chromium } from "playwright-extra"; // Usamos la versión "extra"
import stealth from "puppeteer-extra-plugin-stealth";
import * as fs from "fs";
import * as path from "path";
import chalk from "chalk";

// Inyectamos el plugin de sigilo
chromium.use(stealth());

const COOKIE_PATH = path.resolve(__dirname, "../../cookies.json");

async function harvest() {
  console.log(chalk.yellow("🕵️  HARVESTER v2.0: MODO STEALTH ACTIVADO"));
  console.log(chalk.gray("   Intentando engañar a los sistemas de Google..."));

  // Lanzamos el navegador con argumentos anti-detección específicos
  const browser = await chromium.launch({
    headless: false,
    channel: "chrome", // Intenta usar tu Google Chrome real instalado
    args: [
      "--disable-blink-features=AutomationControlled", // CRÍTICO: Oculta que es un bot
      "--no-sandbox",
      "--disable-setuid-sandbox",
      "--disable-infobars",
      "--window-position=0,0",
      "--ignore-certificate-errors",
      "--ignore-certificate-errors-spki-list",
      "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36", // User Agent real de Windows
    ],
  });

  const context = await browser.newContext({
    viewport: { width: 1366, height: 768 },
    locale: "es-ES",
    timezoneId: "America/Mexico_City", // Ajusta a tu zona si es necesario
  });

  const page = await context.newPage();

  console.log(chalk.cyan("👉 Navegando a Login..."));

  // Usamos una URL de login antigua que a veces tiene menos seguridad, redirigiendo a Colab
  await page.goto(
    "https://accounts.google.com/ServiceLogin?service=wise&passive=1209600&continue=https://colab.research.google.com/&followup=https://colab.research.google.com/",
    { waitUntil: "domcontentloaded" },
  );

  console.log(chalk.white.bold("\n⚠️  INSTRUCCIONES:"));
  console.log(
    "   1. Introduce tu correo y contraseña en la ventana emergente.",
  );
  console.log(
    '   2. Si Google te bloquea de nuevo, NO CIERRES. Intenta usar "Probar otra manera" si aparece.',
  );
  console.log(
    "   3. Una vez que veas la interfaz de Google Colab (tus notebooks), vuelve aquí.",
  );
  console.log(
    chalk.green.bold("   4. PRESIONA [ENTER] EN ESTA TERMINAL PARA GUARDAR.\n"),
  );

  // Esperar input del usuario
  await new Promise((resolve) => process.stdin.once("data", resolve));

  console.log(chalk.blue("💾 Extrayendo cookies..."));
  const cookies = await context.cookies();

  // Guardamos TODAS las cookies para asegurar la sesión
  fs.writeFileSync(COOKIE_PATH, JSON.stringify(cookies, null, 2));

  console.log(chalk.green(`✅ COOKIES GUARDADAS EN: ${COOKIE_PATH}`));

  await browser.close();
  process.exit(0);
}

harvest().catch((err) => {
  console.error(chalk.red("❌ ERROR:"), err);
});
