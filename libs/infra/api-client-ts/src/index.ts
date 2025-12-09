// libs/infra/api-client-ts/src/index.ts
// =================================================================
// APARATO: API CLIENT BARREL
// RESPONSABILIDAD: CONTRATO PÚBLICO DE LA LIBRERÍA
// =================================================================

// Exportamos los esquemas de Zod y tipos inferidos
export * from './lib/schemas';

// Exportamos los hooks de React Query (useSystemStatus, etc)
export * from './lib/hooks';

// Exportamos el cliente HTTP base y la API de administración
// ANTES: export { apiClient } from './lib/client';
// AHORA: Exportamos todo (*) para incluir 'adminApi' y 'IdentityPayload'
export * from './lib/client';
