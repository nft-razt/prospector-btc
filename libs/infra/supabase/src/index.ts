/**
 * =================================================================
 * APARATO: SUPABASE STRATEGIC CLIENT (V10.6 - SSR READY)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (L4)
 * RESPONSABILIDAD: GESTIÓN DE INSTANCIAS SOBERANAS DE BASE DE DATOS
 * =================================================================
 */

import { createBrowserClient } from '@supabase/ssr';
import { type SupabaseClient } from '@supabase/supabase-js';
import { type ArchivedJob } from "@prospector/api-contracts";

/**
 * Cliente soberano de Supabase para el entorno del navegador.
 * Implementa el patrón Singleton para evitar duplicidad de conexiones.
 */
export const supabase: SupabaseClient = createBrowserClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL || "",
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY || ""
);

/**
 * Adaptador de archivo histórico de misiones certificadas.
 */
export const strategicArchive = {
  /**
   * Recupera el histórico de auditoría desde el Motor Estratégico.
   * @param limit_records Cantidad de registros a extraer.
   */
  getHistory: async (limit_records: number = 20): Promise<ArchivedJob[]> => {
    const { data, error } = await supabase
      .from("archived_audit_reports")
      .select("*")
      .order("created_at", { ascending: false })
      .limit(limit_records);

    if (error) {
      throw new Error(`STRATEGIC_UPLINK_FAULT: ${error.message}`);
    }

    return data as ArchivedJob[];
  },
};

export { strategicCensus } from "./lib/census";
