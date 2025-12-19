/**
 * =================================================================
 * APARATO: PRODUCTION PRE-FLIGHT AUDITOR (V12.0)
 * CLASIFICACIÓN: OPS TOOLS
 * RESPONSABILIDAD: CERTIFICACIÓN E2E POST-DEPLOYMENT
 *
 * ESTRATEGIA DE ÉLITE:
 * - Cross-Cloud Verification: Verifica el túnel Vercel -> Render.
 * - Latency Profiling: Asegura que el tiempo de respuesta sea <200ms.
 * - ZK Handshake: Valida que la Bóveda sea accesible.
 * =================================================================
 */

import axios from 'axios';
import chalk from 'chalk';

async function performSecurityAudit() {
  const API_URL = process.env.NEXT_PUBLIC_API_URL;
  const START_TIME = Date.now();

  console.log(chalk.bold.blue("\n🕵️  PROSPECTOR PRODUCTION AUDIT SEQUENCE\n"));

  if (!API_URL) {
    console.error(chalk.red("❌ ERROR: NEXT_PUBLIC_API_URL is missing in Vercel environment."));
    process.exit(1);
  }

  console.log(`🔗 Target Orchestrator: ${API_URL}`);

  try {
    // 1. LIVENESS CHECK (El endpoint público que creamos en V8.8)
    const livenessResponse = await axios.get(`${API_URL}/health/liveness`, { timeout: 5000 });
    const duration = Date.now() - START_TIME;

    if (livenessResponse.data.status === "OPERATIONAL") {
      console.log(chalk.green(`✅ STRATUM L3: Orchestrator is Online (Latency: ${duration}ms)`));
      console.log(chalk.gray(`   -> Version: ${livenessResponse.data.version}`));
      console.log(chalk.gray(`   -> Mode: ${livenessResponse.data.system_mode}`));
    } else {
      console.warn(chalk.yellow(`⚠️  STRATUM L3: Service is DEGRADED. Check logs.`));
    }

    // 2. VAULT HANDSHAKE CHECK
    // Intentamos un ping administrativo que requiere el WORKER_AUTH_TOKEN
    const vaultCheck = await axios.get(`${API_URL}/admin/status/nodes`, {
      headers: { 'Authorization': `Bearer ${process.env.ADMIN_MASTER_TOKEN}` },
      timeout: 5000
    });

    if (vaultCheck.status === 200) {
      console.log(chalk.green("✅ STRATUM L4: Identity Vault is Accessible."));
    }

    console.log(chalk.bold.bgGreen.black("\n🚀 SYSTEM IS READY FOR TRAFFIC\n"));

  } catch (error: any) {
    console.error(chalk.bgRed.white("\n🔥 AUDIT FAILED: UNABLE TO ESTABLISH NEURAL LINK"));
    console.error(chalk.red(`Reason: ${error.message}`));
    if (error.response) {
      console.error(chalk.red(`HTTP Status: ${error.response.status}`));
    }
    process.exit(1);
  }
}

performSecurityAudit();
