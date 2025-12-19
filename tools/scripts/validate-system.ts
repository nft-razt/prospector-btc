/**
 * =================================================================
 * APARATO: SYSTEM INTEGRITY VALIDATOR
 * CLASIFICACIÓN: OPS TOOL (L6)
 * RESPONSABILIDAD: CERTIFICACIÓN E2E DE LA TRÍADA DE DESPLIEGUE
 * USO: pnpm validate:system
 * =================================================================
 */

import axios from "axios";
import chalk from "chalk";
import { createClient } from "@supabase/supabase-js";
import * as dotenv from "dotenv";

dotenv.config();

async function runIntegrityAudit() {
  console.log(chalk.bold.cyan("\n🔍 PROSPECTOR // INTEGRITY AUDIT SEQUENCE\n"));

  const results = {
    orchestrator: false,
    tactical_db: false,
    strategic_db: false,
    binary_host: false,
  };

  // 1. AUDITORÍA DE MANDO (ORCHESTRATOR)
  try {
    const url = process.env.NEXT_PUBLIC_API_URL?.replace("/api/v1", "/health");
    const res = await axios.get(url || "");
    if (res.data === "OK") {
      console.log(chalk.green("✅ L3: Orchestrator is Responsive (Render)"));
      results.orchestrator = true;
    }
  } catch (e) {
    console.log(
      chalk.red("❌ L3: Orchestrator Link Failed. Check RENDER_URL."),
    );
  }

  // 2. AUDITORÍA TÁCTICA (TURSO)
  try {
    const res = await axios.get(
      `${process.env.NEXT_PUBLIC_API_URL}/admin/status`,
      {
        headers: { Authorization: `Bearer ${process.env.WORKER_AUTH_TOKEN}` },
      },
    );
    if (res.status === 200) {
      console.log(
        chalk.green("✅ L3: Tactical Vault is Accessible (Turso/libSQL)"),
      );
      results.tactical_db = true;
    }
  } catch (e) {
    console.log(
      chalk.red("❌ L3: Tactical Auth Failed. Check WORKER_AUTH_TOKEN."),
    );
  }

  // 3. AUDITORÍA ESTRATÉGICA (SUPABASE)
  try {
    const supabase = createClient(
      process.env.NEXT_PUBLIC_SUPABASE_URL || "",
      process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY || "",
    );
    const { error } = await supabase
      .from("archived_jobs")
      .select("count", { count: "exact", head: true });
    if (!error) {
      console.log(chalk.green("✅ L4: Strategic Archive is Secure (Supabase)"));
      results.strategic_db = true;
    }
  } catch (e) {
    console.log(
      chalk.red("❌ L4: Strategic Link Failed. Check SUPABASE_KEYS."),
    );
  }

  // 4. AUDITORÍA DE PAYLOAD (GITHUB RELEASES)
  try {
    const res = await axios.head(process.env.MINER_BINARY_URL || "");
    if (res.status === 200) {
      console.log(
        chalk.green("✅ OPS: Miner Static Binary is Publicly Accessible"),
      );
      results.binary_host = true;
    }
  } catch (e) {
    console.log(
      chalk.red("❌ OPS: Miner Binary 404. Check GITHUB_RELEASES_URL."),
    );
  }

  console.log("\n" + chalk.bold.white("--- AUDIT SUMMARY ---"));
  Object.entries(results).forEach(([key, val]) => {
    console.log(`${val ? "🟢" : "🔴"} ${key.toUpperCase()}`);
  });

  if (Object.values(results).every((v) => v)) {
    console.log(
      chalk.bold.bgGreen.black(
        "\n 🚀 SYSTEM 100% OPERATIONAL. READY FOR DEPLOY. \n",
      ),
    );
    process.exit(0);
  } else {
    console.log(
      chalk.bold.bgRed.white(
        "\n ⚠️ CRITICAL FAULTS DETECTED. ABORT DEPLOY. \n",
      ),
    );
    process.exit(1);
  }
}

runIntegrityAudit();
