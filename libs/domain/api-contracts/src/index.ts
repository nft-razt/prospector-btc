/**
 * =================================================================
 * APARATO: API CONTRACTS MASTER BARREL (V67.0 - TOTAL SYNC)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL SOBERANA DEL DOMINIO
 *
 * # Mathematical Proof:
 * Actúa como el mapa topológico de tipos. Garantiza que la paridad
 * de datos entre Rust (Backend) y TypeScript (Frontend) sea absoluta.
 * =================================================================
 */

// 1. ESTRATO DE TRABAJO Y ESTRATEGIAS
export * from "./lib/work";

// 2. ESTRATO DE TELEMETRÍA Y ESQUEMAS DE NODO (Resolución TS2305)
export {
  AuditReportSchema,
  SwarmHeatmapSegmentSchema,
  SystemMetricsSchema,
  WorkerSnapshotSchema,
  WorkerHeartbeatSchema,
  RealTimeEventSchema,
  type AuditReport,
  type SwarmHeatmapSegment,
  type SystemMetrics,
  type WorkerSnapshot,
  type WorkerHeartbeat,
  type RealTimeEvent,
  type NodeHardwareMetrics, // ✅ Inyectado nominalmente
} from "./lib/schema";

// 3. ESTRATO DE IDENTIDAD SOBERANA (Bóveda ZK)
export {
  IdentityStatusSchema,
  EncryptedIdentityPayloadSchema,
  IdentitySchema,
  IdentityPayloadSchema,
  type IdentityStatus,
  type EncryptedIdentityPayload,
  type Identity,
  type IdentityPayload,
} from "./lib/identity";

// 4. ESTRATO DE ARCHIVO ESTRATÉGICO Y CENSO
export { ArchivedJobSchema, type ArchivedJob } from "./lib/archival";
export {
  WealthCategorySchema,
  CensusMetricsSchema,
  WealthClusterSchema,
  type WealthCategory,
  type CensusMetrics,
  type WealthCluster,
} from "./lib/census";

// 5. ESTRATO DE LABORATORIO (Resolución TS2305 - Full Lab Visibility)
export {
  CreateScenarioSchema,
  VerifyEntropySchema,
  type ScenarioStatus,
  type TestScenario,
  type VerifyEntropyPayload,
  type EntropyResult,
  type CreateScenarioPayload,
  type VerifiedVectorAuditReport, // ✅ Sincronizado para Auditoría de Red
} from "./lib/lab";

// 6. COMANDO, CONTROL Y HALLAZGOS
export { SwarmLaunchSchema, type SwarmLaunchConfig, type WorkflowRun } from "./lib/control";
export { FindingSchema, type Finding } from "./lib/finding";
