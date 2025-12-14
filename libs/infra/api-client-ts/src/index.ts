// libs/infra/api-client-ts/src/index.ts
// =================================================================
// APARATO: API CLIENT ENTRY POINT
// RESPONSABILIDAD: UNIFICACIÓN DE TIPOS Y LÓGICA DE CLIENTE
// =================================================================

// 1. Re-exportación de Contratos (Dominio Puro)
// Esto permite que el Dashboard haga: import { WorkOrder } from '@prospector/api-client';
export * from '@prospector/api-contracts';

// 2. Exportación de Hooks de React Query (Estado)
export * from './lib/hooks';

// 3. Exportación del Cliente HTTP y Facades (Transporte)
export * from './lib/client';
