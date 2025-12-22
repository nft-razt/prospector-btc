/**
 * =================================================================
 * APARATO: I18N COMPILER ENGINE (V3.6 - DETERMINISTIC SYNCHRONIZER)
 * CLASIFICACIÓN: INFRASTRUCTURE TOOL (ESTRATO L6)
 * RESPONSABILIDAD: VALIDACIÓN Y CRISTALIZACIÓN DE DICCIONARIOS TS A JSON
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el proceso de transformación de la Fuente Única de Verdad
 * (TypeScript Content) hacia los artefactos estáticos (JSON) consumidos
 * por el runtime de next-intl. Utiliza el esquema Zod 'AppLocaleSchema'
 * como guardián de integridad para asegurar que no existan llaves
 * faltantes entre idiomas, garantizando CERO REGRESIONES en la interfaz.
 * =================================================================
 */

import * as fs from "fs";
import * as path from "path";
import chalk from "chalk";
import { ZodIssue } from "zod";
import { AppLocaleSchema, type AppLocale } from "../../lib/i18n/schema";
import { localizationMap } from "../../lib/i18n/registry";

/** Configuración de rutas y locales soportados */
const CURRENT_WORKING_DIRECTORY = process.cwd();
const APP_ROOT_DIRECTORY = path.join(CURRENT_WORKING_DIRECTORY, "apps/web-dashboard");
const TARGET_OUTPUT_DIRECTORY = path.join(APP_ROOT_DIRECTORY, "messages");
const SUPPORTED_LOCALES = ["en", "es"];

/**
 * Ejecuta la secuencia de auditoría y cristalización de contenidos.
 * @returns {Promise<void>}
 */
async function execute_localization_compilation(): Promise<void> {
  const start_performance_timestamp = performance.now();

  console.log(
    chalk.bold.magenta("\n🕵️ [I18N_COMPILER]: Initiating sovereign data synchronization...\n")
  );

  for (const locale_identifier of SUPPORTED_LOCALES) {
    console.log(chalk.cyan(`   🛰️  Auditing STRATUM: [${locale_identifier.toUpperCase()}]`));

    const content_registry = localizationMap[locale_identifier];

    if (!content_registry) {
      console.error(chalk.bgRed.white(`\n ❌ FATAL_ERROR: Locale '${locale_identifier}' not found in Registry. \n`));
      process.exit(1);
    }

    // 1. AUDITORÍA DE INTEGRIDAD (ZOD SHIELD)
    const validation_result = AppLocaleSchema.safeParse(content_registry);

    if (!validation_result.success) {
      console.error(
        chalk.bgRed.white(`\n ❌ CONTRACT_MISMATCH in [${locale_identifier.toUpperCase()}] \n`)
      );

      validation_result.error.issues.forEach((issue: ZodIssue) => {
        const error_path = issue.path.join(" -> ");
        console.error(chalk.red(`      [PATH]: ${error_path}`));
        console.error(chalk.yellow(`      [ERROR]: ${issue.message}\n`));
      });

      console.error(chalk.red("   🛑 COMPILATION_ABORTED: Integrity validation failed."));
      process.exit(1);
    }

    // 2. CRISTALIZACIÓN DE ARTEFACTO (JSON CLEAN)
    if (!fs.existsSync(TARGET_OUTPUT_DIRECTORY)) {
      fs.mkdirSync(TARGET_OUTPUT_DIRECTORY, { recursive: true });
    }

    const output_file_path = path.join(TARGET_OUTPUT_DIRECTORY, `${locale_identifier}.json`);

    /**
     * El contenido se guarda sin espacios innecesarios y sin comentarios
     * para optimizar el tiempo de carga en el navegador del operador.
     */
    const serialized_content = JSON.stringify(validation_result.data);

    try {
      fs.writeFileSync(output_file_path, serialized_content, "utf8");
      const file_size_kilobytes = (serialized_content.length / 1024).toFixed(2);
      console.log(chalk.green(`      ✅ CRYSTALLIZED: ${output_file_path} (${file_size_kilobytes} KB)`));
    } catch (write_error: any) {
      console.error(chalk.red(`      ❌ IO_ERROR: Failed to write ${locale_identifier}.json`), write_error.message);
      process.exit(1);
    }
  }

  const duration_milliseconds = (performance.now() - start_performance_timestamp).toFixed(2);
  console.log(
    chalk.bold.magenta(`\n🏁 [COMPILATION_COMPLETE]: All strata synchronized in ${duration_milliseconds}ms.\n`)
  );
}

// Ignición del proceso
execute_localization_compilation().catch((fatal_error) => {
  console.error(chalk.bgRed.white("🔥 [CRITICAL_KERNEL_FAULT]:"), fatal_error);
  process.exit(1);
});
