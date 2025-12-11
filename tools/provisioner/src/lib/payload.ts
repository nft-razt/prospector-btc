// tools/provisioner/src/lib/payload.ts
import { config } from '../config';

export function generateMinerPayload(workerId: string): string {
  // Python Supervisor V3: Smart Caching & Log Tunneling
  return `
import os
import subprocess
import time
import sys
import hashlib
import urllib.request

# --- CONFIGURATION ---
BINARY_URL = "${config.MINER_BINARY_URL}"
ORCH_URL = "${config.ORCHESTRATOR_URL}"
TOKEN = "${config.WORKER_AUTH_TOKEN}"
WORKER_ID = "${workerId}"
BIN_NAME = "prospector-miner"

# --- UTILS ---
def log(msg):
    # Prefijo especial para que el Provisioner TS lo detecte
    print(f"[SUPERVISOR:{WORKER_ID}] {msg}", flush=True)

def download_file(url, filename):
    try:
        log(f"⬇️ Downloading {filename}...")
        urllib.request.urlretrieve(url, filename)
        os.chmod(filename, 0o755)
        return True
    except Exception as e:
        log(f"❌ Download Failed: {e}")
        return False

def setup():
    # 1. Check consistency
    if os.path.exists(BIN_NAME):
        # En una versión futura, verificaríamos SHA256 aquí.
        # Por ahora, asumimos que si existe y es ejecutable, sirve.
        if os.access(BIN_NAME, os.X_OK):
            log("✅ Binary cached and executable.")
            return

    # 2. Download if missing
    if not download_file(BINARY_URL, BIN_NAME):
        sys.exit(1)

def loop():
    cmd = [
        f"./{BIN_NAME}",
        f"--orchestrator-url={ORCH_URL}",
        f"--auth-token={TOKEN}",
        f"--worker-id={WORKER_ID}"
    ]

    backoff = 1

    while True:
        log("🚀 Launching Miner Process...")
        try:
            # Popen con pipes para capturar salida en tiempo real
            proc = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                universal_newlines=True,
                bufsize=1 # Line buffered
            )

            # Tunneling de logs: Miner -> Python -> Colab Stdout -> Playwright -> Orchestrator
            for line in proc.stdout:
                line = line.strip()
                if line:
                    print(line, flush=True)

            return_code = proc.wait()

            log(f"⚠️ Process died (Code: {return_code}). Restarting in {backoff}s...")
            time.sleep(backoff)
            backoff = min(backoff * 2, 60) # Backoff exponencial hasta 60s

        except Exception as e:
            log(f"💀 Critical Supervisor Error: {e}")
            time.sleep(10)

if __name__ == "__main__":
    log("🔥 System Init")
    setup()
    loop()
`;
}
