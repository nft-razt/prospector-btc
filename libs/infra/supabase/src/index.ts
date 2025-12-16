import { createClient } from '@supabase/supabase-js';

// NOTA: Estas variables deben estar en .env.local del dashboard
const SUPABASE_URL = process.env.NEXT_PUBLIC_SUPABASE_URL!;
const SUPABASE_ANON_KEY = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!;

if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
  // Solo lanzamos advertencia en build time, error en runtime
  if (typeof window !== 'undefined') {
    console.error("FATAL: Supabase configuration missing");
  }
}

/**
 * Cliente Singleton de Supabase.
 * Configurado para persistencia de sesión en el navegador.
 */
export const supabase = createClient(SUPABASE_URL, SUPABASE_ANON_KEY, {
  auth: {
    persistSession: true,
    autoRefreshToken: true,
  },
});

// Exportamos tipos si generamos los tipos automáticos de Supabase CLI
// export type Database = ...
