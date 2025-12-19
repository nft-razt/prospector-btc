/**
 * =================================================================
 * APARATO: API CLIENT MASTER BARREL (V46.1 - ALIGNED & SECURED)
 * CLASIFICACIÓN: INFRASTRUCTURE FACADE (L4)
 * RESPONSABILIDAD: UNIFICACIÓN ESTRATÉGICA DE ENLACES (Vercel Ready)
 *
 * ESTRATEGIA DE ÉLITE:
 * - Nominal Strict Exporting: Resuelve todas las colisiones de tipos.
 * - Stratum Bridging: Punto de entrada único para el Dashboard Next.js.
 * =================================================================
 */

// 1. ESTRATO DE DOMINIO (Contratos Nivelados V45.1)
export {
  // Esquemas de validación Zod
  IdentityStatusSchema,
  EncryptedIdentityPayloadSchema,
  IdentitySchema,
  IdentityPayloadSchema,
  SearchStrategySchema,
  WorkOrderSchema,
  AuditReportSchema,
  WorkerHeartbeatSchema,
  WorkerSnapshotSchema,
  SystemMetricsSchema,
  RealTimeEventSchema,

  // ✅ TIPOS SOBERANOS (Resuelven Errores TS2305/TS2724)
  type Identity,
  type IdentityPayload,
  type EncryptedIdentityPayload,
  type SearchStrategy,
  type WorkOrder,
  type AuditReport,
  type WorkerHeartbeat,
  type WorkerSnapshot,
  type SystemMetrics,
  type RealTimeEvent,
  type IdentityStatus,
} from "@prospector/api-contracts";

// 2. ESTRATO TÁCTICO (Control de Enjambre - Turso)
export { adminApi } from "./lib/admin";
export { apiClient } from "./lib/client";
export { useSystemTelemetry } from "./lib/hooks";
export { useRealTimeTelemetry } from "./lib/hooks-rt";
export { labApi } from "./lib/lab";

// 3. ESTRATO ESTRATÉGICO (Archivo & Censo - Supabase)
import {
  strategicCensus,
  strategicArchive,
  supabase,
} from "@prospector/infra-supabase";

export { strategicCensus, strategicArchive, supabase };

/**
 * Nota de Calidad:
 * Todas las exportaciones son nominales para maximizar la eficiencia del
 * Tree Shaking durante el build de producción en Vercel.
 */
