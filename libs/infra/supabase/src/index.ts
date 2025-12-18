/**
 * =================================================================
 * APARATO: SUPABASE INFRASTRUCTURE ENTRYPOINT (V21.0)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (L4)
 * RESPONSABILIDAD: EXPOSICIÓN ATÓMICA DE MOTORES ESTRATÉGICOS
 * ESTADO: FIXED // ZERO ABBREVIATIONS
 * =================================================================
 */

// ✅ RESOLUCIÓN: Exportación de motores especializados
export * from './lib/census';

import { createClient, type SupabaseClient } from '@supabase/supabase-js';
import { type ArchivedJob } from '@prospector/api-contracts';

/**
 * Cliente Soberano de Supabase (L4).
 * Centraliza la comunicación con el Cuartel General de datos.
 */
export const supabase: SupabaseClient = createClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL || '',
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY || ''
);

/**
 * Motor de Archivo Estratégico.
 * Gestiona la persistencia de largo plazo para la tesis doctoral.
 */
export const strategicArchive = {
  /**
   * Recupera el histórico de rangos auditados migrados desde la capa táctica.
   *
   * @param limit - Cantidad de registros a recuperar.
   * @returns Array de trabajos archivados validados.
   */
  getHistory: async (limit: number = 20): Promise<ArchivedJob[]> => {
    const { data, error } = await supabase
      .from('archived_jobs')
      .select('*')
      .order('created_at', { ascending: false })
      .limit(limit);

    if (error) {
      throw new Error(`[L4_ARCHIVE] Connection Failure: ${error.message}`);
    }

    return data as ArchivedJob[];
  }
};
