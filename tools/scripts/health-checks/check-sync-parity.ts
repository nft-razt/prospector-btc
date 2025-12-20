/**
 * =================================================================
 * APARATO: TWIN-ENGINE PARITY AUDITOR (V10.6)
 * RESPONSABILIDAD: Comparar conteos de misiones entre Motores A y B
 * =================================================================
 */

import { createClient as createLibSqlClient } from '@libsql/client';
import { createClient as createSupabaseClient } from '@supabase/supabase-js';
import * as dotenv from 'dotenv';
import chalk from 'chalk';

dotenv.config();

async function check_parity() {
    console.log(chalk.bold.yellow("\n📊 PROSPECTOR // TWIN-ENGINE PARITY REPORT\n"));

    // 1. Handshake Motor A (Turso)
    const turso = createLibSqlClient({
        url: process.env.DATABASE_URL!,
        authToken: process.env.TURSO_AUTH_TOKEN!,
    });

    // 2. Handshake Motor B (Supabase)
    const supabase = createSupabaseClient(
        process.env.NEXT_PUBLIC_SUPABASE_URL!,
        process.env.SUPABASE_SERVICE_ROLE_KEY!
    );

    console.log(chalk.gray("🧮 Counting completed missions in Tactical Strata..."));
    const turso_res = await turso.execute("SELECT COUNT(*) as total FROM jobs WHERE status = 'completed'");
    const tactical_count = Number(turso_res.rows[0].total);

    console.log(chalk.gray("🧮 Counting archived missions in Strategic Strata..."));
    const { count: strategic_count, error } = await supabase
        .from('archived_audit_reports')
        .select('*', { count: 'exact', head: true });

    if (error) throw error;

    const lag = tactical_count - (strategic_count || 0);

    console.log("\n-------------------------------------------");
    console.log(`📡 TACTICAL (Turso):    ${chalk.cyan(tactical_count)} missions`);
    console.log(`🏛️ STRATEGIC (Supabase): ${chalk.magenta(strategic_count)} missions`);
    console.log("-------------------------------------------");

    if (lag > 0) {
        console.log(chalk.amber(`⚠️ SYNC LAG: ${lag} missions pending migration.`));
    } else {
        console.log(chalk.green("✅ SYSTEMS IN SYNC: All tactical data is archived."));
    }
    console.log("-------------------------------------------\n");
}

check_parity();
