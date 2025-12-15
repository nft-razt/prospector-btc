// libs/infra/api-client-ts/src/lib/schemas.ts
/**
 * =================================================================
 * APARATO: SCHEMA PROXY (INFRASTRUCTURE LAYER)
 * RESPONSABILIDAD: RE-EXPORTACIÓN TRANSPARENTE DEL DOMINIO
 * PATRÓN: BARREL / PROXY
 * =================================================================
 */

// ⚠️ NO REDEFINIR SCHEMAS AQUÍ. VIOLA EL PRINCIPIO DE "SINGLE SOURCE OF TRUTH".
// Todos los esquemas Zod viven en @prospector/api-contracts.

export * from "@prospector/api-contracts";
