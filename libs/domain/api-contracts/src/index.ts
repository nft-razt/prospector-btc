/**
 * =================================================================
 * APARATO: API CONTRACTS MASTER BARREL (V45.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL DE LA FUENTE ÚNICA DE VERDAD
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa una política de exportación nominal estricta para eliminar
 * ambigüedades de tipos (TS2308). Centraliza los esquemas de validación
 * Zod y los tipos TypeScript inferidos para toda la Tríada Hydra.
 * =================================================================
 */

// 1. ESTRATO DE TELEMETRÍA Y ESQUEMAS BASE
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
} from "./lib/schema";

// 2. ESTRATO DE IDENTIDAD SOBERANA (Bóveda ZK)
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

// 3. ESTRATO DE ARCHIVO ESTRATÉGICO (Motor B)
export {
  ArchivedJobSchema,
  type ArchivedJob,
} from "./lib/archival";

// 4. ESTRATO DE LABORATORIO Y CENSO (QA)
export {
  CreateScenarioSchema,
  VerifyEntropySchema,
  type ScenarioStatus,
  type TestScenario,
  type VerifyEntropyPayload,
  type EntropyResult,
} from "./lib/lab";

export {
  WealthCategorySchema,
  CensusMetricsSchema,
  WealthClusterSchema,
  type WealthCategory,
  type CensusMetrics,
  type WealthCluster,
} from "./lib/census";

export {
  SwarmLaunchSchema,
  type SwarmLaunchConfig,
  type WorkflowRun,
} from "./lib/control";

export {
  FindingSchema,
  type Finding,
} from "./lib/finding";
