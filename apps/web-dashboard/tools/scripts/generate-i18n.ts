// apps/web-dashboard/tools/scripts/generate-i18n.ts
import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';
import { z } from 'zod'; // Importamos z para usar sus tipos

// ✅ IMPORTACIÓN DE LA FUENTE DE VERDAD
import { enDictionary } from '../../lib/i18n-source/dictionaries/en';
import { AppLocaleSchema } from '../../lib/i18n-source/schema';

// CONFIGURACIÓN DE RUTAS (Context Aware)
const APP_ROOT = path.join(process.cwd(), 'apps/web-dashboard');
const TARGET_DIR = path.join(APP_ROOT, 'messages');
const LOCALES = ['en', 'es']; // Idiomas soportados

async function generate() {
  const startTime = performance.now();

  console.log(chalk.bold.cyan('\n🌐 [I18N COMPILER] Inicializando secuencia de generación...'));
  console.log(chalk.gray(`   📂 Contexto Raíz:   ${process.cwd()}`));
  console.log(chalk.gray(`   🎯 Directorio Destino: ${TARGET_DIR}`));

  // -----------------------------------------------------------------------
  // FASE 1: VALIDACIÓN DE INTEGRIDAD (ZOD)
  // -----------------------------------------------------------------------
  console.log(chalk.blue('\n🔍 [FASE 1] Validando Estructura del Diccionario Maestro...'));

  const validation = AppLocaleSchema.safeParse(enDictionary);

  if (!validation.success) {
    console.error(chalk.bold.red('❌ FATAL: Violación de Contrato en Diccionario Base'));
    console.error(chalk.red('   El objeto TypeScript no cumple con el Schema Zod definido.\n'));

    // Reporte Granular de Errores
    // CORRECCIÓN MAESTRA: Usamos '.issues' en lugar de '.errors'
    validation.error.issues.forEach((err: z.ZodIssue, index: number) => {
      const pathString = err.path.join(chalk.yellow('.'));
      console.error(chalk.bgRed.white.bold(` ERROR #${index + 1} `) + ` en clave: ${pathString}`);
      console.error(chalk.yellow(`   Expectativa: ${err.message}`));
      console.error(chalk.gray(`   Código Zod:  ${err.code}`));
      console.log(''); // Separador
    });

    process.exit(1); // Romper el build inmediatamente
  }

  console.log(chalk.green('✅ Validación Exitosa. El diccionario es matemáticamente correcto.'));

  // -----------------------------------------------------------------------
  // FASE 2: COMPILACIÓN Y ESCRITURA (I/O)
  // -----------------------------------------------------------------------
  console.log(chalk.blue('\nCdE [FASE 2] Generando artefactos JSON...'));

  try {
    // Asegurar existencia del directorio
    if (!fs.existsSync(TARGET_DIR)) {
      console.log(chalk.yellow(`   ⚠️ Directorio no existe. Creando: ${TARGET_DIR}`));
      fs.mkdirSync(TARGET_DIR, { recursive: true });
    }

    for (const locale of LOCALES) {
      const filename = `${locale}.json`;
      const filePath = path.join(TARGET_DIR, filename);

      // Clonamos la estructura base validada.
      const content = JSON.stringify(enDictionary, null, 2);

      const sizeKB = (Buffer.byteLength(content) / 1024).toFixed(2);

      fs.writeFileSync(filePath, content);
      console.log(chalk.green(`   ✨ Compilado: ${chalk.bold(filename)} `) + chalk.gray(`(${sizeKB} KB)`));
    }

  } catch (error: any) {
    console.error(chalk.bold.red('\n❌ FATAL: Error de Sistema de Archivos (I/O)'));
    console.error(chalk.red(`   Mensaje: ${error.message}`));

    if (error.code === 'EACCES') {
      console.error(chalk.yellow('   SUGERENCIA: Verifica los permisos de escritura en la carpeta apps/web-dashboard.'));
    }

    process.exit(1);
  }

  // -----------------------------------------------------------------------
  // RESUMEN
  // -----------------------------------------------------------------------
  const duration = (performance.now() - startTime).toFixed(2);
  console.log(chalk.bold.cyan(`\n🏁 Proceso completado en ${duration}ms`));
  console.log(chalk.gray('---------------------------------------------------\n'));
}

// Ejecución
generate();
