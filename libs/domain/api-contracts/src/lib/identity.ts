/**
 * =================================================================
 * APARATO: IDENTITY STRATA CONTRACTS (V15.0 - ZERO-KNOWLEDGE)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE CREDENCIALES Y ESTADOS DE ACCESO
 * =================================================================
 */

import { z } from "zod";

/**
 * Estados operativos de una identidad en la Bóveda.
 */
export const IdentityStatusSchema = z.enum([
  "active",
  "ratelimited",
  "expired",
  "revoked",
]);

/** Tipo inferido del estado de identidad */
export type IdentityStatus = z.infer<typeof IdentityStatusSchema>;

/**
 * Esquema de payload cifrado (Sincronizado con VaultCryptoEngine L1).
 */
export const EncryptedIdentityPayloadSchema = z.object({
  cipher_text_base64: z.string(),
  initialization_vector_base64: z.string(),
  salt_base64: z.string(),
});

/** Tipo inferido del payload cifrado */
export type EncryptedIdentityPayload = z.infer<typeof EncryptedIdentityPayloadSchema>;

/**
 * Entidad de Identidad Soberana.
 */
export const IdentitySchema = z.object({
  id: z.string().uuid(),
  platform: z.string(),
  email: z.string().email(),
  usage_count: z.number().int().nonnegative(),
  last_used_at: z.string().datetime().nullable(),
  created_at: z.string().datetime(),
  status: IdentityStatusSchema,
});

/** Tipo inferido de identidad */
export type Identity = z.infer<typeof IdentitySchema>;

/**
 * Esquema de inyección (Payload de subida).
 */
export const IdentityPayloadSchema = z.object({
  platform: z.string(),
  email: z.string().email(),
  cookies: EncryptedIdentityPayloadSchema,
  userAgent: z.string(),
});

/** Tipo inferido del payload de inyección */
export type IdentityPayload = z.infer<typeof IdentityPayloadSchema>;
