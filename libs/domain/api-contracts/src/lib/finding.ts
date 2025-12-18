/**
 * =================================================================
 * APARATO: FINDING DATA CONTRACT
 * CLASIFICACIÓN: DOMAIN CONTRACTS (L2)
 * RESPONSABILIDAD: DEFINICIÓN DE COLISIÓN CRIPTOGRÁFICA
 * ESTADO: ELITE TYPING (ZK-COMPLIANT)
 * =================================================================
 */

import { z } from "zod";

/**
 * Esquema de validación para una colisión confirmada.
 * Representa el material criptográfico recuperado del set UTXO.
 */
export const FindingSchema = z.object({
  /** Dirección Bitcoin (P2PKH) colisionada */
  address: z.string().min(26).max(35),

  /** Clave privada en formato WIF (Wallet Import Format) */
  private_key_wif: z.string(),

  /** Metadatos de la fuente de entropía (ej: 'brainwallet:pass123') */
  source_entropy: z.string(),

  /** Clasificación de la billetera: legacy_uncompressed, legacy_compressed */
  wallet_type: z.string(),

  /** Identificador único del nodo que realizó el hallazgo */
  found_by_worker: z.string(),

  /** Identificador de la orden de trabajo (rango) origen */
  job_id: z.string().uuid().nullable().optional(),

  /** Marca de tiempo de la detección (ISO 8601 UTC) */
  detected_at: z.string().datetime(),
});

/**
 * Tipo inferido para uso en el enjambre y el orquestador.
 */
export type Finding = z.infer<typeof FindingSchema>;
