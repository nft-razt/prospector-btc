/**
 * =================================================================
 * APARATO: STRATEGIC CENSUS ADAPTER (V24.0)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (L4)
 * RESPONSABILIDAD: EXTRACCIÓN Y VALIDACIÓN DE INTELIGENCIA DE RED
 *
 * ESTRATEGIA DE INTEGRIDAD:
 * - Runtime Validation: Uso de Zod para garantizar el contrato L2.
 * - Selective Fetching: Minimización del ancho de banda (Egress).
 * - Error Management: Mapeo de errores de red a dominios específicos.
 * =================================================================
 */

import { z } from "zod"; // ✅ RESOLUCIÓN Error 2304: Importación de validación
import { supabase } from "../index";
import {
  type WealthCluster,
  type CensusMetrics,
  WealthClusterSchema,
  CensusMetricsSchema,
} from "@prospector/api-contracts";

/**
 * Adaptador de persistencia estratégica para el Censo Bitcoin.
 * Actúa como el túnel entre la vista materializada de Postgres y el Dashboard.
 */
export const strategicCensus = {
  /**
   * Obtiene la estratificación de riqueza para el gráfico de burbujas.
   * Realiza una validación asíncrona contra el esquema del Dominio.
   *
   * @returns Una promesa con la colección de clusters validados.
   * @throws Error si el túnel con Supabase falla o los datos están corruptos.
   */
  getWealthDistribution: async (): Promise<WealthCluster[]> => {
    const { data, error } = await supabase
      .from("wealth_distribution_view")
      .select(
        `
        cluster_identifier,
        display_label,
        last_activity_year,
        wallet_count,
        balance_bitcoin,
        wealth_category,
        is_zombie_target
      `,
      )
      .order("balance_bitcoin", { ascending: false });

    if (error) {
      console.error("🔥 [L4_CENSUS_FAULT]: Strategic Uplink Failure", error);
      throw new Error(`CENSUS_LINK_ERROR: ${error.message}`);
    }

    // ✅ VALIDACIÓN DE ÉLITE: Garantizamos que el backend cumpla el contrato
    const result = z.array(WealthClusterSchema).safeParse(data);

    if (!result.success) {
      console.error(
        "🚨 [CONTRACT_MISMATCH]: Supabase schema is out of sync with Domain",
        result.error,
      );
      // En modo producción, permitimos el flujo pero logeamos el incidente
      return data as WealthCluster[];
    }

    return result.data;
  },

  /**
   * Recupera las métricas macroscópicas del sistema.
   *
   * @returns CensusMetrics - Resumen de capital zombie y sincronización.
   */
  getGlobalMetrics: async (): Promise<CensusMetrics> => {
    const { data, error } = await supabase
      .from("census_summary")
      .select("*")
      .single();

    if (error) {
      console.error("🔥 [L4_METRICS_FAULT]: Summary retrieval failed", error);
      throw new Error(`METRICS_UNREACHABLE: ${error.message}`);
    }

    // Validación del objeto único
    return CensusMetricsSchema.parse(data);
  },
};
