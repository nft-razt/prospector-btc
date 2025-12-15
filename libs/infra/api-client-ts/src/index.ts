// libs/infra/api-client-ts/src/index.ts

export * from "@prospector/api-contracts";
export * from "./lib/hooks";
export * from "./lib/hooks-rt"; // ✅ Exportar Hook Real-Time
export * from "./lib/client";
export {
  WorkerHeartbeatSchema,
  SearchStrategySchema,
  WorkOrderSchema,
} from "./lib/schemas";
