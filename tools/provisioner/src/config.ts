/**
 * =================================================================
 * APARATO: MULTI-VECTOR CONFIGURATION (V11.0 - SOBERANO)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: VALIDACIÓN Y LÍMITES DE SEGURIDAD OPERATIVA
 * =================================================================
 */

import { z } from "zod";
import "dotenv/config";

const ConfigurationSchema = z.object({
  // ESTRATO DE RED
  ORCHESTRATOR_URL: z.string().url(),
  WORKER_AUTH_TOKEN: z.string().min(8),
  MASTER_VAULT_KEY: z.string().min(8),

  // ESTRATO DE ARTEFACTOS
  MINER_BINARY_URL: z.string().url(),

  // VECTOR ALFA: GOOGLE COLAB
  COLAB_URL: z.string().url().default("https://colab.research.google.com/"),
  GOOGLE_COOKIES_JSON: z.string().optional(),

  // VECTOR BETA: KAGGLE KERNELS
  KAGGLE_URL: z.string().url().default("https://www.kaggle.com/code/"),
  KAGGLE_COOKIES_JSON: z.string().optional(),
  KAGGLE_DISTRIBUTION_RATIO: z.coerce.number().min(0).max(1).default(0.3),

  // PARÁMETROS OPERATIVOS (NIVELADOS PARA TIER GRATUITO)
  WORKER_COUNT: z.coerce.number().int().min(1).default(1),
  FILTER_BASE_URL: z.string().url(),
  FILTER_SHARDS: z.coerce.number().int().positive().default(4),
  HEADLESS: z.coerce.boolean().default(true),

  // Timeout de 120s para compensar latencia de asignación de VM en 2025
  NAV_TIMEOUT: z.coerce.number().default(120000),
});

export const config = ConfigurationSchema.parse(process.env);
