// apps/web-dashboard/tools/scripts/generate-i18n.ts
// =================================================================
// APARATO: I18N COMPILER (ELITE EDITION)
// RESPONSABILIDAD: TRANSFORMACIÓN ZOD -> JSON ESTÁTICO
// =================================================================

import * as fs from "fs";
import * as path from "path";
const chalk = require("chalk");
import { z } from "zod";

// 1. IMPORTACIÓN DE LA ÚNICA VERDAD (i18n-source)
// Si esta importación falla, el build DEBE fallar.
import { enDictionary } from "../../lib/i18n-source/dictionaries/en";
import { AppLocaleSchema, type AppLocale } from "../../lib/i18n-source/schema";

// 2. DETECCIÓN DE ENTORNO (CI/CD AWARE)
const CWD = process.cwd();
const IS_NX_ROOT = fs.existsSync(path.join(CWD, "nx.json"));

// Si corremos desde la raíz (Nx), entramos a la app. Si estamos en la app (Docker), usamos CWD.
const APP_ROOT = IS_NX_ROOT ? path.join(CWD, "apps/web-dashboard") : CWD;

const TARGET_DIR = path.join(APP_ROOT, "messages");
const LOCALES = ["en", "es"];

async function compile() {
  const start = performance.now();

  console.log(
    chalk.bold.blue(
      "\n🌐 [I18N COMPILER] Iniciando secuencia de generación...",
    ),
  );
  console.log(
    chalk.gray(`   📂 Contexto: ${IS_NX_ROOT ? "Monorepo Root" : "App Root"}`),
  );
  console.log(chalk.gray(`   🎯 Output:   ${TARGET_DIR}`));

  // --- FASE 1: VALIDACIÓN DE INTEGRIDAD ---
  console.log(chalk.cyan("   🔍 Auditando esquema Zod..."));

  const validation = AppLocaleSchema.safeParse(enDictionary);

  if (!validation.success) {
    console.error(
      chalk.bgRed.white.bold(
        "\n ❌ FATAL: EL DICCIONARIO MAESTRO ESTÁ CORRUPTO \n",
      ),
    );
    validation.error.issues.forEach((err) => {
      console.error(chalk.red(`   - [${err.path.join(".")}] ${err.message}`));
    });
    process.exit(1); // Romper el build inmediatamente
  }

  console.log(chalk.green("   ✅ Integridad verificada."));

  // --- FASE 2: GENERACIÓN DE ARTEFACTOS ---
  if (!fs.existsSync(TARGET_DIR)) {
    fs.mkdirSync(TARGET_DIR, { recursive: true });
  }

  // Estrategia de Espejo: Por ahora ES = EN (hasta tener traducciones reales)
  const payloads: Record<string, AppLocale> = {
    en: enDictionary,
    es: enDictionary,
  };

  LOCALES.forEach((locale) => {
    const filePath = path.join(TARGET_DIR, `${locale}.json`);
    const data = JSON.stringify(payloads[locale]); // Minificado para producción
    fs.writeFileSync(filePath, data);

    const size = (Buffer.byteLength(data) / 1024).toFixed(2);
    console.log(
      chalk.white(
        `   💾 Artefacto generado: ${chalk.bold(locale + ".json")} (${size} KB)`,
      ),
    );
  });

  const duration = (performance.now() - start).toFixed(2);
  console.log(chalk.bold.green(`\n🏁 I18N LISTO en ${duration}ms\n`));
}

compile();
