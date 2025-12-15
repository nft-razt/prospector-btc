// tools/scripts/validate-system.ts
/**
 * =================================================================
 * APARATO: SYSTEM INTEGRITY VERIFIER (E2E SMOKE TEST - EXTENDED)
 * RESPONSABILIDAD: CERTIFICACIÓN DE CICLO COMPLETO (SWARM + LAB)
 * USO: npx ts-node --project tools/scripts/tsconfig.json tools/scripts/validate-system.ts
 * =================================================================
 */

import axios from "axios";
import chalk from "chalk";
import EventSource from "eventsource";
import { v4 as uuidv4 } from "uuid";

// Configuración
const API_URL = process.env.API_URL || "http://localhost:3000/api/v1";
const AUTH_TOKEN = process.env.WORKER_AUTH_TOKEN || "dev_secret";
const WORKER_ID = uuidv4();

const log = (msg: string) => console.log(chalk.blue(`[TEST] ${msg}`));
const success = (msg: string) => console.log(chalk.green(`✅ ${msg}`));
const fail = (msg: string) => {
  console.error(chalk.red(`❌ ${msg}`));
  process.exit(1);
};

// Interfaces para tipado estricto
interface SsePayload {
  event: string;
  data: any;
}

interface SystemMetrics {
  active_nodes: number;
  global_hashrate: number;
}

interface VerifyResponse {
  address: string;
  is_target: boolean;
  matched_scenario: string | null;
}

async function main() {
  console.log(
    chalk.bold.white("\n🔬 PROSPECTOR SYSTEM CERTIFICATION PROTOCOL v2.0\n"),
  );

  // --- FASE 1: INFRAESTRUCTURA BÁSICA ---
  log("1. Verificando Signos Vitales (Liveness)...");
  try {
    await axios.get("http://localhost:3000/health");
    success("Orchestrator: ONLINE");
  } catch (e: any) {
    fail(`Orchestrator Offline: ${e.message}`);
  }

  // --- FASE 2: SWARM SIMULATION ---
  log("2. Simulando Actividad de Enjambre...");
  try {
    await axios.post(
      `${API_URL}/swarm/heartbeat`,
      {
        worker_id: WORKER_ID,
        hostname: "cert-runner-01",
        hashrate: 10000000,
        timestamp: new Date().toISOString(),
      },
      { headers: { Authorization: `Bearer ${AUTH_TOKEN}` } },
    );
    success("Heartbeat: ACEPTADO");
  } catch (e: any) {
    fail(`Fallo en Heartbeat: ${e.message}`);
  }

  // --- FASE 3: CRYPTO LAB & INTERCEPTOR (NUEVO) ---
  log("3. Validando Módulo Forense (Crypto Lab)...");

  // A. Inyección de Escenario
  const testPhrase = `smoke_test_${Date.now()}`;
  const scenarioName = `AutoTest-${WORKER_ID.substring(0, 6)}`;
  let derivedAddress = "";

  try {
    const createRes = await axios.post(
      `${API_URL}/lab/scenarios`,
      { name: scenarioName, secret_phrase: testPhrase },
      { headers: { Authorization: `Bearer ${AUTH_TOKEN}` } },
    );

    if (createRes.status === 201) {
      derivedAddress = createRes.data.derived_address;
      success(`Escenario Creado: ${scenarioName} -> ${derivedAddress}`);
    } else {
      throw new Error(`Status ${createRes.status}`);
    }
  } catch (e: any) {
    fail(`Fallo creando escenario: ${e.message}`);
  }

  // B. Verificación (The Interceptor)
  try {
    const verifyRes = await axios.post<VerifyResponse>(
      `${API_URL}/lab/verify`,
      { secret: testPhrase, type: "phrase" },
      { headers: { Authorization: `Bearer ${AUTH_TOKEN}` } },
    );

    if (
      verifyRes.data.is_target &&
      verifyRes.data.matched_scenario === scenarioName
    ) {
      success("Interceptor: CONFIRMADO (Match positivo en DB)");
    } else {
      fail("Interceptor: FALLÓ (No detectó el escenario creado)");
    }
  } catch (e: any) {
    fail(`Fallo en verificación: ${e.message}`);
  }

  // --- FASE 4: STREAMING & REAL-TIME ---
  log("4. Verificando Neural Link (SSE)...");

  const es = new EventSource(`${API_URL}/stream/metrics`, {
    headers: { Authorization: `Bearer ${AUTH_TOKEN}` },
  });

  es.onopen = () => {
    success("Canal SSE: ABIERTO");
  };

  es.onmessage = (event: MessageEvent) => {
    try {
      const payload: SsePayload = JSON.parse(event.data);
      if (payload.event === "Metrics") {
        const metrics = payload.data as SystemMetrics;
        // Verificamos que nuestro worker simulado (Fase 2) esté contando
        if (metrics.active_nodes > 0) {
          success(`Telemetría Viva: ${metrics.active_nodes} nodos activos.`);
          es.close();

          console.log(
            chalk.bold.green("\n🎉 SISTEMA CERTIFICADO: 100% OPERATIVO\n"),
          );
          process.exit(0);
        }
      }
    } catch (e) {
      /* Ignorar chunks de keep-alive */
    }
  };

  es.onerror = (err: MessageEvent) => {
    // Si falla inmediatamente, es error. Si tarda, es timeout.
    // Solo logueamos si es el inicio.
  };

  setTimeout(() => {
    es.close();
    fail("TIMEOUT: El stream SSE no reportó métricas a tiempo.");
  }, 10000);
}

main();
