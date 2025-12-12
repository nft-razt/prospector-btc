// =================================================================
// APARATO: I18N GENERATOR SCRIPT
// MODO: EXECUTION-SAFE (COMPATIBLE CON CI/CD)
// =================================================================

import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';
import { z } from 'zod';

// Importación directa de la Fuente de Verdad
import { enDictionary } from '../../lib/i18n-source/dictionaries/en';
import { AppLocaleSchema, type AppLocale } from '../../lib/i18n-source/schema';

// Configuración de Contexto (CI/CD Aware)
const CWD = process.cwd();
// Detectamos si estamos en la raíz del workspace (Vercel standard) o dentro de la app
const IS_ROOT = fs.existsSync(path.join(CWD, 'nx.json'));

const APP_ROOT = IS_ROOT
  ? path.join(CWD, 'apps/web-dashboard')
  : CWD;

const TARGET_DIR = path.join(APP_ROOT, 'messages');
const LOCALES = ['en', 'es'];

async function generate() {
  const startTime = performance.now();

  console.log(chalk.bold.cyan('\n🌐 [I18N COMPILER] Inicializando secuencia de generación...'));
  console.log(chalk.gray(`   📂 Contexto: ${IS_ROOT ? 'Workspace Root' : 'App Root'}`));
  console.log(chalk.gray(`   🎯 Destino:  ${TARGET_DIR}`));

  // -----------------------------------------------------------------------
  // FASE 1: VALIDACIÓN DE INTEGRIDAD (ZOD)
  // -----------------------------------------------------------------------
  console.log(chalk.blue('\n🔍 [FASE 1] Validando Diccionario Maestro (EN)...'));

  const validation = AppLocaleSchema.safeParse(enDictionary);

  if (!validation.success) {
    console.error(chalk.bold.red('❌ FATAL: El diccionario base viola el esquema de tipos.'));

    validation.error.issues.forEach((err, index) => {
      const pathStr = err.path.join(chalk.yellow('.'));
      console.error(chalk.bgRed.white.bold(` ERR #${index + 1} `) + ` ${pathStr}: ${err.message}`);
    });

    process.exit(1);
  }

  console.log(chalk.green('✅ Validación Exitosa. Integridad estructural confirmada.'));

  // -----------------------------------------------------------------------
  // FASE 2: COMPILACIÓN Y ESCRITURA (I/O)
  // -----------------------------------------------------------------------
  console.log(chalk.blue('\nCdE [FASE 2] Generando artefactos JSON...'));

  try {
    if (!fs.existsSync(TARGET_DIR)) {
      console.log(chalk.yellow(`   ⚠️ Creando directorio: ${TARGET_DIR}`));
      fs.mkdirSync(TARGET_DIR, { recursive: true });
    }

    // Estrategia de Espejo para V3.5
    const dictionaries: Record<string, AppLocale> = {
      en: enDictionary,
      es: enDictionary // Placeholder seguro
    };

    for (const locale of LOCALES) {
      const filename = `${locale}.json`;
      const filePath = path.join(TARGET_DIR, filename);
      const content = dictionaries[locale];

      const jsonString = JSON.stringify(content); // Minified
      const sizeKB = (Buffer.byteLength(jsonString) / 1024).toFixed(2);

      fs.writeFileSync(filePath, jsonString);
      console.log(chalk.green(`   ✨ Compilado: ${chalk.bold(filename)} `) + chalk.gray(`(${sizeKB} KB)`));
    }

  } catch (error: any) {
    console.error(chalk.bold.red('\n❌ FATAL: Fallo en sistema de archivos.'));
    console.error(chalk.red(`   ${error.message}`));
    process.exit(1);
  }

  const duration = (performance.now() - startTime).toFixed(2);
  console.log(chalk.bold.cyan(`\n🏁 Proceso completado en ${duration}ms`));
}

generate();
