// ARCHIVO: apps/web-dashboard/tools/scripts/generate-i18n.ts
import * as fs from 'fs';
import * as path from 'path';

// ✅ CAMBIO CRÍTICO: Importamos desde la estructura local de la app
// Ajusta la ruta relativa según donde ejecutes el script, pero idealmente:
import { enDictionary } from '../../lib/i18n-source/dictionaries/en';
import { AppLocaleSchema } from '../../lib/i18n-source/schema';

// Configuración
const TARGET_DIR = path.join(process.cwd(), 'apps/web-dashboard/messages');
const LOCALES = ['en', 'es'];

async function generate() {
  console.log('🌐 [I18N GENERATOR] Iniciando secuencia de compilación (LOCAL MODE)...');

  // 1. Validar la fuente de verdad
  const validation = AppLocaleSchema.safeParse(enDictionary);
  if (!validation.success) {
    console.error('❌ [I18N FATAL] El diccionario base viola el esquema Zod:');
    console.error(validation.error);
    process.exit(1);
  }

  // 2. Asegurar directorio
  if (!fs.existsSync(TARGET_DIR)) {
    fs.mkdirSync(TARGET_DIR, { recursive: true });
  }

  // 3. Generar JSONs
  for (const locale of LOCALES) {
    const filename = path.join(TARGET_DIR, `${locale}.json`);
    const content = JSON.stringify(enDictionary, null, 2);
    fs.writeFileSync(filename, content);
    console.log(`✅ [I18N] Generado: ${locale}.json`);
  }
}

generate();
