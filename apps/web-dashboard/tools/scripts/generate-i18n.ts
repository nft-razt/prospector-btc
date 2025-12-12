import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';
import { z } from 'zod';

// IMPORTACIÓN DIRECTA DE LA SINGLE SOURCE OF TRUTH
// Nota: En tiempo de ejecución de scripts, usamos rutas relativas de archivo, no alias de TS
import { enDictionary } from '../../lib/i18n-source/dictionaries/en';
import { AppLocaleSchema, type AppLocale } from '../../lib/i18n-source/schema';

// CONFIGURACIÓN DE RUTAS RESILIENTE
// Detectamos si estamos corriendo desde la raíz del workspace o desde dentro de la app
const CWD = process.cwd();
const IS_ROOT = fs.existsSync(path.join(CWD, 'nx.json'));

const APP_ROOT = IS_ROOT
  ? path.join(CWD, 'apps/web-dashboard')
  : CWD;

const TARGET_DIR = path.join(APP_ROOT, 'messages');
const LOCALES = ['en', 'es'];

async function generate() {
  const startTime = performance.now();

  console.log(chalk.bold.cyan('\n🌐 [I18N COMPILER] Iniciando secuencia de generación...'));
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

    // Estrategia para Español:
    // En V3.5, simplemente clonamos el inglés. En V4.0 conectaremos API de traducción.
    // Esto evita que la app falle por falta de archivo 'es.json'.
    const dictionaries: Record<string, AppLocale> = {
      en: enDictionary,
      es: enDictionary // TODO: Implementar DeepL o traducción real
    };

    for (const locale of LOCALES) {
      const filename = `${locale}.json`;
      const filePath = path.join(TARGET_DIR, filename);
      const content = dictionaries[locale];

      // Minificamos el JSON para producción
      const jsonString = JSON.stringify(content);
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
