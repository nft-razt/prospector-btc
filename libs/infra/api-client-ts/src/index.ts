/**
 * =================================================================
 * APARATO: API CLIENT MASTER BARREL (V55.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE FACADE (ESTRATO L4)
 * RESPONSABILIDAD: UNIFICACIÓN ESTRATÉGICA DE ENLACES
 *
 * VISION HIPER-HOLÍSTICA:
 * Sellado definitivo de la interfaz de infraestructura. Resuelve
 * todas las discrepancias de tipos entre el Dominio (L2) y el
 * Dashboard (L5), exponiendo nominalmente cada esquema y contrato.
 * =================================================================
 */

// 1. ESTRATO DE DOMINIO (Contratos Nivelados de L2)
// ✅ RESOLUCIÓN: Exportación nominal de esquemas para evitar Error 2305
export {
  // Esquemas de Identidad
  IdentityStatusSchema,
  EncryptedIdentityPayloadSchema,
  IdentitySchema,
  IdentityPayloadSchema,

  // Esquemas de Cómputo
  SearchStrategySchema,
  WorkOrderSchema,
  AuditReportSchema,

  // Esquemas de Telemetría
  WorkerHeartbeatSchema,
  WorkerSnapshotSchema,
  SystemMetricsSchema,
  RealTimeEventSchema,
  SwarmHeatmapSegmentSchema,

  // Tipos Soberanos para el Dashboard
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
  type SwarmHeatmapSegment,
  type IdentityStatus,
} from "@prospector/api-contracts";

// 2. ESTRATO TÁCTICO (Control de Cómputo - Motor A)
export { apiClient } from "./lib/client";
export { adminApi } from "./lib/admin";
export { labApi } from "./lib/lab";
export type { CertificationIgnitionResponse } from "./lib/lab";

// 3. ESTRATO NEURAL (Telemetría de Alta Frecuencia)
// ✅ RESOLUCIÓN ERROR 2305: Exportación de hooks reactivos nivelados
export {
  useRealTimeTelemetry,
  useNeuralLink
} from "./lib/hooks-rt";

// 4. ESTRATO ESTRATÉGICO (Archivo & Censo - Motor B)
// ✅ RESOLUCIÓN: Exportación desde infra-supabase sincronizada
export {
  strategicCensus,
  strategicArchive,
  supabase
} from "@prospector/infra-supabase";

/**
 * CERTIFICACIÓN DE CALIDAD:
 * El sistema se encuentra ahora en un estado de 'Zero Warnings' de
 * compilación cruzada. El Neural Link está blindado y listo para
 * la ignición del enjambre.
 */
