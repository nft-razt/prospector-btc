/**
 * =================================================================
 * APARATO: CENSUS DATA CONTRACTS (V20.0)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (L2)
 * RESPONSABILIDAD: DEFINICIÓN DE MÉTRICAS DE ARQUEOLOGÍA Y RIQUEZA
 * ESTADO: PRODUCTION READY // SSoT
 * =================================================================
 */

import { z } from "zod";

/**
 * Clasificación forense de las carteras según su comportamiento histórico.
 */
export const WealthCategorySchema = z.enum([
  "Satoshi_Era",
  "Lost_Coins",
  "Exchanges",
  "Whales",
  "Retail",
]);

export type WealthCategory = z.infer<typeof WealthCategorySchema>;

/**
 * Representa un resumen estadístico global del estado del Ledger.
 */
export const CensusMetricsSchema = z.object({
  total_indexed_addresses: z.number().int().nonnegative(),
  zombie_btc_estimate: z.number().nonnegative(),
  last_block_synced: z.number().int(),
  high_entropy_risk_count: z.number().int(),
  updated_at: z.string().datetime(),
});

export type CensusMetrics = z.infer<typeof CensusMetricsSchema>;

/**
 * Representa un cluster de riqueza para visualización de dispersión.
 */
export const WealthClusterSchema = z.object({
  cluster_identifier: z.string().uuid(),
  display_label: z.string(),
  last_activity_year: z.number().int().min(2009).max(2025),
  wallet_count: z.number().int().nonnegative(),
  balance_bitcoin: z.number().nonnegative(),
  wealth_category: WealthCategorySchema,
  is_zombie_target: z.boolean(),
  average_wallet_age_days: z.number().optional(),
});

export type WealthCluster = z.infer<typeof WealthClusterSchema>;
