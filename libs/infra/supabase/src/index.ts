/**
 * =================================================================
 * APARATO: SUPABASE INFRASTRUCTURE (V21.1)
 * CLASIFICACIÓN: STRATEGIC PERSISTENCE (L4)
 * RESPONSABILIDAD: ENLACE AL CUARTEL GENERAL (ENGINE B)
 * =================================================================
 */

import { createClient, type SupabaseClient } from "@supabase/supabase-js";
import { type ArchivedJob } from "@prospector/api-contracts";

// Cliente de bajo nivel para operaciones CRUD directas
export const supabase: SupabaseClient = createClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL || "",
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY || "",
);

// Adaptador para el Histórico de la Tesis
export const strategicArchive = {
  getHistory: async (limit: number = 20): Promise<ArchivedJob[]> => {
    const { data, error } = await supabase
      .from("archived_jobs")
      .select("*")
      .order("created_at", { ascending: false })
      .limit(limit);

    if (error) throw new Error(`STRATEGIC_ARCHIVE_FAULT: ${error.message}`);
    return data as ArchivedJob[];
  },
};

// Adaptador para el Censo Visual (Rich List)
export { strategicCensus } from "./lib/census";
