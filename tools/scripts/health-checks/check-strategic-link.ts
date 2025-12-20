/**
 * =================================================================
 * APARATO: STRATEGIC LINK AUDITOR (SUPABASE V10.6)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (L6)
 * RESPONSABILIDAD: Certificación de conectividad y esquema de Motor B
 * =================================================================
 */

import { createClient } from '@supabase/supabase-js';
import * as dotenv from 'dotenv';
import chalk from 'chalk';

dotenv.config();

async function perform_strategic_link_audit() {
    console.log(chalk.bold.magenta("\n🕵️ PROSPECTOR // STRATEGIC LINK AUDIT SEQUENCE\n"));

    const supabase_url = process.env.NEXT_PUBLIC_SUPABASE_URL;
    const service_role_key = process.env.SUPABASE_SERVICE_ROLE_KEY;

    if (!supabase_url || !service_role_key) {
        console.error(chalk.red("❌ ERROR: Missing SUPABASE_URL or SERVICE_ROLE_KEY in .env"));
        process.exit(1);
    }

    // Inicialización con rol de Arquitecto (Service Role bypasses RLS)
    const strategic_client = createClient(supabase_url, service_role_key);

    const tables_to_verify = [
        'profiles',
        'workspaces',
        'archived_audit_reports',
        'strategic_findings'
    ];

    console.log(chalk.cyan("🛰️ Initiating handshake with Motor B (Supabase)..."));

    for (const table_name of tables_to_verify) {
        const { error } = await strategic_client
            .from(table_name)
            .select('count', { count: 'exact', head: true });

        if (error) {
            console.log(chalk.red(`  🔴 TABLE: ${table_name.padEnd(25)} [FAILURE] - ${error.message}`));
        } else {
            console.log(chalk.green(`  🟢 TABLE: ${table_name.padEnd(25)} [OPERATIONAL]`));
        }
    }

    console.log(chalk.bold.magenta("\n🏁 STRATEGIC LINK CERTIFIED // READY FOR CHRONOS BRIDGE\n"));
}

perform_strategic_link_audit().catch(err => {
    console.error(chalk.bgRed.white("🔥 FATAL ERROR DURING AUDIT:"), err);
    process.exit(1);
});
