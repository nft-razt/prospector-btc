/**
 * =================================================================
 * APARATO: API CLIENT MASTER BARREL (V37.0)
 * RESPONSABILIDAD: UNIFICACIÓN DE SERVICIOS TÁCTICOS Y ESTRATÉGICOS
 * ESTADO: FULL SYNC // TS2305 RESOLVED
 * =================================================================
 */

// 1. Contratos de Dominio (Incluye Identity e IdentityStatus)
export * from "@prospector/api-contracts";

// 2. Adaptador Administrativo (adminApi)
export * from "./lib/admin";

// 3. Motores Tácticos y Hooks
export * from "./lib/client";
export * from "./lib/hooks";
export * from "./lib/hooks-rt";
export * from "./lib/lab";

// 4. Motores Estratégicos (Supabase)
export { strategicCensus, strategicArchive, supabase } from "@prospector/infra-supabase";

// 5. Esquemas de Validación
export {
  WorkerHeartbeatSchema,
  SearchStrategySchema,
  WorkOrderSchema,
  IdentitySchema,      // ✅ Exportación explícita
  IdentityStatusSchema // ✅ Exportación explícita
} from "./lib/schemas";
