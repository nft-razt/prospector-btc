/**
 * =================================================================
 * APARATO: API CLIENT MASTER BARREL (V67.0 - GOLD MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE FACADE (ESTRATO L4)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL SOBERANA DEL SISTEMA
 * =================================================================
 */

// --- 1. ESTRATO DE CONTRATOS DE DOMINIO (L2) ---
export {
  type NodeHardwareMetrics,
  type SystemMetrics,
  type WorkerSnapshot,
  type WorkerHeartbeat,
  type RealTimeEvent,
  type AuditReport,
  type WorkOrder,
  type SearchStrategy,
  type TargetStrata,
  type Finding,
  type ArchivedJob,
  type TestScenario,
  type ScenarioStatus,
  type CreateScenarioPayload,
  type VerifiedVectorAuditReport,
  type EntropyResult,
  type VerifyEntropyPayload,
  type WealthCluster,
  type WealthCategory,
  type SwarmLaunchConfig,
  type WorkflowRun,
  // ✅ RESOLUCIÓN TS2305: Exportación nominal de la entidad Identity
  type Identity,
  type IdentityPayload,
  type IdentityStatus,
  // Esquemas de Validación (Zod)
  RealTimeEventSchema,
  AuditReportSchema,
  WorkerSnapshotSchema,
  WorkerHeartbeatSchema,
  FindingSchema
} from "@prospector/api-contracts";

// --- 2. ADAPTADORES DE INFRAESTRUCTURA TÁCTICA (L4) ---
export { apiClient } from "./lib/client";
export { adminApi } from "./lib/admin";
export { labApi, type CertificationIgnitionResponse } from "./lib/lab";
export { controlApi } from "./lib/control";

// --- 3. MOTOR ESTRATÉGICO (SUPABASE / ENGINE B) ---
export {
  strategicArchive,
  strategicCensus,
  supabase
} from "@prospector/infra-supabase";

// --- 4. SINAPSIS NEURAL (HOOKS REACTIVOS L5) ---
export {
  useNeuralLink,
  useRealTimeTelemetry,
  type NeuralLinkInterface,
  type ArchivalDrift
} from "./lib/hooks-rt";
