// libs/infra/api-client-ts/src/index.ts
// =================================================================
// APARATO: API CLIENT ENTRY POINT
// RESPONSABILIDAD: UNIFICACIÓN DE TIPOS Y LÓGICA DE CLIENTE
// ESTADO: SANITIZED (NO AMBIGUOUS EXPORTS)
// =================================================================

// 1. Re-exportación de Contratos (Dominio Puro)
// Esta es la autoridad para tipos como WorkOrder, SearchStrategy, etc.
export * from '@prospector/api-contracts';

// 2. Exportación de Hooks de React Query (Estado)
export * from './lib/hooks';

// 3. Exportación del Cliente HTTP y Facades (Transporte)
export * from './lib/client';

// 4. Exportación de Esquemas Zod (Validación Runtime)
// ⚠️ CORRECCIÓN: No exportamos '*' para evitar colisión de nombres.
// Los esquemas Zod (ej: WorkOrderSchema) sí pueden ser útiles, pero los tipos no.
// Si el consumidor necesita validar, importará el Schema.
export {
    WorkerHeartbeatSchema,
    SearchStrategySchema,
    WorkOrderSchema
} from './lib/schemas';
