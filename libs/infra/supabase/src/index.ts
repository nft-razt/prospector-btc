/**
 * =================================================================
 * APARATO: SUPABASE STRATEGIC CLIENT (V25.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: GESTIÓN DE ENLACES CON EL MOTOR ESTRATÉGICO
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el túnel de comunicación hacia Motor B (Supabase).
 * Expone las interfaces para la recuperación del censo histórico y
 * el archivo inmutable de misiones, garantizando la persistencia
 * de la Tesis Doctoral.
 * =================================================================
 */

import { createBrowserClient } from '@supabase/ssr';
import { type SupabaseClient } from '@supabase/supabase-js';
import { strategicCensus } from "./lib/census";

/**
 * Cliente de Motor B: Repositorio Estratégico de la Tesis.
 * Configurado para operación en entorno Next.js (SSR/Client).
 */
export const supabase: SupabaseClient = createBrowserClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL || "",
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY || ""
);

/**
 * Adaptador de lectura para el Censo UTXO y Arqueología.
 * ✅ RESOLUCIÓN ERROR 2305: Exportación nominal para el API Master Barrel.
 */
export { strategicCensus };

/**
 * Adaptador de archivo histórico de misiones certificadas.
 */
export const strategicArchive = {
  /**
   * Recupera el histórico de auditoría desde el Motor Estratégico.
   * @param limit_records - Cantidad de registros a extraer.
   */
  getHistory: async (limit_records: number = 20) => {
    const { data, error } = await supabase
      .from("archived_audit_reports")
      .select("*")
      .order("created_at", { ascending: false })
      .limit(limit_records);

    if (error) {
      throw new Error(`STRATEGIC_UPLINK_FAULT: ${error.message}`);
    }

    return data;
  },

  /**
   * Recupera métricas agregadas de esfuerzo computacional global.
   */
  getGlobalMetrics: async () => {
    const { data, error } = await supabase
      .from("census_summary")
      .select("*")
      .single();

    if (error) {
      throw new Error(`METRICS_UNREACHABLE: ${error.message}`);
    }

    return data;
  }
};
