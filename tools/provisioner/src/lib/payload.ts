import { config } from '../config';

export function generateMinerPayload(workerId: string): string {
  // Script Python resiliente con auto-reinicio
  return `
# --- PROSPECTOR PAYLOAD v3.0 ---
import os
import subprocess
import time
import sys
import shutil

# CONFIGURACIÓN
BINARY_URL = "${config.MINER_BINARY_URL}"
ORCH_URL = "${config.ORCHESTRATOR_URL}"
TOKEN = "${config.WORKER_AUTH_TOKEN}"
WORKER_ID = "${workerId}"
BIN_NAME = "prospector-miner"

def log(msg):
    print(f"[{WORKER_ID}] {msg}", flush=True)

def setup():
    if os.path.exists(BIN_NAME):
        os.remove(BIN_NAME)

    log(f"⬇️ Downloading miner from {BINARY_URL}...")
    try:
        # Usamos curl por robustez en entornos Linux minimales
        subprocess.check_call(["curl", "-L", "-o", BIN_NAME, BINARY_URL])
        subprocess.check_call(["chmod", "+x", BIN_NAME])
        log("✅ Binary installed and executable.")
    except Exception as e:
        log(f"❌ Setup Failed: {e}")
        sys.exit(1)

def loop():
    cmd = [
        f"./{BIN_NAME}",
        f"--orchestrator-url={ORCH_URL}",
        f"--auth-token={TOKEN}",
        f"--worker-id={WORKER_ID}"
    ]

    while True:
        log("🚀 Starting Miner Process...")
        try:
            # Popen permite streaming de stdout en tiempo real
            proc = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                universal_newlines=True
            )

            # Bridge de logs: Python -> Colab Output -> Playwright
            for line in proc.stdout:
                print(line.strip(), flush=True)

            proc.wait()
            log(f"⚠️ Process exited with code {proc.returncode}. Respawning in 5s...")
            time.sleep(5)

        except Exception as e:
            log(f"💀 Critical Error: {e}")
            time.sleep(10)

if __name__ == "__main__":
    setup()
    loop()
`;
}
