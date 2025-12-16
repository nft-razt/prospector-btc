// INICIO DEL ARCHIVO [apps/web-dashboard/tools/scripts/generate-i18n.ts]
/**
 * =================================================================
 * APARATO: I18N COMPILER v3.5 (STRICT TYPE SAFETY)
 * OBJETIVO: Generar JSON estático desde el Registro TypeScript
 * ESTADO: FIXED (IMPLICIT ANY RESOLVED)
 * =================================================================
 */

import * as fs from "fs";
import * as path from "path";
import chalk from "chalk";
import { ZodIssue } from "zod"; // ✅ Importación necesaria para el tipo

// Importaciones relativas correctas desde 'tools/scripts' hacia 'lib'
// Ruta: ../../lib/i18n... es correcta desde aquí.
import { AppLocaleSchema, type AppLocale } from "../../lib/i18n/schema";
import { enRegistry } from "../../lib/i18n/registry";

// Configuración de entorno agnóstica
const CWD = process.cwd();
const IS_NX_ROOT = fs.existsSync(path.join(CWD, "nx.json"));
const APP_ROOT = IS_NX_ROOT ? path.join(CWD, "apps/web-dashboard") : CWD;
const TARGET_DIR = path.join(APP_ROOT, "messages");
const LOCALES = ["en", "es"];

async function compile() {
  const start = performance.now();
  console.log(chalk.bold.blue("\n🌐 [I18N COMPILER] Sincronizando Fuentes de Verdad..."));

  // 1. FASE DE VALIDACIÓN (AUDITORÍA)
  const validation = AppLocaleSchema.safeParse(enRegistry);

  if (!validation.success) {
    console.error(chalk.bgRed.white("\n ❌ ERROR DE CONTRATO (ZOD SCHEMA MISMATCH) \n"));

    // ✅ CORRECCIÓN 2: Tipado explícito (i: ZodIssue) para silenciar error TS7006
    validation.error.issues.forEach((i: ZodIssue) => {
      console.error(chalk.red(`   - [${i.path.join(".")}] ${i.message}`));
    });

    process.exit(1);
  }

  console.log(chalk.green("   ✅ Integridad de Datos Verificada."));

  // 2. FASE DE GENERACIÓN (ARTEFACTOS)
  if (!fs.existsSync(TARGET_DIR)) {
    fs.mkdirSync(TARGET_DIR, { recursive: true });
  }

  // Estrategia de Multiplicación
  const payloads: Record<string, AppLocale> = {
    en: enRegistry,
    es: enRegistry, // Placeholder: Español usa Inglés hasta tener traducciones reales
  };

  LOCALES.forEach((locale) => {
    const filePath = path.join(TARGET_DIR, `${locale}.json`);

    // Minificación para producción
    const content = JSON.stringify(payloads[locale]);

    fs.writeFileSync(filePath, content);
    console.log(chalk.gray(`   💾 ${locale}.json generado (${(content.length / 1024).toFixed(2)} KB).`));
  });

  const duration = (performance.now() - start).toFixed(2);
  console.log(chalk.bold.green(`\n🏁 Compilación Exitosa en ${duration}ms.\n`));
}

compile().catch((err) => {
  console.error(chalk.red("\n🔥 FATAL ERROR:"), err);
  process.exit(1);
});
// FIN DEL ARCHIVO [apps/web-dashboard/tools/scripts/generate-i18n.ts]
