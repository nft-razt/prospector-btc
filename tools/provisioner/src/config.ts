// tools/provisioner/src/config.ts
import { z } from 'zod';
import 'dotenv/config';

const ConfigSchema = z.object({
  // TARGETS
  COLAB_URL: z.string().url().default('https://colab.research.google.com/'),

  // INFRAESTRUCTURA PROSPECTOR
  ORCHESTRATOR_URL: z.string().url(),
  MINER_BINARY_URL: z.string().url().describe('URL directa al binario compilado MUSL'),

  // CREDENCIALES
  WORKER_AUTH_TOKEN: z.string().min(10, "El token debe ser seguro"),

  // IDENTIDAD (Priority: ENV > FILE)
  GOOGLE_COOKIES_JSON: z.string().optional().describe('Cookies en formato JSON string o path'),

  // PARÁMETROS OPERATIVOS
  WORKER_COUNT: z.coerce.number().min(1).default(1),
  HEADLESS: z.coerce.boolean().default(true),
  DEBUG_MODE: z.coerce.boolean().default(false),

  // TIMEOUTS (Milisegundos)
  NAV_TIMEOUT: z.coerce.number().default(60000),
});

// Validación al inicio (Fail Fast)
export const config = ConfigSchema.parse(process.env);
